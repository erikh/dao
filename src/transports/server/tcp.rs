use crate::protocol::{Instruction, Response};
use anyhow::{anyhow, Result};
use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::mpsc::Receiver as SyncReceiver;
use std::sync::{mpsc::SyncSender, Arc, Mutex};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream as AsyncTcpStream,
    sync::mpsc::{Receiver, Sender},
};

fn read_command(mut input: String) -> Result<(Option<Instruction>, String)> {
    let mut res = None;
    if !input.is_empty() {
        let lines = input.trim().split('\n').collect::<Vec<&str>>();
        if !lines.is_empty() {
            let this_line = lines[0];
            res = Some(this_line.parse()?);
            if lines.len() > 1 {
                input = lines[1..].join("\n").to_string()
            } else {
                input = String::new();
            }
        }
    }

    return Ok((res, input));
}

pub struct AsyncTcpServer {
    io: AsyncTcpStream,
    buf: String,
}

impl AsyncTcpServer {
    pub fn new(io: AsyncTcpStream) -> Self {
        Self {
            io,
            buf: Default::default(),
        }
    }

    pub async fn run(
        &mut self,
        s: Sender<Instruction>,
        r: Arc<tokio::sync::Mutex<Receiver<Response>>>,
        mut c: Receiver<()>,
    ) -> Result<()> {
        'outer: loop {
            if let Ok(_) = c.try_recv() {
                return Ok(());
            }

            let (i, buf) = read_command(self.buf.clone())?;

            self.buf = buf;

            if let Some(i) = i {
                s.send(i).await?;
            } else {
                let mut out = [0_u8; 4096];
                let start = std::time::Instant::now();

                'retry: loop {
                    if std::time::Instant::now() - start > std::time::Duration::new(0, 100) {
                        continue 'outer;
                    }

                    match self.io.read(&mut out).await {
                        Ok(sz) => {
                            if sz > 0 {
                                let out = String::from_utf8(out[..sz].to_vec())?;
                                let (i, buf) = read_command(self.buf.clone() + &out)?;
                                self.buf = buf;

                                if let Some(i) = i {
                                    s.send(i).await?;
                                    break 'retry;
                                }
                            }
                        }
                        Err(e) => match e.kind() {
                            std::io::ErrorKind::WouldBlock => {
                                continue 'retry;
                            }
                            _ => return Err(anyhow!("could not read: {:?}", e)),
                        },
                    }
                }
            }

            let mut buf = r
                .lock()
                .await
                .recv()
                .await
                .unwrap()
                .to_string()
                .as_bytes()
                .to_vec();
            buf.push(b'\n');

            self.io.write_all(&buf).await?;
        }
    }
}

pub struct TcpServer {
    io: TcpStream,
    buf: String,
}

impl TcpServer {
    pub fn new(io: TcpStream) -> Self {
        io.set_nonblocking(true)
            .expect("Failed to set non-blocking mode on tcp stream");

        Self {
            io,
            buf: Default::default(),
        }
    }

    pub fn run(
        &mut self,
        s: SyncSender<Instruction>,
        r: Arc<Mutex<SyncReceiver<Response>>>,
        c: SyncReceiver<()>,
    ) -> Result<()> {
        'outer: loop {
            if let Ok(_) = c.recv_timeout(std::time::Duration::new(0, 100)) {
                return Ok(());
            }

            let (i, buf) = read_command(self.buf.clone())?;

            self.buf = buf;

            if let Some(i) = i {
                s.send(i)?;
            } else {
                let mut out = [0_u8; 4096];
                let start = std::time::Instant::now();

                'retry: loop {
                    if std::time::Instant::now() - start > std::time::Duration::new(0, 100) {
                        continue 'outer;
                    }

                    match self.io.read(&mut out) {
                        Ok(sz) => {
                            if sz > 0 {
                                let out = String::from_utf8(out[..sz].to_vec())?;
                                let (i, buf) = read_command(self.buf.clone() + &out)?;
                                self.buf = buf;

                                if let Some(i) = i {
                                    s.send(i)?;
                                    break 'retry;
                                }
                            }
                        }
                        Err(e) => match e.kind() {
                            std::io::ErrorKind::WouldBlock => {
                                continue 'retry;
                            }
                            _ => return Err(anyhow!("could not read: {:?}", e)),
                        },
                    }
                }
            }
            let mut buf = r.lock().unwrap().recv()?.to_string().as_bytes().to_vec();
            buf.push(b'\n');

            self.io.write_all(&buf)?;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testdata::*;
    use std::io::{Read, Write};

    #[test]
    fn test_tcp_sync() -> Result<()> {
        use std::net::{TcpListener, TcpStream};

        let server = TcpListener::bind("localhost:0")?;
        let addr = server.local_addr()?;
        let (ins_s, ins_r) = std::sync::mpsc::sync_channel(1000);
        let (resp_s, resp_r) = std::sync::mpsc::sync_channel(1000);
        let (close_s, close_r) = std::sync::mpsc::sync_channel(1);

        let resp_r = Arc::new(Mutex::new(resp_r));

        let handle = std::thread::spawn(move || {
            server.set_ttl(10).unwrap();

            let (sock, _) = server.accept().unwrap();
            let mut server = TcpServer::new(sock);
            server.run(ins_s.clone(), resp_r.clone(), close_r).unwrap();
        });

        let mut stream = TcpStream::connect(addr)?;
        for (
            input,
            input_result,
            input_annotation,
            response,
            response_result,
            response_annotation,
        ) in &*GREEN_TABLE
        {
            stream.write_all(input.as_bytes())?;

            assert_eq!(ins_r.recv()?, *input_result, "{}", input_annotation);
            resp_s.send(response.clone())?;
            let mut buf = [0_u8; 4096];

            let start = std::time::Instant::now();

            'retry: loop {
                match stream.read(&mut buf) {
                    Ok(sz) => {
                        if sz > 0 {
                            assert_eq!(
                                String::from_utf8(buf[..sz].to_vec())?.trim().to_string(),
                                *response_result,
                                "{}",
                                response_annotation
                            );

                            break 'retry;
                        } else {
                            if std::time::Instant::now() - start > std::time::Duration::new(0, 100)
                            {
                                return Err(anyhow!("timed out: {}", response_annotation));
                            }
                        }
                    }
                    Err(e) => match e.kind() {
                        std::io::ErrorKind::WouldBlock => {
                            if std::time::Instant::now() - start > std::time::Duration::new(0, 100)
                            {
                                return Err(anyhow!("timed out: {}", response_annotation));
                            }

                            continue 'retry;
                        }
                        _ => return Err(anyhow!("could not read: {:?}", e)),
                    },
                }
            }
        }

        close_s.send(())?;
        drop(stream);

        handle.join().unwrap();

        for (input, annotation) in &*RED_TABLE {
            let server = TcpListener::bind("localhost:0")?;
            let addr = server.local_addr()?;
            let (ins_s, ins_r) = std::sync::mpsc::sync_channel(1000);
            let (_, resp_r) = std::sync::mpsc::sync_channel(1000);
            let (_, close_r) = std::sync::mpsc::sync_channel(1);

            let resp_r = Arc::new(Mutex::new(resp_r));

            let handle = std::thread::spawn(move || {
                server.set_ttl(10).unwrap();

                let (sock, _) = server.accept().unwrap();
                let mut server = TcpServer::new(sock);
                server.run(ins_s.clone(), resp_r.clone(), close_r)
            });

            let mut stream = TcpStream::connect(addr)?;
            stream.write_all(input.as_bytes())?;
            assert!(ins_r.recv().is_err(), "{}", annotation);
            assert!(handle.join().is_ok());
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_tcp_async() -> Result<()> {
        use tokio::net::TcpListener as AsyncTcpListener;

        let server = AsyncTcpListener::bind("localhost:0").await?;
        let addr = server.local_addr()?;
        let (ins_s, mut ins_r) = tokio::sync::mpsc::channel(1000);
        let (resp_s, resp_r) = tokio::sync::mpsc::channel(1000);
        let (close_s, close_r) = tokio::sync::mpsc::channel(1);

        let resp_r = Arc::new(tokio::sync::Mutex::new(resp_r));

        tokio::spawn(async move {
            server.set_ttl(10).unwrap();

            let (sock, _) = server.accept().await.unwrap();
            let mut server = AsyncTcpServer::new(sock);
            server
                .run(ins_s.clone(), resp_r.clone(), close_r)
                .await
                .unwrap();
        });

        let mut stream = AsyncTcpStream::connect(addr).await?;
        for (
            input,
            input_result,
            input_annotation,
            response,
            response_result,
            response_annotation,
        ) in &*GREEN_TABLE
        {
            stream.write_all(input.as_bytes()).await?;

            assert_eq!(
                ins_r.recv().await.unwrap(),
                *input_result,
                "{}",
                input_annotation
            );
            resp_s.send(response.clone()).await?;
            let mut buf = [0_u8; 4096];

            let start = std::time::Instant::now();

            'retry: loop {
                match stream.read(&mut buf).await {
                    Ok(sz) => {
                        if sz > 0 {
                            assert_eq!(
                                String::from_utf8(buf[..sz].to_vec())?.trim().to_string(),
                                *response_result,
                                "{}",
                                response_annotation
                            );

                            break 'retry;
                        } else {
                            if std::time::Instant::now() - start > std::time::Duration::new(0, 100)
                            {
                                return Err(anyhow!("timed out: {}", response_annotation));
                            }
                        }
                    }
                    Err(e) => match e.kind() {
                        std::io::ErrorKind::WouldBlock => {
                            if std::time::Instant::now() - start > std::time::Duration::new(0, 100)
                            {
                                return Err(anyhow!("timed out: {}", response_annotation));
                            }

                            continue 'retry;
                        }
                        _ => return Err(anyhow!("could not read: {:?}", e)),
                    },
                }
            }
        }

        close_s.send(()).await?;
        drop(stream);

        for (input, annotation) in &*RED_TABLE {
            let server = AsyncTcpListener::bind("localhost:0").await?;
            let addr = server.local_addr()?;
            let (ins_s, mut ins_r) = tokio::sync::mpsc::channel(1000);
            let (_, resp_r) = tokio::sync::mpsc::channel(1000);
            let (_, close_r) = tokio::sync::mpsc::channel(1);

            let resp_r = Arc::new(tokio::sync::Mutex::new(resp_r));

            tokio::spawn(async move {
                server.set_ttl(10).unwrap();

                let (sock, _) = server.accept().await.unwrap();
                let mut server = AsyncTcpServer::new(sock);
                server
                    .run(ins_s.clone(), resp_r.clone(), close_r)
                    .await
                    .unwrap();
            });

            let mut stream = AsyncTcpStream::connect(addr).await?;
            stream.write_all(input.as_bytes()).await?;
            assert!(ins_r.try_recv().is_err(), "{}", annotation);
        }

        Ok(())
    }
}

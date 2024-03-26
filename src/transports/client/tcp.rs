use super::*;
use anyhow::anyhow;
use std::io::prelude::*;
use std::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream as AsyncTcpStream;

pub struct TcpClient {
    io: TcpStream,
}

impl TcpClient {
    pub fn new(io: TcpStream) -> Self {
        Self { io }
    }

    pub fn close(&mut self) -> Result<()> {
        Ok(self.io.shutdown(std::net::Shutdown::Both)?)
    }
}

impl Client for TcpClient {
    fn exchange(&mut self, instruction: Instruction) -> Result<Response> {
        let mut buf = instruction.to_string().as_bytes().to_vec();
        buf.push(b'\n');
        self.io.write_all(&buf)?;

        let mut buf = [0_u8; 4096];
        let mut res = String::new();

        'retry: loop {
            self.io
                .set_read_timeout(Some(std::time::Duration::new(0, 100)))?;
            match self.io.read(&mut buf) {
                Ok(sz) => {
                    if sz > 0 {
                        res += &String::from_utf8(buf[..sz].to_vec())?;
                    }

                    match serde_json::from_str(&res) {
                        Ok(res) => return Ok(res),
                        Err(_) => continue 'retry,
                    }
                }
                Err(e) => match e.kind() {
                    std::io::ErrorKind::WouldBlock => continue 'retry,
                    _ => return Err(e.into()),
                },
            }
        }
    }
}

pub struct AsyncTcpClient {
    io: AsyncTcpStream,
}

impl AsyncTcpClient {
    pub fn new(io: AsyncTcpStream) -> Self {
        Self { io }
    }

    pub async fn close(&mut self) -> Result<()> {
        Ok(self.io.shutdown().await?)
    }
}

#[async_trait::async_trait]
impl AsyncClient for AsyncTcpClient {
    async fn exchange(&mut self, instruction: Instruction) -> Result<Response> {
        let mut buf = instruction.to_string().as_bytes().to_vec();
        buf.push(b'\n');
        self.io.write_all(&buf).await?;

        let mut buf = [0_u8; 4096];
        let mut res = String::new();

        let start = std::time::Instant::now();

        'retry: loop {
            match self.io.read(&mut buf).await {
                Ok(sz) => {
                    if sz > 0 {
                        res += &String::from_utf8(buf[..sz].to_vec())?;
                    } else {
                        if std::time::Instant::now() - start > std::time::Duration::new(0, 100) {
                            return Err(anyhow!("timed out"));
                        }
                    }

                    match serde_json::from_str(&res) {
                        Ok(res) => return Ok(res),
                        Err(_) => continue 'retry,
                    }
                }
                Err(e) => match e.kind() {
                    std::io::ErrorKind::WouldBlock => {
                        if std::time::Instant::now() - start > std::time::Duration::new(0, 100) {
                            return Err(anyhow!("timed out"));
                        }
                        continue 'retry;
                    }
                    _ => return Err(e.into()),
                },
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testdata::*;
    use crate::transports::server::tcp::*;
    use std::sync::Arc;

    #[test]
    fn test_client() -> Result<()> {
        use std::net::TcpListener;
        use std::sync::Mutex;

        let server = TcpListener::bind("localhost:0")?;
        let addr = server.local_addr()?;
        let (ins_s, ins_r) = std::sync::mpsc::sync_channel(1000);
        let (resp_s, resp_r) = std::sync::mpsc::sync_channel(1000);
        let (close_s, close_r) = std::sync::mpsc::sync_channel(1);

        let ins_r = Arc::new(Mutex::new(ins_r));
        let resp_r = Arc::new(Mutex::new(resp_r));

        std::thread::spawn(move || -> Result<()> {
            let (sock, _) = server.accept()?;
            let mut server = TcpServer::new(sock);
            server.run(ins_s.clone(), resp_r.clone(), close_r)
        });

        let mut client = TcpClient::new(TcpStream::connect(addr)?);

        for (
            _input,
            input_result,
            input_annotation,
            response,
            _response_result,
            _response_annotation,
        ) in &*GREEN_TABLE
        {
            let s = resp_s.clone();
            let r = ins_r.clone();
            std::thread::spawn(move || -> Result<()> {
                r.lock().unwrap().recv()?;
                s.send(response.clone())?;
                Ok(())
            });

            assert_eq!(
                client.exchange(input_result.clone())?,
                response.clone(),
                "{}",
                input_annotation
            );
        }

        client.close()?;
        close_s.send(())?;
        Ok(())
    }

    #[tokio::test]
    async fn test_client_async() -> Result<()> {
        use tokio::net::TcpListener;
        use tokio::sync::Mutex;

        let server = TcpListener::bind("localhost:0").await?;
        let addr = server.local_addr()?;
        let (ins_s, ins_r) = tokio::sync::mpsc::channel(1000);
        let (resp_s, resp_r) = tokio::sync::mpsc::channel(1000);
        let (close_s, close_r) = tokio::sync::mpsc::channel(1);

        let ins_r = Arc::new(Mutex::new(ins_r));
        let resp_r = Arc::new(Mutex::new(resp_r));

        tokio::spawn(async move {
            let (sock, _) = server.accept().await.unwrap();
            let mut server = AsyncTcpServer::new(sock);
            server
                .run(ins_s.clone(), resp_r.clone(), close_r)
                .await
                .unwrap();
        });

        let mut client = AsyncTcpClient::new(AsyncTcpStream::connect(addr).await?);

        for (
            _input,
            input_result,
            input_annotation,
            response,
            _response_result,
            _response_annotation,
        ) in &*GREEN_TABLE
        {
            let s = resp_s.clone();
            let r = ins_r.clone();
            tokio::spawn(async move {
                r.lock().await.recv().await;
                s.send(response.clone()).await.unwrap();
            });

            assert_eq!(
                client.exchange(input_result.clone()).await?,
                response.clone(),
                "{}",
                input_annotation
            );
        }

        client.close().await?;
        close_s.send(()).await?;
        Ok(())
    }
}

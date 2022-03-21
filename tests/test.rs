#![feature(async_closure)]
use anyhow::Result;
use tokio::io::AsyncReadExt;
use hashcash::Stamp;
use std::str;
use tokio::io::AsyncWriteExt;
use std::collections::HashSet;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref HASHSET: HashSet<&'static str> = {
        let mut m = HashSet::new();
        m.insert("Quote One");
        m.insert("Quote Two");
        m.insert("Quote Three");
        m
    };
    static ref REQUEST_LINE: &'static str = "REQUEST";
}

#[tokio::test]
async fn smoke() -> Result<()> {
    let mut tcp_stream = tokio::net::TcpStream::connect("0.0.0.0:5555").await?;
    let mut read = [0; 4096];
    tcp_stream.write(&REQUEST_LINE.as_bytes()).await?;
    let mut len = tcp_stream.read(&mut read).await?;
    let resource = str::from_utf8(&read[..len])?;
    let s = Stamp::mint(
        Some(&resource),
        None,
        None,
        None,
        None,
        false,
    )?;
    tcp_stream.write(&s.to_string().as_bytes()).await?;
    len = tcp_stream.read(&mut read).await?;
    let quote = str::from_utf8(&read[..len])?;
    assert_ne!(HASHSET.get(quote), None);
    Ok(())
}

#[tokio::test]
async fn wrong_stamp() -> Result<()> {
    let mut tcp_stream = tokio::net::TcpStream::connect("0.0.0.0:5555").await?;
    let mut read = [0; 4096];
    tcp_stream.write(&REQUEST_LINE.as_bytes()).await?;
    let mut len = tcp_stream.read(&mut read).await?;
    let _ = str::from_utf8(&read[..len])?;
    let s = Stamp::default();
    tcp_stream.write(&s.to_string().as_bytes()).await?;
    len = tcp_stream.read(&mut read).await?;
    let quote = str::from_utf8(&read[..len])?;
    assert_eq!(HASHSET.get(quote), None);
    Ok(())
}

#[tokio::test]
async fn wrong_request() -> Result<()> {
    let mut tcp_stream = tokio::net::TcpStream::connect("0.0.0.0:5555").await?;
    let mut read = [0; 4096];
    let req = "WRONG REQUEST";
    tcp_stream.write(&req.as_bytes()).await?;
    let len = tcp_stream.read(&mut read).await?;
    assert_eq!(len, 0);
    Ok(())
}

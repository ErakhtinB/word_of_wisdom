#![feature(async_closure)]
use anyhow::Result;
use std::sync::Arc;
use tcpserver::{Builder, IPeer, ITCPServer};
use tokio::io::AsyncReadExt;
use hashcash::check_with_params;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use std::str;

const QUOTES: &'static [&'static str] = &[
    "Quote One",
    "Quote Two",
    "Quote Three",
];

const REQUEST_LINE: &str = "REQUEST";

fn get_random_quote() -> &'static str {
    return QUOTES[thread_rng().gen_range(0..(QUOTES.len() - 1))];
}

#[tokio::main]
async fn main() -> Result<()> {
    let tcpserver: Arc<dyn ITCPServer<()>> = Builder::new("0.0.0.0:5555")
        .set_stream_init(async move |tcp_stream| {
            Ok(tcp_stream)
        })
        .set_input_event(async move |mut reader, peer, _token| {
            let mut buff = [0; 4096];
            let mut on_verification = false;
            let mut rand_string = String::new();
            while let Ok(len) = reader.read(&mut buff).await {
                if len == 0 {
                    break;
                }
                if !on_verification {
                    if REQUEST_LINE.as_bytes().eq(&buff[..len]) {
                        rand_string = thread_rng()
                        .sample_iter(&Alphanumeric)
                        .take(8)
                        .map(char::from)
                        .collect();
                        peer.send_ref(rand_string.as_bytes()).await?;
                        on_verification = true;
                    }
                    else {
                        break;
                    }
                } else {
                    match check_with_params(&str::from_utf8(&buff[..len].to_vec()).unwrap(),
                        Some(&rand_string), None, None) {
                        Ok(valid) => {
                            if valid {
                                peer.send_ref(&get_random_quote().as_bytes()).await?;
                            } else {
                                println!("Stamp invalid for address {}", peer.addr());
                            }
                        }
                        Err(e) => {
                            println!("Checking error {} for {}", e, peer.addr());
                        }
                    }
                    break;
                }
            }
            Ok(())
        })
        .build()
        .await;
    tcpserver.start_block(()).await?;
    Ok(())
}

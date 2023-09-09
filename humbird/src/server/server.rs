use crate::config::config::*;

use regex::Regex;
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    net::{
        tcp::{OwnedReadHalf, OwnedWriteHalf},
        TcpListener,
    },
    task,
};
use tracing::{info, instrument};

use crate::protocol::http::http::*;

pub struct Server {}

impl Server {
    pub async fn start() -> Result<(), Box<dyn std::error::Error>> {
        // tcp listener
        let l = TcpListener::bind(format!("{}:{}", SERVER_LISTENING_ADDR, unsafe {
            SERVER_LISTENING_PORT
        }))
        .await?;
        loop {
            let (stream, socket) = l.accept().await?;
            info!("new visitor,ip:{}", socket.ip());
            let (r, w) = stream.into_split();
            task::spawn(handle_tcp(r, w));
        }
    }
}

/// handle tcp message
#[instrument]
async fn handle_tcp(r: OwnedReadHalf, w: OwnedWriteHalf) {
    let mut req_str_buf = String::new();
    let mut r_buf: BufReader<OwnedReadHalf> = BufReader::new(r);
    loop {
        match r_buf.read_line(&mut req_str_buf).await {
            Ok(0) => {
                break;
            }
            Ok(_n) => {
                let c = req_str_buf.drain(..).as_str().to_string();
                if is_http_protocol(c.clone()) {
                    // build http
                    let http = Http::new(c, r_buf, w).await;
                    // respose
                    task::spawn(http.response());
                    break;
                } else if c.eq("\r\n") {
                    break;
                } else {
                    break;
                }
            }
            Err(_) => {}
        }
    }
}

/// is http protocol
fn is_http_protocol(c: String) -> bool {
    let re = Regex::new(r"^(GET|HEAD|POST|PUT|DELETE|CONNECT|OPTIONS|TRACE)\s(([/0-9a-zA-Z.]+)?(\?[0-9a-zA-Z&=]+)?)\s(HTTP/1.0|HTTP/1.1|HTTP/2.0)\r\n$").unwrap();
    re.is_match(&c)
}

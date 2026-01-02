use futures::io::BufReader;
use futures::AsyncBufRead;
use std::pin::Pin;

// This function belongs to a different crate to talk to k8s API
pub async fn get_logs() -> anyhow::Result<impl AsyncBufRead> {
    Ok(BufReader::new("Hello, world!".as_bytes()))
}

// The trait to get logs from a backend that has a k8s cluster
trait Loger {
    type T: AsyncBufRead;

    async fn get_log_buffer() -> anyhow::Result<Self::T>;
}

// K8s backend
struct Backend1 {}

impl Loger for Backend1 {
    type T = Pin<Box<dyn AsyncBufRead>>;

    async fn get_log_buffer() -> anyhow::Result<Self::T> {
        Ok(Box::pin(get_logs().await?))
    }
}

// Journalctl backend
struct Backend2 {}

impl Loger for Backend2 {
    type T = Pin<Box<dyn AsyncBufRead>>;

    async fn get_log_buffer() -> anyhow::Result<Self::T> {
        Ok(Box::pin(get_logs().await?))
    }
}

// Function to get the logs from any backend
pub async fn get_system_logs(backend_type: &str) -> anyhow::Result<impl AsyncBufRead> {
    match backend_type {
        "backend1" => Backend1::get_log_buffer().await,
        "backend2" => Backend2::get_log_buffer().await,
        &_ => panic!("Not supported"),
    }
}

pub fn main() {
    let backend_type: &str = "backend1";

    async { get_system_logs(backend_type).await };
}

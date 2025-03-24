mod database;
mod models;
mod sped;

use encoding_rs::UTF_8;
use sped::handle_line;
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, BufReader};

#[tokio::main]
async fn main() {
    database::setup_database(false).await;

    let file_path = "...";
    let file = File::open(file_path).await.expect("Failed to open file");
    let mut reader = BufReader::new(file);
    let mut buffer = Vec::new();

    loop {
        buffer.clear();

        let bytes_read = reader
            .read_until(b'\n', &mut buffer)
            .await
            .expect("Failed to read line");

        if bytes_read == 0 {
            break;
        }

        if let Some(&b'\n') = buffer.last() {
            buffer.pop();
        }

        if let Some(&b'\r') = buffer.last() {
            buffer.pop();
        }

        let (line, _, _) = UTF_8.decode(&buffer);

        handle_line(&line);
    }
}

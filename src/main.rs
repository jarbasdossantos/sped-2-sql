mod database;
mod models;
mod sped;

use encoding_rs::UTF_8;
use sped::handle_line;
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, BufReader};

#[tokio::main]
async fn main() {
    let db = database::setup_database(false).await;

    let file_path = "/home/jarbassantos/Downloads/PISCOFINS_20200301_20200331_12662352000191_Original_20200511170225_B3F563D22F2B80545475CB9D5497EEBA47902FC9.txt";
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

        handle_line(&line, &db);
    }
}

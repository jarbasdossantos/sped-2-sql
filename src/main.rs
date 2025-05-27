use sped_to_database::{self, Export, Load, LoadData};

#[tokio::main]
async fn main() {
    let _ = sped_to_database::clean().await;

    let _ = sped_to_database::load(Load {
        files: vec![
            // "/Users/jarbassantos/Downloads/Caio/ORIGINAIS/19(12).txt".to_string(),
            // "/Users/jarbassantos/Downloads/Caio/ORIGINAIS/20(1).txt".to_string(),
            LoadData {
                file: "/Users/jarbassantos/Downloads/PISCOFINS_20200601_20200630_12662352000191_Original_20200812091255_D08544F39B95698618D6E66F9FACB1BDB7768BD0.txt".to_string(),
                registers: None,
                // registers: Some(vec![
                //     "0000".to_string(),
                //     "0001".to_string(),
                //     "0140".to_string(),
                //     "0150".to_string(),
                //     "0190".to_string(),
                //     "0200".to_string(),
                //     "0500".to_string(),
                //     "0400".to_string(),
                //     "C100".to_string(),
                //     "C170".to_string(),
                //     "C150".to_string(),
                // ]),
            },
        ],
    })
    .await;

    for id in vec![1] {
        println!("Exporting file {}", id);

        let data = match sped_to_database::export(Export {
            id,
            registers: None,
            // registers: Some(vec!["0001".to_string()]),
        })
        .await
        {
            Ok(data) => data,
            Err(e) => {
                println!("{}", e);
                continue;
            }
        };

        for reg in data {
            print!("{}", reg);
        }
    }
}

use sped_to_database::{self, Export, Load, LoadData};

#[tokio::main]
async fn main() {
    let _ = sped_to_database::load(Load {
        files: vec![
            // "/Users/jarbassantos/Downloads/Caio/ORIGINAIS/19(12).txt".to_string(),
            // "/Users/jarbassantos/Downloads/Caio/ORIGINAIS/20(1).txt".to_string(),
            LoadData {
                file: "/Users/jarbassantos/Downloads/Caio/ORIGINAIS/20(2).txt".to_string(),
                registers: None, // registers: Some(vec![
                                 //     "0140".to_string(),
                                 //     "0150".to_string(),
                                 //     "0190".to_string(),
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
            // registers: Some(vec![
            //     "0140".to_string(),
            //     "0150".to_string(),
            //     "0190".to_string(),
            // ]),
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
            println!("{:?}", reg.to_line());
        }

        println!("\n");
    }
}

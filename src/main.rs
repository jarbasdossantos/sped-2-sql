use sped_to_database::{self, Export};

#[tokio::main]
async fn main() {
    // let _ = sped_to_database::load(Load {
    //     files: vec![
    //         "/home/jarbassantos/Downloads/Caio/ORIGINAIS/19(12).txt".to_string(),
    //         "/home/jarbassantos/Downloads/Caio/ORIGINAIS/20(1).txt".to_string(),
    //         "/home/jarbassantos/Downloads/Caio/ORIGINAIS/20(2).txt".to_string(),
    //     ],
    // })
    // .await;

    for id in vec![1] {
        let data = match sped_to_database::export(Export { id }).await {
            Ok(data) => data,
            Err(e) => {
                println!("{}", e);
                continue;
            }
        };

        for reg in data {
            println!("{}", reg.to_line());
        }
    }
}

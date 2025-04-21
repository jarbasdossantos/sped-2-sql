use sqlx::sqlite::SqlitePoolOptions;
use tokio::fs::OpenOptions;

pub async fn setup_database(use_memory: bool) -> sqlx::SqlitePool {
    let db_url = if use_memory {
        "sqlite::memory:".to_string()
    } else {
        let mut open_options = OpenOptions::new();
        let result = open_options.create_new(true).write(true).open("db.sqlite").await;

        match result {
            Ok(_) => println!("Database file created"),
            Err(err) => match err.kind() {
                std::io::ErrorKind::AlreadyExists => println!("Database file already exists"),
                _ => {
                    panic!("Error creating database file {}", err);
                }
            },
        }

        format!("{}?mode=rwc&cache=shared", "db.sqlite")
    };

    let db = SqlitePoolOptions::new()
        .max_connections(20) // Ajuste conforme necessidade
        .connect(&db_url)
        .await
        .unwrap();

    // Configurações de performance
    let pragmas = [
        "PRAGMA journal_mode = WAL", // Write-Ahead Logging para melhor concorrência
        "PRAGMA synchronous = NORMAL", // Bom equilíbrio entre performance e segurança
        "PRAGMA cache_size = -64000", // Cache de 64MB (-64000 * 1KB)
        "PRAGMA page_size = 4096",   // Tamanho de página otimizado
        "PRAGMA temp_store = MEMORY", // Usar memória para tabelas temporárias
        "PRAGMA mmap_size = 30000000000", // Usar memory-mapped I/O (30GB)
        "PRAGMA busy_timeout = 5000", // Timeout de 5 segundos para locks
    ];

    for pragma in pragmas {
        sqlx::query(pragma).execute(&db).await.expect("Failed to set pragma");
    }

    sqlx::migrate!("./migrations").run(&db).await.expect("Failed to migrate");

    db
}

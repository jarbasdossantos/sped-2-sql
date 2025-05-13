use anyhow::Result;
use indexmap::IndexMap;
use std::future::Future;
use std::pin::Pin;
use async_trait::async_trait;

/// Trait que define a interface para registros que podem ser salvos no banco de dados.
/// 
/// Esta trait é implementada por estruturas que representam registros da EFD (Escrituração Fiscal Digital)
/// e fornece métodos para manipulação e persistência desses registros.
/// 
/// Os tipos que implementam esta trait devem ser `Debug`, `Send` e `Sync` para garantir
/// compatibilidade com operações assíncronas e logging.
pub trait Reg: std::fmt::Debug + Send + Sync {
    /// Retorna um mapa ordenado contendo os valores dos campos do registro.
    /// 
    /// # Retorno
    /// 
    /// Um `IndexMap` onde as chaves são os nomes dos campos (como strings estáticas)
    /// e os valores são opcionais (`Option<String>`), representando os valores dos campos.
    fn values(&self) -> IndexMap<&'static str, Option<String>>;

    /// Converte o registro para uma representação em formato de linha de texto.
    /// 
    /// Este método formata os valores do registro como uma linha delimitada por pipes (|),
    /// ignorando os três primeiros campos (id, file_id e parent_id).
    /// 
    /// # Retorno
    /// 
    /// Uma `String` contendo a representação do registro em formato de linha.
    #[allow(dead_code)]
    fn to_line(&self) -> String {
        format!(
            "|{}|",
            self.values()
                .iter()
                .skip(3)
                .map(|(_, v)| v.clone().unwrap_or_default())
                .collect::<Vec<_>>()
                .join("|")
        )
    }

    /// Salva o registro no banco de dados SQLite.
    /// 
    /// Este método é assíncrono e retorna um `Future` que, quando executado,
    /// insere o registro no banco de dados.
    /// 
    /// # Retorno
    /// 
    /// Um `Future` que resolve para um `Result` contendo o resultado da operação SQL
    /// ou um erro do SQLx caso a operação falhe.
    fn save<'a>(
        &'a self,
    ) -> Pin<
        Box<dyn Future<Output = Result<sqlx::sqlite::SqliteQueryResult, sqlx::Error>> + Send + 'a>,
    >;
}

/// Trait que define a interface para modelos que podem ser carregados e criados a partir de campos.
/// 
/// Esta trait é implementada por estruturas que representam modelos de dados da EFD
/// e fornece métodos para criar novas instâncias e carregar dados do banco de dados.
/// 
/// A trait utiliza `async_trait` para permitir métodos assíncronos.
#[async_trait]
pub trait Model {
    /// Cria uma nova instância do modelo a partir de campos e metadados.
    /// 
    /// # Parâmetros
    /// 
    /// * `fields` - Um vetor de strings contendo os valores dos campos do registro
    /// * `id` - Identificador opcional do registro no banco de dados
    /// * `parent_id` - Identificador opcional do registro pai
    /// * `file_id` - Identificador do arquivo ao qual o registro pertence
    /// 
    /// # Retorno
    /// 
    /// Uma nova instância do tipo que implementa esta trait.
    fn new(fields: Vec<&str>, id: Option<i64>, parent_id: Option<i64>, file_id: i64) -> Self;
    /// Carrega registros do banco de dados com base no arquivo e registro pai.
    /// 
    /// Este método assíncrono consulta o banco de dados e retorna todos os registros
    /// que correspondem ao arquivo especificado e, opcionalmente, ao registro pai.
    /// 
    /// # Parâmetros
    /// 
    /// * `file_id` - Identificador do arquivo cujos registros devem ser carregados
    /// * `parent_id` - Identificador opcional do registro pai para filtrar os resultados
    /// 
    /// # Retorno
    /// 
    /// Um `Result` contendo um vetor de instâncias do tipo que implementa esta trait,
    /// ou um erro caso a operação falhe.
    async fn load(file_id: i64, parent_id: Option<i64>) -> Result<Vec<Self>, anyhow::Error>
    where
        Self: Sized;
}

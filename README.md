# SPED-2-SQL

Este projeto Rust, é uma ferramenta desenvolvida para processar arquivos SPED (Sistema Público de Escrituração Digital) e persistir seus dados em um banco de dados (Por enquanto somente SQLite). Ele é capaz de lidar com diferentes tipos de SPED, como EFD (Escrituração Fiscal Digital) e ICMS/IPI.

> [!NOTE]
> Desenvolvi essa biblioteca para uso em uma ferramenta de um cliente. Não domino totalmente as particularidades do SPED, o meu trabalho foi em cima da minha necessidade.

> [!WARNING]
> Sou novo no Rust e essa biblioteca faz parte dos meus estudos e esforços para evoluir na linguagem, portanto caso encontre algo que possa ser melhorado, sinta-se à vontade para abrir uma issue ou enviar um pull request.
> A ferramenta ainda está em desenvolvimento, portanto, algumas funcionalidades podem não estar completas ou ainda não estão prontas para uso.

## O que é?

`sped-2-sql` é uma aplicação que automatiza a leitura, parsing e armazenamento de dados contidos em arquivos SPED. Ele utiliza o Diesel ORM para interagir com bancos de dados SQLite e Tokio para operações assíncronas, garantindo alta performance no processamento de grandes volumes de dados.

## Para que serve?

O principal objetivo deste projeto é facilitar a integração e análise de dados fiscais e contábeis.

## Como usar

### Pré-requisitos

Para compilar e executar este projeto, você precisará ter o Rust e o Cargo instalados. Você pode instalá-los através do `rustup`:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Além disso, é necessário ter o `diesel_cli` para gerenciar as migrações do banco de dados:

```bash
cargo install diesel_cli --no-default-features --features "sqlite"
```

### Configuração do Banco de Dados

O projeto utiliza SQLite por padrão. As configurações do banco de dados são definidas no arquivo `diesel.toml` e nas migrações localizadas em `migrations/`.

Para configurar o banco de dados e aplicar as migrações:

1. Certifique-se de que o arquivo `diesel.toml` esteja configurado corretamente para o seu banco de dados SQLite.
2. Execute as migrações para criar as tabelas necessárias:

```bash
diesel migration run
```

### Compilação e Execução

Para compilar e executar o projeto:

```bash
cargo run
```

O projeto inclui um binário `generate_efd_models` que pode ser usado para gerar modelos de dados a partir de esquemas EFD. Para executá-lo:

```bash
cargo run --bin generate_efd_models
```

### Exemplo de Uso

@TODO

### Estrutura do Projeto

- `src/lib.rs`: Contém a lógica de negócios e as funções principais do projeto.
- `src/models/`: Define as estruturas de dados (modelos) para os registros SPED e a interação com o banco de dados via Diesel.
- `src/schemas/`: Contém os esquemas para os diferentes tipos de registros SPED.
- `migrations/`: Contém os scripts SQL para as migrações do banco de dados.
- `scripts/generate_efd_models.rs`: Script auxiliar para geração de modelos EFD.

## Dependências Principais

- `tokio`: Runtime assíncrono para operações I/O.
- `diesel`: ORM para interação com o banco de dados.
- `serde`: Framework de serialização/desserialização para Rust.
- `anyhow`: Biblioteca para tratamento de erros.
- `indexmap`: Implementação de `HashMap` e `BTreeMap` que preserva a ordem de inserção.
- `async-trait`: Permite o uso de `async` em traits.
- `async-recursion`: Permite funções recursivas assíncronas.
----
[![Ask DeepWiki](https://deepwiki.com/badge.svg)](https://deepwiki.com/jarbasdossantos/sped-2-sql)
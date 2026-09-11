```markdown
# Projeto Gerenciador de Tarefas com SQLite

Este projeto fornece uma ferramenta simples para gerenciar tarefas usando um banco de dados SQLite. Ele permite adicionar, listar, marcar como concluída, verificar e excluir tarefas.

## Funcionalidades

*   **Adicionar Tarefa:** Adiciona uma nova tarefa ao banco de dados.
*   **Listar Tarefas:** Exibe todas as tarefas armazenadas no banco de dados.
*   **Marcar como Concluída:** Marca uma tarefa existente como concluída.
*   **Excluir Tarefa:** Remove uma tarefa do banco de dados.

## Pré-requisitos

*   **Rust:**  É necessário ter o Rust instalado em seu sistema. Você pode encontrar as instruções de instalação em [https://www.rust-lang.org/](https://www.rust-lang.org/).
*   **Cargo:** Cargo é o gerenciador de pacotes do Rust e vem com o Rust.

## Executando o Projeto

1.  **Clone o Repositório:** Clone o repositório do GitHub para sua máquina local.

    ```bash
    git clone [URL do Repositório GitHub]
    cd [Nome do Diretório do Repositório]
    ```

2.  **Execute o Projeto:** Navegue até o diretório do projeto e execute o seguinte comando:

    ```bash
    cargo run
    ```

## Como Usar

O projeto usa a biblioteca `clap` para receber argumentos da linha de comando.

### Adicionar uma Tarefa

```bash
cargo run -- add "Comprar pão"
```

### Listar Todas as Tarefas

```bash
cargo run -- list
```

### Marcar uma Tarefa como Concluída

```bash
cargo run -- check 1
```

### Excluir uma Tarefa

```bash
cargo run -- delete 1
```

## Estrutura do Projeto

*   `database.rs`: Contém as funções para interagir com o banco de dados SQLite (criação da tabela, adição, listagem, marcação como concluída e exclusão de tarefas).
*   `main.rs`:  Ponto de entrada do programa, utiliza a biblioteca `clap` para receber os argumentos da linha de comando e chama as funções correspondentes do módulo `database`.
*   `task.rs`: Define a estrutura `ModelTask` que representa uma tarefa no banco de dados.
*   `Cargo.toml`: Arquivo de configuração do Cargo que define as dependências do projeto.

## Dependências

*   `rusqlite`: Para interagir com o banco de dados SQLite.
*   `clap`: Para processar argumentos da linha de comando.

## Considerações

*   O banco de dados `data.db` será criado no mesmo diretório do executável.
*   O programa usa IDs auto-incrementais para as tarefas.
*   A validação dos dados (por exemplo, verificar se o ID é um número válido) pode ser melhorada.
*   A implementação do tratamento de erros poderia ser expandida para fornecer mensagens de erro mais informativas.
*   Este projeto é um exemplo básico e pode ser expandido com funcionalidades adicionais (por exemplo, adicionar prioridade, data de vencimento, etc.).

## Contribuindo

Se você encontrar algum problema ou tiver alguma sugestão para melhorar o projeto, sinta-se à vontade para abrir um pull request no repositório do GitHub.


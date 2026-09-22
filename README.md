# 🚀 Task List CLI em Rust

Uma aplicação de linha de comando (CLI) para gerenciamento de tarefas (*Task Manager*), desenvolvida em **Rust** com persistência em banco de dados **SQLite** (`rusqlite`). 

O projeto foi construído focando em **boas práticas de arquitetura**, separação clara de responsabilidades, tratamento de erros idiomático e **suíte de testes unitários automatizados com banco em memória**.

---

## 🛠️ Tecnologias e Ferramentas

* **Linguagem:** [Rust](https://www.rust-lang.org/) (Edição 2021)
* **Banco de Dados:** [SQLite](https://www.sqlite.org/) via crate [`rusqlite`](https://crates.io/crates/rusqlite)
* **Testes:** Framework nativo de testes do Rust (`#[test]`) com SQLite In-Memory

---

## 🏗️ Arquitetura do Projeto

O código é organizado em camadas para garantir testabilidade, facilidade de manutenção e desacoplamento:

```text
src/
├── database.rs          # Inicialização e schemas de tabelas no SQLite
├── errors/              # Erros customizados do domínio (TaskError)
├── models/              # Estruturas de dados/entidades (ModelTask)
├── repos/               # Camada de repositório (queries puras SQL)
├── services/            # Regras de negócio, sanitização de dados e testes
└── main.rs              # Interface CLI e interação com o usuário
```


### Destaques da Implementação
---
* Sanitização de Dados: Tratamento automático de strings com .trim() para evitar inserção de registros vazios ou com espaços sobresalentes.

* Isolamento nos Testes: Uso de Connection::open_in_memory() nos testes para garantir execuções rápidas e sem contaminar o banco de dados real (data.db).

* Erros Fortemente Tipados: Mapeamento de exceções com enums customizados (TaskError) e conversão explícita de erros do driver com .map_err().

### 🚀Como Executar o Projeto
---
### Pré-requisitos
Ter o Rust e o Cargo instalados na máquina. Se não tiver, instale pelo site oficial: rustup.rs.

1. Clonar o repositório
Bash
git clone [https://github.com/QuevemCosta/task_list_rust.git](https://github.com/QuevemCosta/task_list_rust.git)
cd task_list_rust
2. Rodar a aplicação (CLI)
Bash
cargo run
🧪 Executando os Testes Automatizados
O projeto conta com testes unitários cobrindo fluxos de sucesso e de erro (como tentativa de salvar títulos vazios ou com apenas espaços em branco).

### Para rodar toda a suíte de testes, execute:

Bash
cargo test
Para ver o output detalhado de cada teste individualmente:

Bash
```texto
cargo test -- --nocapture
```
📝 Funcionalidades

[x] Criar novas tarefas (com validação e sanitização)

[x] Listar todas as tarefas cadastradas

[x] Marcar tarefas como concluídas

[x] Remover tarefas do banco de dados

---
<center>
👤 Autor

Desenvolvido por

Quevem Costa

GitHub: @QuevemCosta
<center>
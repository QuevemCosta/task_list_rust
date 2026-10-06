# 📖 Guia de Desenvolvimento, Padrões e Roadmap — Task List CLI

Este documento estabelece as diretrizes de design, padrões de nomenclatura, plano de testes e o roadmap estratégico para a evolução do projeto **task_list_rust**.

---

## 🏗️ 1. Estrutura de Pastas e Módulos

O projeto adota uma arquitetura em camadas bem definidas dentro do diretório `src/`. Todos os arquivos e diretórios devem seguir o padrão `snake_case`.

```text
src/
├── main.rs                   # Ponto de entrada (Interface CLI e interação)
├── database.rs               # Inicialização, schemas e conexões do SQLite
├── models/                   # Estruturas de dados e entidades puras
│   └── task.rs
├── repositories/             # Camada de persistência (Queries puras SQL)
│   └── task_repository.rs
├── services/                 # Regras de negócio, validações e sanitização
│   └── task_service.rs
└── errors/                   # Tratamento centralizado de exceções strongly-typed
    └── task_error.rs
```

---

## 🔤 2. Convenção de Nomenclatura e Estilo

Para manter a consistência com o compilador do Rust (`clippy`), adotamos regras rígidas baseadas no estilo oficial da linguagem.

### 2.1 Padrões de Caixa (Casing)
*   **`CamelCase` (PascalCase):** Utilizado para Structs, Enums e Traits (ex: `Task`, `TaskError`).
*   **`snake_case`:** Utilizado para funções, métodos, variáveis, módulos e arquivos .rs.

### 2.2 Entidades e Tipos Estritos (`CamelCase`)
*   `Task`: Modelo estrutural da tarefa.
*   `TaskError`: Enum de erros do domínio de negócios e mapeamento de infraestrutura.
*   `TaskRepository`: Estrutura responsável pela comunicação direta com o banco de dados.

### 2.3 Coleções e Variáveis Boas Práticas
*   **Singular vs Plural:** Utilize o singular para instâncias de objetos (`task`) e o plural exclusivamente para coleções/vetores (`tasks: Vec<Task>`).
*   **Expressão de Estado (Booleanos):** Variáveis booleanas devem representar estados claros.
    *   *Exemplos autorizados:* `completed`, `enabled`, `active`, `valid`.
*   **Variáveis Locais:** `task_id`, `task_title`, `description`, `search_id`.

### 2.4 Assinatura Base de Ações e Funções (`snake_case`)
As funções do ciclo de vida das tarefas devem seguir os verbos padronizados abaixo:
* `create_task()`: Insere uma nova entidade.
*   `get_task()`: Recupera uma única entidade por ID.
*   `list_tasks()`: Retorna a coleção completa com filtros.
*   `update_task()`: Atualiza dados genéricos.
*   `delete_task()`: Remove de forma permanente do banco.
*   `complete_task()`: Modifica o estado de conclusão de forma explícita para positivo.
*   `reopen_task()`: Modifica o estado de conclusão explicitamente para aberto.
*   `toggle_task()`: Alterna o estado booleano atual.

---

## 🧪 3. Plano de Testes (Robustez e TDD Multi-usuário)

Os testes devem rodar de forma isolada usando conexões SQLite em memória (`Connection::open_in_memory()`).

### 3.1 Casos de Borda do Domínio (`services/`)
- [x] `test_create_task_with_maximum_length_title`: Valida strings com comprimentos massivos (150 chars).
- [ ] `test_create_task_with_special_characters_and_emojis`: Valida o comportamento de armazenamento UTF-8 (ex: `⚠️ Revisar PR #12`).
- [x ] `test_create_task_with_just_under_maximum_length_title()`: Validar se ate 149 positivo
- [ ] `test_sanitize_title_removes_surrounding_whitespaces`: Garante que `.trim()` limpe espaços involuntários no início/fim das strings.

### 3.2 Infraestrutura e Mapeamento de Falhas (`repositories/`)
- [ ] `test_mark_as_completed_non_existent_id_returns_not_found`: Garante retorno controlado de `TaskError::NotFound` para IDs inválidos.
- [ ] `test_delete_task_non_existent_id_returns_not_found`: Impede que a deleção de um ID inexistente falhe silenciosamente.
- [ ] `test_database_constraint_violation_mapping`: Testa se o `.map_err()` converte erros nativos do driver `rusqlite` para o enum fortemente tipado `TaskError::DatabaseError`.

### 3.3 Controle de Acesso (Abordagem TDD para a Próxima Fase)
- [ ] `test_user_can_only_see_their_own_tasks`: Valida que o isolamento impede que o `User_B` veja os registros do `User_A`.
- [ ] `test_user_cannot_complete_another_users_task`: Bloqueia requisições cruzadas e lança `TaskError::AccessDenied`.
- [ ] `test_user_cannot_delete_another_users_task`: Garante a imutabilidade dos dados por agentes externos não autorizados.
- [ ] `test_create_task_associates_correct_user_id`: Confirma se o campo de chave estrangeira `user_id` foi gravado corretamente.

---

## 🗺️ 4. Roadmap Estratégico do Projeto

### ✅ Fase 1: Refatoração da Arquitetura e Fundações
- [x] Criar estrutura de módulos (`models`, `repositories`, `services`, `errors`, `database`).
- [x] Realizar a separação de responsabilidades (Remover queries SQL das regras de negócio).
- [x] Versionar as alterações de arquitetura em commits semânticos.

### 🔄 Fase 2: Blindagem e Cobertura de Testes (Fase Atual)
- [x] Implementar testes unitários para o caminho feliz (`create_task()`).
- [ ] Finalizar os testes de validação de entrada de dados e sanitização.
- [ ] Desenvolver testes de integração isolados de CRUD no SQLite.
- [ ] Configurar ferramentas de cobertura de testes (ex: `tarpaulin`) visando a meta de **>70%**.

### 🛠️ Fase 3: Autenticação Dinâmica e Controle de Acesso
- [ ] Implementar criptografia de senhas usando hashing seguro (ex: `argon2` ou `bcrypt`).
- [ ] Criar fluxos de Login e geração de Tokens (Sessão baseada em banco ou estrutura JWT).
- [ ] Modificar as assinaturas de funções para exigir e validar permissões do usuário atual.

### 👥 Fase 4: Suporte Multi-usuário Nativo
- [ ] Atualizar o Schema do banco de dados para vincular relacionamentos `1:N` (`User -> Tasks`).
- [ ] Garantir que todas as queries de leitura e mutação passem filtros explícitos por `user_id`.

### 🚀 Fase 5: Modernização Tecnológica e Portfólio
- [ ] Migrar a aplicação inteira para o paradigma assíncrono (**Async Rust com Tokio runtime e SQLx**).
- [ ] Substituir o monólito por uma estrutura distribuída: API Backend (`Axum`) + Cliente Consumidor (`CLI via Reqwest`).
- [ ] Adicionar telemetria estruturada usando a crate `tracing`.
- [ ] Configurar pipelines de CI/CD (GitHub Actions) com linters automáticos (`cargo clippy`, `cargo fmt`).

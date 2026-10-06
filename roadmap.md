# Task List CLI (Rust) - Roadmap

## ✅ 1. Refatoração da Arquitetura
- [x] Criar pasta `models`
- [x] Criar pasta `infra`
- [x] Criar pasta `services`
- [x] Criar pasta `repositories`
- [x] Separar responsabilidades das camadas


## ✅ 2. Controle de Versão
- [x] Realizar commit da refatoração

## 🔄 3. Testes
### Testes Unitários
- [x] create_task()

- [ ] Testar validações de entrada

- [ ] Testar regras de negócio dos services
- [ ] Implementar mocks para dependências

### Testes de Integração
- [ ] Testar repositories com SQLite
- [ ] Validar operações CRUD completas
- [ ] Criar ambiente de teste isolado

### Cobertura de Testes
- [ ] Configurar ferramenta de cobertura
- [ ] Atingir cobertura mínima de 70%
- [ ] Identificar pontos não testados

## 🔄 4. Autenticação
### Gestão de Senhas
- [ ] Implementar hash de senhas
- [ ] Validar credenciais

### Login
- [ ] Criar fluxo de autenticação
- [ ] Tratar falhas de login

### Sessão/JWT
- [ ] Implementar geração de token
- [ ] Validar token
- [ ] Implementar expiração

### Controle de Acesso
- [ ] Restringir recursos por usuário
- [ ] Validar permissões

## 🔄 5. Multiusuário
- [ ] Associar tarefas a usuários
- [ ] Filtrar tarefas por usuário
- [ ] Garantir isolamento de dados

## 🚀 6. Evolução da Aplicação
- [ ] Melhorar tratamento de erros
- [ ] Implementar logs estruturados
- [ ] Melhorar experiência da CLI
- [ ] Configuração por arquivo (.env)
- [ ] Exportação/importação de dados
- [ ] Pipeline de CI/CD
- [ ] Documentação técnica
- [ ] Publicação de novas releases

## 🎯 Objetivo Final
- [ ] Aplicação modular
- [ ] Cobertura de testes consistente
- [ ] Autenticação completa
- [ ] Suporte multiusuário
- [ ] Projeto pronto para portfólio e demonstração profissional





# 🧪 Plano de Testes: Robustez, Casos de Borda e Controle de Acesso

Este documento serve como um guia de implementação para expandir a suíte de testes do projeto `task_list_rust`. O objetivo é garantir a estabilidade das regras de negócio atuais e preparar a arquitetura para receber autenticação e assincronismo.

---

## 1. Casos de Borda do Domínio e Sanitização (`services`)
Garante que a aplicação lide corretamente com entradas inesperadas de usuários sem quebrar o sistema.

- [ ] **`test_create_task_with_maximum_length_title`**
  - **Cenário:** Tentar criar uma tarefa com um título extremamente longo (ex: 255 ou 500 caracteres).
  - **Resultado esperado:** O sistema deve aceitar e persistir sem estourar o buffer, ou rejeitar controladamente via `TaskError::ValidationError`.
- [ ] **`test_create_task_with_special_characters_and_emojis`**
  - **Cenário:** Criar uma tarefa contendo caracteres especiais, quebras de linha (`\n`), abas (`\t`) e emojis (ex: `⚠️ Revisar PR #12`).
  - **Resultado esperado:** O SQLite deve persistir a string em formato UTF-8 corretamente e a recuperação do dado deve manter a formatação original.
- [ ] **`test_sanitize_title_removes_surrounding_whitespaces`**
  - **Cenário:** Passar um título com espaços extras no início e no fim (ex: `"   Comprar café   "`).
  - **Resultado esperado:** O teste deve validar que o título foi salvo no banco já limpo (`"Comprar café"`).

---

## 2. Robustez da Infraestrutura e Erros (`repos` & `database`)
Garante que as falhas de banco de dados ou requisições inválidas sejam mapeadas como erros strongly-typed (`TaskError`) e não causem `panic!`.

- [ ] **`test_mark_as_completed_non_existent_id_returns_not_found`**
  - **Cenário:** Tentar concluir uma tarefa passando um ID que não existe no banco de dados (ex: `999999`).
  - **Resultado esperado:** Retornar `Result::Err(TaskError::NotFound)` explicitamente.
- [ ] **`test_delete_task_non_existent_id_returns_not_found`**
  - **Cenário:** Tentar deletar uma tarefa passando um ID inválido ou inexistente.
  - **Resultado esperado:** Retornar `Result::Err(TaskError::NotFound)` em vez de fingir que a deleção ocorreu com sucesso.
- [ ] **`test_database_constraint_violation_mapping`**
  - **Cenário:** Forçar uma violação de constraint no SQLite (como tentar inserir um valor nulo em campo obrigatório se a validação falhar).
  - **Resultado esperado:** O método `.map_err()` deve converter com sucesso o erro do driver `rusqlite` para uma variante específica do seu enum (ex: `TaskError::DatabaseError`).

---

## 3. Controle de Acesso e Multi-usuário (Abordagem TDD)
*Nota: Escreva estes testes antes de mudar o código. Eles vão falhar inicialmente na compilação, servindo como guia para redesenhar as tabelas e assinaturas de funções.*

- [ ] **`test_user_can_only_see_their_own_tasks`**
  - **Cenário:** O `User_A` cria duas tarefas. O `User_B` faz a listagem geral.
  - **Resultado esperado:** A lista retornada para o `User_B` deve vir vazia (isolamento total de dados).
- [ ] **`test_user_cannot_complete_another_users_task`**
  - **Cenário:** O `User_B` tenta marcar como concluída a tarefa pertencente ao `User_A` passando o ID dela.
  - **Resultado esperado:** Retornar erro de permissão `Result::Err(TaskError::AccessDenied)`.
- [ ] **`test_user_cannot_delete_another_users_task`**
  - **Cenário:** O `User_B` tenta deletar uma tarefa pertencente ao `User_A`.
  - **Resultado esperado:** Retornar erro de permissão `Result::Err(TaskError::AccessDenied)` e a tarefa deve continuar intacta no banco de dados.
- [ ] **`test_create_task_associates_correct_user_id`**
  - **Cenário:** Criar uma tarefa passando a entidade ou ID do usuário logado.
  - **Resultado esperado:** Validar se a chave estrangeira `user_id` correspondente foi gravada na linha da tabela do banco de dados.

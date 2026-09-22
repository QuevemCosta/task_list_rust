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
- [x] new_task()

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
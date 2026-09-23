# TASK LIST — CONVENÇÃO DE NOMENCLATURA

## FUNÇÕES
```
create_task()
get_task()
list_tasks()
update_task()
delete_task()

toggle_task()
complete_task()
reopen_task()

```
## ENTIDADES / STRUCTS

* Task

* TaskError

* TaskRepository


## VARIÁVEIS
```
snake_case

task

task_id

task_title

description

completed

search_id
```

## MÓDULOS
```
snake_case
task
task_service
task_repo
task_errors
database
```

## ARQUIVOS
```
snake_case
task.rs
task_service.rs
task_repo.rs
task_errors.rs
database.rs
```

## COLEÇÕES
```
singular → objeto
plural   → coleção

task
tasks

```
## BOOLEANOS
```
completed
enabled
active
valid

Usar nomes que possam ser lidos como estado.
```

## AÇÕES

* create

* get

* list

* update

* delete

* toggle

* complete

* reopen
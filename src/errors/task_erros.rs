#[derive(Debug)]
pub enum TaskError {
    EmptyDescription,
    InvalidId,
    TaskNotFound,
    DatabaseError,
}

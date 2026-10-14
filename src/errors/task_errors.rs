#[derive(Debug)]
pub enum TaskError {
    EmptyDescription,
    TaskNotFound,
    DatabaseError,
    ListTasksFailed,
    MaxLenChars,
    DeleteError,
}

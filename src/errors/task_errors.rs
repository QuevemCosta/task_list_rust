#[derive(Debug)]
pub enum TaskError {
    Emptydescriptionion,
    InvalidId,
    TaskNotFound,
    DatabaseError,
    ListTasksFailed,
}

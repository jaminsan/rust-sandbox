use ulid::Ulid;

#[derive(Debug, Copy, Clone)]
pub struct CustomerId(pub Ulid);
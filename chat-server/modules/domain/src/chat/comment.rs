use uuid::Uuid;

#[derive(PartialOrd, PartialEq, Eq, Debug, Clone, Ord, )]
pub struct Comment {
    pub id: String,
    pub room_id: String,
    pub audience_id: String,
    pub text: String,
}

impl Comment {
    pub fn new(room_id: String, audience_id: String, text: String) -> Self {
        Comment {
            id: Uuid::new_v4().to_string(),
            room_id,
            audience_id,
            text,
        }
    }
}

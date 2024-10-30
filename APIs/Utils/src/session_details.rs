use uuid::Uuid;

pub struct SessionDetails {
    pub session_id: i32,
    pub user_id: i32,
    pub session_uuid: Uuid,
}
use prost::Message;

#[derive(Message)]
pub struct ViewUserRequest {
    #[prost(string, tag = "1")]
    pub user_id: String,
}

#[derive(Message)]
pub struct ViewProjectRequest {
    #[prost(string, tag = "1")]
    pub project_id: String,
}

use prost::Message;

#[derive(Message)]
pub struct PopularProjectsResponse {
    #[prost(message, repeated, tag = 1)]
    pub projects: Vec<PopularProject>,
}

#[derive(Message)]
pub struct PopularProject {
    #[prost(string, tag = 1)]
    pub id: String,
    #[prost(uint32, tag = 2)]
    pub score: u32,
}

#[derive(Message)]
pub struct PopularUsersResponse {
    #[prost(message, repeated, tag = 1)]
    pub users: Vec<PopularUser>,
}

#[derive(Message)]
pub struct PopularUser {
    #[prost(string, tag = 1)]
    pub id: String,
    #[prost(uint32, tag = 2)]
    pub score: u32,
}
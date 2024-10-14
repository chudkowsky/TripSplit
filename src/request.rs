use serde::{Deserialize, Serialize};

#[derive(Clone,Serialize,Deserialize)]
pub struct CreateGroupRequest{
    pub name: String,
    pub members_id: Vec<i32>,
}
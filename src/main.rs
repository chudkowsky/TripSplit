use models::user::User;

pub mod models;
pub mod server;

#[tokio::main]
async fn main() {
    server::start().await;
    let user1: User = User::new(1, "User1","test","test");
    let group  = models::group::Group::new(1, "Group1", vec![user1.clone()]);
    let summary = group.get_group_summary();
    println!("{:?}", summary);
}
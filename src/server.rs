use std::net::SocketAddr;
use std::sync::Arc;

use axum::extract::{Query, State};
use axum::response::Response;
use axum::Json;
use axum::{
    routing::{get, post},
    Router,
};
use tokio::sync::Mutex;
use crate::request::CreateGroupRequest;
use crate::models::group::Group;
use crate::models::user::User;

async fn create_user(
    State(app_state): State<AppState>,
    Json(user): Json<User>,
) -> Response<String> {
    if app_state
        .users
        .lock()
        .await
        .iter()
        .any(|u| u.email == user.email)
    {
        return Response::new("User already exists".to_string());
    }
    if app_state
        .users
        .lock()
        .await
        .iter()
        .any(|u| u.name == user.name)
    {
        return Response::new("Username already exists".to_string());
    }
    if user.password.len() < 8 {
        return Response::new("Password must be at least 8 characters".to_string());
    }
    let id = if app_state.users.lock().await.len() == 0 {
        0
    } else {
        app_state.users.lock().await.last().unwrap().id + 1
    };

    let user = User::new(
        id,
        user.name.as_str(),
        user.email.as_str(),
        user.password.as_str(),
    );
    println!("User created succesfully: {:?}", user);
    app_state.users.lock().await.push(user.clone());
    Response::new(format!("User created succesfully: {:?}", user))
}
async fn get_users(State(app_state): State<AppState>) -> Response<String> {
    let users = app_state.users.lock().await.clone();
    Response::new(
        users
            .iter()
            .map(|u| u.to_string())
            .collect::<Vec<String>>()
            .join("\n"),
    )
}
async fn create_group(
    State(app_state): State<AppState>,
    Query(group): Query<CreateGroupRequest>,
) -> Response<String> {
    if app_state
        .groups
        .lock()
        .await
        .iter()
        .any(|g| g.name == group.name)
    {
        return Response::new("Group already exists".to_string());
    }
    let id = if app_state.groups.lock().await.len() == 0 {
        0
    } else {
        app_state.groups.lock().await.last().unwrap().id + 1
    };
    let members:Vec<User> = app_state
        .users
        .lock()
        .await
        .iter()
        .filter(|u| group.members_id.contains(&u.id))
        .cloned()
        .collect(); 
        let group = Group::new(id, group.name.as_str(),members);
        
    println!("Group created succesfully: {:?}", group);
    app_state.groups.lock().await.push(group.clone());
    Response::new(format!("Group created succesfully: {:?}", group))
}
async fn get_groups(State(app_state): State<AppState>) -> Response<String> {
    let groups = app_state.groups.lock().await.clone();
    Response::new(
        groups
            .iter()
            .map(|g| g.to_string())
            .collect::<Vec<String>>()
            .join("\n"),
    )
}

#[derive(Clone)]
pub struct AppState {
    pub users: Arc<Mutex<Vec<User>>>,
    pub groups: Arc<Mutex<Vec<Group>>>,
}
pub async fn start() {
    let app_state = AppState {
        users: Arc::new(Mutex::new(Vec::new())),
        groups: Arc::new(Mutex::new(Vec::new())),
    };
    let app = Router::new()
        .route("/create_user", post(create_user))
        .route("/create_group", post(create_group))
        .route("/get_users", get(get_users))
        .route("/get_groups", get(get_groups))
        .route("/", get(ok_handler))
        .fallback(|| async { "There is nothing here" })
        .with_state(app_state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    println!("Listening on http://{}", addr);

    // Run the server
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn ok_handler() -> String {
    "server is working".into()
}

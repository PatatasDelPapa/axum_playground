use axum::{
    body::{box_body, BoxBody},
    extract::{Extension, Form, Json, Query, UrlParams},
    prelude::*,
    response::IntoResponse,
};
use http::{Response, StatusCode};
use uuid::Uuid;

use crate::types::{handler_types::*, templates::*, Db};

pub async fn front_page() -> impl IntoResponse {
    tracing::info!("Called front_page!");
    let template = ChildTemplate;
    HtmlTemplate(template)
}

pub async fn greet(params: extract::UrlParamsMap) -> impl IntoResponse {
    tracing::info!("Called greet!");
    let name = params
        .get("name")
        .expect("'name' will be there if route was matched")
        .to_string();
    let template = HelloTemplate { name };
    HtmlTemplate(template)
}

// async fn user_create() -> impl IntoResponse {
//     (StatusCode::CREATED, "User Created!")
// }

pub async fn todos_index(
    pagination: Option<Query<Pagination>>,
    Extension(db): Extension<Db>,
) -> impl IntoResponse {
    tracing::info!("Called :id");
    let todos = db.read().unwrap();

    let Query(pagination) = pagination.unwrap_or_default();

    let todos = todos
        .values()
        .cloned()
        .skip(pagination.offset.unwrap_or(0))
        .take(pagination.limit.unwrap_or(std::usize::MAX))
        .collect::<Vec<_>>();

    response::Json(todos)
}

pub async fn todos_create(
    Form(input): Form<CreateTodo>,
    Extension(db): Extension<Db>,
) -> impl IntoResponse {
    tracing::info!("Called create!");

    let todo = Todo {
        id: Uuid::new_v4(),
        title: input.title,
        text: input.text,
        completed: false,
    };

    db.write().unwrap().insert(todo.id, todo.clone());

    (StatusCode::CREATED, response::Json(todo))
}

pub async fn todos_create_form() -> impl IntoResponse {
    tracing::info!("Called show form!");
    let template = TodoForm;
    HtmlTemplate(template)
}

pub async fn todos_update(
    UrlParams((id,)): UrlParams<(Uuid,)>,
    Json(input): Json<UpdateTodo>,
    Extension(db): Extension<Db>,
) -> Result<impl IntoResponse, StatusCode> {
    tracing::info!("Called update!");

    let mut todo = db
        .read()
        .unwrap()
        .get(&id)
        .cloned()
        .ok_or(StatusCode::NOT_FOUND)?;

    if let Some(title) = input.title {
        todo.title = title;
    }

    if let Some(text) = input.text {
        todo.text = text;
    }

    if let Some(completed) = input.completed {
        todo.completed = completed;
    }

    db.write().unwrap().insert(todo.id, todo.clone());

    Ok(response::Json(todo))
}

pub async fn todos_delete(
    UrlParams((id,)): UrlParams<(Uuid,)>,
    Extension(db): Extension<Db>,
) -> impl IntoResponse {
    tracing::info!("Called delete!");

    if db.write().unwrap().remove(&id).is_some() {
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}

pub fn map_404(response: Response<BoxBody>) -> Response<BoxBody> {
    tracing::warn!("Got to 404 status = {}", response.status());
    if response.status() != StatusCode::NOT_FOUND {
        return response;
    }

    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(box_body(Body::from("nothing to see here")))
        .unwrap()
}

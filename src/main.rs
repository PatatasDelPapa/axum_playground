mod handlers;
mod trace;
mod types;

use crate::{handlers::*, types::Db};

use std::net::SocketAddr;

use axum::{prelude::*, service::ServiceExt};
use tower::util::MapResponseLayer;
use tower_http::{add_extension::AddExtensionLayer, trace::TraceLayer};

#[tokio::main]
async fn main() {
    // initialize tracing
    trace::initialize_logs();

    // create the app
    let db_todos = Db::default();

    let todo_routes = route("/:id", patch(todos_update).delete(todos_delete))
        .route("/create", get(todos_create_form).post(todos_create))
        .route("/", get(todos_index))
        .layer(AddExtensionLayer::new(db_todos));

    let app = route("/", get(front_page))
        .route("/greet/:name", get(greet))
        .nest("/todos", todo_routes)
        .layer(MapResponseLayer::new(map_404))
        .layer(TraceLayer::new_for_http())
        .check_infallible();

    // define the addr to localhost:3000.
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("Listening on {}", addr);

    // run it with hyper on addr.
    hyper::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(async { tokio::signal::ctrl_c().await.unwrap() })
        .await
        .unwrap();
}

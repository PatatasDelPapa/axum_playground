use askama::Template;
use axum::{prelude::*, response::IntoResponse};
use http::{Response, StatusCode};

#[derive(Template)]
#[template(path = "hello.html")]
pub struct HelloTemplate {
    pub name: String,
}

#[derive(Template)]
#[template(path = "child.html")]
pub struct ChildTemplate;

#[derive(Template)]
#[template(path = "form.html")]
pub struct TodoForm;

pub struct HtmlTemplate<T>(pub T);

impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> http::Response<Body> {
        match self.0.render() {
            Ok(html) => response::Html(html).into_response(),
            Err(err) => Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from(format!(
                    "Failed to render template. Error {}",
                    err
                )))
                .unwrap(),
        }
    }
}

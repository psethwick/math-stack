use axum::{
    extract::Form,
    http::StatusCode,
    response::Html,
    routing::{get, post},
    Router,
};
use maud::{html, Markup, DOCTYPE};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

fn index() -> Markup {
    html! {
        (DOCTYPE)
        head {
            script src="htmx.min.js" {}
            link rel="stylesheet" href="site.css" {}
        }

        #content {
            p { "Hello world!" }
        }

        div {
            form hx-post="/swap" {
                input type="text" id="name" name="name";
                button hx-target="#content" hx-swap="outerHTML" {
                    "Submit!"
                }
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
struct Lol {
    name: String,
}

fn slop(asfd: Lol) -> Markup {
    html! {
        #content {
            p { (asfd.name) }
        }
    }
}

pub async fn serve(port: u16) -> Result<(), Box<dyn std::error::Error>> {
    let app = Router::new()
        .route("/", get(index_handler))
        .route("/swap", post(slop_handler))
        .nest_service("/", ServeDir::new("./public"));

    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).await?;

    axum::serve(listener, app).await?;

    Ok(())
}

async fn index_handler() -> Html<String> {
    Html(index().into())
}

async fn slop_handler(Form(lol): Form<Lol>) -> Result<String, StatusCode> {
    Ok(slop(lol).into())
}

mod entity;
use axum::{
    extract::{Extension, Form, Path, Query},
    http::StatusCode,
    response::Html,
    routing::{get, get_service, post},
    Router, Server,
};
use axum_macros::debug_handler;
use entity::service;
use sea_orm::{prelude::*, Database, QueryOrder, Set};
use serde::{Deserialize, Serialize};
use service::Entity as Service;
use std::str::FromStr;
use std::{env, net::SocketAddr};
use tera::Tera;
use tower::ServiceBuilder;
use tower_cookies::{CookieManagerLayer, Cookies};
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env::set_var("RUST_LOG", "debug");
    tracing_subscriber::fmt::init();

    dotenv::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");
    let server_url = format!("{}:{}", host, port);

    let conn = Database::connect(db_url)
        .await
        .expect("Database connection failed");
    //Migrator::up(&conn, None).await.unwrap();
    let templates = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*"))
        .expect("Tera initialization failed");
    // let state = AppState { templates, conn };

    let app = Router::new()
        .route("/", get(list_services))
        /*.route("/:id", get(edit_post).post(update_post))
        .route("/new", get(new_post))
        .route("/delete/:id", post(delete_post))*/
        .nest(
            "/static",
            get_service(ServeDir::new(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/static"
            )))
            .handle_error(|error: std::io::Error| async move {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Unhandled internal error: {}", error),
                )
            }),
        )
        .layer(
            ServiceBuilder::new()
                .layer(CookieManagerLayer::new())
                .layer(Extension(conn))
                .layer(Extension(templates)),
        );
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

#[derive(Deserialize)]
struct Params {
    page: Option<usize>,
    posts_per_page: Option<usize>,
}
/*
#[derive(Deserialize, Serialize, Debug, Clone)]
struct FlashData {
    kind: String,
    message: String,
}*/

#[debug_handler]
async fn list_services(
    Extension(ref templates): Extension<Tera>,
    Extension(ref conn): Extension<DatabaseConnection>,
    Query(params): Query<Params>,
) -> Result<Html<String>, (StatusCode, &'static str)> {
    let services = Service::find()
        .all(conn)
        .await
        .expect("could not retrieve services");

    let mut ctx = tera::Context::new();
    ctx.insert("service_list", &services);

    //if let Some(value) = get_flash_cookie::<FlashData>(&cookies) {
    //    ctx.insert("flash", &value);
    //}

    let body = templates
        .render("index.html.tera", &ctx)
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Template error"))?;

    Ok(Html(body))
}
/*
async fn new_post(
    Extension(ref templates): Extension<Tera>,
) -> Result<Html<String>, (StatusCode, &'static str)> {
    let ctx = tera::Context::new();
    let body = templates
        .render("new.html.tera", &ctx)
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Template error"))?;

    Ok(Html(body))
}

async fn create_post(
    Extension(ref conn): Extension<DatabaseConnection>,
    form: Form<post::Model>,
    mut cookies: Cookies,
) -> Result<PostResponse, (StatusCode, &'static str)> {
    let model = form.0;

    post::ActiveModel {
        title: Set(model.title.to_owned()),
        text: Set(model.text.to_owned()),
        ..Default::default()
    }
    .save(conn)
    .await
    .expect("could not insert post");

    let data = FlashData {
        kind: "success".to_owned(),
        message: "Post succcessfully added".to_owned(),
    };

    Ok(post_response(&mut cookies, data))
}

async fn edit_post(
    Extension(ref templates): Extension<Tera>,
    Extension(ref conn): Extension<DatabaseConnection>,
    Path(id): Path<i32>,
) -> Result<Html<String>, (StatusCode, &'static str)> {
    let post: post::Model = Post::find_by_id(id)
        .one(conn)
        .await
        .expect("could not find post")
        .unwrap();

    let mut ctx = tera::Context::new();
    ctx.insert("post", &post);

    let body = templates
        .render("edit.html.tera", &ctx)
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Template error"))?;

    Ok(Html(body))
}

async fn update_post(
    Extension(ref conn): Extension<DatabaseConnection>,
    Path(id): Path<i32>,
    form: Form<post::Model>,
    mut cookies: Cookies,
) -> Result<PostResponse, (StatusCode, String)> {
    let model = form.0;

    post::ActiveModel {
        id: Set(id),
        title: Set(model.title.to_owned()),
        text: Set(model.text.to_owned()),
    }
    .save(conn)
    .await
    .expect("could not edit post");

    let data = FlashData {
        kind: "success".to_owned(),
        message: "Post succcessfully updated".to_owned(),
    };

    Ok(post_response(&mut cookies, data))
}

async fn delete_post(
    Extension(ref conn): Extension<DatabaseConnection>,
    Path(id): Path<i32>,
    mut cookies: Cookies,
) -> Result<PostResponse, (StatusCode, &'static str)> {
    let post: post::ActiveModel = Post::find_by_id(id)
        .one(conn)
        .await
        .unwrap()
        .unwrap()
        .into();

    post.delete(conn).await.unwrap();

    let data = FlashData {
        kind: "success".to_owned(),
        message: "Post succcessfully deleted".to_owned(),
    };

    Ok(post_response(&mut cookies, data))
}*/

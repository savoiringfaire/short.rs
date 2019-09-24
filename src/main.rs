#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate tera;
#[macro_use]
extern crate log;

extern crate pretty_env_logger;
extern crate simple_error;

pub mod short;
pub mod shortdb;

use short::Short;
use futures::{future, Future};
use std::env;
use std::error::Error;
use simple_error::SimpleError;
use std::sync::Arc;
use std::collections::HashMap;
use hyper::{
    client::HttpConnector, rt, service::service_fn, Body, Client, Request,
    Response, Server, Method, StatusCode
};
use tera::{Context, Tera};

type GenericError = Box<dyn std::error::Error + Send + Sync>;
type ResponseFuture = Box<dyn Future<Item = Response<Body>, Error = GenericError> + Send>;


lazy_static! {
    pub static ref TERA: Tera = compile_templates!("templates/**/*");
}

fn get_new() -> ResponseFuture {
    let ctx = Context::new();
    let body = Body::from(TERA.render("index.html", &ctx).unwrap().to_string());

    Box::new(future::ok(
        Response::builder()
            .body(body)
            .unwrap(),
    ))
}

fn get_argument_from_url(req: Request<Body>, arg: &str) -> Result<String, SimpleError> {
    let args = url::form_urlencoded::parse(&req.uri().query().unwrap().as_bytes())
        .into_owned()
        .collect::<HashMap<String, String>>();

    match args.get(arg) {
        Some(value) => Ok(value.clone()),
        None => Err(SimpleError::new("Argument Not Found"))
    }
}

fn get_complete(req: Request<Body>) -> Result<ResponseFuture, Box<dyn Error>> {
    let token = get_argument_from_url(req, "token")?;

    let mut ctx = Context::new();
    ctx.insert("token", &token);

    let body = Body::from(TERA.render("complete.html", &ctx)?.to_string());

    Ok(Box::new(future::ok(
        Response::builder()
            .body(body)
            .unwrap(),
    )))
}

fn post_new(req: Request<Body>, redis_client: &Arc<redis::Client>) -> Result<ResponseFuture, Box<dyn Error>> {
    let mut con = redis_client.get_connection()?;
    let target = get_argument_from_url(req, "target")?;
    let short = Short::new(target)?;
    let token = short.token.clone();

    shortdb::add_short(short, &mut con)?;

    Ok(Box::new(future::ok(
        Response::builder()
            .status(StatusCode::MOVED_PERMANENTLY)
            .header("Location", format!("/complete?token={}", token))
            .body(Body::from(""))
            .unwrap(),
    )))
}

/// Handle a request that does't match other requests (and therefore should be a redirect request).
fn get_redirect(req: Request<Body>, redis_client: &Arc<redis::Client>) -> Result<ResponseFuture, Box<dyn Error>> {
    let mut con = redis_client.get_connection()?;
    Ok(Box::new(future::ok(
        Response::builder()
            .status(StatusCode::MOVED_PERMANENTLY)
            .header("Location", format!(
                "{}",
                shortdb::get_short(
                    &req.uri().path()[1..], &mut con
                )?.target
            ))
            .body(Body::from(""))
            .unwrap(),
    )))
}

fn render_error_page(error: Box<dyn Error>) -> ResponseFuture {
    Box::new(future::ok(
        Response::builder()
            .status(500)
            .body(Body::from(
                format!("Internal Server Error: {}", error)
            ))
            .unwrap(),
    ))
}

fn respond_handle_error(result: Result<ResponseFuture, Box<dyn Error>>) -> ResponseFuture {
    match result {
        Ok(response) => response,
        Err(error) => {
            error!("{}", error);
            render_error_page(error)
        }
    }
}

fn router(req: Request<Body>, _client: &Client<HttpConnector>, redis_client: &Arc<redis::Client>) -> ResponseFuture {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => {
            get_new()
        }
        (&Method::GET, "/new") => {
            respond_handle_error(post_new(req, redis_client))
        }
        (&Method::GET, "/complete") => {
            respond_handle_error(get_complete(req))
        }
        _ => {
            respond_handle_error(get_redirect(req, redis_client))
        }
    }
}

fn main() {
    pretty_env_logger::init();

    rt::run(future::lazy(move || {
        // create a Client for all Services
        let client = Client::new();

        let connection_string: &str = &env::var("REDIS_CONNECTION_STRING").unwrap();
        let addr = env::var("LISTEN_ADDRESS").unwrap().parse().unwrap();

        let redis_client = Arc::new(
            redis::Client::open(
                connection_string
            ).unwrap()
        );

        // define a service containing the router function
        let new_service = move || {
            // Move a clone of Client into the service_fn
            let client = client.clone();
            let redis_client = redis_client.clone();
            service_fn(move |req| router(req, &client, &redis_client))
        };

        // Define the server - this is what the future_lazy() we're building will resolve to
        let server = Server::bind(&addr)
            .serve(new_service)
            .map_err(|e| eprintln!("Server error: {}", e));

        println!("Listening on http://{}", addr);
        server
    }));
}

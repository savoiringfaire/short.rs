#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate tera;
#[macro_use]
extern crate log;

extern crate pretty_env_logger;
extern crate simple_error;

pub mod short;

use short::Short;
use futures::{future, Future};
use std::error::Error;
use simple_error::SimpleError;
use std::sync::{RwLock, Arc};
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
    pub static ref SHORTS: Shorts = Arc::new(RwLock::new(Vec::new()));
}

type Shorts = Arc<RwLock<Vec<Short>>>;

fn add_short(t: Short) {
    let shorts = Arc::clone(&SHORTS);
    let mut lock = shorts.write().unwrap();
    lock.push(t);
}

fn get_target(token: String) -> String {
    let shorts = Arc::clone(&SHORTS);
    let lock = shorts.read().unwrap();

    for short in &(*lock) {
        if short.token == token {
            return short.target.clone()
        }
    }

    return "".to_string()
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

fn post_new(req: Request<Body>) -> Result<ResponseFuture, Box<dyn Error>> {
    let target = get_argument_from_url(req, "target")?;
    let short = Short::new(target)?;
    let token = short.token.clone();

    add_short(short);

    Ok(Box::new(future::ok(
        Response::builder()
            .status(StatusCode::MOVED_PERMANENTLY)
            .header("Location", format!("/complete?token={}", token))
            .body(Body::from(""))
            .unwrap(),
    )))
}

/// Handle a request that does't match other requests (and therefore should be a redirect request).
fn get_redirect(req: Request<Body>) -> ResponseFuture {
    Box::new(future::ok(
        Response::builder()
            .status(StatusCode::MOVED_PERMANENTLY)
            .header("Location", format!("{}", get_target(
                req.uri().path()[1..].to_string()))
            )
            .body(Body::from(""))
            .unwrap(),
    ))
}

fn respond_handle_error(result: Result<ResponseFuture, Box<dyn Error>>) -> ResponseFuture {
    match result {
        Ok(response) => response,
        Err(error) => {
            Box::new(future::ok(
                Response::builder()
                    .status(500)
                    .body(Body::from(
                        format!("Internal Server Error: {}", error)
                    ))
                    .unwrap(),
            ))
        }
    }
}

fn router(req: Request<Body>, _client: &Client<HttpConnector>) -> ResponseFuture {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => {
            get_new()
        }
        (&Method::GET, "/new") => {
            respond_handle_error(post_new(req))
        }
        (&Method::GET, "/complete") => {
            get_complete(req)
        }
        _ => {
            get_redirect(req)
        }
    }
}

fn main() {
    pretty_env_logger::init();

    let addr = "127.0.0.1:3000".parse().unwrap();

    rt::run(future::lazy(move || {
        // create a Client for all Services
        let client = Client::new();

        // define a service containing the router function
        let new_service = move || {
            // Move a clone of Client into the service_fn
            let client = client.clone();
            service_fn(move |req| router(req, &client))
        };

        // Define the server - this is what the future_lazy() we're building will resolve to
        let server = Server::bind(&addr)
            .serve(new_service)
            .map_err(|e| eprintln!("Server error: {}", e));

        println!("Listening on http://{}", addr);
        server
    }));
}

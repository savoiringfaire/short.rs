#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate tera;
#[macro_use]
extern crate log;

extern crate pretty_env_logger;

use futures::{future, Future, Stream};
use std::sync::{RwLock, Arc};
use std::borrow::Cow;
use std::collections::HashMap;
use url::{Url, ParseError};
use hyper::{
    client::HttpConnector, rt, service::service_fn, Body, Client, Request,
    Response, Server, Method, StatusCode
};
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use tera::{Context, Tera};

type GenericError = Box<dyn std::error::Error + Send + Sync>;
type ResponseFuture = Box<Future<Item = Response<Body>, Error = GenericError> + Send>;



lazy_static! {
    pub static ref TERA: Tera = compile_templates!("templates/**/*");
    pub static ref SHORTS: Shorts = Arc::new(RwLock::new(Vec::new()));
}

pub struct Short {
    token: String,
    target: String
}

type Shorts = Arc<RwLock<Vec<Short>>>;

impl Short {
    fn new(target: String) -> Self {
        Self {
            token: generate_token(10),
            target: target
        }
    }
}

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
    let mut ctx = Context::new();
    let body = Body::from(TERA.render("index.html", &ctx).unwrap().to_string());

    Box::new(future::ok(
        Response::builder()
            .body(body)
            .unwrap(),
    ))
}

fn get_complete(req: Request<Body>) -> ResponseFuture {
    let args = url::form_urlencoded::parse(&req.uri().query().unwrap().as_bytes())
        .into_owned()
        .collect::<HashMap<String, String>>();

    let mut ctx = Context::new();
    ctx.insert("token", &args["token"].clone());
    let body = Body::from(TERA.render("complete.html", &ctx).unwrap().to_string());

    Box::new(future::ok(
        Response::builder()
            .body(body)
            .unwrap(),
    ))
}

fn post_new(req: Request<Body>) -> ResponseFuture {
    let args = url::form_urlencoded::parse(&req.uri().query().unwrap().as_bytes())
        .into_owned()
        .collect::<HashMap<String, String>>();

    let short = Short::new(args["target"].clone());
    let token = short.token.clone();

    add_short(short);

    Box::new(future::ok(
        Response::builder()
            .status(StatusCode::MOVED_PERMANENTLY)
            .header("Location", format!("/complete?token={}", token))
            .body(Body::from(""))
            .unwrap(),
    ))
}

fn generate_token(n: usize) -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(n)
        .collect()
}

fn router(req: Request<Body>, _client: &Client<HttpConnector>) -> ResponseFuture {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => {
            get_new()
        }
        (&Method::GET, "/new") => {
            post_new(req)
        }
        (&Method::GET, "/complete") => {
            get_complete(req)
        }
        _ => {
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

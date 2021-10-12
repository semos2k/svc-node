#![deny(warnings)]

use std::env::var;
use std::time::Duration;
use hyper::client::HttpConnector;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Client, Request, Response, Server};
use lazy_static::lazy_static;

type GenericError = Box<dyn std::error::Error + Send + Sync>;
type Result<T> = std::result::Result<T, GenericError>;

//static URL: &str = "http://127.0.0.1:3000";
lazy_static! {
    // static ref PASSWORD: String = var("PASSWORD").unwrap();
    static ref API_HOSTNAME: String = var("SVC_API_HOSTNAME").unwrap_or("localhost".to_string());
    static ref API_PORT:String = var("SVC_API_PORT").unwrap_or("4000".to_string());
}

async fn hello(mut _req: Request<Body>, client: Client<HttpConnector>) 
        -> Result<Response<Body>> {
    if _req.uri().path() == "/healthz" {
        Ok(Response::new(Body::from("Ok")))
    }else{
        let uri_string = format!(
            "http://{}:{}{}",
            *API_HOSTNAME, *API_PORT,
            _req.uri()
                .path_and_query()
                .unwrap()
        );
        let uri = uri_string.parse().unwrap();
        *_req.uri_mut() = uri;
        let web_res = client.request(_req).await?;

        Ok(Response::new(web_res.into_body()))
    }
}

#[tokio::main]
pub async fn main() -> Result<()> {
    //pretty_env_logger::init();
    //let client = Client::new();
    let client = Client::builder()
        .pool_idle_timeout(Duration::from_secs(5))
        .build_http();

    let make_svc = make_service_fn(move |_| {
        let client = client.clone();
        async {
            Ok::<_, GenericError>(service_fn(move |req| {
                hello(req, client.to_owned())
            }))
        }
    });

    let addr = ([0, 0, 0, 0], 5000).into();
    let server = Server::bind(&addr).serve(make_svc);
    
    println!("Listening on http://{}", addr);
    println!(" - consumer endpoint: http://{}:{}", *API_HOSTNAME, *API_PORT);
    server.await?;

    Ok(())
}
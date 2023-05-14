use vercel_runtime::{Body, Error, Request, RequestExt, Response, run, StatusCode};

#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::init();

    let h = |req: Request| async move {
        let res = handler(req).await;
        match res {
            Ok(res) => Ok(res),
            Err(e) => {
                Ok(Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(Body::from(format!("Error: {}", e)))?)
            }
        }
    };

    run(h).await
}

pub async fn handler(_req: Request) -> Result<Response<Body>, Error> {
    static DOCUMENT: &'static str = include_str!("../assets/example.svg");
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "image/svg+xml")
        .body(DOCUMENT.to_string().into())?)
}

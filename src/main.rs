use salvo::{prelude::*, server_static::StaticDir};

#[handler]
async fn hello_world() -> &'static str {
    "Hello, world !"
}

#[handler]
async fn about() -> &'static str {
    "about!"
}
#[handler]
async fn services() -> &'static str {
    "services!"
}


#[tokio::main]
async fn main() {
/* 
    let router = Router::new()
        .push(Router::new().path("").get(hello_world))
        .push(Router::new().path("about").get(about))
        .push(Router::new().path("services").get(services));
 */
    let router = Router::with_path("<**path>").get(
        StaticDir::new(["public"])
            .with_defaults("index.html")
            .with_listing(true),
    );

/* 
    Server::new(TcpListener::bind("127.0.0.1:3000"))
        .serve(router)
        .await;
 */
    let acceptor = TcpListener::new("127.0.0.1:3000").bind().await;
    Server::new(acceptor).serve(router).await;
}

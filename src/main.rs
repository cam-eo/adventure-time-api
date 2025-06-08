use axum::{Router, routing::get};
use tokio::signal;

// basic handler that responds with a static string
async fn hello() -> &'static str {
    "Hello, World!"
}

#[tokio::main]
async fn main() {
    // // initialize tracing
    // tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(hello));
    // // `POST /users` goes to `create_user`
    // .route("/users", post(create_user));
    // .route("/", get())

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app)
        .with_graceful_shutdown(async {
            signal::ctrl_c()
                .await
                .expect("failed to install Ctrl+C handler");
            println!("Shutting down...");
        })
        .await
        .unwrap();
}

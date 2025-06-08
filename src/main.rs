use axum::{Router, routing::get};
use tokio::signal;

mod routes;

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Welcome to the Adventure Time API!"
}

#[tokio::main]
async fn main() {
    // // initialize tracing
    // tracing_subscriber::fmt::init();

    // build our application with a route
    let app: Router = Router::new()
        .route("/", get(root))
        .nest("/characters", routes::characters::routes());

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

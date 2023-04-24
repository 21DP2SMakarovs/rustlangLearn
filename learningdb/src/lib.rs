mod routes;


use routes::create_routes;

pub async fn run() {
    //build our app with a single route
    // let app = Router::new().route("/", get(|| async { "Hello, World!"})); // can do this but this bad
    // let app = Router::new().route("/", get(hello_world)); // this better
    let app = create_routes();
    //run it with hyper on 3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}


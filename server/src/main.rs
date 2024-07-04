mod error;

use app::App;
use axum::Router;
use leptos::get_configuration;
use leptos_axum::{generate_route_list, LeptosRoutes};

pub use error::Result;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<()> {
    // init logs

    // read config
    let config = get_configuration(None).await?;
    let leptos_opts = config.leptos_options;
    let addr = leptos_opts.site_addr;
    let routes = generate_route_list(App);

    // build routes
    let routes_all = Router::new()
        .leptos_routes(&leptos_opts, routes, App)
        // fallback service
        .with_state(leptos_opts);

    // start server
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("LISTENING {:?}", listener.local_addr()); // sould be log

    axum::serve(listener, routes_all.into_make_service())
        .await
        .unwrap();

    Ok(())
}

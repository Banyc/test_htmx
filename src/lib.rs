use axum::{routing::get, Router};
use maud::{html, Markup};
use tokio::net::TcpListener;

// Block on the web server
pub async fn serve() {
    // Register routes and serve them
    let router = Router::new()
        .route("/hi-maud", get(hi_maud))
        .nest("/", hi_htmx());
    let listener = TcpListener::bind("127.0.0.1:6969")
        .await
        .expect("failed to bind");
    axum::serve(listener, router.into_make_service())
        .await
        .expect("failed to serve axum");
}

/// Get the feeling of using maud
async fn hi_maud() -> Markup {
    html! {
        p { "Hi maud!" }
    }
}

/// Get the feeling of using htmx
fn hi_htmx() -> Router {
    return Router::new()
        .route("/hi-htmx", get(hi_htmx))
        .route(CLICKED_PATH, get(on_button_click));

    const CLICK_DIV: &str = "click-div";
    const CLICKED_PATH: &str = "/clicked";
    const CLICK_INDICATOR: &str = "click-spinner";
    // The base HTML
    async fn hi_htmx() -> Markup {
        html! {
            head {
                script src="https://unpkg.com/htmx.org@1.9.10"
                    integrity="sha384-D1Kt99CQMDuVetoL1lrYwg5t+9QdHe7NLX/SoJYkXDFfX37iInKRy5xLSi8nO7UC"
                    crossorigin="anonymous" {}
            }
            body {
                p { "Hi htmx!" }
                div id=(CLICK_DIV) {
                    // State transition
                    button hx-get=(CLICKED_PATH)
                        hx-trigger="click"
                        hx-target=(&referred_id(CLICK_DIV))
                        hx-swap="outerHTML"
                        hx-indicator=(&referred_id(CLICK_INDICATOR))
                        {
                            "Click Me!"
                        }
                    img id=(CLICK_INDICATOR) class="htmx-indicator" src="http://samherbert.net/svg-loaders/svg-loaders/oval.svg" {}
                }
            }
        }
    }
    /// Check out the swapping mechanism in htmx
    async fn on_button_click() -> Markup {
        html! {
            p { "You have clicked the button!" }
        }
    }
}

/// Put the return value to the HTML ID tag
pub fn referred_id(id: &str) -> String {
    format!("#{id}")
}

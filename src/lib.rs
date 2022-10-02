use crate::models::Splat3Response;
use reqwest::Error;
use serde::Deserialize;
use serde_json::json;
use std::ops::Add;
use std::thread::current;
use worker::*;

mod models;
mod utils;

/// Paint is everywhere
/// Splatting it everyplace
/// I Disconnected
///                 - taniwha3
fn log_request(req: &Request) {
    console_log!(
        "{} - [{}]",
        Date::now().to_string(),
        req.path() // req.cf().coordinates().unwrap_or_default(),
                   // req.cf().region().unwrap_or_else(|| "unknown region".into())
    );
}

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    log_request(&req);

    // Optionally, get more helpful error messages written to the console in the case of a panic.
    utils::set_panic_hook();

    // Optionally, use the Router to handle matching endpoints, use ":name" placeholders, or "*name"
    // catch-alls to match on specific patterns. Alternatively, use `Router::with_data(D)` to
    // provide arbitrary data that will be accessible in each route via the `ctx.data()` method.
    let router = Router::new();

    // Add as many routes as your Worker needs! Each route will get a `Request` for handling HTTP
    // functionality and a `RouteContext` which you can use to  and get route parameters and
    // Environment bindings like KV Stores, Durable Objects, Secrets, and Variables.
    router
        .get_async("/", |_, _| async move {
            let request_url = "https://splatoon3.ink/data/schedules.json".to_string();

            let response = reqwest::get(&request_url)
                .await
                .expect("Failed to fetch")
                .json::<Splat3Response>()
                .await
                .expect("Failed to deserialize");

            let mut output = String::new();

            output.push_str("🦑 Current maps: ");

            let current_stages = &response.data.regular_schedules.nodes[0]
                .regular_match_setting.vs_stages;
            let current_stages_output = current_stages
                .iter()
                .map(|stage| stage.name.clone())
                .collect::<Vec<String>>()
                .join(", ");
            output.push_str(&current_stages_output);

            output.push_str(". Next maps: ");

            let next_stages = &response.data.regular_schedules.nodes[1]
                .regular_match_setting.vs_stages;
            let next_stages_output = next_stages
                .iter()
                .map(|stage| stage.name.clone())
                .collect::<Vec<String>>()
                .join(", ");

            // AlienatedWorker: I don't think you need that "&*", I think "&" should be ok
            // codyphobe: And it wasnt assigned to output so it was thrown away
            output.push_str(&next_stages_output);
            output.push_str(" 🐙");

            Response::ok(output)
        })
        .post_async("/form/:field", |mut req, ctx| async move {
            if let Some(name) = ctx.param("field") {
                let form = req.form_data().await?;

                return match form.get(name) {
                    Some(FormEntry::Field(value)) => Response::from_json(&json!({ name: value })),
                    Some(FormEntry::File(_)) => {
                        Response::error("`field` param in form shouldn't be a File", 422)
                    }
                    None => Response::error("Bad Request", 400),
                };
            }

            Response::error("Bad Request", 400)
        })
        .get("/worker-version", |_, ctx| {
            let version = ctx.var("WORKERS_RS_VERSION")?.to_string();
            Response::ok(version)
        })
        .run(req, env)
        .await
}

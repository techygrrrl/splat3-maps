use worker::*;

use crate::models::Splat3Response;

mod models;
mod utils;

/// Paint is everywhere
/// Splatting it everyplace
/// I Disconnected
///                 - taniwha3
///
/// Final output should be something like this:
///
///     🦑 Current maps: Scorch Gorge, MakoMart. Next maps: Mincemeat Metalworks, Museum d'Alfonsino 🐙
fn log_request(req: &Request) {
    console_log!("{} - [{}]", Date::now().to_string(), req.path());
}

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    log_request(&req);

    utils::set_panic_hook();

    let router = Router::new();

    router
        .get_async("/", |_, _| async move {
            // Make a GET request to the maps data endpoint
            let request_url = "https://splatoon3.ink/data/schedules.json".to_string();
            let response = reqwest::get(&request_url)
                .await
                .expect("Failed to fetch")
                .json::<Splat3Response>()
                .await
                .expect("Failed to deserialize");

            // This will be the final output
            let mut output = String::new();
            output.push_str("🦑 Current maps: ");

            // Current stages are in the first node
            let current_stages = &response.data.regular_schedules.nodes[0]
                .regular_match_setting
                .vs_stages;
            let current_stages_output = current_stages
                .iter()
                .map(|stage| stage.name.clone())
                .collect::<Vec<String>>()
                .join(", ");
            output.push_str(&current_stages_output);

            output.push_str(". Next maps: ");

            // Next stages are in the 2nd node
            let next_stages = &response.data.regular_schedules.nodes[1]
                .regular_match_setting
                .vs_stages;
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
        .run(req, env)
        .await
}

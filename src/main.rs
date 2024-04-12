mod blackhawk_models;
mod service;

#[tokio::main]
async fn main() {
    service::service::api().await;

    todo!();
}
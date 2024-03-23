mod blackhawk;
mod service;

#[tokio::main]
async fn main() {
    service::service::api().await;

    todo!();
}
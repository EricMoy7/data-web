use std::error::Error;

use async_graphql::{dynamic::*, http::GraphiQLSource};
use async_graphql_axum::*;
use axum::response::IntoResponse;
use tokio::net::TcpListener;

async fn main() -> Result<(), Box<dyn Error>> {
    let query = Object::new("Query").field(Field::new(
        "howdy",
        TypeRef::named_nn(TypeRef::STRING),
        |_| FieldFuture::new(async { "partner" }),
    ));

    // create the schema
    let schema = Schema::build(query, None, None).register(query).finish()?;

    // start the http server
    let app = Route::new().at("/", get(graphiql).post(GraphQL::new(schema)));
    println!("GraphiQL: http://localhost:8000");
    Server::new(TcpListener::bind("0.0.0.0:8000"))
        .run(app)
        .await?;
    Ok(())
}
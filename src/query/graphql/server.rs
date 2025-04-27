// GraphQL server
use actix_web::{App, HttpResponse, HttpServer, guard, web};
use anyhow::{Context, Result};
use async_graphql::http::{GraphQLPlaygroundConfig, playground_source};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};

use crate::query::service::QueryService;

use super::schema::{AppSchema, create_schema};

pub async fn start_graphql_server() -> Result<()> {
    // Create QueryService
    let service = QueryService::new().context("Failed to create query service")?;
    let schema = create_schema(service);

    let http_server = HttpServer::new(move || {
        // TODO: configure CORS
        App::new()
            .app_data(web::Data::new(schema.clone()))
            .service(
                web::resource("/graphql")
                    .guard(guard::Post())
                    .to(graphql_handler),
            )
            .service(
                web::resource("/graphql")
                    .guard(guard::Get())
                    .to(graphql_playground),
            )
    })
    .bind("127.0.0.1:8080")?;

    Ok(http_server.run().await?)
}

// Handle graphql queries
async fn graphql_handler(schema: web::Data<AppSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

// Serve GraphQL playground
async fn graphql_playground() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
}

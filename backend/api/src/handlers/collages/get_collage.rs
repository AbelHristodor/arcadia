use crate::Arcadia;
use actix_web::{
    web::{Data, Query},
    HttpResponse,
};
use arcadia_common::error::Result;
use arcadia_storage::{models::collage::CollageAndAssociatedData, redis::RedisPoolInterface};
use serde::Deserialize;
use utoipa::IntoParams;

#[derive(Debug, Deserialize, IntoParams)]
pub struct GetCollageQuery {
    pub id: i64,
}

#[utoipa::path(
    get,
    operation_id = "Get a collage",
    tag = "Collages",
    path = "/api/collages",
    security(
      ("http" = ["Bearer"])
    ),
    responses(
        (status = 200, description = "Collage information and its entries", body=CollageAndAssociatedData),
    )
)]
pub async fn exec<R: RedisPoolInterface + 'static>(
    query: Query<GetCollageQuery>,
    arc: Data<Arcadia<R>>,
) -> Result<HttpResponse> {
    let collage = arc.pool.find_collage_and_associated_data(&query.id).await?;

    Ok(HttpResponse::Created().json(collage))
}

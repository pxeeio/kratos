use super::{Controller, Result};
use crate::prelude::*;
use crate::http::resources::{ModelResource, MuscleGroupResource};
use crate::http::response::JsonResponse;
use crate::models::MuscleGroup;
use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::{get, Router};
use database::{DatabaseManager, Model};

pub struct MuscleGroupController;

impl Controller for MuscleGroupController {
    fn router(state: DatabaseManager) -> Router {
        Router::new()
            .route("/", get(Self::list))
            .with_state(state)
    }
}

impl MuscleGroupController {
    pub async fn list(
        State(database): State<DatabaseManager>,
    ) -> Result<JsonResponse> {
        let groups = MuscleGroup::query()
            .select(&["*"])
            .order_by("name", true)
            .all(database.connection())
            .await?;

        Ok(JsonResponse::success(
            Some(MuscleGroupResource::list(groups, &database).await),
            StatusCode::OK,
        ))
    }
}
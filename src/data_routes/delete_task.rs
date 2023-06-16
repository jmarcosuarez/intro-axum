use crate::database::tasks::Entity as Tasks;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
};
use sea_orm::{DatabaseConnection, EntityTrait, IntoActiveModel, Set};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct QueryParams {
    soft: bool,
}

pub async fn delete_task(
    Path(task_id): Path<i32>,
    State(database): State<DatabaseConnection>,
    Query(query_params): Query<QueryParams>,
) -> Result<(), StatusCode> {
    // 1. This is one way - return 404 if not found as well

    // let task = if let Some(task) = Tasks::find_by_id(task_id)
    //     .one(&database)
    //     .await
    //     .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    // {
    //     task.into_active_model()
    // } else {
    //     return Err(StatusCode::NOT_FOUND);
    // };

    // Tasks::delete(task)
    //     .exec(&database)
    //     .await
    //     .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Ok(())
    if query_params.soft {
        let mut task = if let Some(task) = Tasks::find_by_id(task_id)
            .one(&database)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        {
            task.into_active_model()
        } else {
            return Err(StatusCode::NOT_FOUND);
        };
        let now = chrono::Utc::now();
        task.deleted_at = Set(Some(now.into()));
        Tasks::update(task)
            .exec(&database)
            .await
            .map_err(|_err| StatusCode::INTERNAL_SERVER_ERROR)?;
    } else {
        //  2. with 200 OK regardless - instead of 404
        Tasks::delete_by_id(task_id)
            .exec(&database)
            .await
            .map_err(|_err| StatusCode::INTERNAL_SERVER_ERROR)?;
    }
    // 3. Delete using delete_many()
    // Tasks::delete_many()
    //     .filter(tasks::Column::Id.eq(task_id))
    //     .exec(&database)
    //     .await
    //     .map_err(|_err| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(())
}

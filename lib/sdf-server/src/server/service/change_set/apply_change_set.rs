use super::ChangeSetResult;
use crate::server::extract::{AccessBuilder, HandlerContext, PosthogClient};
use crate::server::service::change_set::ChangeSetError;
use crate::server::tracking::track;
use axum::extract::OriginalUri;
use axum::Json;
use dal::{ChangeSet, ChangeSetPk};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ApplyChangeSetRequest {
    pub change_set_pk: ChangeSetPk,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ApplyChangeSetResponse {
    pub change_set: ChangeSet,
}

pub async fn apply_change_set(
    HandlerContext(builder): HandlerContext,
    AccessBuilder(access_builder): AccessBuilder,
    PosthogClient(posthog_client): PosthogClient,
    OriginalUri(original_uri): OriginalUri,
    Json(request): Json<ApplyChangeSetRequest>,
) -> ChangeSetResult<Json<ApplyChangeSetResponse>> {
    let mut ctx = builder.build_head(access_builder).await?;

    let mut change_set = ChangeSet::get_by_pk(&ctx, &request.change_set_pk)
        .await?
        .ok_or(ChangeSetError::ChangeSetNotFound)?;
    change_set.apply(&mut ctx).await?;

    track(
        &posthog_client,
        &ctx,
        &original_uri,
        "apply_change_set",
        serde_json::json!({
            "merged_change_set": request.change_set_pk,
        }),
    );

    ctx.commit().await?;

    /* temporarily disabling workspace backups
    tokio::task::spawn(
        super::upload_workspace_backup_module(ctx, raw_access_token)
            .instrument(info_span!("Workspace backup module upload")),
    );
    */

    Ok(Json(ApplyChangeSetResponse { change_set }))
}

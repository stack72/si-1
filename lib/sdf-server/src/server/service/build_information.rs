use axum::{Json, response::{IntoResponse, Response}};
use hyper::StatusCode;
use serde::Serialize;
use thiserror::Error;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BuildInformation {
    git_sha: &'static str,

    cargo_debug: &'static str,
    cargo_features: &'static str,
    cargo_opt_level: &'static str,
    cargo_target_triple: &'static str,

    rustc_channel: &'static str,
    rustc_commit_date: &'static str,
    rustc_commit_hash: &'static str,
    rustc_host_triple: &'static str,
    rustc_llvm_version: &'static str,
    rustc_semver: &'static str,
}

/// There really isn't much of anything that can go wrong here as long
/// as everything compiles.
#[derive(Error, Debug)]
pub enum BuildInformationError {
}

impl IntoResponse for BuildInformationError {
    fn into_response(self) -> Response {
        let status = StatusCode::INTERNAL_SERVER_ERROR;
        (
            status,
            Json(serde_json::json!({
                "error": {
                    "message": "Unable to get build information",
                    "statusCode": status.as_u16()
                }
            }))
        ).into_response()
    }
}

pub type BuildInformationResult<T> = std::result::Result<T, BuildInformationError>;

pub async fn get_build_information() -> BuildInformationResult<Json<BuildInformation>> {
    Ok(Json(BuildInformation {
        git_sha: env!("VERGEN_GIT_SHA"),

        cargo_debug: env!("VERGEN_CARGO_DEBUG"),
        cargo_features: env!("VERGEN_CARGO_FEATURES"),
        cargo_opt_level: env!("VERGEN_CARGO_OPT_LEVEL"),
        cargo_target_triple: env!("VERGEN_CARGO_TARGET_TRIPLE"),

        rustc_channel: env!("VERGEN_RUSTC_CHANNEL"),
        rustc_commit_date: env!("VERGEN_RUSTC_COMMIT_DATE"),
        rustc_commit_hash: env!("VERGEN_RUSTC_COMMIT_HASH"),
        rustc_host_triple: env!("VERGEN_RUSTC_HOST_TRIPLE"),
        rustc_llvm_version: env!("VERGEN_RUSTC_LLVM_VERSION"),
        rustc_semver: env!("VERGEN_RUSTC_SEMVER"),
    }))
}

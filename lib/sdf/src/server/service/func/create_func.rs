use super::{FuncResult, FuncVariant};
use crate::server::extract::{AccessBuilder, HandlerContext};
use crate::service::func::FuncError;
use axum::Json;
use dal::{
    generate_name, prototype_context::HasPrototypeContext, AttributeValueId, ComponentId,
    ConfirmationPrototype, DalContext, Func, FuncBackendKind, FuncBackendResponseType, FuncId,
    SchemaId, SchemaVariantId, StandardModel, Visibility, WsEvent,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum CreateFuncOptions {
    #[serde(rename_all = "camelCase")]
    AttributeOptions {
        value_id: Option<AttributeValueId>,
        parent_value_id: Option<AttributeValueId>,
        component_id: Option<ComponentId>,
        schema_variant_id: Option<SchemaVariantId>,
        schema_id: Option<SchemaId>,
        current_func_id: Option<FuncId>,
    },
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateFuncRequest {
    variant: FuncVariant,
    options: Option<CreateFuncOptions>,
    #[serde(flatten)]
    pub visibility: Visibility,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateFuncResponse {
    pub id: FuncId,
    pub handler: Option<String>,
    pub variant: FuncVariant,
    pub name: String,
    pub code: Option<String>,
}

pub static DEFAULT_ATTRIBUTE_CODE_HANDLER: &str = "setAttribute";
pub static DEFAULT_ATTRIBUTE_CODE: &str = include_str!("./defaults/attribute.ts");
pub static DEFAULT_CODE_GENERATION_HANDLER: &str = "generateCode";
pub static DEFAULT_CODE_GENERATION_CODE: &str = include_str!("./defaults/code_generation.ts");
pub static DEFAULT_QUALIFICATION_HANDLER: &str = "qualification";
pub static DEFAULT_QUALIFICATION_CODE: &str = include_str!("./defaults/qualification.ts");
pub static DEFAULT_CONFIRMATION_HANDLER: &str = "confirm";
pub static DEFAULT_CONFIRMATION_CODE: &str = include_str!("./defaults/confirmation.ts");
pub static DEFAULT_COMMAND_HANDLER: &str = "command";
pub static DEFAULT_COMMAND_CODE: &str = include_str!("./defaults/command.ts");
pub static DEFAULT_VALIDATION_HANDLER: &str = "validate";
pub static DEFAULT_VALIDATION_CODE: &str = include_str!("./defaults/validation.ts");

async fn create_validation_func(ctx: &DalContext) -> FuncResult<Func> {
    let mut func = Func::new(
        ctx,
        generate_name(),
        FuncBackendKind::JsValidation,
        FuncBackendResponseType::Validation,
    )
    .await?;

    func.set_code_plaintext(ctx, Some(DEFAULT_VALIDATION_CODE))
        .await?;
    func.set_handler(ctx, Some(DEFAULT_VALIDATION_HANDLER))
        .await?;

    Ok(func)
}

async fn create_command_func(ctx: &DalContext) -> FuncResult<Func> {
    let mut func = Func::new(
        ctx,
        generate_name(),
        FuncBackendKind::JsCommand,
        FuncBackendResponseType::Command,
    )
    .await?;

    func.set_code_plaintext(ctx, Some(DEFAULT_COMMAND_CODE))
        .await?;
    func.set_handler(ctx, Some(DEFAULT_COMMAND_HANDLER)).await?;

    Ok(func)
}

async fn create_confirmation_func(ctx: &DalContext) -> FuncResult<Func> {
    let mut func = Func::new(
        ctx,
        generate_name(),
        FuncBackendKind::JsConfirmation,
        FuncBackendResponseType::Confirmation,
    )
    .await?;

    func.set_code_plaintext(ctx, Some(DEFAULT_CONFIRMATION_CODE))
        .await?;
    func.set_handler(ctx, Some(DEFAULT_CONFIRMATION_HANDLER))
        .await?;

    ConfirmationPrototype::new(
        ctx,
        func.display_name().unwrap_or("unknown"),
        *func.id(),
        ConfirmationPrototype::new_context(),
    )
    .await?;

    Ok(func)
}

async fn create_attribute_func(ctx: &DalContext, variant: FuncVariant) -> FuncResult<Func> {
    let (code, handler, response_type) = match variant {
        FuncVariant::Attribute => (
            DEFAULT_ATTRIBUTE_CODE,
            DEFAULT_ATTRIBUTE_CODE_HANDLER,
            FuncBackendResponseType::Unset,
        ),
        FuncVariant::CodeGeneration => (
            DEFAULT_CODE_GENERATION_CODE,
            DEFAULT_CODE_GENERATION_HANDLER,
            FuncBackendResponseType::CodeGeneration,
        ),
        FuncVariant::Qualification => (
            DEFAULT_QUALIFICATION_CODE,
            DEFAULT_QUALIFICATION_HANDLER,
            FuncBackendResponseType::Qualification,
        ),
        _ => {
            return Err(FuncError::UnexpectedFuncVariantCreatingAttributeFunc(
                variant,
            ))
        }
    };

    let mut func = Func::new(ctx, generate_name(), variant.into(), response_type).await?;

    func.set_code_plaintext(ctx, Some(code)).await?;
    func.set_handler(ctx, Some(handler)).await?;

    Ok(func)
}

pub async fn create_func(
    HandlerContext(builder): HandlerContext,
    AccessBuilder(request_ctx): AccessBuilder,
    Json(request): Json<CreateFuncRequest>,
) -> FuncResult<Json<CreateFuncResponse>> {
    let ctx = builder.build(request_ctx.build(request.visibility)).await?;

    let func = match request.variant {
        FuncVariant::Attribute => create_attribute_func(&ctx, FuncVariant::Attribute).await?,
        FuncVariant::CodeGeneration => {
            create_attribute_func(&ctx, FuncVariant::CodeGeneration).await?
        }
        FuncVariant::Confirmation => create_confirmation_func(&ctx).await?,
        FuncVariant::Command => create_command_func(&ctx).await?,
        FuncVariant::Validation => create_validation_func(&ctx).await?,
        FuncVariant::Qualification => {
            create_attribute_func(&ctx, FuncVariant::Qualification).await?
        }
    };

    WsEvent::change_set_written(&ctx).publish(&ctx).await?;
    ctx.commit().await?;

    Ok(Json(CreateFuncResponse {
        id: func.id().to_owned(),
        handler: func.handler().map(|h| h.to_owned()),
        variant: (&func).try_into()?,
        name: func.name().to_owned(),
        code: func.code_plaintext()?,
    }))
}

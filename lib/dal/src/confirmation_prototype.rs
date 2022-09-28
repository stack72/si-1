use serde::{Deserialize, Serialize};
use si_data::PgError;
use telemetry::prelude::*;
use thiserror::Error;

use crate::{
    func::backend::js_confirmation::{ConfirmationResult, FuncBackendJsConfirmationArgs},
    func::FuncId,
    impl_standard_model, pk, standard_model, standard_model_accessor, ActionPrototype, Component,
    ComponentError, ComponentId, ConfirmationResolver, ConfirmationResolverContext,
    ConfirmationResolverError, DalContext, FuncBinding, FuncBindingError, HistoryEventError,
    SchemaId, SchemaVariantId, StandardModel, StandardModelError, SystemId, Timestamp, Visibility,
    WriteTenancy,
};

#[derive(Error, Debug)]
pub enum ConfirmationPrototypeError {
    #[error(transparent)]
    Pg(#[from] PgError),
    #[error(transparent)]
    HistoryEvent(#[from] HistoryEventError),
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
    #[error(transparent)]
    StandardModel(#[from] StandardModelError),
    #[error(transparent)]
    Component(#[from] ComponentError),
    #[error(transparent)]
    FuncBinding(#[from] FuncBindingError),
    #[error(transparent)]
    ConfirmationResolver(#[from] ConfirmationResolverError),
}

pub type ConfirmationPrototypeResult<T> = Result<T, ConfirmationPrototypeError>;

const FIND_FOR_CONTEXT: &str =
    include_str!("./queries/confirmation_prototype_find_for_context.sql");

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct ConfirmationPrototypeContext {
    pub component_id: ComponentId,
    pub schema_id: SchemaId,
    pub schema_variant_id: SchemaVariantId,
    pub system_id: SystemId,
}

// Hrm - is this a universal resolver context? -- Adam
impl Default for ConfirmationPrototypeContext {
    fn default() -> Self {
        Self::new()
    }
}

impl ConfirmationPrototypeContext {
    pub fn new() -> Self {
        Self {
            component_id: ComponentId::NONE,
            schema_id: SchemaId::NONE,
            schema_variant_id: SchemaVariantId::NONE,
            system_id: SystemId::NONE,
        }
    }

    pub fn new_for_context_field(context_field: ConfirmationPrototypeContextField) -> Self {
        match context_field {
            ConfirmationPrototypeContextField::Schema(schema_id) => ConfirmationPrototypeContext {
                component_id: ComponentId::NONE,
                schema_id,
                schema_variant_id: SchemaVariantId::NONE,
                system_id: SystemId::NONE,
            },
            ConfirmationPrototypeContextField::System(system_id) => ConfirmationPrototypeContext {
                component_id: ComponentId::NONE,
                schema_id: SchemaId::NONE,
                schema_variant_id: SchemaVariantId::NONE,
                system_id,
            },
            ConfirmationPrototypeContextField::SchemaVariant(schema_variant_id) => {
                ConfirmationPrototypeContext {
                    component_id: ComponentId::NONE,
                    schema_id: SchemaId::NONE,
                    schema_variant_id,
                    system_id: SystemId::NONE,
                }
            }
            ConfirmationPrototypeContextField::Component(component_id) => {
                ConfirmationPrototypeContext {
                    component_id,
                    schema_id: SchemaId::NONE,
                    schema_variant_id: SchemaVariantId::NONE,
                    system_id: SystemId::NONE,
                }
            }
        }
    }

    pub fn component_id(&self) -> ComponentId {
        self.component_id
    }

    pub fn set_component_id(&mut self, component_id: ComponentId) {
        self.component_id = component_id;
    }

    pub fn schema_id(&self) -> SchemaId {
        self.schema_id
    }

    pub fn set_schema_id(&mut self, schema_id: SchemaId) {
        self.schema_id = schema_id;
    }

    pub fn schema_variant_id(&self) -> SchemaVariantId {
        self.schema_variant_id
    }

    pub fn set_schema_variant_id(&mut self, schema_variant_id: SchemaVariantId) {
        self.schema_variant_id = schema_variant_id;
    }

    pub fn system_id(&self) -> SystemId {
        self.system_id
    }

    pub fn set_system_id(&mut self, system_id: SystemId) {
        self.system_id = system_id;
    }
}

pk!(ConfirmationPrototypePk);
pk!(ConfirmationPrototypeId);

// An ConfirmationPrototype joins a `Func` to the context in which
// the component that is created with it can use to generate a ConfirmationResolver.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct ConfirmationPrototype {
    pk: ConfirmationPrototypePk,
    id: ConfirmationPrototypeId,
    func_id: FuncId,
    component_id: ComponentId,
    schema_id: SchemaId,
    schema_variant_id: SchemaVariantId,
    system_id: SystemId,
    #[serde(flatten)]
    tenancy: WriteTenancy,
    #[serde(flatten)]
    timestamp: Timestamp,
    #[serde(flatten)]
    visibility: Visibility,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ConfirmationPrototypeContextField {
    Component(ComponentId),
    Schema(SchemaId),
    SchemaVariant(SchemaVariantId),
    System(SystemId),
}

impl From<ComponentId> for ConfirmationPrototypeContextField {
    fn from(component_id: ComponentId) -> Self {
        ConfirmationPrototypeContextField::Component(component_id)
    }
}

impl From<SchemaId> for ConfirmationPrototypeContextField {
    fn from(schema_id: SchemaId) -> Self {
        ConfirmationPrototypeContextField::Schema(schema_id)
    }
}

impl From<SchemaVariantId> for ConfirmationPrototypeContextField {
    fn from(schema_variant_id: SchemaVariantId) -> Self {
        ConfirmationPrototypeContextField::SchemaVariant(schema_variant_id)
    }
}

impl From<SystemId> for ConfirmationPrototypeContextField {
    fn from(system_id: SystemId) -> Self {
        ConfirmationPrototypeContextField::System(system_id)
    }
}

impl_standard_model! {
    model: ConfirmationPrototype,
    pk: ConfirmationPrototypePk,
    id: ConfirmationPrototypeId,
    table_name: "confirmation_prototypes",
    history_event_label_base: "confirmation_prototype",
    history_event_message_name: "Confirmation Prototype"
}

impl ConfirmationPrototype {
    #[allow(clippy::too_many_arguments)]
    #[instrument(skip_all)]
    pub async fn new(
        ctx: &DalContext,
        func_id: FuncId,
        context: ConfirmationPrototypeContext,
    ) -> ConfirmationPrototypeResult<Self> {
        let row = ctx
            .txns()
            .pg()
            .query_one(
                "SELECT object FROM confirmation_prototype_create_v1($1, $2, $3, $4, $5, $6, $7)",
                &[
                    ctx.write_tenancy(),
                    ctx.visibility(),
                    &func_id,
                    &context.component_id(),
                    &context.schema_id(),
                    &context.schema_variant_id(),
                    &context.system_id(),
                ],
            )
            .await?;
        let object = standard_model::finish_create_from_row(ctx, row).await?;
        Ok(object)
    }

    pub async fn run(
        &self,
        ctx: &DalContext,
        component_id: ComponentId,
        system_id: SystemId,
    ) -> ConfirmationPrototypeResult<ConfirmationResolver> {
        let args = FuncBackendJsConfirmationArgs {
            component: Component::view(ctx, component_id, system_id).await?.into(),
        };

        let json_args = serde_json::to_value(args)?;
        let (func_binding, func_binding_return_value, _created) =
            FuncBinding::find_or_create_and_execute(ctx, json_args, self.func_id()).await?;

        let (success, message, recommended_actions) = if let Some(mut value) =
            func_binding_return_value
                .value()
                .map(ConfirmationResult::deserialize)
                .transpose()?
        {
            let mut recommended_actions = Vec::with_capacity(value.recommended_actions.len());
            for action_name in value.recommended_actions {
                let action = ActionPrototype::find_by_attr(ctx, "name", &action_name)
                    .await?
                    .pop();
                if let Some(action) = action {
                    recommended_actions.push(action);
                } else {
                    value.success = false;
                    value.message = Some(format!("Unable to find action {}", action_name));
                    recommended_actions.clear();
                    break;
                };
            }
            (value.success, value.message, recommended_actions)
        } else {
            (
                false,
                Some(format!(
                    "Unable to deserialize func_binding_return_value's value: {:?}",
                    func_binding_return_value.value()
                )),
                Vec::new(),
            )
        };

        let mut context = ConfirmationResolverContext::new();
        context.set_component_id(self.component_id);
        context.set_schema_id(self.schema_id);
        context.set_schema_variant_id(self.schema_variant_id);
        context.set_system_id(self.system_id);
        if let Some(mut resolver) =
            ConfirmationResolver::find_for_prototype(ctx, self.id(), context.clone())
                .await?
                .pop()
        {
            resolver.set_success(ctx, success).await?;
            resolver.set_message(ctx, message).await?;
            resolver.remove_all_recommended_actions(ctx).await?;

            for recommended_action in recommended_actions {
                resolver
                    .add_recommended_action(ctx, recommended_action.id())
                    .await?;
            }
            Ok(resolver)
        } else {
            Ok(ConfirmationResolver::new(
                ctx,
                *self.id(),
                success,
                message.as_deref(),
                recommended_actions,
                self.func_id(),
                *func_binding.id(),
                context,
            )
            .await?)
        }
    }

    standard_model_accessor!(func_id, Pk(FuncId), ConfirmationPrototypeResult);
    standard_model_accessor!(schema_id, Pk(SchemaId), ConfirmationPrototypeResult);
    standard_model_accessor!(
        schema_variant_id,
        Pk(SchemaVariantId),
        ConfirmationPrototypeResult
    );
    standard_model_accessor!(component_id, Pk(ComponentId), ConfirmationPrototypeResult);

    standard_model_accessor!(system_id, Pk(SystemId), ConfirmationPrototypeResult);

    #[allow(clippy::too_many_arguments)]
    pub async fn find_for_component(
        ctx: &DalContext,
        component_id: ComponentId,
        system_id: SystemId,
    ) -> ConfirmationPrototypeResult<Vec<Self>> {
        let component = Component::get_by_id(ctx, &component_id)
            .await?
            .ok_or(ComponentError::NotFound(component_id))?;
        let schema = component
            .schema(ctx)
            .await?
            .ok_or(ComponentError::SchemaNotFound)?;
        let schema_variant = component
            .schema_variant(ctx)
            .await?
            .ok_or(ComponentError::SchemaVariantNotFound)?;
        let rows = ctx
            .txns()
            .pg()
            .query(
                FIND_FOR_CONTEXT,
                &[
                    ctx.read_tenancy(),
                    ctx.visibility(),
                    &component_id,
                    &system_id,
                    schema_variant.id(),
                    schema.id(),
                ],
            )
            .await?;

        Ok(standard_model::objects_from_rows(rows)?)
    }

    pub fn context(&self) -> ConfirmationPrototypeContext {
        let mut context = ConfirmationPrototypeContext::new();
        context.set_component_id(self.component_id);
        context.set_schema_id(self.schema_id);
        context.set_schema_variant_id(self.schema_variant_id);
        context.set_system_id(self.system_id);

        context
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn context_builder() {
        let mut c = ConfirmationPrototypeContext::new();
        c.set_component_id(22.into());
        assert_eq!(c.component_id(), 22.into());
    }
}

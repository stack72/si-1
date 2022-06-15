use dal::attribute::context::AttributeContextBuilder;
use dal::func::binding::FuncBindingId;
use dal::func::binding_return_value::FuncBindingReturnValueId;

use dal::test_harness::{
    create_prop_of_kind_and_set_parent_with_name, create_prop_of_kind_with_name, create_schema,
    create_schema_variant_with_root,
};
use dal::{
    socket::SocketArity, AttributeContext, AttributePrototypeArgument, AttributeReadContext,
    AttributeValue, AttributeValueError, Component, ComponentId, ComponentView, Connection,
    DalContext, ExternalProvider, Func, FuncBinding, FuncId, InternalProvider, PropId, PropKind,
    SchemaId, SchemaKind, SchemaVariant, SchemaVariantId, SchematicKind, StandardModel,
};
use pretty_assertions_sorted::assert_eq_sorted;
use std::collections::HashMap;

use crate::dal::test;

#[test]
async fn inter_component_identity_update(ctx: &DalContext<'_, '_>) {
    // Setup both components used for inter component identity update.
    let esp_payload = setup_esp(ctx).await;
    let swings_payload = setup_swings(ctx).await;

    // Ensure that they look as we expect.
    assert_eq_sorted!(
        serde_json::json![{
            "domain": {
                "object": {
                    "intermediate": "zero",
                    "source": "zero",
                },
            },
            "si": {
                "name": "esp",
            },
        }], // expected
        esp_payload.component_view_properties(ctx).await // actual
    );
    assert_eq_sorted!(
        serde_json::json![{
            "domain": {
                "destination": "zero",
            },
            "si": {
                "name": "swings",
            },
        }], // expected
        swings_payload.component_view_properties(ctx).await // actual
    );

    // Collect the identity func information we need.
    let (identity_func_id, identity_func_binding_id, identity_func_binding_return_value_id) =
        setup_identity_func(ctx).await;

    // Setup the "esp" intra component update functionality from "source" to "intermediate".
    let intermediate_attribute_value = AttributeValue::find_for_context(
        ctx,
        esp_payload.attribute_read_context_with_prop_id("intermediate"),
    )
    .await
    .expect("cannot find attribute value")
    .expect("attribute value not found");
    let mut intermediate_attribute_prototype = intermediate_attribute_value
        .attribute_prototype(ctx)
        .await
        .expect("cannot find attribute prototype")
        .expect("attribute prototype not found");
    intermediate_attribute_prototype
        .set_func_id(ctx, identity_func_id)
        .await
        .expect("could not set func id on attribute prototype");
    let source_internal_provider =
        InternalProvider::get_for_prop(ctx, esp_payload.get_prop_id("source"))
            .await
            .expect("could not get internal provider")
            .expect("internal provider not found");
    AttributePrototypeArgument::new_for_intra_component(
        ctx,
        *intermediate_attribute_prototype.id(),
        "identity".to_string(),
        *source_internal_provider.id(),
    )
    .await
    .expect("could not create attribute prototype argument");

    // Update the "esp" field, "source", to see if the intra component connection works.
    let object_attribute_value = AttributeValue::find_for_context(
        ctx,
        esp_payload.attribute_read_context_with_prop_id("object"),
    )
    .await
    .expect("cannot find attribute value")
    .expect("attribute value not found");
    let source_attribute_value = AttributeValue::find_for_context(
        ctx,
        esp_payload.attribute_read_context_with_prop_id("source"),
    )
    .await
    .expect("cannot find attribute value")
    .expect("attribute value not found");
    let (_, updated_source_attribute_value_id, _) = AttributeValue::update_for_context(
        ctx,
        *source_attribute_value.id(),
        Some(*object_attribute_value.id()),
        esp_payload.attribute_context_with_prop_id("source"),
        Some(serde_json::to_value("one").expect("could not convert to serde_json::Value")),
        None,
    )
    .await
    .expect("could not update attribute value");

    // Create the "esp" external provider for inter component connection.
    let (esp_external_provider, _socket) = ExternalProvider::new_with_socket(
        ctx,
        esp_payload.schema_id,
        esp_payload.schema_variant_id,
        "output",
        None,
        identity_func_id,
        identity_func_binding_id,
        identity_func_binding_return_value_id,
        SocketArity::Many,
        SchematicKind::Component,
    )
    .await
    .expect("could not create external provider");
    let esp_intermediate_internal_provider =
        InternalProvider::get_for_prop(ctx, esp_payload.get_prop_id("intermediate"))
            .await
            .expect("could not get internal provider")
            .expect("internal provider not found");
    AttributePrototypeArgument::new_for_intra_component(
        ctx,
        *esp_external_provider
            .attribute_prototype_id()
            .expect("no attribute prototype id for external provider"),
        "identity".to_string(),
        *esp_intermediate_internal_provider.id(),
    )
    .await
    .expect("could not create attribute prototype argument");

    // Create the "swings" explicit internal provider for intra component connection.
    let swings_destination_attribute_value = AttributeValue::find_for_context(
        ctx,
        swings_payload.attribute_read_context_with_prop_id("destination"),
    )
    .await
    .expect("cannot find attribute value")
    .expect("attribute value not found");
    let mut swings_destination_attribute_prototype = swings_destination_attribute_value
        .attribute_prototype(ctx)
        .await
        .expect("could not find attribute prototype")
        .expect("attribute prototype not found");
    swings_destination_attribute_prototype
        .set_func_id(ctx, identity_func_id)
        .await
        .expect("could not set func id on attribute prototype");
    let (swings_explicit_internal_provider, _socket) = InternalProvider::new_explicit_with_socket(
        ctx,
        swings_payload.schema_id,
        swings_payload.schema_variant_id,
        "swings",
        identity_func_id,
        identity_func_binding_id,
        identity_func_binding_return_value_id,
        SocketArity::Many,
        SchematicKind::Component,
    )
    .await
    .expect("could not create explicit internal provider");
    AttributePrototypeArgument::new_for_intra_component(
        ctx,
        *swings_destination_attribute_prototype.id(),
        "identity".to_string(),
        *swings_explicit_internal_provider.id(),
    )
    .await
    .expect("could not create attribute prototype argument");

    // Ensure that both components look as we expect when not "connected". The creation of both the
    // "esp" external provider and the "swings" implicit internal provider should not affect intra
    // component identity update working.
    assert_eq_sorted!(
        serde_json::json![{
            "domain": {
                "object": {
                    "intermediate": "one",
                    "source": "one",
                },
            },
            "si": {
                "name": "esp",
            },
        }], // expected
        esp_payload.component_view_properties(ctx).await // actual
    );
    assert_eq_sorted!(
        serde_json::json![{
            "domain": {
                "destination": "zero",
            },
            "si": {
                "name": "swings",
            },
        }], // expected
        swings_payload.component_view_properties(ctx).await // actual
    );

    // Connect the two components.
    Connection::connect_providers(
        ctx,
        "identity".to_string(),
        *esp_external_provider.id(),
        esp_payload.component_id,
        *swings_explicit_internal_provider.id(),
        swings_payload.component_id,
    )
    .await
    .expect("could not connect providers");

    // Ensure that both components continue to look as we expect.
    assert_eq_sorted!(
        serde_json::json![{
            "domain": {
                "object": {
                    "intermediate": "one",
                    "source": "one",
                },
            },
            "si": {
                "name": "esp",
            },
        }], // expected
        esp_payload.component_view_properties(ctx).await // actual
    );
    assert_eq_sorted!(
        serde_json::json![{
            "domain": {
                "destination": "zero",
            },
            "si": {
                "name": "swings",
            },
        }], // expected
        swings_payload.component_view_properties(ctx).await // actual
    );

    // Update the "esp" field, "source", again.
    AttributeValue::update_for_context(
        ctx,
        updated_source_attribute_value_id,
        Some(*object_attribute_value.id()),
        esp_payload.attribute_context_with_prop_id("source"),
        Some(serde_json::to_value("two").expect("could not convert to serde_json::Value")),
        None,
    )
    .await
    .expect("could not update attribute value");

    // Observe that inter component identity updating work.
    assert_eq_sorted!(
        serde_json::json![{
            "domain": {
                "object": {
                    "intermediate": "two",
                    "source": "two",
                },
            },
            "si": {
                "name": "esp",
            },
        }], // expected
        esp_payload.component_view_properties(ctx).await // actual
    );
    assert_eq_sorted!(
        serde_json::json![{
            "domain": {
                "destination": "two",
            },
            "si": {
                "name": "swings",
            },
        }], // expected
        swings_payload.component_view_properties(ctx).await // actual
    );
}

// Get the identity func and execute. We will need it for multiple attribute prototypes.
async fn setup_identity_func(
    ctx: &DalContext<'_, '_>,
) -> (FuncId, FuncBindingId, FuncBindingReturnValueId) {
    let identity_func: Func = Func::find_by_attr(ctx, "name", &"si:identity".to_string())
        .await
        .expect("could not find func by name attr")
        .pop()
        .expect("identity func not found");
    let (identity_func_binding, identity_func_binding_return_value) =
        FuncBinding::find_or_create_and_execute(
            ctx,
            serde_json::json![{ "identity": null }],
            *identity_func.id(),
        )
        .await
        .expect("could not find or create identity func binding");
    (
        *identity_func.id(),
        *identity_func_binding.id(),
        *identity_func_binding_return_value.id(),
    )
}

/// Payload used for bundling a [`Component`](dal::Component) with all metadata needed for the test.
struct ComponentPayload {
    schema_id: SchemaId,
    schema_variant_id: SchemaVariantId,
    component_id: ComponentId,
    /// A map that uses [`Prop`](crate::Prop) names as keys and their ids as values. As a result,
    /// _two props cannot share the same name_ in these test [`Components`](crate::Component).
    prop_map: HashMap<String, PropId>,
    /// An [`AttributeReadContext`](dal::AttributeReadContext) that can be used for generating
    /// a [`ComponentView`](dal::ComponentView).
    base_attribute_read_context: AttributeReadContext,
}

impl ComponentPayload {
    fn get_prop_id(&self, prop_name: &str) -> PropId {
        *self
            .prop_map
            .get(prop_name)
            .expect("could not find PropId for key")
    }

    fn attribute_read_context_with_prop_id(&self, prop_name: &str) -> AttributeReadContext {
        AttributeReadContext {
            prop_id: Some(self.get_prop_id(prop_name)),
            ..self.base_attribute_read_context
        }
    }

    fn attribute_context_with_prop_id(&self, prop_name: &str) -> AttributeContext {
        AttributeContextBuilder::from(self.base_attribute_read_context)
            .set_prop_id(self.get_prop_id(prop_name))
            .to_context()
            .expect("could not convert builder to attribute context")
    }

    /// Generates a new [`ComponentView`](dal::ComponentView) and returns the "properites" field.
    async fn component_view_properties(&self, ctx: &DalContext<'_, '_>) -> serde_json::Value {
        ComponentView::for_context(ctx, self.base_attribute_read_context)
            .await
            .expect("cannot get component view")
            .properties
    }
}

// 38.805354552534816, -77.05091482877533
async fn setup_esp(ctx: &DalContext<'_, '_>) -> ComponentPayload {
    let mut schema = create_schema(ctx, &SchemaKind::Concrete).await;
    let (schema_variant, root_prop) = create_schema_variant_with_root(ctx, *schema.id()).await;
    schema
        .set_default_schema_variant_id(ctx, Some(*schema_variant.id()))
        .await
        .expect("cannot set default schema variant");

    // "esp"
    // domain: Object
    // └─ object: Object
    //    ├─ source: String
    //    └─ intermediate: String
    let object_prop = create_prop_of_kind_and_set_parent_with_name(
        ctx,
        PropKind::Object,
        "object",
        root_prop.domain_prop_id,
    )
    .await;
    let source_prop = create_prop_of_kind_and_set_parent_with_name(
        ctx,
        PropKind::String,
        "source",
        *object_prop.id(),
    )
    .await;
    let intermediate_prop = create_prop_of_kind_and_set_parent_with_name(
        ctx,
        PropKind::String,
        "intermediate",
        *object_prop.id(),
    )
    .await;
    let mut prop_map = HashMap::new();
    prop_map.insert(object_prop.name().to_string(), *object_prop.id());
    prop_map.insert(source_prop.name().to_string(), *source_prop.id());
    prop_map.insert(
        intermediate_prop.name().to_string(),
        *intermediate_prop.id(),
    );

    // Create the internal providers for a schema variant. Afterwards, we can create the component.
    SchemaVariant::create_implicit_internal_providers(ctx, *schema.id(), *schema_variant.id())
        .await
        .expect("could not create internal providers for schema variant");
    let (component, _, _) = Component::new_for_schema_with_node(ctx, "esp", schema.id())
        .await
        .expect("unable to create component");

    // This context can also be used for generating component views.
    let base_attribute_read_context = AttributeReadContext {
        prop_id: None,
        schema_id: Some(*schema.id()),
        schema_variant_id: Some(*schema_variant.id()),
        component_id: Some(*component.id()),
        ..AttributeReadContext::default()
    };

    // Initialize the value corresponding to the "source" prop.
    let unset_object_attribute_value = AttributeValue::find_for_context(
        ctx,
        AttributeReadContext {
            prop_id: Some(*object_prop.id()),
            ..base_attribute_read_context
        },
    )
    .await
    .expect("cannot get attribute value")
    .expect("attribute value not found");
    let source_attribute_value = AttributeValue::find_for_context(
        ctx,
        AttributeReadContext {
            prop_id: Some(*source_prop.id()),
            ..base_attribute_read_context
        },
    )
    .await
    .expect("cannot get attribute value")
    .expect("attribute value not found");
    let source_prop_context = AttributeContextBuilder::from(base_attribute_read_context)
        .set_prop_id(*source_prop.id())
        .to_context()
        .expect("could not convert builder to attribute context");
    let value = serde_json::to_value("zero").expect("could not convert to serde_json::Value");
    AttributeValue::update_for_context(
        ctx,
        *source_attribute_value.id(),
        Some(*unset_object_attribute_value.id()),
        source_prop_context,
        Some(value),
        None,
    )
    .await
    .expect("cannot update value for context");

    // Initialize the value corresponding to the "intermediate" prop.
    let set_object_attribute_value = AttributeValue::find_for_context(
        ctx,
        AttributeReadContext {
            prop_id: Some(*object_prop.id()),
            ..base_attribute_read_context
        },
    )
    .await
    .expect("cannot get attribute value")
    .expect("attribute value not found");
    let intermediate_attribute_value = AttributeValue::find_for_context(
        ctx,
        AttributeReadContext {
            prop_id: Some(*intermediate_prop.id()),
            ..base_attribute_read_context
        },
    )
    .await
    .expect("cannot get attribute value")
    .expect("attribute value not found");
    let intermediate_prop_context = AttributeContextBuilder::from(base_attribute_read_context)
        .set_prop_id(*intermediate_prop.id())
        .to_context()
        .expect("could not convert builder to attribute context");
    let value = serde_json::to_value("zero").expect("could not convert to serde_json::Value");
    AttributeValue::update_for_context(
        ctx,
        *intermediate_attribute_value.id(),
        Some(*set_object_attribute_value.id()),
        intermediate_prop_context,
        Some(value),
        None,
    )
    .await
    .expect("cannot set value for context");

    // Return the payload.
    ComponentPayload {
        schema_id: *schema.id(),
        schema_variant_id: *schema_variant.id(),
        component_id: *component.id(),
        prop_map,
        base_attribute_read_context,
    }
}

// 38.82091849697006, -77.05236860190759
async fn setup_swings(ctx: &DalContext<'_, '_>) -> ComponentPayload {
    let mut schema = create_schema(ctx, &SchemaKind::Concrete).await;
    let (schema_variant, root_prop) = create_schema_variant_with_root(ctx, *schema.id()).await;
    schema
        .set_default_schema_variant_id(ctx, Some(*schema_variant.id()))
        .await
        .expect("cannot set default schema variant");

    // "swings"
    // domain: Object
    // └─ destination: string
    let destination_prop = create_prop_of_kind_and_set_parent_with_name(
        ctx,
        PropKind::String,
        "destination",
        root_prop.domain_prop_id,
    )
    .await;
    let mut prop_map = HashMap::new();
    prop_map.insert(destination_prop.name().to_string(), *destination_prop.id());

    // Create the internal providers for a schema variant. Afterwards, we can create the component.
    SchemaVariant::create_implicit_internal_providers(ctx, *schema.id(), *schema_variant.id())
        .await
        .expect("could not create internal providers for schema variant");
    let (component, _, _) = Component::new_for_schema_with_node(ctx, "swings", schema.id())
        .await
        .expect("unable to create component");

    // This context can also be used for generating component views.
    let base_attribute_read_context = AttributeReadContext {
        prop_id: None,
        schema_id: Some(*schema.id()),
        schema_variant_id: Some(*schema_variant.id()),
        component_id: Some(*component.id()),
        ..AttributeReadContext::default()
    };

    // Initialize the value corresponding to the "destination" prop.
    let domain_attribute_value = AttributeValue::find_for_context(
        ctx,
        AttributeReadContext {
            prop_id: Some(root_prop.domain_prop_id),
            ..base_attribute_read_context
        },
    )
    .await
    .expect("cannot get attribute value")
    .expect("attribute value not found");
    let destination_attribute_value = AttributeValue::find_for_context(
        ctx,
        AttributeReadContext {
            prop_id: Some(*destination_prop.id()),
            ..base_attribute_read_context
        },
    )
    .await
    .expect("cannot get attribute value")
    .expect("attribute value not found");
    let destination_prop_context = AttributeContextBuilder::from(base_attribute_read_context)
        .set_prop_id(*destination_prop.id())
        .to_context()
        .expect("could not convert builder to attribute context");
    let value = serde_json::to_value("zero").expect("could not convert to serde_json::Value");
    AttributeValue::update_for_context(
        ctx,
        *destination_attribute_value.id(),
        Some(*domain_attribute_value.id()),
        destination_prop_context,
        Some(value),
        None,
    )
    .await
    .expect("cannot update value for context");

    // Return the payload.
    ComponentPayload {
        schema_id: *schema.id(),
        schema_variant_id: *schema_variant.id(),
        component_id: *component.id(),
        prop_map,
        base_attribute_read_context,
    }
}

#[test]
async fn with_deep_data_structure(ctx: &DalContext<'_, '_>) {
    let (identity_func_id, identity_func_binding_id, identity_func_binding_return_value_id) =
        setup_identity_func(ctx).await;

    let mut source_schema = create_schema(ctx, &SchemaKind::Concrete).await;
    let (source_schema_variant, source_root) =
        create_schema_variant_with_root(ctx, *source_schema.id()).await;
    source_schema
        .set_default_schema_variant_id(ctx, Some(*source_schema_variant.id()))
        .await
        .expect("cannot set default schema variant");

    let source_object_prop =
        create_prop_of_kind_with_name(ctx, PropKind::Object, "base_object").await;
    source_object_prop
        .set_parent_prop(ctx, source_root.domain_prop_id)
        .await
        .expect("cannot set parent of base_object");
    let source_foo_prop = create_prop_of_kind_with_name(ctx, PropKind::String, "foo_string").await;
    source_foo_prop
        .set_parent_prop(ctx, *source_object_prop.id())
        .await
        .expect("cannot set parent of foo_string");
    let source_bar_prop = create_prop_of_kind_with_name(ctx, PropKind::String, "bar_string").await;
    source_bar_prop
        .set_parent_prop(ctx, *source_object_prop.id())
        .await
        .expect("cannot set parent of bar_string");
    SchemaVariant::create_implicit_internal_providers(
        ctx,
        *source_schema.id(),
        *source_schema_variant.id(),
    )
    .await
    .expect("cannot create internal providers for source schema");
    let (source_external_provider, _socket) = ExternalProvider::new_with_socket(
        ctx,
        *source_schema.id(),
        *source_schema_variant.id(),
        "source_data",
        None,
        identity_func_id,
        identity_func_binding_id,
        identity_func_binding_return_value_id,
        SocketArity::Many,
        SchematicKind::Component,
    )
    .await
    .expect("cannot create source external provider");
    let source_internal_provider = InternalProvider::get_for_prop(ctx, *source_object_prop.id())
        .await
        .expect("cannot get source internal provider")
        .expect("source internal provider not found");
    AttributePrototypeArgument::new_for_intra_component(
        ctx,
        *source_external_provider
            .attribute_prototype_id()
            .expect("no attribute prototype id for external provider"),
        "identity".to_string(),
        *source_internal_provider.id(),
    )
    .await
    .expect("cannot create source external provider attribute prototype argument");

    let mut destination_schema = create_schema(ctx, &SchemaKind::Concrete).await;
    let (destination_schema_variant, destination_root) =
        create_schema_variant_with_root(ctx, *destination_schema.id()).await;
    destination_schema
        .set_default_schema_variant_id(ctx, Some(*destination_schema_variant.id()))
        .await
        .expect("cannot set default schema variant");

    let destination_parent_object_prop =
        create_prop_of_kind_with_name(ctx, PropKind::Object, "parent_object").await;
    destination_parent_object_prop
        .set_parent_prop(ctx, destination_root.domain_prop_id)
        .await
        .expect("cannot set parent of parent_object");
    let destination_object_prop =
        create_prop_of_kind_with_name(ctx, PropKind::Object, "base_object").await;
    destination_object_prop
        .set_parent_prop(ctx, *destination_parent_object_prop.id())
        .await
        .expect("cannot set parent of base_object");
    let destination_object_value = AttributeValue::find_for_context(
        ctx,
        AttributeReadContext {
            prop_id: Some(*destination_object_prop.id()),
            schema_id: Some(*destination_schema.id()),
            schema_variant_id: Some(*destination_schema_variant.id()),
            ..AttributeReadContext::default()
        },
    )
    .await
    .expect("cannot find destination attribute value")
    .expect("destination attribute value not found");
    let mut destination_object_prototype = destination_object_value
        .attribute_prototype(ctx)
        .await
        .expect("cannot find attribute prototype")
        .expect("attribute prototype not found");
    destination_object_prototype
        .set_func_id(ctx, identity_func_id)
        .await
        .expect("cannot set function on destination object prototype");
    let destination_foo_prop =
        create_prop_of_kind_with_name(ctx, PropKind::String, "foo_string").await;
    destination_foo_prop
        .set_parent_prop(ctx, *destination_object_prop.id())
        .await
        .expect("cannot set parent of foo_string");
    let destination_bar_prop =
        create_prop_of_kind_with_name(ctx, PropKind::String, "bar_string").await;
    destination_bar_prop
        .set_parent_prop(ctx, *destination_object_prop.id())
        .await
        .expect("cannot set parent of bar_string");
    SchemaVariant::create_implicit_internal_providers(
        ctx,
        *destination_schema.id(),
        *destination_schema_variant.id(),
    )
    .await
    .expect("cannot create internal providers for destination schema");
    let (destination_internal_provider, _socket) = InternalProvider::new_explicit_with_socket(
        ctx,
        *destination_schema.id(),
        *destination_schema_variant.id(),
        "destination_data",
        identity_func_id,
        identity_func_binding_id,
        identity_func_binding_return_value_id,
        SocketArity::One,
        SchematicKind::Component,
    )
    .await
    .expect("cannot create destination explicit internal provider");
    AttributePrototypeArgument::new_for_intra_component(
        ctx,
        *destination_object_prototype.id(),
        "identity".to_string(),
        *destination_internal_provider.id(),
    )
    .await
    .expect("cannot create prototype argument for destination");

    let (source_component, _, _) =
        Component::new_for_schema_with_node(ctx, "Source Component", source_schema.id())
            .await
            .expect("Unable to create source component");
    let source_attribute_read_context = AttributeReadContext {
        prop_id: None,
        schema_id: Some(*source_schema.id()),
        schema_variant_id: Some(*source_schema_variant.id()),
        component_id: Some(*source_component.id()),
        ..AttributeReadContext::default()
    };

    assert_eq_sorted!(
        serde_json::json![
            {
                "si": {
                    "name": "Source Component",
                },
                "domain": {},
            }
        ],
        ComponentView::for_context(ctx, source_attribute_read_context)
            .await
            .expect("cannot get source component view")
            .properties,
    );

    let (destination_component, _, _) =
        Component::new_for_schema_with_node(ctx, "Destination Component", destination_schema.id())
            .await
            .expect("Unable to create destination component");
    let destination_attribute_read_context = AttributeReadContext {
        prop_id: None,
        schema_id: Some(*destination_schema.id()),
        schema_variant_id: Some(*destination_schema_variant.id()),
        component_id: Some(*destination_component.id()),
        ..AttributeReadContext::default()
    };

    assert_eq_sorted!(
        serde_json::json![
            {
                "si": {
                    "name": "Destination Component",
                },
                "domain": {},
            }
        ],
        ComponentView::for_context(ctx, destination_attribute_read_context)
            .await
            .expect("cannot get destination component view")
            .properties,
    );

    Connection::connect_providers(
        ctx,
        "identity".to_string(),
        *source_external_provider.id(),
        *source_component.id(),
        *destination_internal_provider.id(),
        *destination_component.id(),
    )
    .await
    .expect("could not connect providers");

    let source_domain_attribute_value_id = *AttributeValue::find_for_context(
        ctx,
        AttributeReadContext {
            prop_id: Some(source_root.domain_prop_id),
            ..source_attribute_read_context
        },
    )
    .await
    .expect("cannot get source domain AttributeValue")
    .expect("source domain AttributeValue not found")
    .id();
    let source_object_attribute_value_id = *AttributeValue::find_for_context(
        ctx,
        AttributeReadContext {
            prop_id: Some(*source_object_prop.id()),
            ..source_attribute_read_context
        },
    )
    .await
    .expect("cannot get source object AttributeValue")
    .expect("source object AttributeValue not found")
    .id();
    let source_foo_attribute_value_id = *AttributeValue::find_for_context(
        ctx,
        AttributeReadContext {
            prop_id: Some(*source_foo_prop.id()),
            ..source_attribute_read_context
        },
    )
    .await
    .expect("cannot get source foo AttributeValue")
    .expect("source foo AttributeValue not found")
    .id();

    let source_foo_update_context = AttributeContext::builder()
        .set_prop_id(*source_foo_prop.id())
        .set_schema_id(*source_schema.id())
        .set_schema_variant_id(*source_schema_variant.id())
        .set_component_id(*source_component.id())
        .to_context()
        .expect("could not create source foo update context");

    AttributeValue::update_for_context(
        ctx,
        source_foo_attribute_value_id,
        Some(source_object_attribute_value_id),
        source_foo_update_context,
        Some(serde_json::to_value("deep update").expect("could not convert to serde_json::Value")),
        None,
    )
    .await
    .expect("cannot update source foo_string");

    assert_eq_sorted!(
        serde_json::json![
            {
                "si": {
                    "name": "Source Component",
                },
                "domain": {
                    "base_object": {
                        "foo_string": "deep update",
                    },
                },
            }
        ],
        ComponentView::for_context(ctx, source_attribute_read_context)
            .await
            .expect("cannot get source component view")
            .properties,
    );

    assert_eq_sorted!(
        serde_json::json![
            {
                "si": {
                    "name": "Destination Component",
                },
                "domain": {
                    "parent_object": {
                        "base_object": {
                            "foo_string": "deep update",
                        },
                    },
                },
            }
        ],
        ComponentView::for_context(ctx, destination_attribute_read_context)
            .await
            .expect("cannot get destination component view")
            .properties,
    );

    let source_object_attribute_value_id = *AttributeValue::find_for_context(
        ctx,
        AttributeReadContext {
            prop_id: Some(*source_object_prop.id()),
            ..source_attribute_read_context
        },
    )
    .await
    .expect("cannot get source object AttributeValue")
    .expect("source object AttributeValue not found")
    .id();
    let source_bar_attribute_value_id = *AttributeValue::find_for_context(
        ctx,
        AttributeReadContext {
            prop_id: Some(*source_bar_prop.id()),
            ..source_attribute_read_context
        },
    )
    .await
    .expect("cannot get source bar AttributeValue")
    .expect("source foo AttributeValue not found")
    .id();

    let source_bar_update_context = AttributeContext::builder()
        .set_prop_id(*source_bar_prop.id())
        .set_schema_id(*source_schema.id())
        .set_schema_variant_id(*source_schema_variant.id())
        .set_component_id(*source_component.id())
        .to_context()
        .expect("could not create source foo update context");

    AttributeValue::update_for_context(
        ctx,
        source_bar_attribute_value_id,
        Some(source_object_attribute_value_id),
        source_bar_update_context,
        Some(
            serde_json::to_value("another update").expect("could not convert to serde_json::Value"),
        ),
        None,
    )
    .await
    .expect("cannot update source bar_string");

    assert_eq_sorted!(
        serde_json::json![
            {
                "si": {
                    "name": "Source Component",
                },
                "domain": {
                    "base_object": {
                        "foo_string": "deep update",
                        "bar_string": "another update",
                    },
                },
            }
        ],
        ComponentView::for_context(ctx, source_attribute_read_context)
            .await
            .expect("cannot get source component view")
            .properties,
    );

    assert_eq_sorted!(
        serde_json::json![
            {
                "si": {
                    "name": "Destination Component",
                },
                "domain": {
                    "parent_object": {
                        "base_object": {
                            "foo_string": "deep update",
                            "bar_string": "another update",
                        },
                    },
                },
            }
        ],
        ComponentView::for_context(ctx, destination_attribute_read_context)
            .await
            .expect("cannot get destination component view")
            .properties,
    );
}
use crate::{
    builtins::schema::{
        create_schema, create_string_prop_with_default, kubernetes_metadata::create_metadata_prop,
    },
    code_generation_prototype::CodeGenerationPrototypeContext,
    func::backend::{
        js_code_generation::FuncBackendJsCodeGenerationArgs,
        js_qualification::FuncBackendJsQualificationArgs,
    },
    qualification_prototype::QualificationPrototypeContext,
    schema::{SchemaVariant, UiMenu},
    socket::{Socket, SocketArity, SocketEdgeKind, SocketKind},
    AttributeReadContext, BuiltinsResult, CodeGenerationPrototype, CodeLanguage, DalContext, Func,
    InternalProvider, Prop, PropId, PropKind, QualificationPrototype, Schema, SchemaError,
    SchemaKind, SchematicKind, StandardModel,
};

use super::{
    create_prop, kubernetes::doc_url, kubernetes_selector::create_selector_prop,
    setup_identity_func,
};

pub async fn kubernetes_deployment(ctx: &DalContext<'_, '_>) -> BuiltinsResult<()> {
    let name = "kubernetes_deployment".to_string();
    let mut schema = match create_schema(ctx, &name, &SchemaKind::Concrete).await? {
        Some(schema) => schema,
        None => return Ok(()),
    };

    let (mut variant, root_prop) = SchemaVariant::new(ctx, *schema.id(), "v0").await?;
    variant.set_color(ctx, Some(0x921ed6)).await?;
    variant
        .set_link(
            ctx,
            Some(doc_url(
                "reference/kubernetes-api/workload-resources/deployment-v1/",
            )),
        )
        .await?;

    schema
        .set_default_schema_variant_id(ctx, Some(*variant.id()))
        .await?;

    let base_attribute_read_context = AttributeReadContext {
        schema_id: Some(*schema.id()),
        schema_variant_id: Some(*variant.id()),
        ..AttributeReadContext::default()
    };

    // TODO: add validation (si-registry ensures the value is unchanged)
    let mut api_version_prop = create_string_prop_with_default(
        ctx,
        "apiVersion",
        "apps/v1".to_owned(),
        Some(root_prop.domain_prop_id),
        base_attribute_read_context,
    )
    .await?;
    api_version_prop
        .set_doc_link(
            ctx,
            Some(doc_url(
                "reference/kubernetes-api/workload-resources/deployment-v1/#Deployment",
            )),
        )
        .await?;

    // TODO: add validation (si-registry ensures the value is unchanged)
    let mut kind_prop = create_string_prop_with_default(
        ctx,
        "kind",
        "Deployment".to_owned(),
        Some(root_prop.domain_prop_id),
        base_attribute_read_context,
    )
    .await?;
    kind_prop
        .set_doc_link(
            ctx,
            Some(doc_url(
                "reference/kubernetes-api/workload-resources/deployment-v1/#Deployment",
            )),
        )
        .await?;

    let _metadata_prop = create_metadata_prop(
        ctx,
        true, // is name required, note: bool is not ideal here tho
        root_prop.domain_prop_id,
    )
    .await?;

    let _spec_prop = create_deployment_spec_prop(ctx, root_prop.domain_prop_id).await?;

    // Qualification Prototype
    let qualification_func_name = "si:qualificationYamlKubeval".to_owned();
    let mut qualification_funcs = Func::find_by_attr(ctx, "name", &qualification_func_name).await?;
    let qualification_func = qualification_funcs
        .pop()
        .ok_or(SchemaError::FuncNotFound(qualification_func_name))?;
    let qualification_args = FuncBackendJsQualificationArgs::default();
    let qualification_args_json = serde_json::to_value(&qualification_args)?;
    let mut qualification_prototype_context = QualificationPrototypeContext::new();
    qualification_prototype_context.set_schema_variant_id(*variant.id());

    let mut prototype = QualificationPrototype::new(
        ctx,
        *qualification_func.id(),
        qualification_args_json,
        qualification_prototype_context,
        "Kubeval YAML".to_owned(),
    )
    .await?;
    prototype
        .set_description(ctx, Some("Runs kubeval on the generated YAML".to_owned()))
        .await?;

    // Code Generation Prototype
    let code_generation_func_name = "si:generateYAML".to_owned();
    let mut code_generation_funcs =
        Func::find_by_attr(ctx, "name", &code_generation_func_name).await?;
    let code_generation_func = code_generation_funcs
        .pop()
        .ok_or(SchemaError::FuncNotFound(code_generation_func_name))?;
    let code_generation_args = FuncBackendJsCodeGenerationArgs::default();
    let code_generation_args_json = serde_json::to_value(&code_generation_args)?;
    let mut code_generation_prototype_context = CodeGenerationPrototypeContext::new();
    code_generation_prototype_context.set_schema_variant_id(*variant.id());

    let _prototype = CodeGenerationPrototype::new(
        ctx,
        *code_generation_func.id(),
        code_generation_args_json,
        CodeLanguage::Yaml,
        code_generation_prototype_context,
    )
    .await?;

    let identity_func = setup_identity_func(ctx).await?;

    let (_input_provider, mut input_socket) = InternalProvider::new_explicit_with_socket(
        ctx,
        *schema.id(),
        *variant.id(),
        "docker_image",
        identity_func.0,
        identity_func.1,
        identity_func.2,
        SocketArity::Many,
        SchematicKind::Component,
    )
    .await?;
    input_socket.set_color(ctx, Some(0xd61e8c)).await?;

    let (_input_provider, mut input_socket) = InternalProvider::new_explicit_with_socket(
        ctx,
        *schema.id(),
        *variant.id(),
        "kubernetes_namespace",
        identity_func.0,
        identity_func.1,
        identity_func.2,
        SocketArity::Many,
        SchematicKind::Component,
    )
    .await?;
    input_socket.set_color(ctx, Some(0x85c9a3)).await?;

    let includes_socket = Socket::new(
        ctx,
        "includes",
        SocketKind::Provider,
        &SocketEdgeKind::Includes,
        &SocketArity::Many,
        &SchematicKind::Component,
    )
    .await?;
    variant.add_socket(ctx, includes_socket.id()).await?;

    // TODO: abstract this boilerplate away
    let mut ui_menu = UiMenu::new(ctx, &(*schema.kind()).into()).await?;
    ui_menu.set_name(ctx, Some("deployment".to_owned())).await?;

    ui_menu.set_category(ctx, Some("kubernetes")).await?;
    ui_menu.set_schema(ctx, schema.id()).await?;

    let application_name = "application".to_string();
    let application_schema_results = Schema::find_by_attr(ctx, "name", &application_name).await?;
    let application_schema = application_schema_results
        .first()
        .ok_or(SchemaError::NotFoundByName(application_name))?;
    ui_menu
        .add_root_schematic(ctx, application_schema.id())
        .await?;

    Ok(())
}

async fn create_deployment_spec_prop(
    ctx: &DalContext<'_, '_>,
    parent_prop_id: PropId,
) -> BuiltinsResult<Prop> {
    let mut spec_prop = create_prop(ctx, "spec", PropKind::Object, Some(parent_prop_id)).await?;
    spec_prop
        .set_doc_link(
            ctx,
            Some(doc_url(
                "reference/kubernetes-api/workload-resources/deployment-v1/#DeploymentSpec",
            )),
        )
        .await?;

    let mut replicas_prop =
        create_prop(ctx, "replicas", PropKind::Integer, Some(*spec_prop.id())).await?;
    replicas_prop
        .set_doc_link(
            ctx,
            Some(doc_url(
                "reference/kubernetes-api/workload-resources/deployment-v1/#DeploymentSpec",
            )),
        )
        .await?;

    let _selector_prop = create_selector_prop(ctx, *spec_prop.id()).await?;
    let _template_prop = create_pod_template_spec_prop(ctx, *spec_prop.id()).await?;

    Ok(spec_prop)
}

async fn create_pod_template_spec_prop(
    ctx: &DalContext<'_, '_>,
    parent_prop_id: PropId,
) -> BuiltinsResult<Prop> {
    let mut template_prop =
        create_prop(ctx, "template", PropKind::Object, Some(parent_prop_id)).await?;
    template_prop
        .set_doc_link(
            ctx,
            Some(doc_url(
                "reference/kubernetes-api/workload-resources/pod-template-v1/#PodTemplateSpec",
            )),
        )
        .await?;

    let _metadata_prop = create_metadata_prop(
        ctx,
        true, // is name required, note: bool is not ideal here tho
        *template_prop.id(),
    )
    .await?;

    let _spec_prop = create_pod_spec_prop(ctx, *template_prop.id()).await?;

    Ok(template_prop)
}

async fn create_pod_spec_prop(
    ctx: &DalContext<'_, '_>,
    parent_prop_id: PropId,
) -> BuiltinsResult<Prop> {
    let mut spec_prop = create_prop(ctx, "spec", PropKind::Object, Some(parent_prop_id)).await?;
    spec_prop
        .set_doc_link(
            ctx,
            Some(doc_url(
                "reference/kubernetes-api/workload-resources/pod-v1/#PodSpec",
            )),
        )
        .await?;

    let mut containers_prop =
        create_prop(ctx, "containers", PropKind::Array, Some(*spec_prop.id())).await?;
    containers_prop
        .set_doc_link(
            ctx,
            Some(doc_url(
                "reference/kubernetes-api/workload-resources/pod-v1/#containers",
            )),
        )
        .await?;
    let _containers_element_prop = create_container_prop(ctx, *containers_prop.id()).await?;

    Ok(spec_prop)
}

async fn create_container_prop(
    ctx: &DalContext<'_, '_>,
    parent_prop_id: PropId,
) -> BuiltinsResult<Prop> {
    let mut container_prop =
        create_prop(ctx, "container", PropKind::Object, Some(parent_prop_id)).await?;
    container_prop
        .set_doc_link(
            ctx,
            Some(doc_url(
                "reference/kubernetes-api/workload-resources/pod-v1/#Container",
            )),
        )
        .await?;

    let mut name_prop =
        create_prop(ctx, "name", PropKind::String, Some(*container_prop.id())).await?;
    name_prop
        .set_doc_link(
            ctx,
            Some(doc_url(
                "reference/kubernetes-api/workload-resources/pod-v1/#Container",
            )),
        )
        .await?;

    let mut image_prop =
        create_prop(ctx, "image", PropKind::String, Some(*container_prop.id())).await?;
    image_prop
        .set_doc_link(
            ctx,
            Some(doc_url(
                "reference/kubernetes-api/workload-resources/pod-v1/#image",
            )),
        )
        .await?;

    let mut ports_prop =
        create_prop(ctx, "ports", PropKind::Array, Some(*container_prop.id())).await?;
    ports_prop
        .set_doc_link(
            ctx,
            Some(doc_url(
                "reference/kubernetes-api/workload-resources/pod-v1/#ports",
            )),
        )
        .await?;
    let _ports_element_prop = create_container_port_prop(ctx, *ports_prop.id()).await?;

    Ok(container_prop)
}

async fn create_container_port_prop(
    ctx: &DalContext<'_, '_>,
    parent_prop_id: PropId,
) -> BuiltinsResult<Prop> {
    let mut port_prop = create_prop(ctx, "port", PropKind::Object, Some(parent_prop_id)).await?;
    port_prop
        .set_doc_link(
            ctx,
            Some(doc_url(
                "reference/kubernetes-api/workload-resources/pod-v1/#ports",
            )),
        )
        .await?;

    let mut container_port_prop = create_prop(
        ctx,
        "containerPort",
        PropKind::Integer,
        Some(*port_prop.id()),
    )
    .await?;
    container_port_prop
        .set_doc_link(
            ctx,
            Some(doc_url(
                "reference/kubernetes-api/workload-resources/pod-v1/#ports",
            )),
        )
        .await?;

    let mut protocol_prop =
        create_prop(ctx, "protocol", PropKind::String, Some(*port_prop.id())).await?;
    protocol_prop
        .set_doc_link(
            ctx,
            Some(doc_url(
                "reference/kubernetes-api/workload-resources/pod-v1/#ports",
            )),
        )
        .await?;

    Ok(container_port_prop)
}
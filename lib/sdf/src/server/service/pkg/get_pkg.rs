use super::{pkg_open, PkgResult};
use crate::server::extract::{AccessBuilder, HandlerContext};
use axum::{extract::Query, Json};
use chrono::{DateTime, Utc};
use dal::{installed_pkg::InstalledPkg, PropKind, StandardModel, Visibility};
use serde::{Deserialize, Serialize};
use si_pkg::{SiPkgError, SiPkgProp};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PkgGetRequest {
    pub name: String,
    #[serde(flatten)]
    pub visibility: Visibility,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PkgGetResponse {
    pub name: String,
    pub hash: String,
    pub version: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub created_by: String,
    pub schemas: Vec<PkgSchemaVariant>,
    pub installed: bool,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PkgSchemaVariant {
    schema_name: String,
    props: Vec<PkgPropNode>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PkgPropNode {
    name: String,
    kind: PropKind,
    children: Vec<PkgPropNode>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PkgPropTreeBuilder {
    root: PkgPropNode,
}

impl Default for PkgPropTreeBuilder {
    fn default() -> Self {
        PkgPropTreeBuilder {
            root: PkgPropNode {
                name: "domain".to_string(),
                kind: PropKind::Object,
                children: vec![],
            },
        }
    }
}

impl PkgPropTreeBuilder {
    fn find_prop_by_path_mut(&mut self, path: &str) -> Option<&mut PkgPropNode> {
        let parts: Vec<&str> = path.split('/').filter(|part| !part.is_empty()).collect();

        if parts.is_empty() {
            return Some(&mut self.root);
        }

        // This is a little more complicated than makes me happy because of dancing around rust's
        // borrowing rules. If there's a happier path here, I'd love to know!
        let mut indexes = vec![];
        // Translate the path parts into indexes into the children
        {
            let mut cursor = &self.root;
            for part in parts {
                // we can't use iter_mut here because that borrows the whole of `children` mutably
                match cursor.children.iter().position(|p| p.name == part) {
                    None => return None,
                    Some(next_index) => {
                        indexes.push(next_index);
                        cursor = match cursor.children.get(next_index) {
                            Some(cursor) => cursor,
                            None => return None,
                        }
                    }
                }
            }
        }

        // Now that we have path parts translated to indexes, walk the tree using get_mut (for
        // segmented borrows) and find the droid we're looking for
        if !indexes.is_empty() {
            let mut mut_cursor = &mut self.root;
            for index in indexes {
                mut_cursor = match mut_cursor.children.get_mut(index) {
                    Some(cur) => cur,
                    None => return None,
                }
            }

            Some(mut_cursor)
        } else {
            None
        }
    }

    pub fn insert_prop(&mut self, parent_path: &str, prop: PkgPropNode) {
        if let Some(parent) = self.find_prop_by_path_mut(parent_path) {
            parent.children.push(prop);
        }
    }

    pub fn domain_children(&self) -> Vec<PkgPropNode> {
        self.root.children.clone()
    }
}

type PropVisitorContext = Arc<Mutex<PkgPropTreeBuilder>>;

async fn visit_prop<'a>(
    spec: SiPkgProp<'_>,
    parent_path: Option<String>,
    context: &'a PropVisitorContext,
) -> Result<Option<String>, SiPkgError> {
    let kind = match spec {
        SiPkgProp::String { .. } => PropKind::String,
        SiPkgProp::Number { .. } => PropKind::Integer,
        SiPkgProp::Boolean { .. } => PropKind::Boolean,
        SiPkgProp::Map { .. } => PropKind::Map,
        SiPkgProp::Array { .. } => PropKind::Array,
        SiPkgProp::Object { .. } => PropKind::Object,
    };

    let node = PkgPropNode {
        name: spec.name().to_string(),
        kind,
        children: vec![],
    };

    let insertion_path = parent_path.unwrap_or("".to_string());
    let new_path = format!("{}/{}", insertion_path, spec.name());
    context.lock().await.insert_prop(&insertion_path, node);

    Ok(Some(new_path))
}

pub async fn get_pkg(
    HandlerContext(builder): HandlerContext,
    AccessBuilder(request_ctx): AccessBuilder,
    Query(request): Query<PkgGetRequest>,
) -> PkgResult<Json<PkgGetResponse>> {
    let ctx = builder.build(request_ctx.build(request.visibility)).await?;
    let pkg = pkg_open(&builder, &request.name).await?;

    let schemas = pkg.schemas()?;

    let mut schema_variants = vec![];
    for schema in schemas {
        for variant in schema.variants()? {
            let props = Arc::new(Mutex::new(PkgPropTreeBuilder::default()));
            variant
                .visit_prop_tree_cloneable(visit_prop, None, &props)
                .await?;
            schema_variants.push(PkgSchemaVariant {
                schema_name: schema.name().to_string(),
                props: props.lock().await.domain_children(),
            });
        }
    }

    let metadata = pkg.metadata()?;
    let root_hash = pkg.hash()?.to_string();
    let installed = !InstalledPkg::find_by_attr(&ctx, "root_hash", &root_hash)
        .await?
        .is_empty();

    Ok(Json(PkgGetResponse {
        hash: root_hash,
        name: metadata.name().to_string(),
        version: metadata.version().to_string(),
        description: metadata.description().to_string(),
        created_at: metadata.created_at(),
        created_by: metadata.created_by().to_string(),
        installed,
        schemas: schema_variants,
    }))
}

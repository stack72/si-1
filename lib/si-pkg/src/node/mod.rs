use std::io::{BufRead, Write};

use object_tree::{
    read_key_value_line, write_key_value_line, GraphError, NameStr, ReadBytes, WriteBytes,
};

mod category;
mod package;
mod prop;
mod schema;
mod schema_variant;

pub(crate) use self::{
    category::CategoryNode, package::PackageNode, prop::PropNode, schema::SchemaNode,
    schema_variant::SchemaVariantNode,
};

const NODE_KIND_CATEGORY: &str = "category";
const NODE_KIND_PACKAGE: &str = "package";
const NODE_KIND_PROP: &str = "prop";
const NODE_KIND_SCHEMA: &str = "schema";
const NODE_KIND_SCHEMA_VARIANT: &str = "schema_variant";

const KEY_NODE_KIND_STR: &str = "node_kind";

#[derive(Clone, Debug)]
pub enum PkgNode {
    Category(CategoryNode),
    Package(PackageNode),
    Prop(PropNode),
    Schema(SchemaNode),
    SchemaVariant(SchemaVariantNode),
}

impl PkgNode {
    pub const CATEGORY_KIND_STR: &str = NODE_KIND_CATEGORY;
    pub const PACKAGE_KIND_STR: &str = NODE_KIND_PACKAGE;
    pub const PROP_KIND_STR: &str = NODE_KIND_PROP;
    pub const SCHEMA_KIND_STR: &str = NODE_KIND_SCHEMA;
    pub const SCHEMA_VARIANT_KIND_STR: &str = NODE_KIND_SCHEMA_VARIANT;

    pub fn node_kind_str(&self) -> &'static str {
        match self {
            Self::Category(_) => NODE_KIND_CATEGORY,
            Self::Package(_) => NODE_KIND_PACKAGE,
            Self::Prop(_) => NODE_KIND_PROP,
            Self::Schema(_) => NODE_KIND_SCHEMA,
            Self::SchemaVariant(_) => NODE_KIND_SCHEMA_VARIANT,
        }
    }
}

impl NameStr for PkgNode {
    fn name(&self) -> &str {
        match self {
            Self::Category(node) => node.name(),
            Self::Package(node) => node.name(),
            Self::Prop(node) => node.name(),
            Self::Schema(node) => node.name(),
            Self::SchemaVariant(node) => node.name(),
        }
    }
}

impl WriteBytes for PkgNode {
    fn write_bytes<W: Write>(&self, writer: &mut W) -> Result<(), GraphError> {
        write_key_value_line(writer, KEY_NODE_KIND_STR, self.node_kind_str())?;

        match self {
            Self::Category(node) => node.write_bytes(writer)?,
            Self::Package(node) => node.write_bytes(writer)?,
            Self::Prop(node) => node.write_bytes(writer)?,
            Self::Schema(node) => node.write_bytes(writer)?,
            Self::SchemaVariant(node) => node.write_bytes(writer)?,
        };

        Ok(())
    }
}

impl ReadBytes for PkgNode {
    fn read_bytes<R: BufRead>(reader: &mut R) -> Result<Self, GraphError>
    where
        Self: std::marker::Sized,
    {
        let node_kind_str = read_key_value_line(reader, KEY_NODE_KIND_STR)?;

        let node = match node_kind_str.as_str() {
            NODE_KIND_CATEGORY => Self::Category(CategoryNode::read_bytes(reader)?),
            NODE_KIND_PACKAGE => Self::Package(PackageNode::read_bytes(reader)?),
            NODE_KIND_PROP => Self::Prop(PropNode::read_bytes(reader)?),
            NODE_KIND_SCHEMA => Self::Schema(SchemaNode::read_bytes(reader)?),
            NODE_KIND_SCHEMA_VARIANT => Self::SchemaVariant(SchemaVariantNode::read_bytes(reader)?),
            invalid_kind => {
                return Err(GraphError::parse_custom(format!(
                    "invalid package node kind: {invalid_kind}"
                )))
            }
        };

        Ok(node)
    }
}
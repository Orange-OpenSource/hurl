pub mod c14n;
pub mod document;
pub mod namespace;
pub mod node;
pub mod nodetype;

pub use self::document::{Document, SaveOptions};
pub(crate) use self::document::{DocumentRef, DocumentWeak};
pub use self::namespace::Namespace;
pub use self::node::Node;
// Deprecated, retained for API compatibility (see their definitions in `node`).
#[allow(deprecated)]
pub use self::node::{NODE_RC_MAX_GUARD, set_node_rc_guard};
pub use self::nodetype::NodeType;

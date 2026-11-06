//!
//! Schema Validation Support (XSD)
//!
//! This module exposes wraps xmlschemas in libxml2. See original documentation or
//! look at the example at examples/schema_example.rs for usage.
//!
//! WARNING: This module has not been tested in a multithreaded or multiprocessing
//! environment.
//!
mod common;
mod parser;
mod schema;
mod validation;

use schema::Schema; // internally handled by SchemaValidationContext

pub use parser::SchemaParserContext;
pub use validation::SchemaValidationContext;

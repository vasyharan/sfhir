use super::*;
/// A human-readable summary of the resource conveying the essential clinical and
/// business information for the resource.
#[derive(Debug,Clone,PartialEq)]
pub struct Narrative {
/// The actual narrative content, a stripped down version of XHTML.
pub div: Xhtml,
/// Unique id for the element within a resource (for internal references). This may
/// be any string value that does not contain spaces.
pub id: String,
/// The status of the narrative - whether it's entirely generated (from just the
/// defined data or the extensions too), or whether a human authored it and it may
/// contain additional data.
pub status: Status,
}

#[derive(Debug,Clone,PartialEq)]
pub enum Status {
Generated,
Extensions,
Additional,
Empty,
}

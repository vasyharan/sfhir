use super::*;
/// A container for a collection of resources.
#[derive(Debug,Clone,PartialEq)]
pub struct BundleResponse {
/// The Etag for the resource, if the operation for the entry produced a versioned
/// resource (see [Resource Metadata and Versioning](http.html#versioning) and
/// [Managing Resource Contention](http.html#concurrency)).
pub etag: String,
/// Unique id for the element within a resource (for internal references). This may
/// be any string value that does not contain spaces.
pub id: String,
/// The date/time that the resource was modified on the server.
pub last_modified: Instant,
/// The location header created by processing this operation, populated if the
/// operation returns a location.
pub location: Uri,
/// May be used to represent additional information that is not part of the basic
/// definition of the element and that modifies the understanding of the element
/// in which it is contained and/or the understanding of the containing element's
/// descendants. Usually modifier elements provide negation or qualification.
/// To make the use of extensions safe and managable, there is a strict set
/// of governance applied to the definition and use of extensions. Though any
/// implementer can define an extension, there is a set of requirements that SHALL
/// be met as part of the definition of the extension. Applications processing a
/// resource are required to check for modifier extensions.
///
/// Modifier extensions SHALL NOT change the meaning of any elements on Resource
/// or DomainResource (including cannot change the meaning of modifierExtension
/// itself).
pub modifier_extension: Vec<Extension>,
/// An OperationOutcome containing hints and warnings produced as part of processing
/// this entry in a batch or transaction.
pub outcome: ResourceList,
/// The status code returned by processing this entry. The status SHALL start with
/// a 3 digit HTTP code (e.g. 404) and may contain the standard HTTP description
/// associated with the status code.
pub status: String,
}
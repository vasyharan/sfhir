use super::*;
/// A container for a collection of resources.
#[derive(Debug,Clone,PartialEq)]
pub struct BundleRequest {
/// Unique id for the element within a resource (for internal references). This may
/// be any string value that does not contain spaces.
pub id: String,
/// Only perform the operation if the Etag value matches. For more information, see
/// the API section ["Managing Resource Contention"](http.html#concurrency).
pub if_match: String,
/// Only perform the operation if the last updated date matches. See the API
/// documentation for ["Conditional Read"](http.html#cread).
pub if_modified_since: Instant,
/// Instruct the server not to perform the create if a specified resource already
/// exists. For further information, see the API documentation for ["Conditional
/// Create"](http.html#ccreate). This is just the query portion of the URL - what
/// follows the "?" (not including the "?").
pub if_none_exist: String,
/// If the ETag values match, return a 304 Not Modified status. See the API
/// documentation for ["Conditional Read"](http.html#cread).
pub if_none_match: String,
/// In a transaction or batch, this is the HTTP action to be executed for this
/// entry. In a history bundle, this indicates the HTTP action that occurred.
pub method: Code,
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
/// The URL for this entry, relative to the root (the address to which the request
/// is posted).
pub url: Uri,
}

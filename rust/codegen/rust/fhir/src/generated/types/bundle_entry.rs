use super::*;
/// A container for a collection of resources.
#[derive(Debug,Clone,PartialEq)]
pub struct BundleEntry {
/// The Absolute URL for the resource. Except for transactions and batches, each
/// entry in a Bundle must have a fullUrl. The fullUrl SHALL NOT disagree with
/// the id in the resource - i.e. if the fullUrl is not a urn:uuid, the URL shall
/// be version-independent URL consistent with the Resource.id. The fullUrl is a
/// version independent reference to the resource. Even when not required, fullUrl
/// MAY be set to a urn:uuid to allow referencing entries in a transaction. The
/// fullUrl can be an arbitrary URI and is not limited to urn:uuid, urn:oid, http,
/// and https. The fullUrl element SHALL have a value except when:
/// * invoking a create
/// * invoking or responding to an operation where the body is not a single
/// identified resource
/// * invoking or returning the results of a search or history operation.
pub full_url: Uri,
/// Unique id for the element within a resource (for internal references). This may
/// be any string value that does not contain spaces.
pub id: String,
/// A series of links that provide context to this entry.
pub link: Vec<BundleLink>,
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
/// Additional information about how this entry should be processed as part of a
/// transaction or batch.  For history, it shows how the entry was processed to
/// create the version contained in the entry.
pub request: BundleRequest,
/// The Resource for the entry. The purpose/meaning of the resource is determined by
/// the Bundle.type. This is allowed to be a Parameters resource if and only if it
/// is referenced by something else within the Bundle that provides context/meaning.
pub resource: ResourceList,
/// Indicates the results of processing the corresponding 'request' entry in the
/// batch or transaction being responded to or what the results of an operation
/// where when returning history.
pub response: BundleResponse,
/// Information about the search process that lead to the creation of this entry.
pub search: BundleSearch,
}

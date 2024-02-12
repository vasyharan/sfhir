use super::*;
/// A ValueSet resource instance specifies a set of codes drawn from one or more
/// code systems, intended for use in a particular context. Value sets link between
/// [CodeSystem](codesystem.html) definitions and their use in [coded elements]
/// (terminologies.html).
#[derive(Debug,Clone,PartialEq)]
pub struct ValueSetExpansion {
/// The codes that are contained in the value set expansion.
pub contains: Vec<ValueSetContains>,
/// Unique id for the element within a resource (for internal references). This may
/// be any string value that does not contain spaces.
pub id: String,
/// An identifier that uniquely identifies this expansion of the valueset, based on
/// a unique combination of the provided parameters, the system default parameters,
/// and the underlying system code system versions etc. Systems may re-use the same
/// identifier as long as those factors remain the same, and the expansion is the
/// same, but are not required to do so. This is a business identifier.
pub identifier: Uri,
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
/// As per paging Search results, the next URLs are opaque to the client, have no
/// dictated structure, and only the server understands them.
pub next: Uri,
/// If paging is being used, the offset at which this resource starts.  I.e. this
/// resource is a partial view into the expansion. If paging is not being used, this
/// element SHALL NOT be present.
pub offset: Integer,
/// A parameter that controlled the expansion process. These parameters may be used
/// by users of expanded value sets to check whether the expansion is suitable for a
/// particular purpose, or to pick the correct expansion.
pub parameter: Vec<ValueSetParameter>,
/// A property defines an additional slot through which additional information can
/// be provided about a concept.
pub property: Vec<ValueSetProperty>,
/// The time at which the expansion was produced by the expanding system.
pub timestamp: DateTime,
/// The total number of concepts in the expansion. If the number of concept nodes
/// in this resource is less than the stated number, then the server can return more
/// using the offset parameter.
pub total: Integer,
}

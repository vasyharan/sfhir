use super::*;
/// A ValueSet resource instance specifies a set of codes drawn from one or more
/// code systems, intended for use in a particular context. Value sets link between
/// [CodeSystem](codesystem.html) definitions and their use in [coded elements]
/// (terminologies.html).
#[derive(Debug,Clone,PartialEq)]
pub struct ValueSetCompose {
/// Exclude one or more codes from the value set based on code system filters and/or
/// other value sets.
pub exclude: Vec<ValueSetInclude>,
/// Unique id for the element within a resource (for internal references). This may
/// be any string value that does not contain spaces.
pub id: String,
/// Whether inactive codes - codes that are not approved for current use - are in
/// the value set. If inactive = true, inactive codes are to be included in the
/// expansion, if inactive = false, the inactive codes will not be included in
/// the expansion. If absent, the behavior is determined by the implementation, or
/// by the applicable $expand parameters (but generally, inactive codes would be
/// expected to be included).
pub inactive: Boolean,
/// Include one or more codes from a code system or other value set(s).
pub include: Vec<ValueSetInclude>,
/// The Locked Date is  the effective date that is used to determine the version of
/// all referenced Code Systems and Value Set Definitions included in the compose
/// that are not already tied to a specific version.
pub locked_date: Date,
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
/// A property to return in the expansion, if the client doesn't ask for any
/// particular properties. May be either a code from the code system definition
/// (convenient) or a the formal URI that refers to the property. The special value
/// '*' means all properties known to the server.
pub property: Vec<String>,
}

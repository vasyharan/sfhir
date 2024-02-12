use super::*;
/// A ValueSet resource instance specifies a set of codes drawn from one or more
/// code systems, intended for use in a particular context. Value sets link between
/// [CodeSystem](codesystem.html) definitions and their use in [coded elements]
/// (terminologies.html).
#[derive(Debug,Clone,PartialEq)]
pub struct ValueSetContains {
/// If true, this entry is included in the expansion for navigational purposes, and
/// the user cannot select the code directly as a proper value.
pub r#abstract: Boolean,
/// The code for this item in the expansion hierarchy. If this code is missing the
/// entry in the hierarchy is a place holder (abstract) and does not represent a
/// valid code in the value set.
pub code: Code,
/// Other codes and entries contained under this entry in the hierarchy.
pub contains: Vec<ValueSetContains>,
/// Additional representations for this item - other languages, aliases, specialized
/// purposes, used for particular purposes, etc. These are relevant when the
/// conditions of the expansion do not fix to a single correct representation.
pub designation: Vec<ValueSetDesignation>,
/// The recommended display for this item in the expansion.
pub display: String,
/// Unique id for the element within a resource (for internal references). This may
/// be any string value that does not contain spaces.
pub id: String,
/// If the concept is inactive in the code system that defines it. Inactive codes
/// are those that are no longer to be used, but are maintained by the code system
/// for understanding legacy data. It might not be known or specified whether a
/// concept is inactive (and it may depend on the context of use).
pub inactive: Boolean,
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
/// A property value for this concept.
pub property: Vec<ValueSetProperty1>,
/// An absolute URI which is the code system in which the code for this item in the
/// expansion is defined.
pub system: Uri,
/// The version of the code system from this code was taken. Note that a well-
/// maintained code system does not need the version reported, because the meaning
/// of codes is consistent across versions. However this cannot consistently be
/// assured, and when the meaning is not guaranteed to be consistent, the version
/// SHOULD be exchanged.
pub version: String,
}

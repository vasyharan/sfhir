use super::*;
/// A curated namespace that issues unique symbols within that namespace for the
/// identification of concepts, people, devices, etc.  Represents a "System" used
/// within the Identifier and Coding data types.
#[derive(Debug,Clone,PartialEq)]
pub struct NamingSystemUniqueId {
/// Indicates whether this identifier ie endorsed by the official owner of the
/// associated naming system.
pub authoritative: Boolean,
/// Notes about the past or intended usage of this identifier.
pub comment: String,
/// Unique id for the element within a resource (for internal references). This may
/// be any string value that does not contain spaces.
pub id: String,
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
/// Identifies the period of time over which this identifier is considered
/// appropriate to refer to the naming system.  Outside of this window, the
/// identifier might be non-deterministic.
pub period: Period,
/// Indicates whether this identifier is the "preferred" identifier of this type.
pub preferred: Boolean,
/// Identifies the unique identifier scheme used for this particular identifier.
pub r#type: Code,
/// The string that should be sent over the wire to identify the code system or
/// identifier system.
pub value: String,
}
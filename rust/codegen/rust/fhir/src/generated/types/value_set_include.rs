use super::*;
/// A ValueSet resource instance specifies a set of codes drawn from one or more
/// code systems, intended for use in a particular context. Value sets link between
/// [CodeSystem](codesystem.html) definitions and their use in [coded elements]
/// (terminologies.html).
#[derive(Debug,Clone,PartialEq)]
pub struct ValueSetInclude {
/// Specifies a concept to be included or excluded.
pub concept: Vec<ValueSetConcept>,
/// A copyright statement for the specific code system asserted by the
/// containing ValueSet.compose.include element's system value (if the associated
/// ValueSet.compose.include.version element is not present); or the code system and
/// version combination (if the associated ValueSet.compose.include.version element
/// is present).
pub copyright: String,
/// Select concepts by specifying a matching criterion based on the properties
/// (including relationships) defined by the system, or on filters defined by the
/// system. If multiple filters are specified within the include, they SHALL all
/// be true.
pub filter: Vec<ValueSetFilter>,
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
/// An absolute URI which is the code system from which the selected codes come
/// from.
pub system: Uri,
/// Selects the concepts found in this value set (based on its value set
/// definition). This is an absolute URI that is a reference to ValueSet.url.  If
/// multiple value sets are specified this includes the intersection of the contents
/// of all of the referenced value sets.
pub value_set: Vec<Canonical>,
/// The version of the code system that the codes are selected from, or the special
/// version '*' for all versions.
pub version: String,
}

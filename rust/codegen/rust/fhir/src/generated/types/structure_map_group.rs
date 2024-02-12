use super::*;
/// A Map of relationships between 2 structures that can be used to transform data.
#[derive(Debug,Clone,PartialEq)]
pub struct StructureMapGroup {
/// Additional supporting documentation that explains the purpose of the group and
/// the types of mappings within it.
pub documentation: String,
/// Another group that this group adds rules to.
pub extends: Id,
/// Unique id for the element within a resource (for internal references). This may
/// be any string value that does not contain spaces.
pub id: String,
/// A name assigned to an instance of data. The instance must be provided when the
/// mapping is invoked.
pub input: Vec<StructureMapInput>,
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
/// A unique name for the group for the convenience of human readers.
pub name: Id,
/// Transform Rule from source to target.
pub rule: Vec<StructureMapRule>,
/// If this is the default rule set to apply for the source type or this combination
/// of types.
pub type_mode: Code,
}

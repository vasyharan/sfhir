use super::*;
/// A Map of relationships between 2 structures that can be used to transform data.
#[derive(Debug,Clone,PartialEq)]
pub struct StructureMapRule {
/// Which other rules to apply in the context of this rule.
pub dependent: Vec<StructureMapDependent>,
/// Documentation for this instance of data.
pub documentation: String,
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
/// Name of the rule for internal references.
pub name: Id,
/// Rules contained in this rule.
pub rule: Vec<StructureMapRule>,
/// Source inputs to the mapping.
pub source: Vec<StructureMapSource>,
/// Content to create because of this mapping rule.
pub target: Vec<StructureMapTarget>,
}

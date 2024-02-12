use super::*;
/// The detailed description of a substance, typically at a level beyond what is
/// used for prescribing.
#[derive(Debug,Clone,PartialEq)]
pub struct SubstanceDefinitionRelationship {
/// A numeric factor for the relationship, for instance to express that the salt
/// of a substance has some percentage of the active substance in relation to some
/// other.
pub amount_quantity: Quantity,
/// A numeric factor for the relationship, for instance to express that the salt
/// of a substance has some percentage of the active substance in relation to some
/// other.
pub amount_ratio: Ratio,
/// A numeric factor for the relationship, for instance to express that the salt
/// of a substance has some percentage of the active substance in relation to some
/// other.
pub amount_string: String,
/// An operator for the amount, for example "average", "approximately", "less than".
pub comparator: CodeableConcept,
/// Unique id for the element within a resource (for internal references). This may
/// be any string value that does not contain spaces.
pub id: String,
/// For example where an enzyme strongly bonds with a particular substance, this
/// is a defining relationship for that enzyme, out of several possible substance
/// relationships.
pub is_defining: Boolean,
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
/// For use when the numeric has an uncertain range.
pub ratio_high_limit_amount: Ratio,
/// Supporting literature.
pub source: Vec<Reference>,
/// A pointer to another substance, as a resource or just a representational code.
pub substance_definition_codeable_concept: CodeableConcept,
/// A pointer to another substance, as a resource or just a representational code.
pub substance_definition_reference: Reference,
/// For example "salt to parent", "active moiety", "starting material", "polymorph",
/// "impurity of".
pub r#type: CodeableConcept,
}

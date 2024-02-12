use super::*;
/// Properties of a substance specific to it being a polymer.
#[derive(Debug,Clone,PartialEq)]
pub struct SubstancePolymerRepeatUnit {
/// Number of repeats of this unit.
pub amount: Integer,
/// Applies to homopolymer and block co-polymers where the degree of polymerisation
/// within a block can be described.
pub degree_of_polymerisation: Vec<SubstancePolymerDegreeOfPolymerisation>,
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
/// The orientation of the polymerisation, e.g. head-tail, head-head, random.
pub orientation: CodeableConcept,
/// A graphical structure for this SRU.
pub structural_representation: Vec<SubstancePolymerStructuralRepresentation>,
/// Structural repeat units are essential elements for defining polymers.
pub unit: String,
}

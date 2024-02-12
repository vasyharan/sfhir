use super::*;
/// A kind of specimen with associated set of requirements.
#[derive(Debug,Clone,PartialEq)]
pub struct SpecimenDefinitionTypeTested {
/// The specimen's container.
pub container: SpecimenDefinitionContainer,
/// Set of instructions for preservation/transport of the specimen at a defined
/// temperature interval, prior the testing process.
pub handling: Vec<SpecimenDefinitionHandling>,
/// Unique id for the element within a resource (for internal references). This may
/// be any string value that does not contain spaces.
pub id: String,
/// Primary of secondary specimen.
pub is_derived: Boolean,
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
/// The preference for this type of conditioned specimen.
pub preference: Code,
/// Criterion for rejection of the specimen in its container by the laboratory.
pub rejection_criterion: Vec<CodeableConcept>,
/// Requirements for delivery and special handling of this kind of conditioned
/// specimen.
pub requirement: Markdown,
/// The usual time that a specimen of this kind is retained after the ordered tests
/// are completed, for the purpose of additional testing.
pub retention_time: Duration,
/// Specimen can be used by only one test or panel if the value is "true".
pub single_use: Boolean,
/// Where the specimen will be tested: e.g., lab, sector, device or any combination
/// of these.
pub testing_destination: Vec<CodeableConcept>,
/// The kind of specimen conditioned for testing expected by lab.
pub r#type: CodeableConcept,
}

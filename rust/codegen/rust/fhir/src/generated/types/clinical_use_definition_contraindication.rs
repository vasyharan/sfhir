use super::*;
/// A single issue - either an indication, contraindication, interaction or an
/// undesirable effect for a medicinal product, medication, device or procedure.
#[derive(Debug,Clone,PartialEq)]
pub struct ClinicalUseDefinitionContraindication {
/// An expression that returns true or false, indicating whether the indication is
/// applicable or not, after having applied its other elements.
pub applicability: Expression,
/// A comorbidity (concurrent condition) or coinfection.
pub comorbidity: Vec<CodeableReference>,
/// The status of the disease or symptom for the contraindication, for example
/// "chronic" or "metastatic".
pub disease_status: CodeableReference,
/// The situation that is being documented as contraindicating against this item.
pub disease_symptom_procedure: CodeableReference,
/// Unique id for the element within a resource (for internal references). This may
/// be any string value that does not contain spaces.
pub id: String,
/// The indication which this is a contraidication for.
pub indication: Vec<Reference>,
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
/// Information about the use of the medicinal product in relation to other
/// therapies described as part of the contraindication.
pub other_therapy: Vec<ClinicalUseDefinitionOtherTherapy>,
}

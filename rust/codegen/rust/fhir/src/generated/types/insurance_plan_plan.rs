use super::*;
/// Details of a Health Insurance product/plan provided by an organization.
#[derive(Debug,Clone,PartialEq)]
pub struct InsurancePlanPlan {
/// The geographic region in which a health insurance plan's benefits apply.
pub coverage_area: Vec<Reference>,
/// Overall costs associated with the plan.
pub general_cost: Vec<InsurancePlanGeneralCost>,
/// Unique id for the element within a resource (for internal references). This may
/// be any string value that does not contain spaces.
pub id: String,
/// Business identifiers assigned to this health insurance plan which remain
/// constant as the resource is updated and propagates from server to server.
pub identifier: Vec<Identifier>,
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
/// Reference to the network that providing the type of coverage.
pub network: Vec<Reference>,
/// Costs associated with the coverage provided by the product.
pub specific_cost: Vec<InsurancePlanSpecificCost>,
/// Type of plan. For example, "Platinum" or "High Deductable".
pub r#type: CodeableConcept,
}

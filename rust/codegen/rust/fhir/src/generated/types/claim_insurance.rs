use super::*;
/// A provider issued list of professional services and products which have been
/// provided, or are to be provided, to a patient which is sent to an insurer for
/// reimbursement.
#[derive(Debug,Clone,PartialEq)]
pub struct ClaimInsurance {
/// A business agreement number established between the provider and the insurer for
/// special business processing purposes.
pub business_arrangement: String,
/// The result of the adjudication of the line items for the Coverage specified in
/// this insurance.
pub claim_response: Reference,
/// Reference to the insurance card level information contained in the Coverage
/// resource. The coverage issuing insurer will use these details to locate the
/// patient's actual coverage within the insurer's information system.
pub coverage: Reference,
/// A flag to indicate that this Coverage is to be used for adjudication of this
/// claim when set to true.
pub focal: Boolean,
/// Unique id for the element within a resource (for internal references). This may
/// be any string value that does not contain spaces.
pub id: String,
/// The business identifier to be used when the claim is sent for adjudication
/// against this insurance policy.
pub identifier: Identifier,
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
/// Reference numbers previously provided by the insurer to the provider to be
/// quoted on subsequent claims containing services or products related to the prior
/// authorization.
pub pre_auth_ref: Vec<String>,
/// A number to uniquely identify insurance entries and provide a sequence of
/// coverages to convey coordination of benefit order.
pub sequence: PositiveInt,
}

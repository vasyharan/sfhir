use super::*;
/// A record of an event relevant for purposes such as operations, privacy,
/// security, maintenance, and performance analysis.
#[derive(Debug,Clone,PartialEq)]
pub struct AuditEventAgent {
/// The authorization (e.g., PurposeOfUse) that was used during the event being
/// recorded.
pub authorization: Vec<CodeableConcept>,
/// Unique id for the element within a resource (for internal references). This may
/// be any string value that does not contain spaces.
pub id: String,
/// Where the agent location is known, the agent location when the event occurred.
pub location: Reference,
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
/// When the event utilizes a network there should be an agent describing the
/// local system, and an agent describing remote system, with the network interface
/// details.
pub network_reference: Reference,
/// When the event utilizes a network there should be an agent describing the
/// local system, and an agent describing remote system, with the network interface
/// details.
pub network_string: String,
/// When the event utilizes a network there should be an agent describing the
/// local system, and an agent describing remote system, with the network interface
/// details.
pub network_uri: String,
/// Where the policy(ies) are known that authorized the agent participation in the
/// event. Typically, a single activity may have multiple applicable policies, such
/// as patient consent, guarantor funding, etc. The policy would also indicate the
/// security token used.
pub policy: Vec<Uri>,
/// Indicator that the user is or is not the requestor, or initiator, for the event
/// being audited.
pub requestor: Boolean,
/// The structural roles of the agent indicating the agent's competency. The
/// security role enabling the agent with respect to the activity.
pub role: Vec<CodeableConcept>,
/// The Functional Role of the user when performing the event.
pub r#type: CodeableConcept,
/// Reference to who this agent is that was involved in the event.
pub who: Reference,
}

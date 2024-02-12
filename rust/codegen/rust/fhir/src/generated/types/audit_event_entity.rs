use super::*;
/// A record of an event relevant for purposes such as operations, privacy,
/// security, maintenance, and performance analysis.
#[derive(Debug,Clone,PartialEq)]
pub struct AuditEventEntity {
/// The entity is attributed to an agent to express the agent's responsibility for
/// that entity in the activity. This is most used to indicate when persistence
/// media (the entity) are used by an agent. For example when importing data from a
/// device, the device would be described in an entity, and the user importing data
/// from that media would be indicated as the entity.agent.
pub agent: Vec<AuditEventAgent>,
/// Tagged value pairs for conveying additional information about the entity.
pub detail: Vec<AuditEventDetail>,
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
/// The query parameters for a query-type entities.
pub query: Base64Binary,
/// Code representing the role the entity played in the event being audited.
pub role: CodeableConcept,
/// Security labels for the identified entity.
pub security_label: Vec<CodeableConcept>,
/// Identifies a specific instance of the entity. The reference should be version
/// specific. This is allowed to be a Parameters resource.
pub what: Reference,
}

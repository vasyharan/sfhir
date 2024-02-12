use super::*;
/// The header for a message exchange that is either requesting or responding to an
/// action.  The reference(s) that are the subject of the action as well as other
/// information related to the action are typically transmitted in a bundle in which
/// the MessageHeader resource instance is the first resource in the bundle.
#[derive(Debug,Clone,PartialEq)]
pub struct MessageHeaderDestination {
/// Indicates where the message should be routed.
pub endpoint_reference: Reference,
/// Indicates where the message should be routed.
pub endpoint_url: String,
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
/// Human-readable name for the target system.
pub name: String,
/// Allows data conveyed by a message to be addressed to a particular person or
/// department when routing to a specific application isn't sufficient.
pub receiver: Reference,
/// Identifies the target end system in situations where the initial message
/// transmission is to an intermediary system.
pub target: Reference,
}

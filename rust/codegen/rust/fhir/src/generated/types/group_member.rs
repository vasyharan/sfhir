use super::*;
/// Represents a defined collection of entities that may be discussed or acted
/// upon collectively but which are not expected to act collectively, and are not
/// formally or legally recognized; i.e. a collection of entities that isn't an
/// Organization.
#[derive(Debug,Clone,PartialEq)]
pub struct GroupMember {
/// A reference to the entity that is a member of the group. Must be consistent with
/// Group.type. If the entity is another group, then the type must be the same.
pub entity: Reference,
/// Unique id for the element within a resource (for internal references). This may
/// be any string value that does not contain spaces.
pub id: String,
/// A flag to indicate that the member is no longer in the group, but previously may
/// have been a member.
pub inactive: Boolean,
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
/// The period that the member was in the group, if known.
pub period: Period,
}
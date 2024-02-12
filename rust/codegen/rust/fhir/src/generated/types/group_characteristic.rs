use super::*;
/// Represents a defined collection of entities that may be discussed or acted
/// upon collectively but which are not expected to act collectively, and are not
/// formally or legally recognized; i.e. a collection of entities that isn't an
/// Organization.
#[derive(Debug,Clone,PartialEq)]
pub struct GroupCharacteristic {
/// A code that identifies the kind of trait being asserted.
pub code: CodeableConcept,
/// If true, indicates the characteristic is one that is NOT held by members of
/// the group.
pub exclude: Boolean,
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
/// The period over which the characteristic is tested; e.g. the patient had an
/// operation during the month of June.
pub period: Period,
/// The value of the trait that holds (or does not hold - see 'exclude') for members
/// of the group.
pub value_boolean: bool,
/// The value of the trait that holds (or does not hold - see 'exclude') for members
/// of the group.
pub value_codeable_concept: CodeableConcept,
/// The value of the trait that holds (or does not hold - see 'exclude') for members
/// of the group.
pub value_quantity: Quantity,
/// The value of the trait that holds (or does not hold - see 'exclude') for members
/// of the group.
pub value_range: Range,
/// The value of the trait that holds (or does not hold - see 'exclude') for members
/// of the group.
pub value_reference: Reference,
}

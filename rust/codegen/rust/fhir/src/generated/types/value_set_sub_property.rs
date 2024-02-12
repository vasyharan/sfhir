use super::*;
/// A ValueSet resource instance specifies a set of codes drawn from one or more
/// code systems, intended for use in a particular context. Value sets link between
/// [CodeSystem](codesystem.html) definitions and their use in [coded elements]
/// (terminologies.html).
#[derive(Debug,Clone,PartialEq)]
pub struct ValueSetSubProperty {
/// A code that is a reference to ValueSet.expansion.property.code.
pub code: Code,
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
/// The value of this subproperty.
pub value_boolean: bool,
/// The value of this subproperty.
pub value_code: String,
/// The value of this subproperty.
pub value_coding: Coding,
/// The value of this subproperty.
pub value_date_time: String,
/// The value of this subproperty.
pub value_decimal: f64,
/// The value of this subproperty.
pub value_integer: i64,
/// The value of this subproperty.
pub value_string: String,
}

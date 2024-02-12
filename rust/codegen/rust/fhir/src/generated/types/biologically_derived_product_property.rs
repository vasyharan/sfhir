use super::*;
/// This resource reflects an instance of a biologically derived product. A material
/// substance originating from a biological entity intended to be transplanted or
/// infused
/// into another (possibly the same) biological entity.
#[derive(Debug,Clone,PartialEq)]
pub struct BiologicallyDerivedProductProperty {
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
/// Code that specifies the property. It should reference an established coding
/// system.
pub r#type: CodeableConcept,
/// Property values.
pub value_attachment: Attachment,
/// Property values.
pub value_boolean: bool,
/// Property values.
pub value_codeable_concept: CodeableConcept,
/// Property values.
pub value_integer: i64,
/// Property values.
pub value_period: Period,
/// Property values.
pub value_quantity: Quantity,
/// Property values.
pub value_range: Range,
/// Property values.
pub value_ratio: Ratio,
/// Property values.
pub value_string: String,
}

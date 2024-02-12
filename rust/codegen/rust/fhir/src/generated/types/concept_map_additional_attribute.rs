use super::*;
/// A statement of relationships from one set of concepts to one or more other
/// concepts - either concepts in code systems, or data element/data element
/// concepts, or classes in class models.
#[derive(Debug,Clone,PartialEq)]
pub struct ConceptMapAdditionalAttribute {
/// A code that is used to identify this additional data attribute. The code is
/// used internally in ConceptMap.group.element.target.dependsOn.attribute and
/// ConceptMap.group.element.target.product.attribute.
pub code: Code,
/// A description of the additional attribute and/or the data element it refers to -
/// why it is defined, and how the value might be used in mappings, and a discussion
/// of issues associated with the use of the data element.
pub description: String,
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
/// The type of the source data contained in this concept map for this data element.
pub r#type: Code,
/// Reference to the formal definition of the source/target data element. For
/// elements defined by the FHIR specification, or using a FHIR logical model, the
/// correct format is {canonical-url}#{element-id}.
pub uri: Uri,
}

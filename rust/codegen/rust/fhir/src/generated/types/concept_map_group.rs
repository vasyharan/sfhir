use super::*;
/// A statement of relationships from one set of concepts to one or more other
/// concepts - either concepts in code systems, or data element/data element
/// concepts, or classes in class models.
#[derive(Debug,Clone,PartialEq)]
pub struct ConceptMapGroup {
/// Mappings for an individual concept in the source to one or more concepts in
/// the target.
pub element: Vec<ConceptMapElement>,
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
/// An absolute URI that identifies the source system where the concepts to be
/// mapped are defined.
pub source: Canonical,
/// An absolute URI that identifies the target system that the concepts will be
/// mapped to.
pub target: Canonical,
/// What to do when there is no mapping to a target concept from the source concept
/// and ConceptMap.group.element.noMap is not true. This provides the "default" to
/// be applied when there is no target concept mapping specified or the expansion of
/// ConceptMap.group.element.target.valueSet is empty.
pub unmapped: ConceptMapUnmapped,
}
use super::*;
/// A formal computable definition of a graph of resources - that is, a coherent
/// set of resources that form a graph by following references. The Graph Definition
/// resource defines a set and makes rules about the set.
#[derive(Debug,Clone,PartialEq)]
pub struct GraphDefinitionLink {
/// Compartment Consistency Rules.
pub compartment: Vec<GraphDefinitionCompartment>,
/// Information about why this link is of interest in this graph definition.
pub description: String,
/// Unique id for the element within a resource (for internal references). This may
/// be any string value that does not contain spaces.
pub id: String,
/// Maximum occurrences for this link.
pub max: String,
/// Minimum occurrences for this link.
pub min: Integer,
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
/// A set of parameters to look up.
pub params: String,
/// A FHIRPath expression that identifies one of FHIR References to other resources.
pub path: String,
/// Which slice (if profiled).
pub slice_name: String,
/// The source node for this link.
pub source_id: Id,
/// The target node for this link.
pub target_id: Id,
}

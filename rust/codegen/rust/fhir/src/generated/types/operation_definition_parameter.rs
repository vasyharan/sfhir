use super::*;
/// A formal computable definition of an operation (on the RESTful interface) or a
/// named query (using the search interaction).
#[derive(Debug,Clone,PartialEq)]
pub struct OperationDefinitionParameter {
/// Support for polymorphic types. If the parameter type is abstract, this element
/// lists allowed sub-types for the parameter.
pub allowed_type: Vec<Code>,
/// Binds to a value set if this parameter is coded (code, Coding, CodeableConcept).
pub binding: OperationDefinitionBinding,
/// Describes the meaning or use of this parameter.
pub documentation: Markdown,
/// Unique id for the element within a resource (for internal references). This may
/// be any string value that does not contain spaces.
pub id: String,
/// The maximum number of times this element is permitted to appear in the request
/// or response.
pub max: String,
/// The minimum number of times this parameter SHALL appear in the request or
/// response.
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
/// The name of used to identify the parameter.
pub name: Code,
/// The parts of a nested Parameter.
pub part: Vec<OperationDefinitionParameter>,
/// Identifies other resource parameters within the operation invocation that are
/// expected to resolve to this resource.
pub referenced_from: Vec<OperationDefinitionReferencedFrom>,
/// If present, indicates that the parameter applies when the operation is being
/// invoked at the specified level.
pub scope: Vec<Code>,
/// How the parameter is understood if/when it used as search parameter. This is
/// only used if the parameter is a string.
pub search_type: Code,
/// Used when the type is "Reference" or "canonical", and identifies a profile
/// structure or implementation Guide that applies to the target of the reference
/// this parameter refers to. If any profiles are specified, then the content
/// must conform to at least one of them. The URL can be a local reference - to a
/// contained StructureDefinition, or a reference to another StructureDefinition
/// or Implementation Guide by a canonical URL. When an implementation guide is
/// specified, the target resource SHALL conform to at least one profile defined in
/// the implementation guide.
pub target_profile: Vec<Canonical>,
/// The type for this parameter.
pub r#type: Code,
/// Whether this is an input or an output parameter.
pub r#use: Code,
}

use super::*;
/// Captures constraints on each element within the resource, profile, or extension.
#[derive(Debug,Clone,PartialEq)]
pub struct ElementDefinitionBinding {
/// Additional bindings that help applications implementing this element. Additional
/// bindings do not replace the main binding but provide more information and/
/// or context.
pub additional: Vec<ElementDefinitionAdditional>,
/// Describes the intended use of this particular set of codes.
pub description: Markdown,
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
/// Indicates the degree of conformance expectations associated with this binding
/// - that is, the degree to which the provided value set must be adhered to in
/// the instances.
pub strength: Strength,
/// Refers to the value set that identifies the set of codes the binding refers to.
pub value_set: Canonical,
}

#[derive(Debug,Clone,PartialEq)]
pub enum Strength {
Required,
Extensible,
Preferred,
Example,
}

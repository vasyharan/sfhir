use super::*;
/// Captures constraints on each element within the resource, profile, or extension.
#[derive(Debug,Clone,PartialEq)]
pub struct ElementDefinitionAdditional {
/// Whether the binding applies to all repeats, or just to any one of them. This is
/// only relevant for elements that can repeat.
pub any: Boolean,
/// Documentation of the purpose of use of the bindingproviding additional
/// information about how it is intended to be used.
pub documentation: Markdown,
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
/// The use of this additional binding.
pub purpose: Code,
/// Concise documentation - for summary tables.
pub short_doco: String,
/// Qualifies the usage of the binding. Typically bindings are qualified by
/// jurisdiction, but they may also be qualified by gender, workflow status,
/// clinical domain etc. The information to decide whether a usege context applies
/// is usually outside the resource, determined by context, and this might present
/// challenges for validation tooling.
pub usage: Vec<UsageContext>,
/// The valueSet that is being bound for the purpose.
pub value_set: Canonical,
}

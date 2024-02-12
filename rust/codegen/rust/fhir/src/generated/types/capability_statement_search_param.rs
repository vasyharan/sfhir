use super::*;
/// A Capability Statement documents a set of capabilities (behaviors) of a
/// FHIR Server or Client for a particular version of FHIR that may be used as a
/// statement of actual server functionality or a statement of required or desired
/// server implementation.
#[derive(Debug,Clone,PartialEq)]
pub struct CapabilityStatementSearchParam {
/// An absolute URI that is a formal reference to where this parameter was
/// first defined, so that a client can be confident of the meaning of the
/// search parameter (a reference to [SearchParameter.url](searchparameter-
/// definitions.html#SearchParameter.url)). This element SHALL be populated if
/// the search parameter refers to a SearchParameter defined by the FHIR core
/// specification or externally defined IGs.
pub definition: Canonical,
/// This allows documentation of any distinct behaviors about how the search
/// parameter is used.  For example, text matching algorithms.
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
/// The label used for the search parameter in this particular system's API -
/// i.e. the 'name' portion of the name-value pair that will appear as part of the
/// search URL.  This SHOULD be the same as the SearchParameter.code of the defining
/// SearchParameter.  However, it can sometimes differ if necessary to disambiguate
/// when a server supports multiple SearchParameters that happen to share the same
/// code.
pub name: String,
/// The type of value a search parameter refers to, and how the content is
/// interpreted.
pub r#type: Code,
}

use super::*;
/// A set of rules of how a particular interoperability or standards problem is
/// solved - typically through the use of FHIR resources. This resource is used
/// to gather all the parts of an implementation guide into a logical whole and to
/// publish a computable definition of all the parts.
#[derive(Debug,Clone,PartialEq)]
pub struct ImplementationGuidePage {
/// A code that indicates how the page is generated.
pub generation: Code,
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
/// The url by which the page should be known when published.
pub name: Url,
/// Nested Pages/Sections under this page.
pub page: Vec<ImplementationGuidePage>,
/// Indicates the URL or the actual content to provide for the page.
pub source_markdown: String,
/// Indicates the URL or the actual content to provide for the page.
pub source_string: String,
/// Indicates the URL or the actual content to provide for the page.
pub source_url: String,
/// A short title used to represent this page in navigational structures such as
/// table of contents, bread crumbs, etc.
pub title: String,
}

use super::*;
/// Demographics and administrative information about a person independent of a
/// specific health-related context.
#[derive(Debug,Clone,PartialEq)]
pub struct PersonCommunication {
/// Unique id for the element within a resource (for internal references). This may
/// be any string value that does not contain spaces.
pub id: String,
/// The ISO-639-1 alpha 2 code in lower case for the language, optionally followed
/// by a hyphen and the ISO-3166-1 alpha 2 code for the region in upper case; e.g.
/// "en" for English, or "en-US" for American English versus "en-AU" for Australian
/// English.
pub language: CodeableConcept,
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
/// Indicates whether or not the person prefers this language (over other languages
/// he masters up a certain level).
pub preferred: Boolean,
}
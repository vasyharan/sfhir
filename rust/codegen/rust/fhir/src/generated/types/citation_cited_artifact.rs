use super::*;
/// The Citation Resource enables reference to any knowledge artifact for purposes
/// of identification and attribution. The Citation Resource supports existing
/// reference structures and developing publication practices such as versioning,
/// expressing complex contributorship roles, and referencing computable resources.
#[derive(Debug,Clone,PartialEq)]
pub struct CitationCitedArtifact {
/// The abstract may be used to convey article-contained abstracts, externally-
/// created abstracts, or other descriptive summaries.
pub r#abstract: Vec<CitationAbstract>,
/// The assignment to an organizing scheme.
pub classification: Vec<CitationClassification1>,
/// This element is used to list authors and other contributors, their contact
/// information, specific contributions, and summary statements.
pub contributorship: CitationContributorship,
/// The status of the cited artifact.
pub current_state: Vec<CodeableConcept>,
/// When the cited artifact was accessed.
pub date_accessed: DateTime,
/// Unique id for the element within a resource (for internal references). This may
/// be any string value that does not contain spaces.
pub id: String,
/// A formal identifier that is used to identify the cited artifact when it is
/// represented in other formats, or referenced in a specification, model, design or
/// an instance.
pub identifier: Vec<Identifier>,
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
/// Any additional information or content for the article or artifact.
pub note: Vec<Annotation>,
/// The component of the article or artifact.
pub part: CitationPart,
/// If multiple, used to represent alternative forms of the article that are not
/// separate citations.
pub publication_form: Vec<CitationPublicationForm>,
/// A formal identifier that is used to identify things closely related to the cited
/// artifact.
pub related_identifier: Vec<Identifier>,
/// The artifact related to the cited artifact.
pub relates_to: Vec<CitationRelatesTo>,
/// An effective date or period, historical or future, actual or expected, for a
/// status of the cited artifact.
pub status_date: Vec<CitationStatusDate1>,
/// The title details of the article or artifact.
pub title: Vec<CitationTitle>,
/// The defined version of the cited artifact.
pub version: CitationVersion,
/// Used for any URL for the article or artifact cited.
pub web_location: Vec<CitationWebLocation>,
}

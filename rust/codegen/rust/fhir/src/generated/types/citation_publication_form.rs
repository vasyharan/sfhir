use super::*;
/// The Citation Resource enables reference to any knowledge artifact for purposes
/// of identification and attribution. The Citation Resource supports existing
/// reference structures and developing publication practices such as versioning,
/// expressing complex contributorship roles, and referencing computable resources.
#[derive(Debug,Clone,PartialEq)]
pub struct CitationPublicationForm {
/// Entry number or identifier for inclusion in a database.
pub accession_number: String,
/// The date the article was added to the database, or the date the article was
/// released.
pub article_date: DateTime,
/// Describes the form of the medium cited. Common codes are "Internet" or "Print".
/// The CitedMedium value set has 6 codes. The codes internet, print, and offline-
/// digital-storage are the common codes for a typical publication form, though
/// internet and print are more common for study citations. Three additional codes
/// (each appending one of the primary codes with "-without-issue" are used for
/// situations when a study is published both within an issue (of a periodical
/// release as commonly done for journals) AND is published separately from the
/// issue (as commonly done with early online publication), to represent specific
/// identification of the publication form not associated with the issue.
pub cited_medium: CodeableConcept,
/// Copyright notice for the full article or artifact.
pub copyright: Markdown,
/// Used for isolated representation of first page.
pub first_page: String,
/// Unique id for the element within a resource (for internal references). This may
/// be any string value that does not contain spaces.
pub id: String,
/// Issue, part or supplement of journal or other collection in which the article
/// is published.
pub issue: String,
/// The language or languages in which this form of the article is published.
pub language: Vec<CodeableConcept>,
/// Used for isolated representation of last page.
pub last_page: String,
/// The date the article was last revised or updated in the database.
pub last_revision_date: DateTime,
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
/// Actual or approximate number of pages or screens. Distinct from reporting the
/// page numbers.
pub page_count: String,
/// Used for full display of pagination.
pub page_string: String,
/// Spring, Summer, Fall/Autumn, Winter.
pub publication_date_season: String,
/// Text representation of the date on which the issue of the cited artifact was
/// published.
pub publication_date_text: String,
/// The collection the cited article or artifact is published in.
pub published_in: CitationPublishedIn,
/// Volume number of journal or other collection in which the article is published.
pub volume: String,
}

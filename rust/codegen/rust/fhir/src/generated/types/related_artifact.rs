use super::*;
/// Related artifacts such as additional documentation, justification, or
/// bibliographic references.
#[derive(Debug,Clone,PartialEq)]
pub struct RelatedArtifact {
/// A bibliographic citation for the related artifact. This text SHOULD be formatted
/// according to an accepted citation format.
pub citation: Markdown,
/// Provides additional classifiers of the related artifact.
pub classifier: Vec<CodeableConcept>,
/// A brief description of the document or knowledge resource being referenced,
/// suitable for display to a consumer.
pub display: String,
/// The document being referenced, represented as an attachment. This is exclusive
/// with the resource element.
pub document: Attachment,
/// Unique id for the element within a resource (for internal references). This may
/// be any string value that does not contain spaces.
pub id: String,
/// A short label that can be used to reference the citation from elsewhere in the
/// containing artifact, such as a footnote index.
pub label: String,
/// The date of publication of the artifact being referred to.
pub publication_date: Date,
/// The publication status of the artifact being referred to.
pub publication_status: Code,
/// The related artifact, such as a library, value set, profile, or other knowledge
/// resource.
pub resource: Canonical,
/// The related artifact, if the artifact is not a canonical resource, or a resource
/// reference to a canonical resource.
pub resource_reference: Reference,
/// The type of relationship to the related artifact.
pub r#type: Type,
}

#[derive(Debug,Clone,PartialEq)]
pub enum Type {
Documentation,
Justification,
Citation,
Predecessor,
Successor,
DerivedFrom,
DependsOn,
ComposedOf,
PartOf,
Amends,
AmendedWith,
Appends,
AppendedWith,
Cites,
CitedBy,
CommentsOn,
CommentIn,
Contains,
ContainedIn,
Corrects,
CorrectionIn,
Replaces,
ReplacedWith,
Retracts,
RetractedBy,
Signs,
SimilarTo,
Supports,
SupportedWith,
Transforms,
TransformedInto,
TransformedWith,
Documents,
SpecificationOf,
CreatedWith,
CiteAs,
}
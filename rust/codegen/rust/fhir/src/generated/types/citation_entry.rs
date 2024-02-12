use super::*;
/// The Citation Resource enables reference to any knowledge artifact for purposes
/// of identification and attribution. The Citation Resource supports existing
/// reference structures and developing publication practices such as versioning,
/// expressing complex contributorship roles, and referencing computable resources.
#[derive(Debug,Clone,PartialEq)]
pub struct CitationEntry {
/// Organization affiliated with the contributor.
pub affiliation: Vec<Reference>,
/// Contributions with accounting for time or number.
pub contribution_instance: Vec<CitationContributionInstance>,
/// This element identifies the specific nature of an individual’s contribution with
/// respect to the cited work.
pub contribution_type: Vec<CodeableConcept>,
/// The identity of the individual contributor.
pub contributor: Reference,
/// Whether the contributor is the corresponding contributor for the role.
pub corresponding_contact: Boolean,
/// For citation styles that use initials.
pub forename_initials: String,
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
/// Provides a numerical ranking to represent the degree of contributorship relative
/// to other contributors, such as 1 for first author and 2 for second author.
pub ranking_order: PositiveInt,
/// The role of the contributor (e.g. author, editor, reviewer, funder).
pub role: CodeableConcept,
}

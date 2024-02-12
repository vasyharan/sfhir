use super::*;
/// A set of requirements - a list of features or behaviors of designed systems that
/// are necessary to achieve organizational or regulatory goals.
#[derive(Debug,Clone,PartialEq)]
pub struct RequirementsStatement {
/// This boolean flag is set to true of the text of the requirement is conditional
/// on something e.g. it includes lanauage like 'if x then y'. This conditionality
/// flag is introduced for purposes of filtering and colour highlighting etc.
pub conditionality: Boolean,
/// A short human usable label for this statement.
pub conformance: Vec<Code>,
/// Another statement on one of the requirements that this requirement clarifies
/// or restricts.
pub derived_from: String,
/// Unique id for the element within a resource (for internal references). This may
/// be any string value that does not contain spaces.
pub id: String,
/// Key that identifies this statement (unique within this resource).
pub key: Id,
/// A short human usable label for this statement.
pub label: String,
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
/// A larger requirement that this requirement helps to refine and enable.
pub parent: String,
/// A reference to another artifact that created this requirement. This could be
/// a Profile, etc., or external regulation, or business requirements expressed
/// elsewhere.
pub reference: Vec<Url>,
/// The actual requirement for human consumption.
pub requirement: Markdown,
/// A reference to another artifact that satisfies this requirement. This could be
/// a Profile, extension, or an element in one of those, or a CapabilityStatement,
/// OperationDefinition, SearchParameter, CodeSystem(/code), ValueSet, Libary etc.
pub satisfied_by: Vec<Url>,
/// Who asked for this statement to be a requirement. By default, it's assumed that
/// the publisher knows who it is if it matters.
pub source: Vec<Reference>,
}

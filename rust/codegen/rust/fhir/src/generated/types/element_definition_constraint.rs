use super::*;
/// Captures constraints on each element within the resource, profile, or extension.
#[derive(Debug,Clone,PartialEq)]
pub struct ElementDefinitionConstraint {
/// A [FHIRPath](fhirpath.html) expression of constraint that can be executed to see
/// if this constraint is met.
pub expression: String,
/// Text that can be used to describe the constraint in messages identifying that
/// the constraint has been violated.
pub human: String,
/// Unique id for the element within a resource (for internal references). This may
/// be any string value that does not contain spaces.
pub id: String,
/// Allows identification of which elements have their cardinalities impacted by
/// the constraint.  Will not be referenced for constraints that do not affect
/// cardinality.
pub key: Id,
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
/// Description of why this constraint is necessary or appropriate.
pub requirements: Markdown,
/// Identifies the impact constraint violation has on the conformance of the
/// instance.
pub severity: Severity,
/// A reference to the original source of the constraint, for traceability purposes.
pub source: Canonical,
/// If true, indicates that the warning or best practice guideline should be
/// suppressed.
pub suppress: Boolean,
}

#[derive(Debug,Clone,PartialEq)]
pub enum Severity {
Error,
Warning,
}

use super::*;
/// Representation of a molecular sequence.
#[derive(Debug,Clone,PartialEq)]
pub struct MolecularSequenceRelative {
/// These are different ways of identifying nucleotides or amino acids within a
/// sequence. Different databases and file types may use different systems. For
/// detail definitions, see https://loinc.org/92822-6/ for more detail.
pub coordinate_system: CodeableConcept,
/// Changes in sequence from the starting sequence.
pub edit: Vec<MolecularSequenceEdit>,
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
/// Indicates the order in which the sequence should be considered when putting
/// multiple 'relative' elements together.
pub ordinal_position: Integer,
/// Indicates the nucleotide range in the composed sequence when multiple 'relative'
/// elements are used together.
pub sequence_range: Range,
/// A sequence that is used as a starting sequence to describe variants that are
/// present in a sequence analyzed.
pub starting_sequence: MolecularSequenceStartingSequence,
}

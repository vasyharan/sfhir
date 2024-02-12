use super::*;
/// Representation of a molecular sequence.
#[derive(Debug,Clone,PartialEq)]
pub struct MolecularSequenceEdit {
/// End position of the edit on the starting sequence. If the coordinate system
/// is 0-based then end is exclusive and does not include the last position. If
/// the coordinate system is 1-base, then end is inclusive and includes the last
/// position.
pub end: Integer,
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
/// Allele in the starting sequence. Nucleotide(s)/amino acids from start position
/// of sequence to stop position of sequence on the positive (+) strand of the
/// starting sequence. When the sequence  type is DNA, it should be the sequence
/// on the positive (+) strand. This will lay in the range between variant.start
/// and variant.end.
pub replaced_sequence: String,
/// Allele that was observed. Nucleotide(s)/amino acids from start position
/// of sequence to stop position of sequence on the positive (+) strand of the
/// observed sequence. When the sequence type is DNA, it should be the sequence on
/// the positive (+) strand. This will lay in the range between variant.start and
/// variant.end.
pub replacement_sequence: String,
/// Start position of the edit on the starting sequence. If the coordinate system is
/// either 0-based or 1-based, then start position is inclusive.
pub start: Integer,
}

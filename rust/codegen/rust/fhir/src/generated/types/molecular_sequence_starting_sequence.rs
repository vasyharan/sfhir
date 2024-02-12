use super::*;
/// Representation of a molecular sequence.
#[derive(Debug,Clone,PartialEq)]
pub struct MolecularSequenceStartingSequence {
/// Structural unit composed of a nucleic acid molecule which controls its own
/// replication through the interaction of specific proteins at one or more
/// origins of replication ([SO:0000340](http://www.sequenceontology.org/browser/
/// current_svn/term/SO:0000340)).
pub chromosome: CodeableConcept,
/// The genome assembly used for starting sequence, e.g. GRCh38.
pub genome_assembly: CodeableConcept,
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
/// A relative reference to a DNA strand based on gene orientation. The strand
/// that contains the open reading frame of the gene is the "sense" strand, and the
/// opposite complementary strand is the "antisense" strand.
pub orientation: Code,
/// The reference sequence that represents the starting sequence.
pub sequence_codeable_concept: CodeableConcept,
/// The reference sequence that represents the starting sequence.
pub sequence_reference: Reference,
/// The reference sequence that represents the starting sequence.
pub sequence_string: String,
/// An absolute reference to a strand. The Watson strand is the strand whose 5'-end
/// is on the short arm of the chromosome, and the Crick strand as the one whose
/// 5'-end is on the long arm.
pub strand: Code,
/// End position of the window on the starting sequence. This value should honor the
/// rules of the  coordinateSystem.
pub window_end: Integer,
/// Start position of the window on the starting sequence. This value should honor
/// the rules of the coordinateSystem.
pub window_start: Integer,
}

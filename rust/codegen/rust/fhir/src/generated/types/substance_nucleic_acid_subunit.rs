use super::*;
/// Nucleic acids are defined by three distinct elements: the base, sugar and
/// linkage. Individual substance/moiety IDs will be created for each of these
/// elements. The nucleotide sequence will be always entered in the 5’-3’ direction.
#[derive(Debug,Clone,PartialEq)]
pub struct SubstanceNucleicAcidSubunit {
/// The nucleotide present at the 5’ terminal shall be specified based on a
/// controlled vocabulary. Since the sequence is represented from the 5' to the
/// 3' end, the 5’ prime nucleotide is the letter at the first position in the
/// sequence. A separate representation would be redundant.
pub five_prime: CodeableConcept,
/// Unique id for the element within a resource (for internal references). This may
/// be any string value that does not contain spaces.
pub id: String,
/// The length of the sequence shall be captured.
pub length: Integer,
/// The linkages between sugar residues will also be captured.
pub linkage: Vec<SubstanceNucleicAcidLinkage>,
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
/// Actual nucleotide sequence notation from 5' to 3' end using standard single
/// letter codes. In addition to the base sequence, sugar and type of phosphate or
/// non-phosphate linkage should also be captured.
pub sequence: String,
/// (TBC).
pub sequence_attachment: Attachment,
/// Index of linear sequences of nucleic acids in order of decreasing length.
/// Sequences of the same length will be ordered by molecular weight. Subunits that
/// have identical sequences will be repeated and have sequential subscripts.
pub subunit: Integer,
/// 5.3.6.8.1 Sugar ID (Mandatory).
pub sugar: Vec<SubstanceNucleicAcidSugar>,
/// The nucleotide present at the 3’ terminal shall be specified based on a
/// controlled vocabulary. Since the sequence is represented from the 5' to the 3'
/// end, the 5’ prime nucleotide is the letter at the last position in the sequence.
/// A separate representation would be redundant.
pub three_prime: CodeableConcept,
}
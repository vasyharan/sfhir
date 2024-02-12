use super::*;
/// A GenomicStudy is a set of analyses performed to analyze and generate genomic
/// data.
#[derive(Debug,Clone,PartialEq)]
pub struct GenomicStudyAnalysis {
/// Type of the genomic changes studied in the analysis, e.g., DNA, RNA, or amino
/// acid change.
pub change_type: Vec<CodeableConcept>,
/// The date of the analysis event.
pub date: DateTime,
/// Devices used for the analysis (e.g., instruments, software), with settings and
/// parameters.
pub device: Vec<GenomicStudyDevice>,
/// The focus of a genomic analysis when it is not the patient of record
/// representing something or someone associated with the patient such
/// as a spouse, parent, child, or sibling. For example, in trio testing,
/// the GenomicStudy.subject would be the child (proband) and the
/// GenomicStudy.analysis.focus of a specific analysis would be the parent.
pub focus: Vec<Reference>,
/// The reference genome build that is used in this analysis.
pub genome_build: CodeableConcept,
/// Unique id for the element within a resource (for internal references). This may
/// be any string value that does not contain spaces.
pub id: String,
/// Identifiers for the analysis event.
pub identifier: Vec<Identifier>,
/// Inputs for the analysis event.
pub input: Vec<GenomicStudyInput>,
/// The defined protocol that describes the analysis.
pub instantiates_canonical: Canonical,
/// The URL pointing to an externally maintained protocol that describes the
/// analysis.
pub instantiates_uri: Uri,
/// Type of the methods used in the analysis, e.g., Fluorescence in situ
/// hybridization (FISH), Karyotyping, or Microsatellite instability testing (MSI).
pub method_type: Vec<CodeableConcept>,
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
/// Any notes capture with the analysis event.
pub note: Vec<Annotation>,
/// Outputs for the analysis event.
pub output: Vec<GenomicStudyOutput>,
/// Performer for the analysis event.
pub performer: Vec<GenomicStudyPerformer>,
/// The protocol that was performed for the analysis event.
pub protocol_performed: Reference,
/// Genomic regions actually called in the analysis event (BED file).
pub regions_called: Vec<Reference>,
/// The genomic regions to be studied in the analysis (BED file).
pub regions_studied: Vec<Reference>,
/// The specimen used in the analysis event.
pub specimen: Vec<Reference>,
/// Name of the analysis event (human friendly).
pub title: String,
}

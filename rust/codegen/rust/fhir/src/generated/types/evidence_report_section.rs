use super::*;
/// The EvidenceReport Resource is a specialized container for a collection of
/// resources and codeable concepts, adapted to support compositions of Evidence,
/// EvidenceVariable, and Citation resources and related concepts.
#[derive(Debug,Clone,PartialEq)]
pub struct EvidenceReportSection {
/// Identifies who is responsible for the information in this section, not
/// necessarily who typed it in.
pub author: Vec<Reference>,
/// If the section is empty, why the list is empty. An empty section typically has
/// some text explaining the empty reason.
pub empty_reason: CodeableConcept,
/// Specifies any type of classification of the evidence report.
pub entry_classifier: Vec<CodeableConcept>,
/// Quantity as content.
pub entry_quantity: Vec<Quantity>,
/// A reference to the actual resource from which the narrative in the section is
/// derived.
pub entry_reference: Vec<Reference>,
/// A code identifying the kind of content contained within the section. This should
/// be consistent with the section title.
pub focus: CodeableConcept,
/// A definitional Resource identifying the kind of content contained within the
/// section. This should be consistent with the section title.
pub focus_reference: Reference,
/// Unique id for the element within a resource (for internal references). This may
/// be any string value that does not contain spaces.
pub id: String,
/// How the entry list was prepared - whether it is a working list that is suitable
/// for being maintained on an ongoing basis, or if it represents a snapshot of a
/// list of items from another source, or whether it is a prepared list where items
/// may be marked as added, modified or deleted.
pub mode: Code,
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
/// Specifies the order applied to the items in the section entries.
pub ordered_by: CodeableConcept,
/// A nested sub-section within this section.
pub section: Vec<EvidenceReportSection>,
/// A human-readable narrative that contains the attested content of the section,
/// used to represent the content of the resource to a human. The narrative need not
/// encode all the structured data, but is peferred to contain sufficient detail to
/// make it acceptable for a human to just read the narrative.
pub text: Narrative,
/// The label for this particular section.  This will be part of the rendered
/// content for the document, and is often used to build a table of contents.
pub title: String,
}

/// Representation of a molecular sequence.
#[derive(Debug, Clone, PartialEq)]
pub struct MolecularSequence {
    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, nor can they have
    /// their own independent transaction scope. This is allowed to be a Parameters
    /// resource if and only if it is referenced by a resource that provides context/
    /// meaning.
    pub contained: Vec<super::resource_list::ResourceList>,
    /// The method for sequencing, for example, chip information.
    pub device: super::reference::Reference,
    /// The actual focus of a molecular sequence when it is not the patient of record
    /// representing something or someone associated with the patient such as a spouse,
    /// parent, child, or sibling. For example, in trio testing, the subject would be
    /// the child (proband) and the focus would be the parent.
    pub focus: Vec<super::reference::Reference>,
    /// Sequence that was observed as file content. Can be an actual file contents, or
    /// referenced by a URL to an external system.
    pub formatted: Vec<super::attachment::Attachment>,
    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub id: super::id::Id,
    /// A unique identifier for this particular sequence instance.
    pub identifier: Vec<super::identifier::Identifier>,
    /// A reference to a set of rules that were followed when the resource was
    /// constructed, and which must be understood when processing the content. Often,
    /// this is a reference to an implementation guide that defines the special rules
    /// along with other profiles etc.
    pub implicit_rules: super::uri::Uri,
    /// The base language in which the resource is written.
    pub language: super::code::Code,
    /// Sequence that was observed.
    pub literal: super::string::String,
    /// The metadata about the resource. This is content that is maintained by the
    /// infrastructure. Changes to the content might not always be associated with
    /// version changes to the resource.
    pub meta: super::meta::Meta,
    /// May be used to represent additional information that is not part of the
    /// basic definition of the resource and that modifies the understanding of the
    /// element that contains it and/or the understanding of the containing element's
    /// descendants. Usually modifier elements provide negation or qualification.
    /// To make the use of extensions safe and managable, there is a strict set
    /// of governance applied to the definition and use of extensions. Though any
    /// implementer is allowed to define an extension, there is a set of requirements
    /// that SHALL be met as part of the definition of the extension. Applications
    /// processing a resource are required to check for modifier extensions.
    ///
    /// Modifier extensions SHALL NOT change the meaning of any elements on Resource
    /// or DomainResource (including cannot change the meaning of modifierExtension
    /// itself).
    pub modifier_extension: Vec<super::extension::Extension>,
    /// The organization or lab that should be responsible for this result.
    pub performer: super::reference::Reference,
    /// A sequence defined relative to another sequence.
    pub relative: Vec<super::molecular_sequence::MolecularSequenceRelative>,
    /// This is a MolecularSequence resource
    pub resource_type: String,
    /// Specimen used for sequencing.
    pub specimen: super::reference::Reference,
    /// Indicates the subject this sequence is associated too.
    pub subject: super::reference::Reference,
    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need
    /// not encode all the structured data, but is required to contain sufficient detail
    /// to make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub text: super::narrative::Narrative,
    /// Amino Acid Sequence/ DNA Sequence / RNA Sequence.
    pub r#type: super::code::Code,
}

/// Representation of a molecular sequence.
#[derive(Debug, Clone, PartialEq)]
pub struct MolecularSequenceEdit {
    /// End position of the edit on the starting sequence. If the coordinate system
    /// is 0-based then end is exclusive and does not include the last position. If
    /// the coordinate system is 1-base, then end is inclusive and includes the last
    /// position.
    pub end: super::integer::Integer,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
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
    pub modifier_extension: Vec<super::extension::Extension>,
    /// Allele in the starting sequence. Nucleotide(s)/amino acids from start position
    /// of sequence to stop position of sequence on the positive (+) strand of the
    /// starting sequence. When the sequence  type is DNA, it should be the sequence
    /// on the positive (+) strand. This will lay in the range between variant.start
    /// and variant.end.
    pub replaced_sequence: super::string::String,
    /// Allele that was observed. Nucleotide(s)/amino acids from start position
    /// of sequence to stop position of sequence on the positive (+) strand of the
    /// observed sequence. When the sequence type is DNA, it should be the sequence on
    /// the positive (+) strand. This will lay in the range between variant.start and
    /// variant.end.
    pub replacement_sequence: super::string::String,
    /// Start position of the edit on the starting sequence. If the coordinate system is
    /// either 0-based or 1-based, then start position is inclusive.
    pub start: super::integer::Integer,
}

/// Representation of a molecular sequence.
#[derive(Debug, Clone, PartialEq)]
pub struct MolecularSequenceRelative {
    /// These are different ways of identifying nucleotides or amino acids within a
    /// sequence. Different databases and file types may use different systems. For
    /// detail definitions, see https://loinc.org/92822-6/ for more detail.
    pub coordinate_system: super::codeable_concept::CodeableConcept,
    /// Changes in sequence from the starting sequence.
    pub edit: Vec<super::molecular_sequence::MolecularSequenceEdit>,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
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
    pub modifier_extension: Vec<super::extension::Extension>,
    /// Indicates the order in which the sequence should be considered when putting
    /// multiple 'relative' elements together.
    pub ordinal_position: super::integer::Integer,
    /// Indicates the nucleotide range in the composed sequence when multiple 'relative'
    /// elements are used together.
    pub sequence_range: super::range::Range,
    /// A sequence that is used as a starting sequence to describe variants that are
    /// present in a sequence analyzed.
    pub starting_sequence: super::molecular_sequence::MolecularSequenceStartingSequence,
}

/// Representation of a molecular sequence.
#[derive(Debug, Clone, PartialEq)]
pub struct MolecularSequenceStartingSequence {
    /// Structural unit composed of a nucleic acid molecule which controls its own
    /// replication through the interaction of specific proteins at one or more
    /// origins of replication ([SO:0000340](http://www.sequenceontology.org/browser/
    /// current_svn/term/SO:0000340)).
    pub chromosome: super::codeable_concept::CodeableConcept,
    /// The genome assembly used for starting sequence, e.g. GRCh38.
    pub genome_assembly: super::codeable_concept::CodeableConcept,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
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
    pub modifier_extension: Vec<super::extension::Extension>,
    /// A relative reference to a DNA strand based on gene orientation. The strand
    /// that contains the open reading frame of the gene is the "sense" strand, and the
    /// opposite complementary strand is the "antisense" strand.
    pub orientation: super::code::Code,
    /// The reference sequence that represents the starting sequence.
    pub sequence_codeable_concept: super::codeable_concept::CodeableConcept,
    /// The reference sequence that represents the starting sequence.
    pub sequence_reference: super::reference::Reference,
    /// The reference sequence that represents the starting sequence.
    pub sequence_string: String,
    /// An absolute reference to a strand. The Watson strand is the strand whose 5'-end
    /// is on the short arm of the chromosome, and the Crick strand as the one whose
    /// 5'-end is on the long arm.
    pub strand: super::code::Code,
    /// End position of the window on the starting sequence. This value should honor the
    /// rules of the  coordinateSystem.
    pub window_end: super::integer::Integer,
    /// Start position of the window on the starting sequence. This value should honor
    /// the rules of the coordinateSystem.
    pub window_start: super::integer::Integer,
}

/// Nucleic acids are defined by three distinct elements: the base, sugar and
/// linkage. Individual substance/moiety IDs will be created for each of these
/// elements. The nucleotide sequence will be always entered in the 5’-3’ direction.
#[derive(Debug, Clone, PartialEq)]
pub struct SubstanceNucleicAcid {
    /// The area of hybridisation shall be described if applicable for double stranded
    /// RNA or DNA. The number associated with the subunit followed by the number
    /// associated to the residue shall be specified in increasing order. The underscore
    /// “” shall be used as separator as follows: “Subunitnumber Residue”.
    pub area_of_hybridisation: super::string::String,
    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, nor can they have
    /// their own independent transaction scope. This is allowed to be a Parameters
    /// resource if and only if it is referenced by a resource that provides context/
    /// meaning.
    pub contained: Vec<super::resource_list::ResourceList>,
    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub id: super::id::Id,
    /// A reference to a set of rules that were followed when the resource was
    /// constructed, and which must be understood when processing the content. Often,
    /// this is a reference to an implementation guide that defines the special rules
    /// along with other profiles etc.
    pub implicit_rules: super::uri::Uri,
    /// The base language in which the resource is written.
    pub language: super::code::Code,
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
    /// The number of linear sequences of nucleotides linked through phosphodiester
    /// bonds shall be described. Subunits would be strands of nucleic acids that are
    /// tightly associated typically through Watson-Crick base pairing. NOTE: If not
    /// specified in the reference source, the assumption is that there is 1 subunit.
    pub number_of_subunits: super::integer::Integer,
    /// (TBC).
    pub oligo_nucleotide_type: super::codeable_concept::CodeableConcept,
    /// This is a SubstanceNucleicAcid resource
    pub resource_type: String,
    /// The type of the sequence shall be specified based on a controlled vocabulary.
    pub sequence_type: super::codeable_concept::CodeableConcept,
    /// Subunits are listed in order of decreasing length; sequences of the same length
    /// will be ordered by molecular weight; subunits that have identical sequences will
    /// be repeated multiple times.
    pub subunit: Vec<super::substance_nucleic_acid::SubstanceNucleicAcidSubunit>,
    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need
    /// not encode all the structured data, but is required to contain sufficient detail
    /// to make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub text: super::narrative::Narrative,
}

/// Nucleic acids are defined by three distinct elements: the base, sugar and
/// linkage. Individual substance/moiety IDs will be created for each of these
/// elements. The nucleotide sequence will be always entered in the 5’-3’ direction.
#[derive(Debug, Clone, PartialEq)]
pub struct SubstanceNucleicAcidLinkage {
    /// The entity that links the sugar residues together should also be captured for
    /// nearly all naturally occurring nucleic acid the linkage is a phosphate group.
    /// For many synthetic oligonucleotides phosphorothioate linkages are often seen.
    /// Linkage connectivity is assumed to be 3’-5’. If the linkage is either 3’-3’ or
    /// 5’-5’ this should be specified.
    pub connectivity: super::string::String,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// Each linkage will be registered as a fragment and have an ID.
    pub identifier: super::identifier::Identifier,
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
    /// Each linkage will be registered as a fragment and have at least one name. A
    /// single name shall be assigned to each linkage.
    pub name: super::string::String,
    /// Residues shall be captured as described in 5.3.6.8.3.
    pub residue_site: super::string::String,
}

/// Nucleic acids are defined by three distinct elements: the base, sugar and
/// linkage. Individual substance/moiety IDs will be created for each of these
/// elements. The nucleotide sequence will be always entered in the 5’-3’ direction.
#[derive(Debug, Clone, PartialEq)]
pub struct SubstanceNucleicAcidSubunit {
    /// The nucleotide present at the 5’ terminal shall be specified based on a
    /// controlled vocabulary. Since the sequence is represented from the 5' to the
    /// 3' end, the 5’ prime nucleotide is the letter at the first position in the
    /// sequence. A separate representation would be redundant.
    pub five_prime: super::codeable_concept::CodeableConcept,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// The length of the sequence shall be captured.
    pub length: super::integer::Integer,
    /// The linkages between sugar residues will also be captured.
    pub linkage: Vec<super::substance_nucleic_acid::SubstanceNucleicAcidLinkage>,
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
    /// Actual nucleotide sequence notation from 5' to 3' end using standard single
    /// letter codes. In addition to the base sequence, sugar and type of phosphate or
    /// non-phosphate linkage should also be captured.
    pub sequence: super::string::String,
    /// (TBC).
    pub sequence_attachment: super::attachment::Attachment,
    /// Index of linear sequences of nucleic acids in order of decreasing length.
    /// Sequences of the same length will be ordered by molecular weight. Subunits that
    /// have identical sequences will be repeated and have sequential subscripts.
    pub subunit: super::integer::Integer,
    /// 5.3.6.8.1 Sugar ID (Mandatory).
    pub sugar: Vec<super::substance_nucleic_acid::SubstanceNucleicAcidSugar>,
    /// The nucleotide present at the 3’ terminal shall be specified based on a
    /// controlled vocabulary. Since the sequence is represented from the 5' to the 3'
    /// end, the 5’ prime nucleotide is the letter at the last position in the sequence.
    /// A separate representation would be redundant.
    pub three_prime: super::codeable_concept::CodeableConcept,
}

/// Nucleic acids are defined by three distinct elements: the base, sugar and
/// linkage. Individual substance/moiety IDs will be created for each of these
/// elements. The nucleotide sequence will be always entered in the 5’-3’ direction.
#[derive(Debug, Clone, PartialEq)]
pub struct SubstanceNucleicAcidSugar {
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// The Substance ID of the sugar or sugar-like component that make up the
    /// nucleotide.
    pub identifier: super::identifier::Identifier,
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
    /// The name of the sugar or sugar-like component that make up the nucleotide.
    pub name: super::string::String,
    /// The residues that contain a given sugar will be captured. The order of given
    /// residues will be captured in the 5‘-3‘direction consistent with the base
    /// sequences listed above.
    pub residue_site: super::string::String,
}

/// The detailed description of a substance, typically at a level beyond what is
/// used for prescribing.
#[derive(Debug, Clone, PartialEq)]
pub struct SubstanceDefinition {
    /// General specifications for this substance.
    pub characterization: Vec<super::substance_definition::SubstanceDefinitionCharacterization>,
    /// A high level categorization, e.g. polymer or nucleic acid, or food, chemical,
    /// biological, or a lower level such as the general types of polymer (linear or
    /// branch chain) or type of impurity (process related or contaminant).
    pub classification: Vec<super::codeable_concept::CodeableConcept>,
    /// Codes associated with the substance.
    pub code: Vec<super::substance_definition::SubstanceDefinitionCode>,
    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, nor can they have
    /// their own independent transaction scope. This is allowed to be a Parameters
    /// resource if and only if it is referenced by a resource that provides context/
    /// meaning.
    pub contained: Vec<super::resource_list::ResourceList>,
    /// Textual description of the substance.
    pub description: super::markdown::Markdown,
    /// If the substance applies to human or veterinary use.
    pub domain: super::codeable_concept::CodeableConcept,
    /// The quality standard, established benchmark, to which substance complies (e.g.
    /// USP/NF, Ph. Eur, JP, BP, Company Standard).
    pub grade: Vec<super::codeable_concept::CodeableConcept>,
    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub id: super::id::Id,
    /// Identifier by which this substance is known.
    pub identifier: Vec<super::identifier::Identifier>,
    /// A reference to a set of rules that were followed when the resource was
    /// constructed, and which must be understood when processing the content. Often,
    /// this is a reference to an implementation guide that defines the special rules
    /// along with other profiles etc.
    pub implicit_rules: super::uri::Uri,
    /// Supporting literature.
    pub information_source: Vec<super::reference::Reference>,
    /// The base language in which the resource is written.
    pub language: super::code::Code,
    /// The entity that creates, makes, produces or fabricates the substance. This is a
    /// set of potential manufacturers but is not necessarily comprehensive.
    pub manufacturer: Vec<super::reference::Reference>,
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
    /// Moiety, for structural modifications.
    pub moiety: Vec<super::substance_definition::SubstanceDefinitionMoiety>,
    /// The average mass of a molecule of a compound compared to 1/12 the mass of carbon
    /// 12 and calculated as the sum of the atomic weights of the constituent atoms.
    pub molecular_weight: Vec<super::substance_definition::SubstanceDefinitionMolecularWeight>,
    /// Names applicable to this substance.
    pub name: Vec<super::substance_definition::SubstanceDefinitionName>,
    /// Textual comment about the substance's catalogue or registry record.
    pub note: Vec<super::annotation::Annotation>,
    /// Data items specific to nucleic acids.
    pub nucleic_acid: super::reference::Reference,
    /// Data items specific to polymers.
    pub polymer: super::reference::Reference,
    /// General specifications for this substance.
    pub property: Vec<super::substance_definition::SubstanceDefinitionProperty>,
    /// Data items specific to proteins.
    pub protein: super::reference::Reference,
    /// General information detailing this substance.
    pub reference_information: super::reference::Reference,
    /// A link between this substance and another, with details of the relationship.
    pub relationship: Vec<super::substance_definition::SubstanceDefinitionRelationship>,
    /// This is a SubstanceDefinition resource
    pub resource_type: String,
    /// Material or taxonomic/anatomical source for the substance.
    pub source_material: super::substance_definition::SubstanceDefinitionSourceMaterial,
    /// Status of substance within the catalogue e.g. active, retired.
    pub status: super::codeable_concept::CodeableConcept,
    /// Structural information.
    pub structure: super::substance_definition::SubstanceDefinitionStructure,
    /// An entity that is the source for the substance. It may be different from the
    /// manufacturer. Supplier is synonymous to a distributor.
    pub supplier: Vec<super::reference::Reference>,
    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need
    /// not encode all the structured data, but is required to contain sufficient detail
    /// to make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub text: super::narrative::Narrative,
    /// A business level version identifier of the substance.
    pub version: super::string::String,
}

/// The detailed description of a substance, typically at a level beyond what is
/// used for prescribing.
#[derive(Debug, Clone, PartialEq)]
pub struct SubstanceDefinitionCharacterization {
    /// The description or justification in support of the interpretation of the data
    /// file.
    pub description: super::markdown::Markdown,
    /// The data produced by the analytical instrument or a pictorial representation of
    /// that data. Examples: a JCAMP, JDX, or ADX file, or a chromatogram or spectrum
    /// analysis.
    pub file: Vec<super::attachment::Attachment>,
    /// Describes the nature of the chemical entity and explains, for instance, whether
    /// this is a base or a salt form.
    pub form: super::codeable_concept::CodeableConcept,
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
    /// The method used to elucidate the characterization of the drug substance.
    /// Example: HPLC.
    pub technique: super::codeable_concept::CodeableConcept,
}

/// The detailed description of a substance, typically at a level beyond what is
/// used for prescribing.
#[derive(Debug, Clone, PartialEq)]
pub struct SubstanceDefinitionCode {
    /// The specific code.
    pub code: super::codeable_concept::CodeableConcept,
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
    /// Any comment can be provided in this field, if necessary.
    pub note: Vec<super::annotation::Annotation>,
    /// Supporting literature.
    pub source: Vec<super::reference::Reference>,
    /// Status of the code assignment, for example 'provisional', 'approved'.
    pub status: super::codeable_concept::CodeableConcept,
    /// The date at which the code status was changed as part of the terminology
    /// maintenance.
    pub status_date: super::date_time::DateTime,
}

/// The detailed description of a substance, typically at a level beyond what is
/// used for prescribing.
#[derive(Debug, Clone, PartialEq)]
pub struct SubstanceDefinitionMoiety {
    /// Quantitative value for this moiety.
    pub amount_quantity: super::quantity::Quantity,
    /// Quantitative value for this moiety.
    pub amount_string: String,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// Identifier by which this moiety substance is known.
    pub identifier: super::identifier::Identifier,
    /// The measurement type of the quantitative value. In capturing the actual relative
    /// amounts of substances or molecular fragments it may be necessary to indicate
    /// whether the amount refers to, for example, a mole ratio or weight ratio.
    pub measurement_type: super::codeable_concept::CodeableConcept,
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
    /// Molecular formula for this moiety of this substance, typically using the Hill
    /// system.
    pub molecular_formula: super::string::String,
    /// Textual name for this moiety substance.
    pub name: super::string::String,
    /// Optical activity type.
    pub optical_activity: super::codeable_concept::CodeableConcept,
    /// Role that the moiety is playing.
    pub role: super::codeable_concept::CodeableConcept,
    /// Stereochemistry type.
    pub stereochemistry: super::codeable_concept::CodeableConcept,
}

/// The detailed description of a substance, typically at a level beyond what is
/// used for prescribing.
#[derive(Debug, Clone, PartialEq)]
pub struct SubstanceDefinitionMolecularWeight {
    /// Used to capture quantitative values for a variety of elements. If only limits
    /// are given, the arithmetic mean would be the average. If only a single definite
    /// value for a given element is given, it would be captured in this field.
    pub amount: super::quantity::Quantity,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// The method by which the molecular weight was determined.
    pub method: super::codeable_concept::CodeableConcept,
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
    /// Type of molecular weight such as exact, average (also known as. number average),
    /// weight average.
    pub r#type: super::codeable_concept::CodeableConcept,
}

/// The detailed description of a substance, typically at a level beyond what is
/// used for prescribing.
#[derive(Debug, Clone, PartialEq)]
pub struct SubstanceDefinitionName {
    /// The use context of this name for example if there is a different name a drug
    /// active ingredient as opposed to a food colour additive.
    pub domain: Vec<super::codeable_concept::CodeableConcept>,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// The jurisdiction where this name applies.
    pub jurisdiction: Vec<super::codeable_concept::CodeableConcept>,
    /// Human language that the name is written in.
    pub language: Vec<super::codeable_concept::CodeableConcept>,
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
    /// The actual name.
    pub name: super::string::String,
    /// Details of the official nature of this name.
    pub official: Vec<super::substance_definition::SubstanceDefinitionOfficial>,
    /// If this is the preferred name for this substance.
    pub preferred: super::boolean::Boolean,
    /// Supporting literature.
    pub source: Vec<super::reference::Reference>,
    /// The status of the name, for example 'current', 'proposed'.
    pub status: super::codeable_concept::CodeableConcept,
    /// A synonym of this particular name, by which the substance is also known.
    pub synonym: Vec<super::substance_definition::SubstanceDefinitionName>,
    /// A translation for this name into another human language.
    pub translation: Vec<super::substance_definition::SubstanceDefinitionName>,
    /// Name type, for example 'systematic',  'scientific, 'brand'.
    pub r#type: super::codeable_concept::CodeableConcept,
}

/// The detailed description of a substance, typically at a level beyond what is
/// used for prescribing.
#[derive(Debug, Clone, PartialEq)]
pub struct SubstanceDefinitionOfficial {
    /// Which authority uses this official name.
    pub authority: super::codeable_concept::CodeableConcept,
    /// Date of the official name change.
    pub date: super::date_time::DateTime,
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
    /// The status of the official name, for example 'draft', 'active', 'retired'.
    pub status: super::codeable_concept::CodeableConcept,
}

/// The detailed description of a substance, typically at a level beyond what is
/// used for prescribing.
#[derive(Debug, Clone, PartialEq)]
pub struct SubstanceDefinitionProperty {
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
    /// A code expressing the type of property.
    pub r#type: super::codeable_concept::CodeableConcept,
    /// A value for the property.
    pub value_attachment: super::attachment::Attachment,
    /// A value for the property.
    pub value_boolean: bool,
    /// A value for the property.
    pub value_codeable_concept: super::codeable_concept::CodeableConcept,
    /// A value for the property.
    pub value_date: String,
    /// A value for the property.
    pub value_quantity: super::quantity::Quantity,
}

/// The detailed description of a substance, typically at a level beyond what is
/// used for prescribing.
#[derive(Debug, Clone, PartialEq)]
pub struct SubstanceDefinitionRelationship {
    /// A numeric factor for the relationship, for instance to express that the salt
    /// of a substance has some percentage of the active substance in relation to some
    /// other.
    pub amount_quantity: super::quantity::Quantity,
    /// A numeric factor for the relationship, for instance to express that the salt
    /// of a substance has some percentage of the active substance in relation to some
    /// other.
    pub amount_ratio: super::ratio::Ratio,
    /// A numeric factor for the relationship, for instance to express that the salt
    /// of a substance has some percentage of the active substance in relation to some
    /// other.
    pub amount_string: String,
    /// An operator for the amount, for example "average", "approximately", "less than".
    pub comparator: super::codeable_concept::CodeableConcept,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// For example where an enzyme strongly bonds with a particular substance, this
    /// is a defining relationship for that enzyme, out of several possible substance
    /// relationships.
    pub is_defining: super::boolean::Boolean,
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
    /// For use when the numeric has an uncertain range.
    pub ratio_high_limit_amount: super::ratio::Ratio,
    /// Supporting literature.
    pub source: Vec<super::reference::Reference>,
    /// A pointer to another substance, as a resource or just a representational code.
    pub substance_definition_codeable_concept: super::codeable_concept::CodeableConcept,
    /// A pointer to another substance, as a resource or just a representational code.
    pub substance_definition_reference: super::reference::Reference,
    /// For example "salt to parent", "active moiety", "starting material", "polymorph",
    /// "impurity of".
    pub r#type: super::codeable_concept::CodeableConcept,
}

/// The detailed description of a substance, typically at a level beyond what is
/// used for prescribing.
#[derive(Debug, Clone, PartialEq)]
pub struct SubstanceDefinitionRepresentation {
    /// An attached file with the structural representation e.g. a molecular structure
    /// graphic of the substance, a JCAMP or AnIML file.
    pub document: super::reference::Reference,
    /// The format of the representation e.g. InChI, SMILES, MOLFILE, CDX, SDF, PDB,
    /// mmCIF. The logical content type rather than the physical file format of a
    /// document.
    pub format: super::codeable_concept::CodeableConcept,
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
    /// The structural representation as a text string in a standard format.
    pub representation: super::string::String,
    /// The kind of structural representation (e.g. full, partial).
    pub r#type: super::codeable_concept::CodeableConcept,
}

/// The detailed description of a substance, typically at a level beyond what is
/// used for prescribing.
#[derive(Debug, Clone, PartialEq)]
pub struct SubstanceDefinitionSourceMaterial {
    /// The country or countries where the material is harvested.
    pub country_of_origin: Vec<super::codeable_concept::CodeableConcept>,
    /// The genus of an organism, typically referring to the Latin epithet of the genus
    /// element of the plant/animal scientific name.
    pub genus: super::codeable_concept::CodeableConcept,
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
    /// An anatomical origin of the source material within an organism.
    pub part: super::codeable_concept::CodeableConcept,
    /// The species of an organism, typically referring to the Latin epithet of the
    /// species of the plant/animal.
    pub species: super::codeable_concept::CodeableConcept,
    /// A classification that provides the origin of the raw material. Example: cat hair
    /// would be an Animal source type.
    pub r#type: super::codeable_concept::CodeableConcept,
}

/// The detailed description of a substance, typically at a level beyond what is
/// used for prescribing.
#[derive(Debug, Clone, PartialEq)]
pub struct SubstanceDefinitionStructure {
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
    /// An expression which states the number and type of atoms present in a molecule of
    /// a substance.
    pub molecular_formula: super::string::String,
    /// Specified per moiety according to the Hill system, i.e. first C, then H, then
    /// alphabetical, each moiety separated by a dot.
    pub molecular_formula_by_moiety: super::string::String,
    /// The molecular weight or weight range (for proteins, polymers or nucleic acids).
    pub molecular_weight: super::substance_definition::SubstanceDefinitionMolecularWeight,
    /// Optical activity type.
    pub optical_activity: super::codeable_concept::CodeableConcept,
    /// A depiction of the structure of the substance.
    pub representation: Vec<super::substance_definition::SubstanceDefinitionRepresentation>,
    /// The source of information about the structure.
    pub source_document: Vec<super::reference::Reference>,
    /// Stereochemistry type.
    pub stereochemistry: super::codeable_concept::CodeableConcept,
    /// The method used to elucidate the structure of the drug substance. Examples: X-
    /// ray, NMR, Peptide mapping, Ligand binding assay.
    pub technique: Vec<super::codeable_concept::CodeableConcept>,
}

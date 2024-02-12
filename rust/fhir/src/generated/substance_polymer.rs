/// Properties of a substance specific to it being a polymer.
#[derive(Debug, Clone, PartialEq)]
pub struct SubstancePolymer {
    /// Overall type of the polymer.
    pub class: super::codeable_concept::CodeableConcept,
    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, nor can they have
    /// their own independent transaction scope. This is allowed to be a Parameters
    /// resource if and only if it is referenced by a resource that provides context/
    /// meaning.
    pub contained: Vec<super::resource_list::ResourceList>,
    /// Descrtibes the copolymer sequence type (polymer connectivity).
    pub copolymer_connectivity: Vec<super::codeable_concept::CodeableConcept>,
    /// Polymer geometry, e.g. linear, branched, cross-linked, network or dendritic.
    pub geometry: super::codeable_concept::CodeableConcept,
    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub id: super::id::Id,
    /// A business idenfier for this polymer, but typically this is handled by a
    /// SubstanceDefinition identifier.
    pub identifier: super::identifier::Identifier,
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
    /// Todo - this is intended to connect to a repeating full modification structure,
    /// also used by Protein and Nucleic Acid . String is just a placeholder.
    pub modification: super::string::String,
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
    /// Todo.
    pub monomer_set: Vec<super::substance_polymer::SubstancePolymerMonomerSet>,
    /// Specifies and quantifies the repeated units and their configuration.
    pub repeat: Vec<super::substance_polymer::SubstancePolymerRepeat>,
    /// This is a SubstancePolymer resource
    pub resource_type: String,
    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need
    /// not encode all the structured data, but is required to contain sufficient detail
    /// to make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub text: super::narrative::Narrative,
}

/// Properties of a substance specific to it being a polymer.
#[derive(Debug, Clone, PartialEq)]
pub struct SubstancePolymerDegreeOfPolymerisation {
    /// An average amount of polymerisation.
    pub average: super::integer::Integer,
    /// A high expected limit of the amount.
    pub high: super::integer::Integer,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// A low expected limit of the amount.
    pub low: super::integer::Integer,
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
    /// The type of the degree of polymerisation shall be described, e.g. SRU/Polymer
    /// Ratio.
    pub r#type: super::codeable_concept::CodeableConcept,
}

/// Properties of a substance specific to it being a polymer.
#[derive(Debug, Clone, PartialEq)]
pub struct SubstancePolymerMonomerSet {
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
    /// Captures the type of ratio to the entire polymer, e.g. Monomer/Polymer ratio,
    /// SRU/Polymer Ratio.
    pub ratio_type: super::codeable_concept::CodeableConcept,
    /// The starting materials - monomer(s) used in the synthesis of the polymer.
    pub starting_material: Vec<super::substance_polymer::SubstancePolymerStartingMaterial>,
}

/// Properties of a substance specific to it being a polymer.
#[derive(Debug, Clone, PartialEq)]
pub struct SubstancePolymerRepeat {
    /// A representation of an (average) molecular formula from a polymer.
    pub average_molecular_formula: super::string::String,
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
    /// An SRU - Structural Repeat Unit.
    pub repeat_unit: Vec<super::substance_polymer::SubstancePolymerRepeatUnit>,
    /// How the quantitative amount of Structural Repeat Units is captured (e.g. Exact,
    /// Numeric, Average).
    pub repeat_unit_amount_type: super::codeable_concept::CodeableConcept,
}

/// Properties of a substance specific to it being a polymer.
#[derive(Debug, Clone, PartialEq)]
pub struct SubstancePolymerRepeatUnit {
    /// Number of repeats of this unit.
    pub amount: super::integer::Integer,
    /// Applies to homopolymer and block co-polymers where the degree of polymerisation
    /// within a block can be described.
    pub degree_of_polymerisation:
        Vec<super::substance_polymer::SubstancePolymerDegreeOfPolymerisation>,
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
    /// The orientation of the polymerisation, e.g. head-tail, head-head, random.
    pub orientation: super::codeable_concept::CodeableConcept,
    /// A graphical structure for this SRU.
    pub structural_representation:
        Vec<super::substance_polymer::SubstancePolymerStructuralRepresentation>,
    /// Structural repeat units are essential elements for defining polymers.
    pub unit: super::string::String,
}

/// Properties of a substance specific to it being a polymer.
#[derive(Debug, Clone, PartialEq)]
pub struct SubstancePolymerStartingMaterial {
    /// A percentage.
    pub amount: super::quantity::Quantity,
    /// Substance high level category, e.g. chemical substance.
    pub category: super::codeable_concept::CodeableConcept,
    /// The type of substance for this starting material.
    pub code: super::codeable_concept::CodeableConcept,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// Used to specify whether the attribute described is a defining element for the
    /// unique identification of the polymer.
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
}

/// Properties of a substance specific to it being a polymer.
#[derive(Debug, Clone, PartialEq)]
pub struct SubstancePolymerStructuralRepresentation {
    /// An attached file with the structural representation.
    pub attachment: super::attachment::Attachment,
    /// The format of the representation e.g. InChI, SMILES, MOLFILE, CDX, SDF, PDB,
    /// mmCIF.
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
    /// The structural representation as text string in a standard format e.g. InChI,
    /// SMILES, MOLFILE, CDX, SDF, PDB, mmCIF.
    pub representation: super::string::String,
    /// The type of structure (e.g. Full, Partial, Representative).
    pub r#type: super::codeable_concept::CodeableConcept,
}

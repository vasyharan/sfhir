/// A homogeneous material with a definite composition.
#[derive(Debug, Clone, PartialEq)]
pub struct Substance {
    /// A code that classifies the general type of substance.  This is used  for
    /// searching, sorting and display purposes.
    pub category: Vec<super::codeable_concept::CodeableConcept>,
    /// A code (or set of codes) that identify this substance.
    pub code: super::codeable_reference::CodeableReference,
    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, nor can they have
    /// their own independent transaction scope. This is allowed to be a Parameters
    /// resource if and only if it is referenced by a resource that provides context/
    /// meaning.
    pub contained: Vec<super::resource_list::ResourceList>,
    /// A description of the substance - its appearance, handling requirements, and
    /// other usage notes.
    pub description: super::markdown::Markdown,
    /// When the substance is no longer valid to use. For some substances, a single
    /// arbitrary date is used for expiry.
    pub expiry: super::date_time::DateTime,
    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub id: super::id::Id,
    /// Unique identifier for the substance. For an instance, an identifier associated
    /// with the package/container (usually a label affixed directly).
    pub identifier: Vec<super::identifier::Identifier>,
    /// A reference to a set of rules that were followed when the resource was
    /// constructed, and which must be understood when processing the content. Often,
    /// this is a reference to an implementation guide that defines the special rules
    /// along with other profiles etc.
    pub implicit_rules: super::uri::Uri,
    /// A substance can be composed of other substances.
    pub ingredient: Vec<super::substance::SubstanceIngredient>,
    /// A boolean to indicate if this an instance of a substance or a kind of one (a
    /// definition).
    pub instance: super::boolean::Boolean,
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
    /// The amount of the substance.
    pub quantity: super::quantity::Quantity,
    /// This is a Substance resource
    pub resource_type: String,
    /// A code to indicate if the substance is actively used.
    pub status: super::code::Code,
    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need
    /// not encode all the structured data, but is required to contain sufficient detail
    /// to make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub text: super::narrative::Narrative,
}

/// A homogeneous material with a definite composition.
#[derive(Debug, Clone, PartialEq)]
pub struct SubstanceIngredient {
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
    /// The amount of the ingredient in the substance - a concentration ratio.
    pub quantity: super::ratio::Ratio,
    /// Another substance that is a component of this substance.
    pub substance_codeable_concept: super::codeable_concept::CodeableConcept,
    /// Another substance that is a component of this substance.
    pub substance_reference: super::reference::Reference,
}
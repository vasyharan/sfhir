/// This resource reflects an instance of a biologically derived product dispense.
/// The supply or dispense of a biologically derived product from the supply
/// organization or department (e.g. hospital transfusion laboratory) to the
/// clinical team responsible for clinical application.
#[derive(Debug, Clone, PartialEq)]
pub struct BiologicallyDerivedProductDispense {
    /// The order or request that the dispense is fulfilling. This is a reference to a
    /// ServiceRequest resource.
    pub based_on: Vec<super::reference::Reference>,
    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, nor can they have
    /// their own independent transaction scope. This is allowed to be a Parameters
    /// resource if and only if it is referenced by a resource that provides context/
    /// meaning.
    pub contained: Vec<super::resource_list::ResourceList>,
    /// Link to a resource identifying the physical location that the product was
    /// dispatched to.
    pub destination: super::reference::Reference,
    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub id: super::id::Id,
    /// Unique instance identifiers assigned to a biologically derived product dispense.
    /// Note: This is a business identifier, not a resource identifier.
    pub identifier: Vec<super::identifier::Identifier>,
    /// A reference to a set of rules that were followed when the resource was
    /// constructed, and which must be understood when processing the content. Often,
    /// this is a reference to an implementation guide that defines the special rules
    /// along with other profiles etc.
    pub implicit_rules: super::uri::Uri,
    /// The base language in which the resource is written.
    pub language: super::code::Code,
    /// The physical location where the dispense was performed.
    pub location: super::reference::Reference,
    /// Indicates the type of matching associated with the dispense.
    pub match_status: super::codeable_concept::CodeableConcept,
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
    /// Additional notes.
    pub note: Vec<super::annotation::Annotation>,
    /// Indicates the relationship between the donor of the biologically derived product
    /// and the intended recipient.
    pub origin_relationship_type: super::codeable_concept::CodeableConcept,
    /// A larger event of which this particular event is a component.
    pub part_of: Vec<super::reference::Reference>,
    /// A link to a resource representing the patient that the product is dispensed for.
    pub patient: super::reference::Reference,
    /// Indicates who or what performed an action.
    pub performer: Vec<
        super::biologically_derived_product_dispense::BiologicallyDerivedProductDispensePerformer,
    >,
    /// When the product was selected/ matched.
    pub prepared_date: super::date_time::DateTime,
    /// A link to a resource identifying the biologically derived product that is being
    /// dispensed.
    pub product: super::reference::Reference,
    /// The amount of product in the dispense. Quantity will depend on the product being
    /// dispensed. Examples are: volume; cell count; concentration.
    pub quantity: super::quantity::Quantity,
    /// This is a BiologicallyDerivedProductDispense resource
    pub resource_type: String,
    /// A code specifying the state of the dispense event.
    pub status: super::code::Code,
    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need
    /// not encode all the structured data, but is required to contain sufficient detail
    /// to make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub text: super::narrative::Narrative,
    /// Specific instructions for use.
    pub usage_instruction: super::string::String,
    /// When the product was dispatched for clinical use.
    pub when_handed_over: super::date_time::DateTime,
}

/// This resource reflects an instance of a biologically derived product dispense.
/// The supply or dispense of a biologically derived product from the supply
/// organization or department (e.g. hospital transfusion laboratory) to the
/// clinical team responsible for clinical application.
#[derive(Debug, Clone, PartialEq)]
pub struct BiologicallyDerivedProductDispensePerformer {
    /// Identifies the person responsible for the action.
    pub actor: super::reference::Reference,
    /// Identifies the function of the performer during the dispense.
    pub function: super::codeable_concept::CodeableConcept,
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
}

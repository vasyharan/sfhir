/// Record of delivery of what is supplied.
#[derive(Debug, Clone, PartialEq)]
pub struct SupplyDelivery {
    /// A plan, proposal or order that is fulfilled in whole or in part by this event.
    pub based_on: Vec<super::reference::Reference>,
    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, nor can they have
    /// their own independent transaction scope. This is allowed to be a Parameters
    /// resource if and only if it is referenced by a resource that provides context/
    /// meaning.
    pub contained: Vec<super::resource_list::ResourceList>,
    /// Identification of the facility/location where the delivery was shipped to.
    pub destination: super::reference::Reference,
    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub id: super::id::Id,
    /// Identifier for the supply delivery event that is used to identify it across
    /// multiple disparate systems.
    pub identifier: Vec<super::identifier::Identifier>,
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
    /// The date or time(s) the activity occurred.
    pub occurrence_date_time: String,
    /// The date or time(s) the activity occurred.
    pub occurrence_period: super::period::Period,
    /// The date or time(s) the activity occurred.
    pub occurrence_timing: super::timing::Timing,
    /// A larger event of which this particular event is a component or step.
    pub part_of: Vec<super::reference::Reference>,
    /// A link to a resource representing the person whom the delivered item is for.
    pub patient: super::reference::Reference,
    /// Identifies the individual or organization that received the delivery.
    pub receiver: Vec<super::reference::Reference>,
    /// This is a SupplyDelivery resource
    pub resource_type: String,
    /// A code specifying the state of the dispense event.
    pub status: super::code::Code,
    /// The item that is being delivered or has been supplied.
    pub supplied_item: Vec<super::supply_delivery::SupplyDeliverySuppliedItem>,
    /// The individual or organization responsible for supplying the delivery.
    pub supplier: super::reference::Reference,
    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need
    /// not encode all the structured data, but is required to contain sufficient detail
    /// to make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub text: super::narrative::Narrative,
    /// Indicates the type of supply being provided.  Examples include: Medication,
    /// Device, Biologically Derived Product.
    pub r#type: super::codeable_concept::CodeableConcept,
}

/// Record of delivery of what is supplied.
#[derive(Debug, Clone, PartialEq)]
pub struct SupplyDeliverySuppliedItem {
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// Identifies the medication, substance, device or biologically derived product
    /// being supplied. This is either a link to a resource representing the details of
    /// the item or a code that identifies the item from a known list.
    pub item_codeable_concept: super::codeable_concept::CodeableConcept,
    /// Identifies the medication, substance, device or biologically derived product
    /// being supplied. This is either a link to a resource representing the details of
    /// the item or a code that identifies the item from a known list.
    pub item_reference: super::reference::Reference,
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
    /// The amount of the item that has been supplied.  Unit of measure may be included.
    pub quantity: super::quantity::Quantity,
}

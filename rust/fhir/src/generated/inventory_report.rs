/// A report of inventory or stock items.
#[derive(Debug, Clone, PartialEq)]
pub struct InventoryReport {
    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, nor can they have
    /// their own independent transaction scope. This is allowed to be a Parameters
    /// resource if and only if it is referenced by a resource that provides context/
    /// meaning.
    pub contained: Vec<super::resource_list::ResourceList>,
    /// Whether the report is about the current inventory count (snapshot) or a
    /// differential change in inventory (change).
    pub count_type: super::code::Code,
    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub id: super::id::Id,
    /// Business identifier for the InventoryReport.
    pub identifier: Vec<super::identifier::Identifier>,
    /// A reference to a set of rules that were followed when the resource was
    /// constructed, and which must be understood when processing the content. Often,
    /// this is a reference to an implementation guide that defines the special rules
    /// along with other profiles etc.
    pub implicit_rules: super::uri::Uri,
    /// An inventory listing section (grouped by any of the attributes).
    pub inventory_listing: Vec<super::inventory_report::InventoryReportInventoryListing>,
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
    /// A note associated with the InventoryReport.
    pub note: Vec<super::annotation::Annotation>,
    /// What type of operation is being performed - addition or subtraction.
    pub operation_type: super::codeable_concept::CodeableConcept,
    /// The reason for this count - regular count, ad-hoc count, new arrivals, etc.
    pub operation_type_reason: super::codeable_concept::CodeableConcept,
    /// When the report has been submitted.
    pub reported_date_time: super::date_time::DateTime,
    /// Who submits the report.
    pub reporter: super::reference::Reference,
    /// The period the report refers to.
    pub reporting_period: super::period::Period,
    /// This is a InventoryReport resource
    pub resource_type: String,
    /// The status of the inventory check or notification - whether this is draft (e.g.
    /// the report is still pending some updates) or active.
    pub status: super::code::Code,
    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need
    /// not encode all the structured data, but is required to contain sufficient detail
    /// to make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub text: super::narrative::Narrative,
}

/// A report of inventory or stock items.
#[derive(Debug, Clone, PartialEq)]
pub struct InventoryReportInventoryListing {
    /// The date and time when the items were counted.
    pub counting_date_time: super::date_time::DateTime,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// The item or items in this listing.
    pub item: Vec<super::inventory_report::InventoryReportItem>,
    /// The status of the items.
    pub item_status: super::codeable_concept::CodeableConcept,
    /// Location of the inventory items.
    pub location: super::reference::Reference,
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

/// A report of inventory or stock items.
#[derive(Debug, Clone, PartialEq)]
pub struct InventoryReportItem {
    /// The inventory category or classification of the items being reported. This is
    /// meant not for defining the product, but for inventory categories e.g. 'pending
    /// recount' or 'damaged'.
    pub category: super::codeable_concept::CodeableConcept,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// The code or reference to the item type.
    pub item: super::codeable_reference::CodeableReference,
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
    /// The quantity of the item or items being reported.
    pub quantity: super::quantity::Quantity,
}

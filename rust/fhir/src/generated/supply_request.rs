/// A record of a request to deliver a medication, substance or device used in
/// the healthcare setting to a particular destination for a particular person or
/// organization.
#[derive(Debug, Clone, PartialEq)]
pub struct SupplyRequest {
    /// When the request was made.
    pub authored_on: super::date_time::DateTime,
    /// Plan/proposal/order fulfilled by this request.
    pub based_on: Vec<super::reference::Reference>,
    /// Category of supply, e.g.  central, non-stock, etc. This is used to support work
    /// flows associated with the supply process.
    pub category: super::codeable_concept::CodeableConcept,
    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, nor can they have
    /// their own independent transaction scope. This is allowed to be a Parameters
    /// resource if and only if it is referenced by a resource that provides context/
    /// meaning.
    pub contained: Vec<super::resource_list::ResourceList>,
    /// The patient to whom the supply will be given or for whom they will be used.
    pub deliver_for: super::reference::Reference,
    /// Where the supply is expected to come from.
    pub deliver_from: super::reference::Reference,
    /// Where the supply is destined to go.
    pub deliver_to: super::reference::Reference,
    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub id: super::id::Id,
    /// Business identifiers assigned to this SupplyRequest by the author and/or other
    /// systems. These identifiers remain constant as the resource is updated and
    /// propagates from server to server.
    pub identifier: Vec<super::identifier::Identifier>,
    /// A reference to a set of rules that were followed when the resource was
    /// constructed, and which must be understood when processing the content. Often,
    /// this is a reference to an implementation guide that defines the special rules
    /// along with other profiles etc.
    pub implicit_rules: super::uri::Uri,
    /// The item that is requested to be supplied. This is either a link to a resource
    /// representing the details of the item or a code that identifies the item from a
    /// known list.
    pub item: super::codeable_reference::CodeableReference,
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
    /// When the request should be fulfilled.
    pub occurrence_date_time: String,
    /// When the request should be fulfilled.
    pub occurrence_period: super::period::Period,
    /// When the request should be fulfilled.
    pub occurrence_timing: super::timing::Timing,
    /// Specific parameters for the ordered item.  For example, the size of the
    /// indicated item.
    pub parameter: Vec<super::supply_request::SupplyRequestParameter>,
    /// Indicates how quickly this SupplyRequest should be addressed with respect to
    /// other requests.
    pub priority: super::code::Code,
    /// The amount that is being ordered of the indicated item.
    pub quantity: super::quantity::Quantity,
    /// The reason why the supply item was requested.
    pub reason: Vec<super::codeable_reference::CodeableReference>,
    /// The device, practitioner, etc. who initiated the request.
    pub requester: super::reference::Reference,
    /// This is a SupplyRequest resource
    pub resource_type: String,
    /// Status of the supply request.
    pub status: super::code::Code,
    /// Who is intended to fulfill the request.
    pub supplier: Vec<super::reference::Reference>,
    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need
    /// not encode all the structured data, but is required to contain sufficient detail
    /// to make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub text: super::narrative::Narrative,
}

/// A record of a request to deliver a medication, substance or device used in
/// the healthcare setting to a particular destination for a particular person or
/// organization.
#[derive(Debug, Clone, PartialEq)]
pub struct SupplyRequestParameter {
    /// A code or string that identifies the device detail being asserted.
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
    /// The value of the device detail.
    pub value_boolean: bool,
    /// The value of the device detail.
    pub value_codeable_concept: super::codeable_concept::CodeableConcept,
    /// The value of the device detail.
    pub value_quantity: super::quantity::Quantity,
    /// The value of the device detail.
    pub value_range: super::range::Range,
}

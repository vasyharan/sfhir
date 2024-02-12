/// The resource ChargeItem describes the provision of healthcare provider products
/// for a certain patient, therefore referring not only to the product, but
/// containing in addition details of the provision, like date, time, amounts and
/// participating organizations and persons. Main Usage of the ChargeItem is to
/// enable the billing process and internal cost allocation.
#[derive(Debug, Clone, PartialEq)]
pub struct ChargeItem {
    /// Account into which this ChargeItems belongs.
    pub account: Vec<super::reference::Reference>,
    /// The anatomical location where the related service has been applied.
    pub bodysite: Vec<super::codeable_concept::CodeableConcept>,
    /// A code that identifies the charge, like a billing code.
    pub code: super::codeable_concept::CodeableConcept,
    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, nor can they have
    /// their own independent transaction scope. This is allowed to be a Parameters
    /// resource if and only if it is referenced by a resource that provides context/
    /// meaning.
    pub contained: Vec<super::resource_list::ResourceList>,
    /// The financial cost center permits the tracking of charge attribution.
    pub cost_center: super::reference::Reference,
    /// References the source of pricing information, rules of application for the code
    /// this ChargeItem uses.
    pub definition_canonical: Vec<super::canonical::Canonical>,
    /// References the (external) source of pricing information, rules of application
    /// for the code this ChargeItem uses.
    pub definition_uri: Vec<super::uri::Uri>,
    /// This ChargeItem has the details of how the associated Encounter should be billed
    /// or otherwise be handled by finance systems.
    pub encounter: super::reference::Reference,
    /// Date the charge item was entered.
    pub entered_date: super::date_time::DateTime,
    /// The device, practitioner, etc. who entered the charge item.
    pub enterer: super::reference::Reference,
    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub id: super::id::Id,
    /// Identifiers assigned to this event performer or other systems.
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
    /// Comments made about the event by the performer, subject or other participants.
    pub note: Vec<super::annotation::Annotation>,
    /// Date/time(s) or duration when the charged service was applied.
    pub occurrence_date_time: String,
    /// Date/time(s) or duration when the charged service was applied.
    pub occurrence_period: super::period::Period,
    /// Date/time(s) or duration when the charged service was applied.
    pub occurrence_timing: super::timing::Timing,
    /// If the list price or the rule-based factor associated with the code is
    /// overridden, this attribute can capture a text to indicate the  reason for this
    /// action.
    pub override_reason: super::codeable_concept::CodeableConcept,
    /// ChargeItems can be grouped to larger ChargeItems covering the whole set.
    pub part_of: Vec<super::reference::Reference>,
    /// Indicates who or what performed or participated in the charged service.
    pub performer: Vec<super::charge_item::ChargeItemPerformer>,
    /// The organization performing the service.
    pub performing_organization: super::reference::Reference,
    /// Identifies the device, food, drug or other product being charged either by type
    /// code or reference to an instance.
    pub product: Vec<super::codeable_reference::CodeableReference>,
    /// Quantity of which the charge item has been serviced.
    pub quantity: super::quantity::Quantity,
    /// Describes why the event occurred in coded or textual form.
    pub reason: Vec<super::codeable_concept::CodeableConcept>,
    /// The organization requesting the service.
    pub requesting_organization: super::reference::Reference,
    /// This is a ChargeItem resource
    pub resource_type: String,
    /// Indicated the rendered service that caused this charge.
    pub service: Vec<super::codeable_reference::CodeableReference>,
    /// The current state of the ChargeItem.
    pub status: super::code::Code,
    /// The individual or set of individuals the action is being or was performed on.
    pub subject: super::reference::Reference,
    /// Further information supporting this charge.
    pub supporting_information: Vec<super::reference::Reference>,
    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need
    /// not encode all the structured data, but is required to contain sufficient detail
    /// to make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub text: super::narrative::Narrative,
    /// The total price for the chargable item, accounting for the quantity.
    pub total_price_component: super::monetary_component::MonetaryComponent,
    /// The unit price of the chargable item.
    pub unit_price_component: super::monetary_component::MonetaryComponent,
}

/// The resource ChargeItem describes the provision of healthcare provider products
/// for a certain patient, therefore referring not only to the product, but
/// containing in addition details of the provision, like date, time, amounts and
/// participating organizations and persons. Main Usage of the ChargeItem is to
/// enable the billing process and internal cost allocation.
#[derive(Debug, Clone, PartialEq)]
pub struct ChargeItemPerformer {
    /// The device, practitioner, etc. who performed or participated in the service.
    pub actor: super::reference::Reference,
    /// Describes the type of performance or participation(e.g. primary surgeon,
    /// anesthesiologiest, etc.).
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

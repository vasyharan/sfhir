/// Invoice containing collected ChargeItems from an Account with calculated
/// individual and total price for Billing purpose.
#[derive(Debug, Clone, PartialEq)]
pub struct Invoice {
    /// Account which is supposed to be balanced with this Invoice.
    pub account: super::reference::Reference,
    /// In case of Invoice cancellation a reason must be given (entered in error,
    /// superseded by corrected invoice etc.).
    pub cancelled_reason: super::string::String,
    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, nor can they have
    /// their own independent transaction scope. This is allowed to be a Parameters
    /// resource if and only if it is referenced by a resource that provides context/
    /// meaning.
    pub contained: Vec<super::resource_list::ResourceList>,
    /// Date/time(s) of when this Invoice was posted.
    pub creation: super::date_time::DateTime,
    /// Depricared by the element below.
    pub date: super::date_time::DateTime,
    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub id: super::id::Id,
    /// Identifier of this Invoice, often used for reference in correspondence about
    /// this invoice or for tracking of payments.
    pub identifier: Vec<super::identifier::Identifier>,
    /// A reference to a set of rules that were followed when the resource was
    /// constructed, and which must be understood when processing the content. Often,
    /// this is a reference to an implementation guide that defines the special rules
    /// along with other profiles etc.
    pub implicit_rules: super::uri::Uri,
    /// The organizationissuing the Invoice.
    pub issuer: super::reference::Reference,
    /// The base language in which the resource is written.
    pub language: super::code::Code,
    /// Each line item represents one charge for goods and services rendered. Details
    /// such.ofType(date), code and amount are found in the referenced ChargeItem
    /// resource.
    pub line_item: Vec<super::invoice::InvoiceLineItem>,
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
    /// Comments made about the invoice by the issuer, subject, or other participants.
    pub note: Vec<super::annotation::Annotation>,
    /// Indicates who or what performed or participated in the charged service.
    pub participant: Vec<super::invoice::InvoiceParticipant>,
    /// Payment details such as banking details, period of payment, deductibles, methods
    /// of payment.
    pub payment_terms: super::markdown::Markdown,
    /// Date/time(s) range of services included in this invoice.
    pub period_date: String,
    /// Date/time(s) range of services included in this invoice.
    pub period_period: super::period::Period,
    /// The individual or Organization responsible for balancing of this invoice.
    pub recipient: super::reference::Reference,
    /// This is a Invoice resource
    pub resource_type: String,
    /// The current state of the Invoice.
    pub status: super::code::Code,
    /// The individual or set of individuals receiving the goods and services billed in
    /// this invoice.
    pub subject: super::reference::Reference,
    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need
    /// not encode all the structured data, but is required to contain sufficient detail
    /// to make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub text: super::narrative::Narrative,
    /// Invoice total, tax included.
    pub total_gross: super::money::Money,
    /// Invoice total , taxes excluded.
    pub total_net: super::money::Money,
    /// The total amount for the Invoice may be calculated as the sum of the line items
    /// with surcharges/deductions that apply in certain conditions.  The priceComponent
    /// element can be used to offer transparency to the recipient of the Invoice of how
    /// the total price was calculated.
    pub total_price_component: Vec<super::monetary_component::MonetaryComponent>,
    /// Type of Invoice depending on domain, realm an usage (e.g. internal/external,
    /// dental, preliminary).
    pub r#type: super::codeable_concept::CodeableConcept,
}

/// Invoice containing collected ChargeItems from an Account with calculated
/// individual and total price for Billing purpose.
#[derive(Debug, Clone, PartialEq)]
pub struct InvoiceLineItem {
    /// The ChargeItem contains information such as the billing code, date, amount etc.
    /// If no further details are required for the lineItem, inline billing codes can be
    /// added using the CodeableConcept data type instead of the Reference.
    pub charge_item_codeable_concept: super::codeable_concept::CodeableConcept,
    /// The ChargeItem contains information such as the billing code, date, amount etc.
    /// If no further details are required for the lineItem, inline billing codes can be
    /// added using the CodeableConcept data type instead of the Reference.
    pub charge_item_reference: super::reference::Reference,
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
    /// The price for a ChargeItem may be calculated as a base price with surcharges/
    /// deductions that apply in certain conditions. A ChargeItemDefinition resource
    /// that defines the prices, factors and conditions that apply to a billing code
    /// is currently under development. The priceComponent element can be used to offer
    /// transparency to the recipient of the Invoice as to how the prices have been
    /// calculated.
    pub price_component: Vec<super::monetary_component::MonetaryComponent>,
    /// Sequence in which the items appear on the invoice.
    pub sequence: super::positive_int::PositiveInt,
    /// Date/time(s) range when this service was delivered or completed.
    pub serviced_date: String,
    /// Date/time(s) range when this service was delivered or completed.
    pub serviced_period: super::period::Period,
}

/// Invoice containing collected ChargeItems from an Account with calculated
/// individual and total price for Billing purpose.
#[derive(Debug, Clone, PartialEq)]
pub struct InvoiceParticipant {
    /// The device, practitioner, etc. who performed or participated in the service.
    pub actor: super::reference::Reference,
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
    /// Describes the type of involvement (e.g. transcriptionist, creator etc.). If the
    /// invoice has been created automatically, the Participant may be a billing engine
    /// or another kind of device.
    pub role: super::codeable_concept::CodeableConcept,
}

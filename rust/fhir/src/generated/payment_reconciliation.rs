/// This resource provides the details including amount of a payment and allocates
/// the payment items being paid.
#[derive(Debug, Clone, PartialEq)]
pub struct PaymentReconciliation {
    /// A portion of the account number, often the last 4 digits, used for verification
    /// not charging purposes.
    pub account_number: super::string::String,
    /// Distribution of the payment amount for a previously acknowledged payable.
    pub allocation: Vec<super::payment_reconciliation::PaymentReconciliationAllocation>,
    /// Total payment amount as indicated on the financial instrument.
    pub amount: super::money::Money,
    /// An alphanumeric issued by the processor to confirm the successful issuance of
    /// payment.
    pub authorization: super::string::String,
    /// The card brand such as debit, Visa, Amex etc. used if a card is the method of
    /// payment.
    pub card_brand: super::string::String,
    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, nor can they have
    /// their own independent transaction scope. This is allowed to be a Parameters
    /// resource if and only if it is referenced by a resource that provides context/
    /// meaning.
    pub contained: Vec<super::resource_list::ResourceList>,
    /// The date when the resource was created.
    pub created: super::date_time::DateTime,
    /// The date of payment as indicated on the financial instrument.
    pub date: super::date::Date,
    /// A human readable description of the status of the request for the
    /// reconciliation.
    pub disposition: super::string::String,
    /// Payment enterer if not the actual payment issuer.
    pub enterer: super::reference::Reference,
    /// The year and month (YYYY-MM) when the instrument, typically card, expires.
    pub expiration_date: super::date::Date,
    /// A code for the form to be used for printing the content.
    pub form_code: super::codeable_concept::CodeableConcept,
    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub id: super::id::Id,
    /// A unique identifier assigned to this payment reconciliation.
    pub identifier: Vec<super::identifier::Identifier>,
    /// A reference to a set of rules that were followed when the resource was
    /// constructed, and which must be understood when processing the content. Often,
    /// this is a reference to an implementation guide that defines the special rules
    /// along with other profiles etc.
    pub implicit_rules: super::uri::Uri,
    /// The type of the source such as patient or insurance.
    pub issuer_type: super::codeable_concept::CodeableConcept,
    /// The workflow or activity which gave rise to or during which the payment ocurred
    /// such as a kiosk, deposit on account, periodic payment etc.
    pub kind: super::codeable_concept::CodeableConcept,
    /// The base language in which the resource is written.
    pub language: super::code::Code,
    /// The location of the site or device for electronic transfers or physical location
    /// for cash payments.
    pub location: super::reference::Reference,
    /// The metadata about the resource. This is content that is maintained by the
    /// infrastructure. Changes to the content might not always be associated with
    /// version changes to the resource.
    pub meta: super::meta::Meta,
    /// The means of payment such as check, card cash, or electronic funds transfer.
    pub method: super::codeable_concept::CodeableConcept,
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
    /// The outcome of a request for a reconciliation.
    pub outcome: super::code::Code,
    /// Issuer's unique identifier for the payment instrument.
    pub payment_identifier: super::identifier::Identifier,
    /// The party who generated the payment.
    pub payment_issuer: super::reference::Reference,
    /// The period of time for which payments have been gathered into this bulk payment
    /// for settlement.
    pub period: super::period::Period,
    /// A note that describes or explains the processing in a human readable form.
    pub process_note: Vec<super::payment_reconciliation::PaymentReconciliationProcessNote>,
    /// The name of the card processor, etf processor, bank for checks.
    pub processor: super::string::String,
    /// The check number, eft reference, car processor reference.
    pub reference_number: super::string::String,
    /// Original request resource reference.
    pub request: super::reference::Reference,
    /// The practitioner who is responsible for the services rendered to the patient.
    pub requestor: super::reference::Reference,
    /// This is a PaymentReconciliation resource
    pub resource_type: String,
    /// The amount returned by the receiver which is excess to the amount payable, often
    /// referred to as 'change'.
    pub returned_amount: super::money::Money,
    /// The status of the resource instance.
    pub status: super::code::Code,
    /// The amount offered by the issuer, typically applies to cash when the issuer
    /// provides an amount in bank note denominations equal to or excess of the amount
    /// actually being paid.
    pub tendered_amount: super::money::Money,
    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need
    /// not encode all the structured data, but is required to contain sufficient detail
    /// to make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub text: super::narrative::Narrative,
    /// Code to indicate the nature of the payment such as payment, adjustment.
    pub r#type: super::codeable_concept::CodeableConcept,
}

/// This resource provides the details including amount of a payment and allocates
/// the payment items being paid.
#[derive(Debug, Clone, PartialEq)]
pub struct PaymentReconciliationAllocation {
    /// The Account to which this payment applies, may be completed by the receiver,
    /// used for search.
    pub account: super::reference::Reference,
    /// The monetary amount allocated from the total payment to the payable.
    pub amount: super::money::Money,
    /// The date from the response resource containing a commitment to pay.
    pub date: super::date::Date,
    /// The Encounter to which this payment applies, may be completed by the receiver,
    /// used for search.
    pub encounter: super::reference::Reference,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// Unique identifier for the current payment item for the referenced payable.
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
    /// The party which is receiving the payment.
    pub payee: super::reference::Reference,
    /// Unique identifier for the prior payment item for the referenced payable.
    pub predecessor: super::identifier::Identifier,
    /// A resource, such as a ClaimResponse, which contains a commitment to payment.
    pub response: super::reference::Reference,
    /// A reference to the individual who is responsible for inquiries regarding the
    /// response and its payment.
    pub responsible: super::reference::Reference,
    /// The party which submitted the claim or financial transaction.
    pub submitter: super::reference::Reference,
    /// Specific resource to which the payment/adjustment/advance applies.
    pub target: super::reference::Reference,
    ///  Identifies the claim line item, encounter or other sub-element being paid. Note
    /// payment may be partial, that is not match the then outstanding balance or amount
    /// incurred.
    pub target_item_identifier: super::identifier::Identifier,
    ///  Identifies the claim line item, encounter or other sub-element being paid. Note
    /// payment may be partial, that is not match the then outstanding balance or amount
    /// incurred.
    pub target_item_positive_int: u64,
    ///  Identifies the claim line item, encounter or other sub-element being paid. Note
    /// payment may be partial, that is not match the then outstanding balance or amount
    /// incurred.
    pub target_item_string: String,
    /// Code to indicate the nature of the payment.
    pub r#type: super::codeable_concept::CodeableConcept,
}

/// This resource provides the details including amount of a payment and allocates
/// the payment items being paid.
#[derive(Debug, Clone, PartialEq)]
pub struct PaymentReconciliationProcessNote {
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
    /// The explanation or description associated with the processing.
    pub text: super::string::String,
    /// The business purpose of the note text.
    pub r#type: super::code::Code,
}

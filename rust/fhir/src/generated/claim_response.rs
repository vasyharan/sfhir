/// This resource provides the adjudication details from the processing of a Claim
/// resource.
#[derive(Debug, Clone, PartialEq)]
pub struct ClaimResponse {
    /// The first-tier service adjudications for payor added product or service lines.
    pub add_item: Vec<super::claim_response::ClaimResponseAddItem>,
    /// The adjudication results which are presented at the header level rather than at
    /// the line-item or add-item levels.
    pub adjudication: Vec<super::claim_response::ClaimResponseAdjudication>,
    /// Request for additional supporting or authorizing information.
    pub communication_request: Vec<super::reference::Reference>,
    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, nor can they have
    /// their own independent transaction scope. This is allowed to be a Parameters
    /// resource if and only if it is referenced by a resource that provides context/
    /// meaning.
    pub contained: Vec<super::resource_list::ResourceList>,
    /// The date this resource was created.
    pub created: super::date_time::DateTime,
    /// The result of the claim, predetermination, or preauthorization adjudication.
    pub decision: super::codeable_concept::CodeableConcept,
    /// A package billing code or bundle code used to group products and services
    /// to a particular health condition (such as heart attack) which is based on a
    /// predetermined grouping code system.
    pub diagnosis_related_group: super::codeable_concept::CodeableConcept,
    /// A human readable description of the status of the adjudication.
    pub disposition: super::string::String,
    /// Healthcare encounters related to this claim.
    pub encounter: Vec<super::reference::Reference>,
    /// Errors encountered during the processing of the adjudication.
    pub error: Vec<super::claim_response::ClaimResponseError>,
    /// Information code for an event with a corresponding date or period.
    pub event: Vec<super::claim_response::ClaimResponseEvent>,
    /// The actual form, by reference or inclusion, for printing the content or an EOB.
    pub form: super::attachment::Attachment,
    /// A code for the form to be used for printing the content.
    pub form_code: super::codeable_concept::CodeableConcept,
    /// A code, used only on a response to a preauthorization, to indicate whether the
    /// benefits payable have been reserved and for whom.
    pub funds_reserve: super::codeable_concept::CodeableConcept,
    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub id: super::id::Id,
    /// A unique identifier assigned to this claim response.
    pub identifier: Vec<super::identifier::Identifier>,
    /// A reference to a set of rules that were followed when the resource was
    /// constructed, and which must be understood when processing the content. Often,
    /// this is a reference to an implementation guide that defines the special rules
    /// along with other profiles etc.
    pub implicit_rules: super::uri::Uri,
    /// Financial instruments for reimbursement for the health care products and
    /// services specified on the claim.
    pub insurance: Vec<super::claim_response::ClaimResponseInsurance>,
    /// The party responsible for authorization, adjudication and reimbursement.
    pub insurer: super::reference::Reference,
    /// A claim line. Either a simple (a product or service) or a 'group' of details
    /// which can also be a simple items or groups of sub-details.
    pub item: Vec<super::claim_response::ClaimResponseItem>,
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
    /// The outcome of the claim, predetermination, or preauthorization processing.
    pub outcome: super::code::Code,
    /// The party to whom the professional services and/or products have been supplied
    /// or are being considered and for whom actual for facast reimbursement is sought.
    pub patient: super::reference::Reference,
    /// Type of Party to be reimbursed: subscriber, provider, other.
    pub payee_type: super::codeable_concept::CodeableConcept,
    /// Payment details for the adjudication of the claim.
    pub payment: super::claim_response::ClaimResponsePayment,
    /// The time frame during which this authorization is effective.
    pub pre_auth_period: super::period::Period,
    /// Reference from the Insurer which is used in later communications which refers to
    /// this adjudication.
    pub pre_auth_ref: super::string::String,
    /// A note that describes or explains adjudication results in a human readable form.
    pub process_note: Vec<super::claim_response::ClaimResponseProcessNote>,
    /// Original request resource reference.
    pub request: super::reference::Reference,
    /// The provider which is responsible for the claim, predetermination or
    /// preauthorization.
    pub requestor: super::reference::Reference,
    /// This is a ClaimResponse resource
    pub resource_type: String,
    /// The status of the resource instance.
    pub status: super::code::Code,
    /// A finer grained suite of claim type codes which may convey additional
    /// information such as Inpatient vs Outpatient and/or a specialty service.
    pub sub_type: super::codeable_concept::CodeableConcept,
    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need
    /// not encode all the structured data, but is required to contain sufficient detail
    /// to make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub text: super::narrative::Narrative,
    /// Categorized monetary totals for the adjudication.
    pub total: Vec<super::claim_response::ClaimResponseTotal>,
    /// Trace number for tracking purposes. May be defined at the jurisdiction level or
    /// between trading partners.
    pub trace_number: Vec<super::identifier::Identifier>,
    /// A finer grained suite of claim type codes which may convey additional
    /// information such as Inpatient vs Outpatient and/or a specialty service.
    pub r#type: super::codeable_concept::CodeableConcept,
    /// A code to indicate whether the nature of the request is: Claim - A request
    /// to an Insurer to adjudicate the supplied charges for health care goods and
    /// services under the identified policy and to pay the determined Benefit amount,
    /// if any; Preauthorization - A request to an Insurer to adjudicate the supplied
    /// proposed future charges for health care goods and services under the identified
    /// policy and to approve the services and provide the expected benefit amounts and
    /// potentially to reserve funds to pay the benefits when Claims for the indicated
    /// services are later submitted; or, Pre-determination - A request to an Insurer
    /// to adjudicate the supplied 'what if' charges for health care goods and services
    /// under the identified policy and report back what the Benefit payable would be
    /// had the services actually been provided.
    pub r#use: super::code::Code,
}

/// This resource provides the adjudication details from the processing of a Claim
/// resource.
#[derive(Debug, Clone, PartialEq)]
pub struct ClaimResponseAddItem {
    /// The adjudication results.
    pub adjudication: Vec<super::claim_response::ClaimResponseAdjudication>,
    /// Physical location where the service is performed or applies.
    pub body_site: Vec<super::claim_response::ClaimResponseBodySite>,
    /// The second-tier service adjudications for payor added services.
    pub detail: Vec<super::claim_response::ClaimResponseDetail1>,
    /// The sequence number of the details within the claim item which this line is
    /// intended to replace.
    pub detail_sequence: Vec<super::positive_int::PositiveInt>,
    /// A real number that represents a multiplier used in determining the overall value
    /// of services delivered and/or goods received. The concept of a Factor allows for
    /// a discount or surcharge multiplier to be applied to a monetary amount.
    pub factor: super::decimal::Decimal,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// Claim items which this service line is intended to replace.
    pub item_sequence: Vec<super::positive_int::PositiveInt>,
    /// Where the product or service was provided.
    pub location_address: super::address::Address,
    /// Where the product or service was provided.
    pub location_codeable_concept: super::codeable_concept::CodeableConcept,
    /// Where the product or service was provided.
    pub location_reference: super::reference::Reference,
    /// Item typification or modifiers codes to convey additional context for the
    /// product or service.
    pub modifier: Vec<super::codeable_concept::CodeableConcept>,
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
    /// The total amount claimed for the group (if a grouper) or the addItem. Net = unit
    /// price * quantity * factor.
    pub net: super::money::Money,
    /// The numbers associated with notes below which apply to the adjudication of this
    /// item.
    pub note_number: Vec<super::positive_int::PositiveInt>,
    /// When the value is a group code then this item collects a set of related item
    /// details, otherwise this contains the product, service, drug or other billing
    /// code for the item. This element may be the start of a range of .productOrService
    /// codes used in conjunction with .productOrServiceEnd or it may be a solo element
    /// where .productOrServiceEnd is not used.
    pub product_or_service: super::codeable_concept::CodeableConcept,
    /// This contains the end of a range of product, service, drug or other billing
    /// codes for the item. This element is not used when the .productOrService is a
    /// group code. This value may only be present when a .productOfService code has
    /// been provided to convey the start of the range. Typically this value may be used
    /// only with preauthorizations and not with claims.
    pub product_or_service_end: super::codeable_concept::CodeableConcept,
    /// Identifies the program under which this may be recovered.
    pub program_code: Vec<super::codeable_concept::CodeableConcept>,
    /// The providers who are authorized for the services rendered to the patient.
    pub provider: Vec<super::reference::Reference>,
    /// The number of repetitions of a service or product.
    pub quantity: super::quantity::Quantity,
    /// Request or Referral for Goods or Service to be rendered.
    pub request: Vec<super::reference::Reference>,
    /// The type of revenue or cost center providing the product and/or service.
    pub revenue: super::codeable_concept::CodeableConcept,
    /// The high-level results of the adjudication if adjudication has been performed.
    pub review_outcome: super::claim_response::ClaimResponseReviewOutcome,
    /// The date or dates when the service or product was supplied, performed or
    /// completed.
    pub serviced_date: String,
    /// The date or dates when the service or product was supplied, performed or
    /// completed.
    pub serviced_period: super::period::Period,
    /// The sequence number of the sub-details within the details within the claim item
    /// which this line is intended to replace.
    pub subdetail_sequence: Vec<super::positive_int::PositiveInt>,
    /// The total of taxes applicable for this product or service.
    pub tax: super::money::Money,
    /// Trace number for tracking purposes. May be defined at the jurisdiction level or
    /// between trading partners.
    pub trace_number: Vec<super::identifier::Identifier>,
    /// If the item is not a group then this is the fee for the product or service,
    /// otherwise this is the total of the fees for the details of the group.
    pub unit_price: super::money::Money,
}

/// This resource provides the adjudication details from the processing of a Claim
/// resource.
#[derive(Debug, Clone, PartialEq)]
pub struct ClaimResponseAdjudication {
    /// Monetary amount associated with the category.
    pub amount: super::money::Money,
    /// A code to indicate the information type of this adjudication record. Information
    /// types may include the value submitted, maximum values or percentages allowed
    /// or payable under the plan, amounts that: the patient is responsible for in
    /// aggregate or pertaining to this item; amounts paid by other coverages; and, the
    /// benefit payable for this item.
    pub category: super::codeable_concept::CodeableConcept,
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
    /// A non-monetary value associated with the category. Mutually exclusive to the
    /// amount element above.
    pub quantity: super::quantity::Quantity,
    /// A code supporting the understanding of the adjudication result and explaining
    /// variance from expected amount.
    pub reason: super::codeable_concept::CodeableConcept,
}

/// This resource provides the adjudication details from the processing of a Claim
/// resource.
#[derive(Debug, Clone, PartialEq)]
pub struct ClaimResponseBodySite {
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
    /// Physical service site on the patient (limb, tooth, etc.).
    pub site: Vec<super::codeable_reference::CodeableReference>,
    /// A region or surface of the bodySite, e.g. limb region or tooth surface(s).
    pub sub_site: Vec<super::codeable_concept::CodeableConcept>,
}

/// This resource provides the adjudication details from the processing of a Claim
/// resource.
#[derive(Debug, Clone, PartialEq)]
pub struct ClaimResponseDetail {
    /// The adjudication results.
    pub adjudication: Vec<super::claim_response::ClaimResponseAdjudication>,
    /// A number to uniquely reference the claim detail entry.
    pub detail_sequence: super::positive_int::PositiveInt,
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
    /// The numbers associated with notes below which apply to the adjudication of this
    /// item.
    pub note_number: Vec<super::positive_int::PositiveInt>,
    /// The high-level results of the adjudication if adjudication has been performed.
    pub review_outcome: super::claim_response::ClaimResponseReviewOutcome,
    /// A sub-detail adjudication of a simple product or service.
    pub sub_detail: Vec<super::claim_response::ClaimResponseSubDetail>,
    /// Trace number for tracking purposes. May be defined at the jurisdiction level or
    /// between trading partners.
    pub trace_number: Vec<super::identifier::Identifier>,
}

/// This resource provides the adjudication details from the processing of a Claim
/// resource.
#[derive(Debug, Clone, PartialEq)]
pub struct ClaimResponseDetail1 {
    /// The adjudication results.
    pub adjudication: Vec<super::claim_response::ClaimResponseAdjudication>,
    /// A real number that represents a multiplier used in determining the overall value
    /// of services delivered and/or goods received. The concept of a Factor allows for
    /// a discount or surcharge multiplier to be applied to a monetary amount.
    pub factor: super::decimal::Decimal,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// Item typification or modifiers codes to convey additional context for the
    /// product or service.
    pub modifier: Vec<super::codeable_concept::CodeableConcept>,
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
    /// The total amount claimed for the group (if a grouper) or the addItem.detail. Net
    /// = unit price * quantity * factor.
    pub net: super::money::Money,
    /// The numbers associated with notes below which apply to the adjudication of this
    /// item.
    pub note_number: Vec<super::positive_int::PositiveInt>,
    /// When the value is a group code then this item collects a set of related item
    /// details, otherwise this contains the product, service, drug or other billing
    /// code for the item. This element may be the start of a range of .productOrService
    /// codes used in conjunction with .productOrServiceEnd or it may be a solo element
    /// where .productOrServiceEnd is not used.
    pub product_or_service: super::codeable_concept::CodeableConcept,
    /// This contains the end of a range of product, service, drug or other billing
    /// codes for the item. This element is not used when the .productOrService is a
    /// group code. This value may only be present when a .productOfService code has
    /// been provided to convey the start of the range. Typically this value may be used
    /// only with preauthorizations and not with claims.
    pub product_or_service_end: super::codeable_concept::CodeableConcept,
    /// The number of repetitions of a service or product.
    pub quantity: super::quantity::Quantity,
    /// The type of revenue or cost center providing the product and/or service.
    pub revenue: super::codeable_concept::CodeableConcept,
    /// The high-level results of the adjudication if adjudication has been performed.
    pub review_outcome: super::claim_response::ClaimResponseReviewOutcome,
    /// The third-tier service adjudications for payor added services.
    pub sub_detail: Vec<super::claim_response::ClaimResponseSubDetail1>,
    /// The total of taxes applicable for this product or service.
    pub tax: super::money::Money,
    /// Trace number for tracking purposes. May be defined at the jurisdiction level or
    /// between trading partners.
    pub trace_number: Vec<super::identifier::Identifier>,
    /// If the item is not a group then this is the fee for the product or service,
    /// otherwise this is the total of the fees for the details of the group.
    pub unit_price: super::money::Money,
}

/// This resource provides the adjudication details from the processing of a Claim
/// resource.
#[derive(Debug, Clone, PartialEq)]
pub struct ClaimResponseError {
    /// An error code, from a specified code system, which details why the claim could
    /// not be adjudicated.
    pub code: super::codeable_concept::CodeableConcept,
    /// The sequence number of the detail within the line item submitted which contains
    /// the error. This value is omitted when the error occurs outside of the item
    /// structure.
    pub detail_sequence: super::positive_int::PositiveInt,
    /// A [simple subset of FHIRPath](fhirpath.html#simple) limited to element names,
    /// repetition indicators and the default child accessor that identifies one of the
    /// elements in the resource that caused this issue to be raised.
    pub expression: Vec<super::string::String>,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// The sequence number of the line item submitted which contains the error. This
    /// value is omitted when the error occurs outside of the item structure.
    pub item_sequence: super::positive_int::PositiveInt,
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
    /// The sequence number of the sub-detail within the detail within the line item
    /// submitted which contains the error. This value is omitted when the error occurs
    /// outside of the item structure.
    pub sub_detail_sequence: super::positive_int::PositiveInt,
}

/// This resource provides the adjudication details from the processing of a Claim
/// resource.
#[derive(Debug, Clone, PartialEq)]
pub struct ClaimResponseEvent {
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
    /// A coded event such as when a service is expected or a card printed.
    pub r#type: super::codeable_concept::CodeableConcept,
    /// A date or period in the past or future indicating when the event occurred or is
    /// expectd to occur.
    pub when_date_time: String,
    /// A date or period in the past or future indicating when the event occurred or is
    /// expectd to occur.
    pub when_period: super::period::Period,
}

/// This resource provides the adjudication details from the processing of a Claim
/// resource.
#[derive(Debug, Clone, PartialEq)]
pub struct ClaimResponseInsurance {
    /// A business agreement number established between the provider and the insurer for
    /// special business processing purposes.
    pub business_arrangement: super::string::String,
    /// The result of the adjudication of the line items for the Coverage specified in
    /// this insurance.
    pub claim_response: super::reference::Reference,
    /// Reference to the insurance card level information contained in the Coverage
    /// resource. The coverage issuing insurer will use these details to locate the
    /// patient's actual coverage within the insurer's information system.
    pub coverage: super::reference::Reference,
    /// A flag to indicate that this Coverage is to be used for adjudication of this
    /// claim when set to true.
    pub focal: super::boolean::Boolean,
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
    /// A number to uniquely identify insurance entries and provide a sequence of
    /// coverages to convey coordination of benefit order.
    pub sequence: super::positive_int::PositiveInt,
}

/// This resource provides the adjudication details from the processing of a Claim
/// resource.
#[derive(Debug, Clone, PartialEq)]
pub struct ClaimResponseItem {
    /// If this item is a group then the values here are a summary of the adjudication
    /// of the detail items. If this item is a simple product or service then this is
    /// the result of the adjudication of this item.
    pub adjudication: Vec<super::claim_response::ClaimResponseAdjudication>,
    /// A claim detail. Either a simple (a product or service) or a 'group' of sub-
    /// details which are simple items.
    pub detail: Vec<super::claim_response::ClaimResponseDetail>,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// A number to uniquely reference the claim item entries.
    pub item_sequence: super::positive_int::PositiveInt,
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
    /// The numbers associated with notes below which apply to the adjudication of this
    /// item.
    pub note_number: Vec<super::positive_int::PositiveInt>,
    /// The high-level results of the adjudication if adjudication has been performed.
    pub review_outcome: super::claim_response::ClaimResponseReviewOutcome,
    /// Trace number for tracking purposes. May be defined at the jurisdiction level or
    /// between trading partners.
    pub trace_number: Vec<super::identifier::Identifier>,
}

/// This resource provides the adjudication details from the processing of a Claim
/// resource.
#[derive(Debug, Clone, PartialEq)]
pub struct ClaimResponsePayment {
    /// Total amount of all adjustments to this payment included in this transaction
    /// which are not related to this claim's adjudication.
    pub adjustment: super::money::Money,
    /// Reason for the payment adjustment.
    pub adjustment_reason: super::codeable_concept::CodeableConcept,
    /// Benefits payable less any payment adjustment.
    pub amount: super::money::Money,
    /// Estimated date the payment will be issued or the actual issue date of payment.
    pub date: super::date::Date,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// Issuer's unique identifier for the payment instrument.
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
    /// Whether this represents partial or complete payment of the benefits payable.
    pub r#type: super::codeable_concept::CodeableConcept,
}

/// This resource provides the adjudication details from the processing of a Claim
/// resource.
#[derive(Debug, Clone, PartialEq)]
pub struct ClaimResponseProcessNote {
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// A code to define the language used in the text of the note.
    pub language: super::codeable_concept::CodeableConcept,
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
    /// A number to uniquely identify a note entry.
    pub number: super::positive_int::PositiveInt,
    /// The explanation or description associated with the processing.
    pub text: super::string::String,
    /// The business purpose of the note text.
    pub r#type: super::codeable_concept::CodeableConcept,
}

/// This resource provides the adjudication details from the processing of a Claim
/// resource.
#[derive(Debug, Clone, PartialEq)]
pub struct ClaimResponseReviewOutcome {
    /// The result of the claim, predetermination, or preauthorization adjudication.
    pub decision: super::codeable_concept::CodeableConcept,
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
    /// The time frame during which this authorization is effective.
    pub pre_auth_period: super::period::Period,
    /// Reference from the Insurer which is used in later communications which refers to
    /// this adjudication.
    pub pre_auth_ref: super::string::String,
    /// The reasons for the result of the claim, predetermination, or preauthorization
    /// adjudication.
    pub reason: Vec<super::codeable_concept::CodeableConcept>,
}

/// This resource provides the adjudication details from the processing of a Claim
/// resource.
#[derive(Debug, Clone, PartialEq)]
pub struct ClaimResponseSubDetail {
    /// The adjudication results.
    pub adjudication: Vec<super::claim_response::ClaimResponseAdjudication>,
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
    /// The numbers associated with notes below which apply to the adjudication of this
    /// item.
    pub note_number: Vec<super::positive_int::PositiveInt>,
    /// The high-level results of the adjudication if adjudication has been performed.
    pub review_outcome: super::claim_response::ClaimResponseReviewOutcome,
    /// A number to uniquely reference the claim sub-detail entry.
    pub sub_detail_sequence: super::positive_int::PositiveInt,
    /// Trace number for tracking purposes. May be defined at the jurisdiction level or
    /// between trading partners.
    pub trace_number: Vec<super::identifier::Identifier>,
}

/// This resource provides the adjudication details from the processing of a Claim
/// resource.
#[derive(Debug, Clone, PartialEq)]
pub struct ClaimResponseSubDetail1 {
    /// The adjudication results.
    pub adjudication: Vec<super::claim_response::ClaimResponseAdjudication>,
    /// A real number that represents a multiplier used in determining the overall value
    /// of services delivered and/or goods received. The concept of a Factor allows for
    /// a discount or surcharge multiplier to be applied to a monetary amount.
    pub factor: super::decimal::Decimal,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// Item typification or modifiers codes to convey additional context for the
    /// product or service.
    pub modifier: Vec<super::codeable_concept::CodeableConcept>,
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
    /// The total amount claimed for the addItem.detail.subDetail. Net = unit price *
    /// quantity * factor.
    pub net: super::money::Money,
    /// The numbers associated with notes below which apply to the adjudication of this
    /// item.
    pub note_number: Vec<super::positive_int::PositiveInt>,
    /// When the value is a group code then this item collects a set of related item
    /// details, otherwise this contains the product, service, drug or other billing
    /// code for the item. This element may be the start of a range of .productOrService
    /// codes used in conjunction with .productOrServiceEnd or it may be a solo element
    /// where .productOrServiceEnd is not used.
    pub product_or_service: super::codeable_concept::CodeableConcept,
    /// This contains the end of a range of product, service, drug or other billing
    /// codes for the item. This element is not used when the .productOrService is a
    /// group code. This value may only be present when a .productOfService code has
    /// been provided to convey the start of the range. Typically this value may be used
    /// only with preauthorizations and not with claims.
    pub product_or_service_end: super::codeable_concept::CodeableConcept,
    /// The number of repetitions of a service or product.
    pub quantity: super::quantity::Quantity,
    /// The type of revenue or cost center providing the product and/or service.
    pub revenue: super::codeable_concept::CodeableConcept,
    /// The high-level results of the adjudication if adjudication has been performed.
    pub review_outcome: super::claim_response::ClaimResponseReviewOutcome,
    /// The total of taxes applicable for this product or service.
    pub tax: super::money::Money,
    /// Trace number for tracking purposes. May be defined at the jurisdiction level or
    /// between trading partners.
    pub trace_number: Vec<super::identifier::Identifier>,
    /// If the item is not a group then this is the fee for the product or service,
    /// otherwise this is the total of the fees for the details of the group.
    pub unit_price: super::money::Money,
}

/// This resource provides the adjudication details from the processing of a Claim
/// resource.
#[derive(Debug, Clone, PartialEq)]
pub struct ClaimResponseTotal {
    /// Monetary total amount associated with the category.
    pub amount: super::money::Money,
    /// A code to indicate the information type of this adjudication record. Information
    /// types may include: the value submitted, maximum values or percentages allowed or
    /// payable under the plan, amounts that the patient is responsible for in aggregate
    /// or pertaining to this item, amounts paid by other coverages, and the benefit
    /// payable for this item.
    pub category: super::codeable_concept::CodeableConcept,
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

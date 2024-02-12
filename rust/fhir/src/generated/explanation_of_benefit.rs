/// This resource provides: the claim details; adjudication details from the
/// processing of a Claim; and optionally account balance information, for informing
/// the subscriber of the benefits provided.
#[derive(Debug, Clone, PartialEq)]
pub struct ExplanationOfBenefit {
    /// Details of a accident which resulted in injuries which required the products and
    /// services listed in the claim.
    pub accident: super::explanation_of_benefit::ExplanationOfBenefitAccident,
    /// The first-tier service adjudications for payor added product or service lines.
    pub add_item: Vec<super::explanation_of_benefit::ExplanationOfBenefitAddItem>,
    /// The adjudication results which are presented at the header level rather than at
    /// the line-item or add-item levels.
    pub adjudication: Vec<super::explanation_of_benefit::ExplanationOfBenefitAdjudication>,
    /// Balance by Benefit Category.
    pub benefit_balance: Vec<super::explanation_of_benefit::ExplanationOfBenefitBenefitBalance>,
    /// The term of the benefits documented in this response.
    pub benefit_period: super::period::Period,
    /// The period for which charges are being submitted.
    pub billable_period: super::period::Period,
    /// The members of the team who provided the products and services.
    pub care_team: Vec<super::explanation_of_benefit::ExplanationOfBenefitCareTeam>,
    /// The business identifier for the instance of the adjudication request: claim
    /// predetermination or preauthorization.
    pub claim: super::reference::Reference,
    /// The business identifier for the instance of the adjudication response: claim,
    /// predetermination or preauthorization response.
    pub claim_response: super::reference::Reference,
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
    /// Information about diagnoses relevant to the claim items.
    pub diagnosis: Vec<super::explanation_of_benefit::ExplanationOfBenefitDiagnosis>,
    /// A package billing code or bundle code used to group products and services
    /// to a particular health condition (such as heart attack) which is based on a
    /// predetermined grouping code system.
    pub diagnosis_related_group: super::codeable_concept::CodeableConcept,
    /// A human readable description of the status of the adjudication.
    pub disposition: super::string::String,
    /// Healthcare encounters related to this claim.
    pub encounter: Vec<super::reference::Reference>,
    /// Individual who created the claim, predetermination or preauthorization.
    pub enterer: super::reference::Reference,
    /// Information code for an event with a corresponding date or period.
    pub event: Vec<super::explanation_of_benefit::ExplanationOfBenefitEvent>,
    /// Facility where the services were provided.
    pub facility: super::reference::Reference,
    /// The actual form, by reference or inclusion, for printing the content or an EOB.
    pub form: super::attachment::Attachment,
    /// A code for the form to be used for printing the content.
    pub form_code: super::codeable_concept::CodeableConcept,
    /// A code, used only on a response to a preauthorization, to indicate whether the
    /// benefits payable have been reserved and for whom.
    pub funds_reserve: super::codeable_concept::CodeableConcept,
    /// A code to indicate whether and for whom funds are to be reserved for future
    /// claims.
    pub funds_reserve_requested: super::codeable_concept::CodeableConcept,
    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub id: super::id::Id,
    /// A unique identifier assigned to this explanation of benefit.
    pub identifier: Vec<super::identifier::Identifier>,
    /// A reference to a set of rules that were followed when the resource was
    /// constructed, and which must be understood when processing the content. Often,
    /// this is a reference to an implementation guide that defines the special rules
    /// along with other profiles etc.
    pub implicit_rules: super::uri::Uri,
    /// Financial instruments for reimbursement for the health care products and
    /// services specified on the claim.
    pub insurance: Vec<super::explanation_of_benefit::ExplanationOfBenefitInsurance>,
    /// The party responsible for authorization, adjudication and reimbursement.
    pub insurer: super::reference::Reference,
    /// A claim line. Either a simple (a product or service) or a 'group' of details
    /// which can also be a simple items or groups of sub-details.
    pub item: Vec<super::explanation_of_benefit::ExplanationOfBenefitItem>,
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
    /// Original prescription which has been superseded by this prescription to support
    /// the dispensing of pharmacy services, medications or products.
    pub original_prescription: super::reference::Reference,
    /// The outcome of the claim, predetermination, or preauthorization processing.
    pub outcome: super::code::Code,
    /// The party to whom the professional services and/or products have been supplied
    /// or are being considered and for whom actual for forecast reimbursement is
    /// sought.
    pub patient: super::reference::Reference,
    /// The amount paid by the patient, in total at the claim claim level or
    /// specifically for the item and detail level, to the provider for goods and
    /// services.
    pub patient_paid: super::money::Money,
    /// The party to be reimbursed for cost of the products and services according to
    /// the terms of the policy.
    pub payee: super::explanation_of_benefit::ExplanationOfBenefitPayee,
    /// Payment details for the adjudication of the claim.
    pub payment: super::explanation_of_benefit::ExplanationOfBenefitPayment,
    /// Reference from the Insurer which is used in later communications which refers to
    /// this adjudication.
    pub pre_auth_ref: Vec<super::string::String>,
    /// The timeframe during which the supplied preauthorization reference may be quoted
    /// on claims to obtain the adjudication as provided.
    pub pre_auth_ref_period: Vec<super::period::Period>,
    /// This indicates the relative order of a series of EOBs related to different
    /// coverages for the same suite of services.
    pub precedence: super::positive_int::PositiveInt,
    /// Prescription is the document/authorization given to the claim author for them
    /// to provide products and services for which consideration (reimbursement) is
    /// sought. Could be a RX for medications, an 'order' for oxygen or wheelchair or
    /// physiotherapy treatments.
    pub prescription: super::reference::Reference,
    /// The provider-required urgency of processing the request. Typical values include:
    /// stat, normal deferred.
    pub priority: super::codeable_concept::CodeableConcept,
    /// Procedures performed on the patient relevant to the billing items with the
    /// claim.
    pub procedure: Vec<super::explanation_of_benefit::ExplanationOfBenefitProcedure>,
    /// A note that describes or explains adjudication results in a human readable form.
    pub process_note: Vec<super::explanation_of_benefit::ExplanationOfBenefitProcessNote>,
    /// The provider which is responsible for the claim, predetermination or
    /// preauthorization.
    pub provider: super::reference::Reference,
    /// The referral information received by the claim author, it is not to be used when
    /// the author generates a referral for a patient. A copy of that referral may be
    /// provided as supporting information. Some insurers require proof of referral to
    /// pay for services or to pay specialist rates for services.
    pub referral: super::reference::Reference,
    /// Other claims which are related to this claim such as prior submissions or claims
    /// for related services or for the same event.
    pub related: Vec<super::explanation_of_benefit::ExplanationOfBenefitRelated>,
    /// This is a ExplanationOfBenefit resource
    pub resource_type: String,
    /// The status of the resource instance.
    pub status: super::code::Code,
    /// A finer grained suite of claim type codes which may convey additional
    /// information such as Inpatient vs Outpatient and/or a specialty service.
    pub sub_type: super::codeable_concept::CodeableConcept,
    /// Additional information codes regarding exceptions, special considerations, the
    /// condition, situation, prior or concurrent issues.
    pub supporting_info: Vec<super::explanation_of_benefit::ExplanationOfBenefitSupportingInfo>,
    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need
    /// not encode all the structured data, but is required to contain sufficient detail
    /// to make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub text: super::narrative::Narrative,
    /// Categorized monetary totals for the adjudication.
    pub total: Vec<super::explanation_of_benefit::ExplanationOfBenefitTotal>,
    /// Trace number for tracking purposes. May be defined at the jurisdiction level or
    /// between trading partners.
    pub trace_number: Vec<super::identifier::Identifier>,
    /// The category of claim, e.g. oral, pharmacy, vision, institutional, professional.
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

/// This resource provides: the claim details; adjudication details from the
/// processing of a Claim; and optionally account balance information, for informing
/// the subscriber of the benefits provided.
#[derive(Debug, Clone, PartialEq)]
pub struct ExplanationOfBenefitAccident {
    /// Date of an accident event  related to the products and services contained in
    /// the claim.
    pub date: super::date::Date,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// The physical location of the accident event.
    pub location_address: super::address::Address,
    /// The physical location of the accident event.
    pub location_reference: super::reference::Reference,
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
    /// The type or context of the accident event for the purposes of selection
    /// of potential insurance coverages and determination of coordination between
    /// insurers.
    pub r#type: super::codeable_concept::CodeableConcept,
}

/// This resource provides: the claim details; adjudication details from the
/// processing of a Claim; and optionally account balance information, for informing
/// the subscriber of the benefits provided.
#[derive(Debug, Clone, PartialEq)]
pub struct ExplanationOfBenefitAddItem {
    /// The adjudication results.
    pub adjudication: Vec<super::explanation_of_benefit::ExplanationOfBenefitAdjudication>,
    /// Physical location where the service is performed or applies.
    pub body_site: Vec<super::explanation_of_benefit::ExplanationOfBenefitBodySite1>,
    /// The second-tier service adjudications for payor added services.
    pub detail: Vec<super::explanation_of_benefit::ExplanationOfBenefitDetail1>,
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
    /// The amount paid by the patient, in total at the claim claim level or
    /// specifically for the item and detail level, to the provider for goods and
    /// services.
    pub patient_paid: super::money::Money,
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
    pub review_outcome: super::explanation_of_benefit::ExplanationOfBenefitReviewOutcome,
    /// The date or dates when the service or product was supplied, performed or
    /// completed.
    pub serviced_date: String,
    /// The date or dates when the service or product was supplied, performed or
    /// completed.
    pub serviced_period: super::period::Period,
    /// The sequence number of the sub-details woithin the details within the claim item
    /// which this line is intended to replace.
    pub sub_detail_sequence: Vec<super::positive_int::PositiveInt>,
    /// The total of taxes applicable for this product or service.
    pub tax: super::money::Money,
    /// Trace number for tracking purposes. May be defined at the jurisdiction level or
    /// between trading partners.
    pub trace_number: Vec<super::identifier::Identifier>,
    /// If the item is not a group then this is the fee for the product or service,
    /// otherwise this is the total of the fees for the details of the group.
    pub unit_price: super::money::Money,
}

/// This resource provides: the claim details; adjudication details from the
/// processing of a Claim; and optionally account balance information, for informing
/// the subscriber of the benefits provided.
#[derive(Debug, Clone, PartialEq)]
pub struct ExplanationOfBenefitAdjudication {
    /// Monetary amount associated with the category.
    pub amount: super::money::Money,
    /// A code to indicate the information type of this adjudication record. Information
    /// types may include: the value submitted, maximum values or percentages allowed or
    /// payable under the plan, amounts that the patient is responsible for in-aggregate
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
    /// A non-monetary value associated with the category. Mutually exclusive to the
    /// amount element above.
    pub quantity: super::quantity::Quantity,
    /// A code supporting the understanding of the adjudication result and explaining
    /// variance from expected amount.
    pub reason: super::codeable_concept::CodeableConcept,
}

/// This resource provides: the claim details; adjudication details from the
/// processing of a Claim; and optionally account balance information, for informing
/// the subscriber of the benefits provided.
#[derive(Debug, Clone, PartialEq)]
pub struct ExplanationOfBenefitBenefitBalance {
    /// Code to identify the general type of benefits under which products and services
    /// are provided.
    pub category: super::codeable_concept::CodeableConcept,
    /// A richer description of the benefit or services covered.
    pub description: super::string::String,
    /// True if the indicated class of service is excluded from the plan, missing or
    /// False indicates the product or service is included in the coverage.
    pub excluded: super::boolean::Boolean,
    /// Benefits Used to date.
    pub financial: Vec<super::explanation_of_benefit::ExplanationOfBenefitFinancial>,
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
    /// A short name or tag for the benefit.
    pub name: super::string::String,
    /// Is a flag to indicate whether the benefits refer to in-network providers or out-
    /// of-network providers.
    pub network: super::codeable_concept::CodeableConcept,
    /// The term or period of the values such as 'maximum lifetime benefit' or 'maximum
    /// annual visits'.
    pub term: super::codeable_concept::CodeableConcept,
    /// Indicates if the benefits apply to an individual or to the family.
    pub unit: super::codeable_concept::CodeableConcept,
}

/// This resource provides: the claim details; adjudication details from the
/// processing of a Claim; and optionally account balance information, for informing
/// the subscriber of the benefits provided.
#[derive(Debug, Clone, PartialEq)]
pub struct ExplanationOfBenefitBodySite {
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

/// This resource provides: the claim details; adjudication details from the
/// processing of a Claim; and optionally account balance information, for informing
/// the subscriber of the benefits provided.
#[derive(Debug, Clone, PartialEq)]
pub struct ExplanationOfBenefitBodySite1 {
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

/// This resource provides: the claim details; adjudication details from the
/// processing of a Claim; and optionally account balance information, for informing
/// the subscriber of the benefits provided.
#[derive(Debug, Clone, PartialEq)]
pub struct ExplanationOfBenefitCareTeam {
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
    /// Member of the team who provided the product or service.
    pub provider: super::reference::Reference,
    /// The party who is billing and/or responsible for the claimed products or
    /// services.
    pub responsible: super::boolean::Boolean,
    /// The lead, assisting or supervising practitioner and their discipline if a
    /// multidisciplinary team.
    pub role: super::codeable_concept::CodeableConcept,
    /// A number to uniquely identify care team entries.
    pub sequence: super::positive_int::PositiveInt,
    /// The specialization of the practitioner or provider which is applicable for this
    /// service.
    pub specialty: super::codeable_concept::CodeableConcept,
}

/// This resource provides: the claim details; adjudication details from the
/// processing of a Claim; and optionally account balance information, for informing
/// the subscriber of the benefits provided.
#[derive(Debug, Clone, PartialEq)]
pub struct ExplanationOfBenefitDetail {
    /// The adjudication results.
    pub adjudication: Vec<super::explanation_of_benefit::ExplanationOfBenefitAdjudication>,
    /// Code to identify the general type of benefits under which products and services
    /// are provided.
    pub category: super::codeable_concept::CodeableConcept,
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
    /// The total amount claimed for the group (if a grouper) or the line item.detail.
    /// Net = unit price * quantity * factor.
    pub net: super::money::Money,
    /// The numbers associated with notes below which apply to the adjudication of this
    /// item.
    pub note_number: Vec<super::positive_int::PositiveInt>,
    /// The amount paid by the patient, in total at the claim claim level or
    /// specifically for the item and detail level, to the provider for goods and
    /// services.
    pub patient_paid: super::money::Money,
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
    /// The number of repetitions of a service or product.
    pub quantity: super::quantity::Quantity,
    /// The type of revenue or cost center providing the product and/or service.
    pub revenue: super::codeable_concept::CodeableConcept,
    /// The high-level results of the adjudication if adjudication has been performed.
    pub review_outcome: super::explanation_of_benefit::ExplanationOfBenefitReviewOutcome,
    /// A claim detail line. Either a simple (a product or service) or a 'group' of sub-
    /// details which are simple items.
    pub sequence: super::positive_int::PositiveInt,
    /// Third-tier of goods and services.
    pub sub_detail: Vec<super::explanation_of_benefit::ExplanationOfBenefitSubDetail>,
    /// The total of taxes applicable for this product or service.
    pub tax: super::money::Money,
    /// Trace number for tracking purposes. May be defined at the jurisdiction level or
    /// between trading partners.
    pub trace_number: Vec<super::identifier::Identifier>,
    /// Unique Device Identifiers associated with this line item.
    pub udi: Vec<super::reference::Reference>,
    /// If the item is not a group then this is the fee for the product or service,
    /// otherwise this is the total of the fees for the details of the group.
    pub unit_price: super::money::Money,
}

/// This resource provides: the claim details; adjudication details from the
/// processing of a Claim; and optionally account balance information, for informing
/// the subscriber of the benefits provided.
#[derive(Debug, Clone, PartialEq)]
pub struct ExplanationOfBenefitDetail1 {
    /// The adjudication results.
    pub adjudication: Vec<super::explanation_of_benefit::ExplanationOfBenefitAdjudication>,
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
    /// The amount paid by the patient, in total at the claim claim level or
    /// specifically for the item and detail level, to the provider for goods and
    /// services.
    pub patient_paid: super::money::Money,
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
    pub review_outcome: super::explanation_of_benefit::ExplanationOfBenefitReviewOutcome,
    /// The third-tier service adjudications for payor added services.
    pub sub_detail: Vec<super::explanation_of_benefit::ExplanationOfBenefitSubDetail1>,
    /// The total of taxes applicable for this product or service.
    pub tax: super::money::Money,
    /// Trace number for tracking purposes. May be defined at the jurisdiction level or
    /// between trading partners.
    pub trace_number: Vec<super::identifier::Identifier>,
    /// If the item is not a group then this is the fee for the product or service,
    /// otherwise this is the total of the fees for the details of the group.
    pub unit_price: super::money::Money,
}

/// This resource provides: the claim details; adjudication details from the
/// processing of a Claim; and optionally account balance information, for informing
/// the subscriber of the benefits provided.
#[derive(Debug, Clone, PartialEq)]
pub struct ExplanationOfBenefitDiagnosis {
    /// The nature of illness or problem in a coded form or as a reference to an
    /// external defined Condition.
    pub diagnosis_codeable_concept: super::codeable_concept::CodeableConcept,
    /// The nature of illness or problem in a coded form or as a reference to an
    /// external defined Condition.
    pub diagnosis_reference: super::reference::Reference,
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
    /// Indication of whether the diagnosis was present on admission to a facility.
    pub on_admission: super::codeable_concept::CodeableConcept,
    /// A number to uniquely identify diagnosis entries.
    pub sequence: super::positive_int::PositiveInt,
    /// When the condition was observed or the relative ranking.
    pub r#type: Vec<super::codeable_concept::CodeableConcept>,
}

/// This resource provides: the claim details; adjudication details from the
/// processing of a Claim; and optionally account balance information, for informing
/// the subscriber of the benefits provided.
#[derive(Debug, Clone, PartialEq)]
pub struct ExplanationOfBenefitEvent {
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

/// This resource provides: the claim details; adjudication details from the
/// processing of a Claim; and optionally account balance information, for informing
/// the subscriber of the benefits provided.
#[derive(Debug, Clone, PartialEq)]
pub struct ExplanationOfBenefitFinancial {
    /// The quantity of the benefit which is permitted under the coverage.
    pub allowed_money: super::money::Money,
    /// The quantity of the benefit which is permitted under the coverage.
    pub allowed_string: String,
    /// The quantity of the benefit which is permitted under the coverage.
    pub allowed_unsigned_int: u64,
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
    /// Classification of benefit being provided.
    pub r#type: super::codeable_concept::CodeableConcept,
    /// The quantity of the benefit which have been consumed to date.
    pub used_money: super::money::Money,
    /// The quantity of the benefit which have been consumed to date.
    pub used_unsigned_int: u64,
}

/// This resource provides: the claim details; adjudication details from the
/// processing of a Claim; and optionally account balance information, for informing
/// the subscriber of the benefits provided.
#[derive(Debug, Clone, PartialEq)]
pub struct ExplanationOfBenefitInsurance {
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
    /// Reference numbers previously provided by the insurer to the provider to be
    /// quoted on subsequent claims containing services or products related to the prior
    /// authorization.
    pub pre_auth_ref: Vec<super::string::String>,
}

/// This resource provides: the claim details; adjudication details from the
/// processing of a Claim; and optionally account balance information, for informing
/// the subscriber of the benefits provided.
#[derive(Debug, Clone, PartialEq)]
pub struct ExplanationOfBenefitItem {
    /// If this item is a group then the values here are a summary of the adjudication
    /// of the detail items. If this item is a simple product or service then this is
    /// the result of the adjudication of this item.
    pub adjudication: Vec<super::explanation_of_benefit::ExplanationOfBenefitAdjudication>,
    /// Physical location where the service is performed or applies.
    pub body_site: Vec<super::explanation_of_benefit::ExplanationOfBenefitBodySite>,
    /// Care team members related to this service or product.
    pub care_team_sequence: Vec<super::positive_int::PositiveInt>,
    /// Code to identify the general type of benefits under which products and services
    /// are provided.
    pub category: super::codeable_concept::CodeableConcept,
    /// Second-tier of goods and services.
    pub detail: Vec<super::explanation_of_benefit::ExplanationOfBenefitDetail>,
    /// Diagnoses applicable for this service or product.
    pub diagnosis_sequence: Vec<super::positive_int::PositiveInt>,
    /// Healthcare encounters related to this claim.
    pub encounter: Vec<super::reference::Reference>,
    /// A real number that represents a multiplier used in determining the overall value
    /// of services delivered and/or goods received. The concept of a Factor allows for
    /// a discount or surcharge multiplier to be applied to a monetary amount.
    pub factor: super::decimal::Decimal,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// Exceptions, special conditions and supporting information applicable for this
    /// service or product.
    pub information_sequence: Vec<super::positive_int::PositiveInt>,
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
    /// The total amount claimed for the group (if a grouper) or the line item. Net =
    /// unit price * quantity * factor.
    pub net: super::money::Money,
    /// The numbers associated with notes below which apply to the adjudication of this
    /// item.
    pub note_number: Vec<super::positive_int::PositiveInt>,
    /// The amount paid by the patient, in total at the claim claim level or
    /// specifically for the item and detail level, to the provider for goods and
    /// services.
    pub patient_paid: super::money::Money,
    /// Procedures applicable for this service or product.
    pub procedure_sequence: Vec<super::positive_int::PositiveInt>,
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
    /// The number of repetitions of a service or product.
    pub quantity: super::quantity::Quantity,
    /// Request or Referral for Goods or Service to be rendered.
    pub request: Vec<super::reference::Reference>,
    /// The type of revenue or cost center providing the product and/or service.
    pub revenue: super::codeable_concept::CodeableConcept,
    /// The high-level results of the adjudication if adjudication has been performed.
    pub review_outcome: super::explanation_of_benefit::ExplanationOfBenefitReviewOutcome,
    /// A number to uniquely identify item entries.
    pub sequence: super::positive_int::PositiveInt,
    /// The date or dates when the service or product was supplied, performed or
    /// completed.
    pub serviced_date: String,
    /// The date or dates when the service or product was supplied, performed or
    /// completed.
    pub serviced_period: super::period::Period,
    /// The total of taxes applicable for this product or service.
    pub tax: super::money::Money,
    /// Trace number for tracking purposes. May be defined at the jurisdiction level or
    /// between trading partners.
    pub trace_number: Vec<super::identifier::Identifier>,
    /// Unique Device Identifiers associated with this line item.
    pub udi: Vec<super::reference::Reference>,
    /// If the item is not a group then this is the fee for the product or service,
    /// otherwise this is the total of the fees for the details of the group.
    pub unit_price: super::money::Money,
}

/// This resource provides: the claim details; adjudication details from the
/// processing of a Claim; and optionally account balance information, for informing
/// the subscriber of the benefits provided.
#[derive(Debug, Clone, PartialEq)]
pub struct ExplanationOfBenefitPayee {
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
    /// Reference to the individual or organization to whom any payment will be made.
    pub party: super::reference::Reference,
    /// Type of Party to be reimbursed: Subscriber, provider, other.
    pub r#type: super::codeable_concept::CodeableConcept,
}

/// This resource provides: the claim details; adjudication details from the
/// processing of a Claim; and optionally account balance information, for informing
/// the subscriber of the benefits provided.
#[derive(Debug, Clone, PartialEq)]
pub struct ExplanationOfBenefitPayment {
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

/// This resource provides: the claim details; adjudication details from the
/// processing of a Claim; and optionally account balance information, for informing
/// the subscriber of the benefits provided.
#[derive(Debug, Clone, PartialEq)]
pub struct ExplanationOfBenefitProcedure {
    /// Date and optionally time the procedure was performed.
    pub date: super::date_time::DateTime,
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
    /// The code or reference to a Procedure resource which identifies the clinical
    /// intervention performed.
    pub procedure_codeable_concept: super::codeable_concept::CodeableConcept,
    /// The code or reference to a Procedure resource which identifies the clinical
    /// intervention performed.
    pub procedure_reference: super::reference::Reference,
    /// A number to uniquely identify procedure entries.
    pub sequence: super::positive_int::PositiveInt,
    /// When the condition was observed or the relative ranking.
    pub r#type: Vec<super::codeable_concept::CodeableConcept>,
    /// Unique Device Identifiers associated with this line item.
    pub udi: Vec<super::reference::Reference>,
}

/// This resource provides: the claim details; adjudication details from the
/// processing of a Claim; and optionally account balance information, for informing
/// the subscriber of the benefits provided.
#[derive(Debug, Clone, PartialEq)]
pub struct ExplanationOfBenefitProcessNote {
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

/// This resource provides: the claim details; adjudication details from the
/// processing of a Claim; and optionally account balance information, for informing
/// the subscriber of the benefits provided.
#[derive(Debug, Clone, PartialEq)]
pub struct ExplanationOfBenefitRelated {
    /// Reference to a related claim.
    pub claim: super::reference::Reference,
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
    /// An alternate organizational reference to the case or file to which this
    /// particular claim pertains.
    pub reference: super::identifier::Identifier,
    /// A code to convey how the claims are related.
    pub relationship: super::codeable_concept::CodeableConcept,
}

/// This resource provides: the claim details; adjudication details from the
/// processing of a Claim; and optionally account balance information, for informing
/// the subscriber of the benefits provided.
#[derive(Debug, Clone, PartialEq)]
pub struct ExplanationOfBenefitReviewOutcome {
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

/// This resource provides: the claim details; adjudication details from the
/// processing of a Claim; and optionally account balance information, for informing
/// the subscriber of the benefits provided.
#[derive(Debug, Clone, PartialEq)]
pub struct ExplanationOfBenefitSubDetail {
    /// The adjudication results.
    pub adjudication: Vec<super::explanation_of_benefit::ExplanationOfBenefitAdjudication>,
    /// Code to identify the general type of benefits under which products and services
    /// are provided.
    pub category: super::codeable_concept::CodeableConcept,
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
    /// The total amount claimed for the line item.detail.subDetail. Net = unit price *
    /// quantity * factor.
    pub net: super::money::Money,
    /// The numbers associated with notes below which apply to the adjudication of this
    /// item.
    pub note_number: Vec<super::positive_int::PositiveInt>,
    /// The amount paid by the patient, in total at the claim claim level or
    /// specifically for the item and detail level, to the provider for goods and
    /// services.
    pub patient_paid: super::money::Money,
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
    /// The number of repetitions of a service or product.
    pub quantity: super::quantity::Quantity,
    /// The type of revenue or cost center providing the product and/or service.
    pub revenue: super::codeable_concept::CodeableConcept,
    /// The high-level results of the adjudication if adjudication has been performed.
    pub review_outcome: super::explanation_of_benefit::ExplanationOfBenefitReviewOutcome,
    /// A claim detail line. Either a simple (a product or service) or a 'group' of sub-
    /// details which are simple items.
    pub sequence: super::positive_int::PositiveInt,
    /// The total of taxes applicable for this product or service.
    pub tax: super::money::Money,
    /// Trace number for tracking purposes. May be defined at the jurisdiction level or
    /// between trading partners.
    pub trace_number: Vec<super::identifier::Identifier>,
    /// Unique Device Identifiers associated with this line item.
    pub udi: Vec<super::reference::Reference>,
    /// If the item is not a group then this is the fee for the product or service,
    /// otherwise this is the total of the fees for the details of the group.
    pub unit_price: super::money::Money,
}

/// This resource provides: the claim details; adjudication details from the
/// processing of a Claim; and optionally account balance information, for informing
/// the subscriber of the benefits provided.
#[derive(Debug, Clone, PartialEq)]
pub struct ExplanationOfBenefitSubDetail1 {
    /// The adjudication results.
    pub adjudication: Vec<super::explanation_of_benefit::ExplanationOfBenefitAdjudication>,
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
    /// The amount paid by the patient, in total at the claim claim level or
    /// specifically for the item and detail level, to the provider for goods and
    /// services.
    pub patient_paid: super::money::Money,
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
    pub review_outcome: super::explanation_of_benefit::ExplanationOfBenefitReviewOutcome,
    /// The total of taxes applicable for this product or service.
    pub tax: super::money::Money,
    /// Trace number for tracking purposes. May be defined at the jurisdiction level or
    /// between trading partners.
    pub trace_number: Vec<super::identifier::Identifier>,
    /// If the item is not a group then this is the fee for the product or service,
    /// otherwise this is the total of the fees for the details of the group.
    pub unit_price: super::money::Money,
}

/// This resource provides: the claim details; adjudication details from the
/// processing of a Claim; and optionally account balance information, for informing
/// the subscriber of the benefits provided.
#[derive(Debug, Clone, PartialEq)]
pub struct ExplanationOfBenefitSupportingInfo {
    /// The general class of the information supplied: information; exception; accident,
    /// employment; onset, etc.
    pub category: super::codeable_concept::CodeableConcept,
    /// System and code pertaining to the specific information regarding special
    /// conditions relating to the setting, treatment or patient  for which care is
    /// sought.
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
    /// Provides the reason in the situation where a reason code is required in addition
    /// to the content.
    pub reason: super::coding::Coding,
    /// A number to uniquely identify supporting information entries.
    pub sequence: super::positive_int::PositiveInt,
    /// The date when or period to which this information refers.
    pub timing_date: String,
    /// The date when or period to which this information refers.
    pub timing_period: super::period::Period,
    /// Additional data or information such as resources, documents, images etc.
    /// including references to the data or the actual inclusion of the data.
    pub value_attachment: super::attachment::Attachment,
    /// Additional data or information such as resources, documents, images etc.
    /// including references to the data or the actual inclusion of the data.
    pub value_boolean: bool,
    /// Additional data or information such as resources, documents, images etc.
    /// including references to the data or the actual inclusion of the data.
    pub value_identifier: super::identifier::Identifier,
    /// Additional data or information such as resources, documents, images etc.
    /// including references to the data or the actual inclusion of the data.
    pub value_quantity: super::quantity::Quantity,
    /// Additional data or information such as resources, documents, images etc.
    /// including references to the data or the actual inclusion of the data.
    pub value_reference: super::reference::Reference,
    /// Additional data or information such as resources, documents, images etc.
    /// including references to the data or the actual inclusion of the data.
    pub value_string: String,
}

/// This resource provides: the claim details; adjudication details from the
/// processing of a Claim; and optionally account balance information, for informing
/// the subscriber of the benefits provided.
#[derive(Debug, Clone, PartialEq)]
pub struct ExplanationOfBenefitTotal {
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

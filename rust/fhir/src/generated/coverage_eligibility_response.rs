/// This resource provides eligibility and plan details from the processing of an
/// CoverageEligibilityRequest resource.
#[derive(Debug, Clone, PartialEq)]
pub struct CoverageEligibilityResponse {
    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, nor can they have
    /// their own independent transaction scope. This is allowed to be a Parameters
    /// resource if and only if it is referenced by a resource that provides context/
    /// meaning.
    pub contained: Vec<super::resource_list::ResourceList>,
    /// The date this resource was created.
    pub created: super::date_time::DateTime,
    /// A human readable description of the status of the adjudication.
    pub disposition: super::string::String,
    /// Errors encountered during the processing of the request.
    pub error: Vec<super::coverage_eligibility_response::CoverageEligibilityResponseError>,
    /// Information code for an event with a corresponding date or period.
    pub event: Vec<super::coverage_eligibility_response::CoverageEligibilityResponseEvent>,
    /// A code for the form to be used for printing the content.
    pub form: super::codeable_concept::CodeableConcept,
    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub id: super::id::Id,
    /// A unique identifier assigned to this coverage eligiblity request.
    pub identifier: Vec<super::identifier::Identifier>,
    /// A reference to a set of rules that were followed when the resource was
    /// constructed, and which must be understood when processing the content. Often,
    /// this is a reference to an implementation guide that defines the special rules
    /// along with other profiles etc.
    pub implicit_rules: super::uri::Uri,
    /// Financial instruments for reimbursement for the health care products and
    /// services.
    pub insurance: Vec<super::coverage_eligibility_response::CoverageEligibilityResponseInsurance>,
    /// The Insurer who issued the coverage in question and is the author of the
    /// response.
    pub insurer: super::reference::Reference,
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
    /// The outcome of the request processing.
    pub outcome: super::code::Code,
    /// The party who is the beneficiary of the supplied coverage and for whom
    /// eligibility is sought.
    pub patient: super::reference::Reference,
    /// A reference from the Insurer to which these services pertain to be used on
    /// further communication and as proof that the request occurred.
    pub pre_auth_ref: super::string::String,
    /// Code to specify whether requesting: prior authorization requirements for
    /// some service categories or billing codes; benefits for coverages specified or
    /// discovered; discovery and return of coverages for the patient; and/or validation
    /// that the specified coverage is in-force at the date/period specified or 'now' if
    /// not specified.
    pub purpose: Vec<super::code::Code>,
    /// Reference to the original request resource.
    pub request: super::reference::Reference,
    /// The provider which is responsible for the request.
    pub requestor: super::reference::Reference,
    /// This is a CoverageEligibilityResponse resource
    pub resource_type: String,
    /// The date or dates when the enclosed suite of services were performed or
    /// completed.
    pub serviced_date: String,
    /// The date or dates when the enclosed suite of services were performed or
    /// completed.
    pub serviced_period: super::period::Period,
    /// The status of the resource instance.
    pub status: super::code::Code,
    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need
    /// not encode all the structured data, but is required to contain sufficient detail
    /// to make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub text: super::narrative::Narrative,
}

/// This resource provides eligibility and plan details from the processing of an
/// CoverageEligibilityRequest resource.
#[derive(Debug, Clone, PartialEq)]
pub struct CoverageEligibilityResponseBenefit {
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
    pub used_string: String,
    /// The quantity of the benefit which have been consumed to date.
    pub used_unsigned_int: u64,
}

/// This resource provides eligibility and plan details from the processing of an
/// CoverageEligibilityRequest resource.
#[derive(Debug, Clone, PartialEq)]
pub struct CoverageEligibilityResponseError {
    /// An error code,from a specified code system, which details why the eligibility
    /// check could not be performed.
    pub code: super::codeable_concept::CodeableConcept,
    /// A [simple subset of FHIRPath](fhirpath.html#simple) limited to element names,
    /// repetition indicators and the default child accessor that identifies one of the
    /// elements in the resource that caused this issue to be raised.
    pub expression: Vec<super::string::String>,
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

/// This resource provides eligibility and plan details from the processing of an
/// CoverageEligibilityRequest resource.
#[derive(Debug, Clone, PartialEq)]
pub struct CoverageEligibilityResponseEvent {
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

/// This resource provides eligibility and plan details from the processing of an
/// CoverageEligibilityRequest resource.
#[derive(Debug, Clone, PartialEq)]
pub struct CoverageEligibilityResponseInsurance {
    /// The term of the benefits documented in this response.
    pub benefit_period: super::period::Period,
    /// Reference to the insurance card level information contained in the Coverage
    /// resource. The coverage issuing insurer will use these details to locate the
    /// patient's actual coverage within the insurer's information system.
    pub coverage: super::reference::Reference,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// Flag indicating if the coverage provided is inforce currently if no service
    /// date(s) specified or for the whole duration of the service dates.
    pub inforce: super::boolean::Boolean,
    /// Benefits and optionally current balances, and authorization details by category
    /// or service.
    pub item: Vec<super::coverage_eligibility_response::CoverageEligibilityResponseItem>,
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

/// This resource provides eligibility and plan details from the processing of an
/// CoverageEligibilityRequest resource.
#[derive(Debug, Clone, PartialEq)]
pub struct CoverageEligibilityResponseItem {
    /// A boolean flag indicating whether a preauthorization is required prior to actual
    /// service delivery.
    pub authorization_required: super::boolean::Boolean,
    /// Codes or comments regarding information or actions associated with the
    /// preauthorization.
    pub authorization_supporting: Vec<super::codeable_concept::CodeableConcept>,
    /// A web location for obtaining requirements or descriptive information regarding
    /// the preauthorization.
    pub authorization_url: super::uri::Uri,
    /// Benefits used to date.
    pub benefit: Vec<super::coverage_eligibility_response::CoverageEligibilityResponseBenefit>,
    /// Code to identify the general type of benefits under which products and services
    /// are provided.
    pub category: super::codeable_concept::CodeableConcept,
    /// A richer description of the benefit or services covered.
    pub description: super::string::String,
    /// True if the indicated class of service is excluded from the plan, missing or
    /// False indicates the product or service is included in the coverage.
    pub excluded: super::boolean::Boolean,
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
    /// A short name or tag for the benefit.
    pub name: super::string::String,
    /// Is a flag to indicate whether the benefits refer to in-network providers or out-
    /// of-network providers.
    pub network: super::codeable_concept::CodeableConcept,
    /// This contains the product, service, drug or other billing code for the item.
    pub product_or_service: super::codeable_concept::CodeableConcept,
    /// The practitioner who is eligible for the provision of the product or service.
    pub provider: super::reference::Reference,
    /// The term or period of the values such as 'maximum lifetime benefit' or 'maximum
    /// annual visits'.
    pub term: super::codeable_concept::CodeableConcept,
    /// Indicates if the benefits apply to an individual or to the family.
    pub unit: super::codeable_concept::CodeableConcept,
}

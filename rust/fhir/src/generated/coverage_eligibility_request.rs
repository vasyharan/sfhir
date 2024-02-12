/// The CoverageEligibilityRequest provides patient and insurance coverage
/// information to an insurer for them to respond, in the form of an
/// CoverageEligibilityResponse, with information regarding whether the stated
/// coverage is valid and in-force and optionally to provide the insurance details
/// of the policy.
#[derive(Debug, Clone, PartialEq)]
pub struct CoverageEligibilityRequest {
    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, nor can they have
    /// their own independent transaction scope. This is allowed to be a Parameters
    /// resource if and only if it is referenced by a resource that provides context/
    /// meaning.
    pub contained: Vec<super::resource_list::ResourceList>,
    /// The date when this resource was created.
    pub created: super::date_time::DateTime,
    /// Person who created the request.
    pub enterer: super::reference::Reference,
    /// Information code for an event with a corresponding date or period.
    pub event: Vec<super::coverage_eligibility_request::CoverageEligibilityRequestEvent>,
    /// Facility where the services are intended to be provided.
    pub facility: super::reference::Reference,
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
    pub insurance: Vec<super::coverage_eligibility_request::CoverageEligibilityRequestInsurance>,
    /// The Insurer who issued the coverage in question and is the recipient of the
    /// request.
    pub insurer: super::reference::Reference,
    /// Service categories or billable services for which benefit details and/or an
    /// authorization prior to service delivery may be required by the payor.
    pub item: Vec<super::coverage_eligibility_request::CoverageEligibilityRequestItem>,
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
    /// The party who is the beneficiary of the supplied coverage and for whom
    /// eligibility is sought.
    pub patient: super::reference::Reference,
    /// When the requestor expects the processor to complete processing.
    pub priority: super::codeable_concept::CodeableConcept,
    /// The provider which is responsible for the request.
    pub provider: super::reference::Reference,
    /// Code to specify whether requesting: prior authorization requirements for
    /// some service categories or billing codes; benefits for coverages specified or
    /// discovered; discovery and return of coverages for the patient; and/or validation
    /// that the specified coverage is in-force at the date/period specified or 'now' if
    /// not specified.
    pub purpose: Vec<super::code::Code>,
    /// This is a CoverageEligibilityRequest resource
    pub resource_type: String,
    /// The date or dates when the enclosed suite of services were performed or
    /// completed.
    pub serviced_date: String,
    /// The date or dates when the enclosed suite of services were performed or
    /// completed.
    pub serviced_period: super::period::Period,
    /// The status of the resource instance.
    pub status: super::code::Code,
    /// Additional information codes regarding exceptions, special considerations, the
    /// condition, situation, prior or concurrent issues.
    pub supporting_info:
        Vec<super::coverage_eligibility_request::CoverageEligibilityRequestSupportingInfo>,
    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need
    /// not encode all the structured data, but is required to contain sufficient detail
    /// to make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub text: super::narrative::Narrative,
}

/// The CoverageEligibilityRequest provides patient and insurance coverage
/// information to an insurer for them to respond, in the form of an
/// CoverageEligibilityResponse, with information regarding whether the stated
/// coverage is valid and in-force and optionally to provide the insurance details
/// of the policy.
#[derive(Debug, Clone, PartialEq)]
pub struct CoverageEligibilityRequestDiagnosis {
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
}

/// The CoverageEligibilityRequest provides patient and insurance coverage
/// information to an insurer for them to respond, in the form of an
/// CoverageEligibilityResponse, with information regarding whether the stated
/// coverage is valid and in-force and optionally to provide the insurance details
/// of the policy.
#[derive(Debug, Clone, PartialEq)]
pub struct CoverageEligibilityRequestEvent {
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

/// The CoverageEligibilityRequest provides patient and insurance coverage
/// information to an insurer for them to respond, in the form of an
/// CoverageEligibilityResponse, with information regarding whether the stated
/// coverage is valid and in-force and optionally to provide the insurance details
/// of the policy.
#[derive(Debug, Clone, PartialEq)]
pub struct CoverageEligibilityRequestInsurance {
    /// A business agreement number established between the provider and the insurer for
    /// special business processing purposes.
    pub business_arrangement: super::string::String,
    /// Reference to the insurance card level information contained in the Coverage
    /// resource. The coverage issuing insurer will use these details to locate the
    /// patient's actual coverage within the insurer's information system.
    pub coverage: super::reference::Reference,
    /// A flag to indicate that this Coverage is to be used for evaluation of this
    /// request when set to true.
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
}

/// The CoverageEligibilityRequest provides patient and insurance coverage
/// information to an insurer for them to respond, in the form of an
/// CoverageEligibilityResponse, with information regarding whether the stated
/// coverage is valid and in-force and optionally to provide the insurance details
/// of the policy.
#[derive(Debug, Clone, PartialEq)]
pub struct CoverageEligibilityRequestItem {
    /// Code to identify the general type of benefits under which products and services
    /// are provided.
    pub category: super::codeable_concept::CodeableConcept,
    /// The plan/proposal/order describing the proposed service in detail.
    pub detail: Vec<super::reference::Reference>,
    /// Patient diagnosis for which care is sought.
    pub diagnosis: Vec<super::coverage_eligibility_request::CoverageEligibilityRequestDiagnosis>,
    /// Facility where the services will be provided.
    pub facility: super::reference::Reference,
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
    /// This contains the product, service, drug or other billing code for the item.
    pub product_or_service: super::codeable_concept::CodeableConcept,
    /// The practitioner who is responsible for the product or service to be rendered to
    /// the patient.
    pub provider: super::reference::Reference,
    /// The number of repetitions of a service or product.
    pub quantity: super::quantity::Quantity,
    /// Exceptions, special conditions and supporting information applicable for this
    /// service or product line.
    pub supporting_info_sequence: Vec<super::positive_int::PositiveInt>,
    /// The amount charged to the patient by the provider for a single unit.
    pub unit_price: super::money::Money,
}

/// The CoverageEligibilityRequest provides patient and insurance coverage
/// information to an insurer for them to respond, in the form of an
/// CoverageEligibilityResponse, with information regarding whether the stated
/// coverage is valid and in-force and optionally to provide the insurance details
/// of the policy.
#[derive(Debug, Clone, PartialEq)]
pub struct CoverageEligibilityRequestSupportingInfo {
    /// The supporting materials are applicable for all detail items, product/servce
    /// categories and specific billing codes.
    pub applies_to_all: super::boolean::Boolean,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// Additional data or information such as resources, documents, images etc.
    /// including references to the data or the actual inclusion of the data.
    pub information: super::reference::Reference,
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
    /// A number to uniquely identify supporting information entries.
    pub sequence: super::positive_int::PositiveInt,
}

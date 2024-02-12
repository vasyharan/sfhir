/// Financial instrument which may be used to reimburse or pay for health care
/// products and services. Includes both insurance and self-payment.
#[derive(Debug, Clone, PartialEq)]
pub struct Coverage {
    /// The party who benefits from the insurance coverage; the patient when products
    /// and/or services are provided.
    pub beneficiary: super::reference::Reference,
    /// A suite of underwriter specific classifiers.
    pub class: Vec<super::coverage::CoverageClass>,
    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, nor can they have
    /// their own independent transaction scope. This is allowed to be a Parameters
    /// resource if and only if it is referenced by a resource that provides context/
    /// meaning.
    pub contained: Vec<super::resource_list::ResourceList>,
    /// The policy(s) which constitute this insurance coverage.
    pub contract: Vec<super::reference::Reference>,
    /// A suite of codes indicating the cost category and associated amount which have
    /// been detailed in the policy and may have been  included on the health card.
    pub cost_to_beneficiary: Vec<super::coverage::CoverageCostToBeneficiary>,
    /// A designator for a dependent under the coverage.
    pub dependent: super::string::String,
    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub id: super::id::Id,
    /// The identifier of the coverage as issued by the insurer.
    pub identifier: Vec<super::identifier::Identifier>,
    /// A reference to a set of rules that were followed when the resource was
    /// constructed, and which must be understood when processing the content. Often,
    /// this is a reference to an implementation guide that defines the special rules
    /// along with other profiles etc.
    pub implicit_rules: super::uri::Uri,
    /// The insurance plan details, benefits and costs, which constitute this insurance
    /// coverage.
    pub insurance_plan: super::reference::Reference,
    /// The program or plan underwriter, payor, insurance company.
    pub insurer: super::reference::Reference,
    /// The nature of the coverage be it insurance, or cash payment such as self-pay.
    pub kind: super::code::Code,
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
    /// The insurer-specific identifier for the insurer-defined network of providers
    /// to which the beneficiary may seek treatment which will be covered at the 'in-
    /// network' rate, otherwise 'out of network' terms and conditions apply.
    pub network: super::string::String,
    /// The order of applicability of this coverage relative to other coverages which
    /// are currently in force. Note, there may be gaps in the numbering and this does
    /// not imply primary, secondary etc. as the specific positioning of coverages
    /// depends upon the episode of care. For example; a patient might have (0) auto
    /// insurance (1) their own health insurance and (2) spouse's health insurance. When
    /// claiming for treatments which were not the result of an auto accident then only
    /// coverages (1) and (2) above would be applicatble and would apply in the order
    /// specified in parenthesis.
    pub order: super::positive_int::PositiveInt,
    /// Link to the paying party and optionally what specifically they will be
    /// responsible to pay.
    pub payment_by: Vec<super::coverage::CoveragePaymentBy>,
    /// Time period during which the coverage is in force. A missing start date
    /// indicates the start date isn't known, a missing end date means the coverage is
    /// continuing to be in force.
    pub period: super::period::Period,
    /// The party who 'owns' the insurance policy.
    pub policy_holder: super::reference::Reference,
    /// The relationship of beneficiary (patient) to the subscriber.
    pub relationship: super::codeable_concept::CodeableConcept,
    /// This is a Coverage resource
    pub resource_type: String,
    /// The status of the resource instance.
    pub status: super::code::Code,
    /// When 'subrogation=true' this insurance instance has been included not for
    /// adjudication but to provide insurers with the details to recover costs.
    pub subrogation: super::boolean::Boolean,
    /// The party who has signed-up for or 'owns' the contractual relationship to the
    /// policy or to whom the benefit of the policy for services rendered to them or
    /// their family is due.
    pub subscriber: super::reference::Reference,
    /// The insurer assigned ID for the Subscriber.
    pub subscriber_id: Vec<super::identifier::Identifier>,
    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need
    /// not encode all the structured data, but is required to contain sufficient detail
    /// to make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub text: super::narrative::Narrative,
    /// The type of coverage: social program, medical plan, accident coverage (workers
    /// compensation, auto), group health or payment by an individual or organization.
    pub r#type: super::codeable_concept::CodeableConcept,
}

/// Financial instrument which may be used to reimburse or pay for health care
/// products and services. Includes both insurance and self-payment.
#[derive(Debug, Clone, PartialEq)]
pub struct CoverageClass {
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
    /// A short description for the class.
    pub name: super::string::String,
    /// The type of classification for which an insurer-specific class label or number
    /// and optional name is provided.  For example, type may be used to identify a
    /// class of coverage or employer group, policy, or plan.
    pub r#type: super::codeable_concept::CodeableConcept,
    /// The alphanumeric identifier associated with the insurer issued label.
    pub value: super::identifier::Identifier,
}

/// Financial instrument which may be used to reimburse or pay for health care
/// products and services. Includes both insurance and self-payment.
#[derive(Debug, Clone, PartialEq)]
pub struct CoverageCostToBeneficiary {
    /// Code to identify the general type of benefits under which products and services
    /// are provided.
    pub category: super::codeable_concept::CodeableConcept,
    /// A suite of codes indicating exceptions or reductions to patient costs and their
    /// effective periods.
    pub exception: Vec<super::coverage::CoverageException>,
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
    /// Is a flag to indicate whether the benefits refer to in-network providers or out-
    /// of-network providers.
    pub network: super::codeable_concept::CodeableConcept,
    /// The term or period of the values such as 'maximum lifetime benefit' or 'maximum
    /// annual visits'.
    pub term: super::codeable_concept::CodeableConcept,
    /// The category of patient centric costs associated with treatment.
    pub r#type: super::codeable_concept::CodeableConcept,
    /// Indicates if the benefits apply to an individual or to the family.
    pub unit: super::codeable_concept::CodeableConcept,
    /// The amount due from the patient for the cost category.
    pub value_money: super::money::Money,
    /// The amount due from the patient for the cost category.
    pub value_quantity: super::quantity::Quantity,
}

/// Financial instrument which may be used to reimburse or pay for health care
/// products and services. Includes both insurance and self-payment.
#[derive(Debug, Clone, PartialEq)]
pub struct CoverageException {
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
    /// The timeframe the exception is in force.
    pub period: super::period::Period,
    /// The code for the specific exception.
    pub r#type: super::codeable_concept::CodeableConcept,
}

/// Financial instrument which may be used to reimburse or pay for health care
/// products and services. Includes both insurance and self-payment.
#[derive(Debug, Clone, PartialEq)]
pub struct CoveragePaymentBy {
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
    /// The list of parties providing non-insurance payment for the treatment costs.
    pub party: super::reference::Reference,
    ///  Description of the financial responsibility.
    pub responsibility: super::string::String,
}

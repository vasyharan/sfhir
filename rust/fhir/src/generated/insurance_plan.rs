/// Details of a Health Insurance product/plan provided by an organization.
#[derive(Debug, Clone, PartialEq)]
pub struct InsurancePlan {
    /// An organization which administer other services such as underwriting, customer
    /// service and/or claims processing on behalf of the health insurance product
    /// owner.
    pub administered_by: super::reference::Reference,
    /// A list of alternate names that the product is known as, or was known as in the
    /// past.
    pub alias: Vec<super::string::String>,
    /// The contact details of communication devices available relevant to the specific
    /// Insurance Plan/Product. This can include addresses, phone numbers, fax numbers,
    /// mobile numbers, email addresses and web sites.
    pub contact: Vec<super::extended_contact_detail::ExtendedContactDetail>,
    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, nor can they have
    /// their own independent transaction scope. This is allowed to be a Parameters
    /// resource if and only if it is referenced by a resource that provides context/
    /// meaning.
    pub contained: Vec<super::resource_list::ResourceList>,
    /// Details about the coverage offered by the insurance product.
    pub coverage: Vec<super::insurance_plan::InsurancePlanCoverage>,
    /// The geographic region in which a health insurance product's benefits apply.
    pub coverage_area: Vec<super::reference::Reference>,
    /// The technical endpoints providing access to services operated for the health
    /// insurance product.
    pub endpoint: Vec<super::reference::Reference>,
    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub id: super::id::Id,
    /// Business identifiers assigned to this health insurance product which remain
    /// constant as the resource is updated and propagates from server to server.
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
    /// Official name of the health insurance product as designated by the owner.
    pub name: super::string::String,
    /// Reference to the network included in the health insurance product.
    pub network: Vec<super::reference::Reference>,
    /// The entity that is providing  the health insurance product and underwriting the
    /// risk.  This is typically an insurance carriers, other third-party payers, or
    /// health plan sponsors comonly referred to as 'payers'.
    pub owned_by: super::reference::Reference,
    /// The period of time that the health insurance product is available.
    pub period: super::period::Period,
    /// Details about an insurance plan.
    pub plan: Vec<super::insurance_plan::InsurancePlanPlan>,
    /// This is a InsurancePlan resource
    pub resource_type: String,
    /// The current state of the health insurance product.
    pub status: super::code::Code,
    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need
    /// not encode all the structured data, but is required to contain sufficient detail
    /// to make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub text: super::narrative::Narrative,
    /// The kind of health insurance product.
    pub r#type: Vec<super::codeable_concept::CodeableConcept>,
}

/// Details of a Health Insurance product/plan provided by an organization.
#[derive(Debug, Clone, PartialEq)]
pub struct InsurancePlanBenefit {
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// The specific limits on the benefit.
    pub limit: Vec<super::insurance_plan::InsurancePlanLimit>,
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
    /// The referral requirements to have access/coverage for this benefit.
    pub requirement: super::string::String,
    /// Type of benefit (primary care; speciality care; inpatient; outpatient).
    pub r#type: super::codeable_concept::CodeableConcept,
}

/// Details of a Health Insurance product/plan provided by an organization.
#[derive(Debug, Clone, PartialEq)]
pub struct InsurancePlanBenefit1 {
    /// List of the costs associated with a specific benefit.
    pub cost: Vec<super::insurance_plan::InsurancePlanCost>,
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
    /// Type of specific benefit (preventative; primary care office visit; speciality
    /// office visit; hospitalization; emergency room; urgent care).
    pub r#type: super::codeable_concept::CodeableConcept,
}

/// Details of a Health Insurance product/plan provided by an organization.
#[derive(Debug, Clone, PartialEq)]
pub struct InsurancePlanCost {
    /// Whether the cost applies to in-network or out-of-network providers (in-network;
    /// out-of-network; other).
    pub applicability: super::codeable_concept::CodeableConcept,
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
    /// Additional information about the cost, such as information about funding sources
    /// (e.g. HSA, HRA, FSA, RRA).
    pub qualifiers: Vec<super::codeable_concept::CodeableConcept>,
    /// Type of cost (copay; individual cap; family cap; coinsurance; deductible).
    pub r#type: super::codeable_concept::CodeableConcept,
    /// The actual cost value. (some of the costs may be represented as percentages
    /// rather than currency, e.g. 10% coinsurance).
    pub value: super::quantity::Quantity,
}

/// Details of a Health Insurance product/plan provided by an organization.
#[derive(Debug, Clone, PartialEq)]
pub struct InsurancePlanCoverage {
    /// Specific benefits under this type of coverage.
    pub benefit: Vec<super::insurance_plan::InsurancePlanBenefit>,
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
    /// Reference to the network that providing the type of coverage.
    pub network: Vec<super::reference::Reference>,
    /// Type of coverage  (Medical; Dental; Mental Health; Substance Abuse; Vision;
    /// Drug; Short Term; Long Term Care; Hospice; Home Health).
    pub r#type: super::codeable_concept::CodeableConcept,
}

/// Details of a Health Insurance product/plan provided by an organization.
#[derive(Debug, Clone, PartialEq)]
pub struct InsurancePlanGeneralCost {
    /// Additional information about the general costs associated with this plan.
    pub comment: super::string::String,
    /// Value of the cost.
    pub cost: super::money::Money,
    /// Number of participants enrolled in the plan.
    pub group_size: super::positive_int::PositiveInt,
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
    /// Type of cost.
    pub r#type: super::codeable_concept::CodeableConcept,
}

/// Details of a Health Insurance product/plan provided by an organization.
#[derive(Debug, Clone, PartialEq)]
pub struct InsurancePlanLimit {
    /// The specific limit on the benefit.
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
    /// The maximum amount of a service item a plan will pay for a covered benefit.  For
    /// examples. wellness visits, or eyeglasses.
    pub value: super::quantity::Quantity,
}

/// Details of a Health Insurance product/plan provided by an organization.
#[derive(Debug, Clone, PartialEq)]
pub struct InsurancePlanPlan {
    /// The geographic region in which a health insurance plan's benefits apply.
    pub coverage_area: Vec<super::reference::Reference>,
    /// Overall costs associated with the plan.
    pub general_cost: Vec<super::insurance_plan::InsurancePlanGeneralCost>,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// Business identifiers assigned to this health insurance plan which remain
    /// constant as the resource is updated and propagates from server to server.
    pub identifier: Vec<super::identifier::Identifier>,
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
    /// Reference to the network that providing the type of coverage.
    pub network: Vec<super::reference::Reference>,
    /// Costs associated with the coverage provided by the product.
    pub specific_cost: Vec<super::insurance_plan::InsurancePlanSpecificCost>,
    /// Type of plan. For example, "Platinum" or "High Deductable".
    pub r#type: super::codeable_concept::CodeableConcept,
}

/// Details of a Health Insurance product/plan provided by an organization.
#[derive(Debug, Clone, PartialEq)]
pub struct InsurancePlanSpecificCost {
    /// List of the specific benefits under this category of benefit.
    pub benefit: Vec<super::insurance_plan::InsurancePlanBenefit1>,
    /// General category of benefit (Medical; Dental; Vision; Drug; Mental Health;
    /// Substance Abuse; Hospice, Home Health).
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

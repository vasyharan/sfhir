/// A record of a healthcare consumer’s  choices  or choices made on their behalf
/// by a third party, which permits or denies identified recipient(s) or recipient
/// role(s) to perform one or more actions within a given policy context, for
/// specific purposes and periods of time.
#[derive(Debug, Clone, PartialEq)]
pub struct Consent {
    /// A classification of the type of consents found in the statement. This element
    /// supports indexing and retrieval of consent statements.
    pub category: Vec<super::codeable_concept::CodeableConcept>,
    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, nor can they have
    /// their own independent transaction scope. This is allowed to be a Parameters
    /// resource if and only if it is referenced by a resource that provides context/
    /// meaning.
    pub contained: Vec<super::resource_list::ResourceList>,
    /// The actor that controls/enforces the access according to the consent.
    pub controller: Vec<super::reference::Reference>,
    /// Date the consent instance was agreed to.
    pub date: super::date::Date,
    /// Action to take - permit or deny - as default.
    pub decision: super::code::Code,
    /// The entity responsible for complying with the Consent Directive, including any
    /// obligations or limitations on authorizations and enforcement of prohibitions.
    pub grantee: Vec<super::reference::Reference>,
    /// The entity responsible for granting the rights listed in a Consent Directive.
    pub grantor: Vec<super::reference::Reference>,
    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub id: super::id::Id,
    /// Unique identifier for this copy of the Consent Statement.
    pub identifier: Vec<super::identifier::Identifier>,
    /// A reference to a set of rules that were followed when the resource was
    /// constructed, and which must be understood when processing the content. Often,
    /// this is a reference to an implementation guide that defines the special rules
    /// along with other profiles etc.
    pub implicit_rules: super::uri::Uri,
    /// The base language in which the resource is written.
    pub language: super::code::Code,
    /// The actor that manages the consent through its lifecycle.
    pub manager: Vec<super::reference::Reference>,
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
    /// Effective period for this Consent Resource and all provisions unless specified
    /// in that provision.
    pub period: super::period::Period,
    /// A Reference or URL used to uniquely identify the policy the organization will
    /// enforce for this Consent. This Reference or URL should be specific to the
    /// version of the policy and should be dereferencable to a computable policy of
    /// some form.
    pub policy_basis: super::consent::ConsentPolicyBasis,
    /// A Reference to the human readable policy explaining the basis for the Consent.
    pub policy_text: Vec<super::reference::Reference>,
    /// An exception to the base policy of this consent. An exception can be an addition
    /// or removal of access permissions.
    pub provision: Vec<super::consent::ConsentProvision>,
    /// A set of codes that indicate the regulatory basis (if any) that this consent
    /// supports.
    pub regulatory_basis: Vec<super::codeable_concept::CodeableConcept>,
    /// This is a Consent resource
    pub resource_type: String,
    /// The source on which this consent statement is based. The source might be a
    /// scanned original paper form.
    pub source_attachment: Vec<super::attachment::Attachment>,
    /// A reference to a consent that links back to such a source, a reference to a
    /// document repository (e.g. XDS) that stores the original consent document.
    pub source_reference: Vec<super::reference::Reference>,
    /// Indicates the current state of this Consent resource.
    pub status: super::code::Code,
    /// The patient/healthcare practitioner or group of persons to whom this consent
    /// applies.
    pub subject: super::reference::Reference,
    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need
    /// not encode all the structured data, but is required to contain sufficient detail
    /// to make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub text: super::narrative::Narrative,
    /// Whether a treatment instruction (e.g. artificial respiration: yes or no) was
    /// verified with the patient, his/her family or another authorized person.
    pub verification: Vec<super::consent::ConsentVerification>,
}

/// A record of a healthcare consumer’s  choices  or choices made on their behalf
/// by a third party, which permits or denies identified recipient(s) or recipient
/// role(s) to perform one or more actions within a given policy context, for
/// specific purposes and periods of time.
#[derive(Debug, Clone, PartialEq)]
pub struct ConsentActor {
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
    /// The resource that identifies the actor. To identify actors by type, use group
    /// to identify a set of actors by some property they share (e.g. 'admitting
    /// officers').
    pub reference: super::reference::Reference,
    /// How the individual is involved in the resources content that is described in
    /// the exception.
    pub role: super::codeable_concept::CodeableConcept,
}

/// A record of a healthcare consumer’s  choices  or choices made on their behalf
/// by a third party, which permits or denies identified recipient(s) or recipient
/// role(s) to perform one or more actions within a given policy context, for
/// specific purposes and periods of time.
#[derive(Debug, Clone, PartialEq)]
pub struct ConsentData {
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// How the resource reference is interpreted when testing consent restrictions.
    pub meaning: super::code::Code,
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
    /// A reference to a specific resource that defines which resources are covered by
    /// this consent.
    pub reference: super::reference::Reference,
}

/// A record of a healthcare consumer’s  choices  or choices made on their behalf
/// by a third party, which permits or denies identified recipient(s) or recipient
/// role(s) to perform one or more actions within a given policy context, for
/// specific purposes and periods of time.
#[derive(Debug, Clone, PartialEq)]
pub struct ConsentPolicyBasis {
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
    /// A Reference that identifies the policy the organization will enforce for this
    /// Consent.
    pub reference: super::reference::Reference,
    /// A URL that links to a computable version of the policy the organization will
    /// enforce for this Consent.
    pub url: super::url::Url,
}

/// A record of a healthcare consumer’s  choices  or choices made on their behalf
/// by a third party, which permits or denies identified recipient(s) or recipient
/// role(s) to perform one or more actions within a given policy context, for
/// specific purposes and periods of time.
#[derive(Debug, Clone, PartialEq)]
pub struct ConsentProvision {
    /// Actions controlled by this provision.
    pub action: Vec<super::codeable_concept::CodeableConcept>,
    /// Who or what is controlled by this provision. Use group to identify a set of
    /// actors by some property they share (e.g. 'admitting officers').
    pub actor: Vec<super::consent::ConsentActor>,
    /// If this code is found in an instance, then the provision applies.
    pub code: Vec<super::codeable_concept::CodeableConcept>,
    /// The resources controlled by this provision if specific resources are referenced.
    pub data: Vec<super::consent::ConsentData>,
    /// Clinical or Operational Relevant period of time that bounds the data controlled
    /// by this provision.
    pub data_period: super::period::Period,
    /// The documentType(s) covered by this provision. The type can be a CDA document,
    /// or some other type that indicates what sort of information the consent relates
    /// to.
    pub document_type: Vec<super::coding::Coding>,
    /// A computable (FHIRPath or other) definition of what is controlled by this
    /// consent.
    pub expression: super::expression::Expression,
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
    /// Timeframe for this provision.
    pub period: super::period::Period,
    /// Provisions which provide exceptions to the base provision or subprovisions.
    pub provision: Vec<super::consent::ConsentProvision>,
    /// The context of the activities a user is taking - why the user is accessing the
    /// data - that are controlled by this provision.
    pub purpose: Vec<super::coding::Coding>,
    /// The resourceType(s) covered by this provision. The type can be a FHIR resource
    /// type or a profile on a type that indicates what information the consent relates
    /// to.
    pub resource_type: Vec<super::coding::Coding>,
    /// A security label, comprised of 0..* security label fields (Privacy tags), which
    /// define which resources are controlled by this exception.
    pub security_label: Vec<super::coding::Coding>,
}

/// A record of a healthcare consumer’s  choices  or choices made on their behalf
/// by a third party, which permits or denies identified recipient(s) or recipient
/// role(s) to perform one or more actions within a given policy context, for
/// specific purposes and periods of time.
#[derive(Debug, Clone, PartialEq)]
pub struct ConsentVerification {
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
    /// Date(s) verification was collected.
    pub verification_date: Vec<super::date_time::DateTime>,
    /// Extensible list of verification type starting with verification and re-
    /// validation.
    pub verification_type: super::codeable_concept::CodeableConcept,
    /// Has the instruction been verified.
    pub verified: super::boolean::Boolean,
    /// The person who conducted the verification/validation of the Grantor decision.
    pub verified_by: super::reference::Reference,
    /// Who verified the instruction (Patient, Relative or other Authorized Person).
    pub verified_with: super::reference::Reference,
}

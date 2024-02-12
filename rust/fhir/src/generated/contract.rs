/// Legally enforceable, formally recorded unilateral or bilateral directive i.e., a
/// policy or agreement.
#[derive(Debug, Clone, PartialEq)]
pub struct Contract {
    /// Alternative representation of the title for this Contract definition,
    /// derivative, or instance in any legal state., e.g., a domain specific contract
    /// number related to legislation.
    pub alias: Vec<super::string::String>,
    /// Relevant time or time-period when this Contract is applicable.
    pub applies: super::period::Period,
    /// The individual or organization that authored the Contract definition,
    /// derivative, or instance in any legal state.
    pub author: super::reference::Reference,
    /// A formally or informally recognized grouping of people, principals,
    /// organizations, or jurisdictions formed for the purpose of achieving some form
    /// of collective action such as the promulgation, administration and enforcement of
    /// contracts and policies.
    pub authority: Vec<super::reference::Reference>,
    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, nor can they have
    /// their own independent transaction scope. This is allowed to be a Parameters
    /// resource if and only if it is referenced by a resource that provides context/
    /// meaning.
    pub contained: Vec<super::resource_list::ResourceList>,
    /// Precusory content developed with a focus and intent of supporting the formation
    /// a Contract instance, which may be associated with and transformable into a
    /// Contract.
    pub content_definition: super::contract::ContractContentDefinition,
    /// The minimal content derived from the basal information source at a specific
    /// stage in its lifecycle.
    pub content_derivative: super::codeable_concept::CodeableConcept,
    /// Recognized governance framework or system operating with a circumscribed scope
    /// in accordance with specified principles, policies, processes or procedures
    /// for managing rights, actions, or behaviors of parties or principals relative
    /// to resources.
    pub domain: Vec<super::reference::Reference>,
    /// Event resulting in discontinuation or termination of this Contract instance by
    /// one or more parties to the contract.
    pub expiration_type: super::codeable_concept::CodeableConcept,
    /// The "patient friendly language" versionof the Contract in whole or in parts.
    /// "Patient friendly language" means the representation of the Contract and
    /// Contract Provisions in a manner that is readily accessible and understandable
    /// by a layperson in accordance with best practices for communication styles that
    /// ensure that those agreeing to or signing the Contract understand the roles,
    /// actions, obligations, responsibilities, and implication of the agreement.
    pub friendly: Vec<super::contract::ContractFriendly>,
    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub id: super::id::Id,
    /// Unique identifier for this Contract or a derivative that references a Source
    /// Contract.
    pub identifier: Vec<super::identifier::Identifier>,
    /// A reference to a set of rules that were followed when the resource was
    /// constructed, and which must be understood when processing the content. Often,
    /// this is a reference to an implementation guide that defines the special rules
    /// along with other profiles etc.
    pub implicit_rules: super::uri::Uri,
    /// The URL pointing to a FHIR-defined Contract Definition that is adhered to in
    /// whole or part by this Contract.
    pub instantiates_canonical: super::reference::Reference,
    /// The URL pointing to an externally maintained definition that is adhered to in
    /// whole or in part by this Contract.
    pub instantiates_uri: super::uri::Uri,
    /// When this  Contract was issued.
    pub issued: super::date_time::DateTime,
    /// The base language in which the resource is written.
    pub language: super::code::Code,
    /// List of Legal expressions or representations of this Contract.
    pub legal: Vec<super::contract::ContractLegal>,
    /// Legal states of the formation of a legal instrument, which is a formally
    /// executed written document that can be formally attributed to its author, records
    /// and formally expresses a legally enforceable act, process, or contractual duty,
    /// obligation, or right, and therefore evidences that act, process, or agreement.
    pub legal_state: super::codeable_concept::CodeableConcept,
    /// Legally binding Contract: This is the signed and legally recognized
    /// representation of the Contract, which is considered the "source of truth"
    /// and which would be the basis for legal action related to enforcement of this
    /// Contract.
    pub legally_binding_attachment: super::attachment::Attachment,
    /// Legally binding Contract: This is the signed and legally recognized
    /// representation of the Contract, which is considered the "source of truth"
    /// and which would be the basis for legal action related to enforcement of this
    /// Contract.
    pub legally_binding_reference: super::reference::Reference,
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
    /// A natural language name identifying this Contract definition, derivative, or
    /// instance in any legal state. Provides additional information about its content.
    /// This name should be usable as an identifier for the module by machine processing
    /// applications such as code generation.
    pub name: super::string::String,
    /// Links to Provenance records for past versions of this Contract definition,
    /// derivative, or instance, which identify key state transitions or updates that
    /// are likely to be relevant to a user looking at the current version of the
    /// Contract.  The Provenance.entity indicates the target that was changed in the
    /// update (see [Provenance.entity](provenance-definitions.html#Provenance.entity)).
    pub relevant_history: Vec<super::reference::Reference>,
    /// This is a Contract resource
    pub resource_type: String,
    /// List of Computable Policy Rule Language Representations of this Contract.
    pub rule: Vec<super::contract::ContractRule>,
    /// A selector of legal concerns for this Contract definition, derivative, or
    /// instance in any legal state.
    pub scope: super::codeable_concept::CodeableConcept,
    /// Parties with legal standing in the Contract, including the principal parties,
    /// the grantor(s) and grantee(s), which are any person or organization bound by
    /// the contract, and any ancillary parties, which facilitate the execution of the
    /// contract such as a notary or witness.
    pub signer: Vec<super::contract::ContractSigner>,
    /// Sites in which the contract is complied with,  exercised, or in force.
    pub site: Vec<super::reference::Reference>,
    /// The status of the resource instance.
    pub status: super::code::Code,
    /// Sub-category for the Contract that distinguishes the kinds of systems that would
    /// be interested in the Contract within the context of the Contract's scope.
    pub sub_type: Vec<super::codeable_concept::CodeableConcept>,
    /// The target entity impacted by or of interest to parties to the agreement.
    pub subject: Vec<super::reference::Reference>,
    /// A more detailed or qualifying explanatory or alternate user-friendly title for
    /// this Contract definition, derivative, or instance in any legal state.
    pub subtitle: super::string::String,
    /// Information that may be needed by/relevant to the performer in their execution
    /// of this term action.
    pub supporting_info: Vec<super::reference::Reference>,
    /// One or more Contract Provisions, which may be related and conveyed as a group,
    /// and may contain nested groups.
    pub term: Vec<super::contract::ContractTerm>,
    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need
    /// not encode all the structured data, but is required to contain sufficient detail
    /// to make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub text: super::narrative::Narrative,
    /// A short, descriptive, user-friendly title for this Contract definition,
    /// derivative, or instance in any legal state.
    pub title: super::string::String,
    /// Narrows the range of legal concerns to focus on the achievement of specific
    /// contractual objectives.
    pub topic_codeable_concept: super::codeable_concept::CodeableConcept,
    /// Narrows the range of legal concerns to focus on the achievement of specific
    /// contractual objectives.
    pub topic_reference: super::reference::Reference,
    /// A high-level category for the legal instrument, whether constructed as a
    /// Contract definition, derivative, or instance in any legal state.  Provides
    /// additional information about its content within the context of the Contract's
    /// scope to distinguish the kinds of systems that would be interested in the
    /// contract.
    pub r#type: super::codeable_concept::CodeableConcept,
    /// Canonical identifier for this contract, represented as a URI (globally unique).
    pub url: super::uri::Uri,
    /// An edition identifier used for business purposes to label business significant
    /// variants.
    pub version: super::string::String,
}

/// Legally enforceable, formally recorded unilateral or bilateral directive i.e., a
/// policy or agreement.
#[derive(Debug, Clone, PartialEq)]
pub struct ContractAction {
    /// Encounter or Episode with primary association to the specified term activity.
    pub context: super::reference::Reference,
    /// Id [identifier??] of the clause or question text related to the requester of
    /// this action in the referenced form or QuestionnaireResponse.
    pub context_link_id: Vec<super::string::String>,
    /// True if the term prohibits the  action.
    pub do_not_perform: super::boolean::Boolean,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// Reason or purpose for the action stipulated by this Contract Provision.
    pub intent: super::codeable_concept::CodeableConcept,
    /// Id [identifier??] of the clause or question text related to this action in the
    /// referenced form or QuestionnaireResponse.
    pub link_id: Vec<super::string::String>,
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
    /// Comments made about the term action made by the requester, performer, subject or
    /// other participants.
    pub note: Vec<super::annotation::Annotation>,
    /// When action happens.
    pub occurrence_date_time: String,
    /// When action happens.
    pub occurrence_period: super::period::Period,
    /// When action happens.
    pub occurrence_timing: super::timing::Timing,
    /// Indicates who or what is being asked to perform (or not perform) the ction.
    pub performer: super::reference::Reference,
    /// Id [identifier??] of the clause or question text related to the reason type or
    /// reference of this  action in the referenced form or QuestionnaireResponse.
    pub performer_link_id: Vec<super::string::String>,
    /// The type of role or competency of an individual desired or required to perform
    /// or not perform the action.
    pub performer_role: super::codeable_concept::CodeableConcept,
    /// The type of individual that is desired or required to perform or not perform
    /// the action.
    pub performer_type: Vec<super::codeable_concept::CodeableConcept>,
    /// Rationale for the action to be performed or not performed. Describes why the
    /// action is permitted or prohibited. Either a coded concept, or another resource
    /// whose existence justifies permitting or not permitting this action.
    pub reason: Vec<super::codeable_reference::CodeableReference>,
    /// Id [identifier??] of the clause or question text related to the reason type or
    /// reference of this  action in the referenced form or QuestionnaireResponse.
    pub reason_link_id: Vec<super::string::String>,
    /// Who or what initiated the action and has responsibility for its activation.
    pub requester: Vec<super::reference::Reference>,
    /// Id [identifier??] of the clause or question text related to the requester of
    /// this action in the referenced form or QuestionnaireResponse.
    pub requester_link_id: Vec<super::string::String>,
    /// Security labels that protects the action.
    pub security_label_number: Vec<super::unsigned_int::UnsignedInt>,
    /// Current state of the term action.
    pub status: super::codeable_concept::CodeableConcept,
    /// Entity of the action.
    pub subject: Vec<super::contract::ContractSubject>,
    /// Activity or service obligation to be done or not done, performed or not
    /// performed, effectuated or not by this Contract term.
    pub r#type: super::codeable_concept::CodeableConcept,
}

/// Legally enforceable, formally recorded unilateral or bilateral directive i.e., a
/// policy or agreement.
#[derive(Debug, Clone, PartialEq)]
pub struct ContractAnswer {
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
    /// Response to an offer clause or question text,  which enables selection of values
    /// to be agreed to, e.g., the period of participation, the date of occupancy of
    /// a rental, warranty duration, or whether biospecimen may be used for further
    /// research.
    pub value_attachment: super::attachment::Attachment,
    /// Response to an offer clause or question text,  which enables selection of values
    /// to be agreed to, e.g., the period of participation, the date of occupancy of
    /// a rental, warranty duration, or whether biospecimen may be used for further
    /// research.
    pub value_boolean: bool,
    /// Response to an offer clause or question text,  which enables selection of values
    /// to be agreed to, e.g., the period of participation, the date of occupancy of
    /// a rental, warranty duration, or whether biospecimen may be used for further
    /// research.
    pub value_coding: super::coding::Coding,
    /// Response to an offer clause or question text,  which enables selection of values
    /// to be agreed to, e.g., the period of participation, the date of occupancy of
    /// a rental, warranty duration, or whether biospecimen may be used for further
    /// research.
    pub value_date: String,
    /// Response to an offer clause or question text,  which enables selection of values
    /// to be agreed to, e.g., the period of participation, the date of occupancy of
    /// a rental, warranty duration, or whether biospecimen may be used for further
    /// research.
    pub value_date_time: String,
    /// Response to an offer clause or question text,  which enables selection of values
    /// to be agreed to, e.g., the period of participation, the date of occupancy of
    /// a rental, warranty duration, or whether biospecimen may be used for further
    /// research.
    pub value_decimal: f64,
    /// Response to an offer clause or question text,  which enables selection of values
    /// to be agreed to, e.g., the period of participation, the date of occupancy of
    /// a rental, warranty duration, or whether biospecimen may be used for further
    /// research.
    pub value_integer: i64,
    /// Response to an offer clause or question text,  which enables selection of values
    /// to be agreed to, e.g., the period of participation, the date of occupancy of
    /// a rental, warranty duration, or whether biospecimen may be used for further
    /// research.
    pub value_quantity: super::quantity::Quantity,
    /// Response to an offer clause or question text,  which enables selection of values
    /// to be agreed to, e.g., the period of participation, the date of occupancy of
    /// a rental, warranty duration, or whether biospecimen may be used for further
    /// research.
    pub value_reference: super::reference::Reference,
    /// Response to an offer clause or question text,  which enables selection of values
    /// to be agreed to, e.g., the period of participation, the date of occupancy of
    /// a rental, warranty duration, or whether biospecimen may be used for further
    /// research.
    pub value_string: String,
    /// Response to an offer clause or question text,  which enables selection of values
    /// to be agreed to, e.g., the period of participation, the date of occupancy of
    /// a rental, warranty duration, or whether biospecimen may be used for further
    /// research.
    pub value_time: String,
    /// Response to an offer clause or question text,  which enables selection of values
    /// to be agreed to, e.g., the period of participation, the date of occupancy of
    /// a rental, warranty duration, or whether biospecimen may be used for further
    /// research.
    pub value_uri: String,
}

/// Legally enforceable, formally recorded unilateral or bilateral directive i.e., a
/// policy or agreement.
#[derive(Debug, Clone, PartialEq)]
pub struct ContractAsset {
    /// Response to assets.
    pub answer: Vec<super::contract::ContractAnswer>,
    /// Description of the quality and completeness of the asset that may be a factor in
    /// its valuation.
    pub condition: super::string::String,
    /// Circumstance of the asset.
    pub context: Vec<super::contract::ContractContext>,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// Id [identifier??] of the clause or question text about the asset in the
    /// referenced form or QuestionnaireResponse.
    pub link_id: Vec<super::string::String>,
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
    /// Asset relevant contractual time period.
    pub period: Vec<super::period::Period>,
    /// Type of Asset availability for use or ownership.
    pub period_type: Vec<super::codeable_concept::CodeableConcept>,
    /// Specifies the applicability of the term to an asset resource instance, and
    /// instances it refers to or instances that refer to it, and/or are owned by the
    /// offeree.
    pub relationship: super::coding::Coding,
    /// Differentiates the kind of the asset .
    pub scope: super::codeable_concept::CodeableConcept,
    /// Security labels that protects the asset.
    pub security_label_number: Vec<super::unsigned_int::UnsignedInt>,
    /// May be a subtype or part of an offered asset.
    pub subtype: Vec<super::codeable_concept::CodeableConcept>,
    /// Clause or question text (Prose Object) concerning the asset in a linked form,
    /// such as a QuestionnaireResponse used in the formation of the contract.
    pub text: super::string::String,
    /// Target entity type about which the term may be concerned.
    pub r#type: Vec<super::codeable_concept::CodeableConcept>,
    /// Associated entities.
    pub type_reference: Vec<super::reference::Reference>,
    /// Time period of asset use.
    pub use_period: Vec<super::period::Period>,
    /// Contract Valued Item List.
    pub valued_item: Vec<super::contract::ContractValuedItem>,
}

/// Legally enforceable, formally recorded unilateral or bilateral directive i.e., a
/// policy or agreement.
#[derive(Debug, Clone, PartialEq)]
pub struct ContractContentDefinition {
    /// A copyright statement relating to Contract precursor content. Copyright
    /// statements are generally legal restrictions on the use and publishing of the
    /// Contract precursor content.
    pub copyright: super::markdown::Markdown,
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
    /// The date (and optionally time) when the contract was last significantly changed.
    /// The date must change when the business version changes and it must change if the
    /// status code changes. In addition, it should change when the substantive content
    /// of the contract changes.
    pub publication_date: super::date_time::DateTime,
    /// amended | appended | cancelled | disputed | entered-in-error | executable +.
    pub publication_status: super::code::Code,
    /// The  individual or organization that published the Contract precursor content.
    pub publisher: super::reference::Reference,
    /// Detailed Precusory content type.
    pub sub_type: super::codeable_concept::CodeableConcept,
    /// Precusory content structure and use, i.e., a boilerplate, template, application
    /// for a contract such as an insurance policy or benefits under a program, e.g.,
    /// workers compensation.
    pub r#type: super::codeable_concept::CodeableConcept,
}

/// Legally enforceable, formally recorded unilateral or bilateral directive i.e., a
/// policy or agreement.
#[derive(Debug, Clone, PartialEq)]
pub struct ContractContext {
    /// Coded representation of the context generally or of the Referenced entity, such
    /// as the asset holder type or location.
    pub code: Vec<super::codeable_concept::CodeableConcept>,
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
    /// Asset context reference may include the creator, custodian, or owning Person
    /// or Organization (e.g., bank, repository),  location held, e.g., building,
    /// jurisdiction.
    pub reference: super::reference::Reference,
    /// Context description.
    pub text: super::string::String,
}

/// Legally enforceable, formally recorded unilateral or bilateral directive i.e., a
/// policy or agreement.
#[derive(Debug, Clone, PartialEq)]
pub struct ContractFriendly {
    /// Human readable rendering of this Contract in a format and representation
    /// intended to enhance comprehension and ensure understandability.
    pub content_attachment: super::attachment::Attachment,
    /// Human readable rendering of this Contract in a format and representation
    /// intended to enhance comprehension and ensure understandability.
    pub content_reference: super::reference::Reference,
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

/// Legally enforceable, formally recorded unilateral or bilateral directive i.e., a
/// policy or agreement.
#[derive(Debug, Clone, PartialEq)]
pub struct ContractLegal {
    /// Contract legal text in human renderable form.
    pub content_attachment: super::attachment::Attachment,
    /// Contract legal text in human renderable form.
    pub content_reference: super::reference::Reference,
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

/// Legally enforceable, formally recorded unilateral or bilateral directive i.e., a
/// policy or agreement.
#[derive(Debug, Clone, PartialEq)]
pub struct ContractOffer {
    /// Response to offer text.
    pub answer: Vec<super::contract::ContractAnswer>,
    /// Type of choice made by accepting party with respect to an offer made by an
    /// offeror/ grantee.
    pub decision: super::codeable_concept::CodeableConcept,
    /// How the decision about a Contract was conveyed.
    pub decision_mode: Vec<super::codeable_concept::CodeableConcept>,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// Unique identifier for this particular Contract Provision.
    pub identifier: Vec<super::identifier::Identifier>,
    /// The id of the clause or question text of the offer in the referenced
    /// questionnaire/response.
    pub link_id: Vec<super::string::String>,
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
    /// Offer Recipient.
    pub party: Vec<super::contract::ContractParty>,
    /// Security labels that protects the offer.
    pub security_label_number: Vec<super::unsigned_int::UnsignedInt>,
    /// Human readable form of this Contract Offer.
    pub text: super::string::String,
    /// The owner of an asset has the residual control rights over the asset: the right
    /// to decide all usages of the asset in any way not inconsistent with a prior
    /// contract, custom, or law (Hart, 1995, p. 30).
    pub topic: super::reference::Reference,
    /// Type of Contract Provision such as specific requirements, purposes for actions,
    /// obligations, prohibitions, e.g. life time maximum benefit.
    pub r#type: super::codeable_concept::CodeableConcept,
}

/// Legally enforceable, formally recorded unilateral or bilateral directive i.e., a
/// policy or agreement.
#[derive(Debug, Clone, PartialEq)]
pub struct ContractParty {
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
    /// Participant in the offer.
    pub reference: Vec<super::reference::Reference>,
    /// How the party participates in the offer.
    pub role: super::codeable_concept::CodeableConcept,
}

/// Legally enforceable, formally recorded unilateral or bilateral directive i.e., a
/// policy or agreement.
#[derive(Debug, Clone, PartialEq)]
pub struct ContractRule {
    /// Computable Contract conveyed using a policy rule language (e.g. XACML, DKAL,
    /// SecPal).
    pub content_attachment: super::attachment::Attachment,
    /// Computable Contract conveyed using a policy rule language (e.g. XACML, DKAL,
    /// SecPal).
    pub content_reference: super::reference::Reference,
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

/// Legally enforceable, formally recorded unilateral or bilateral directive i.e., a
/// policy or agreement.
#[derive(Debug, Clone, PartialEq)]
pub struct ContractSecurityLabel {
    /// Security label privacy tag that specifies the applicable privacy and security
    /// policies governing this term and/or term elements.
    pub category: Vec<super::coding::Coding>,
    /// Security label privacy tag that specifies the level of confidentiality
    /// protection required for this term and/or term elements.
    pub classification: super::coding::Coding,
    /// Security label privacy tag that specifies the manner in which term and/or term
    /// elements are to be protected.
    pub control: Vec<super::coding::Coding>,
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
    /// Number used to link this term or term element to the applicable Security Label.
    pub number: Vec<super::unsigned_int::UnsignedInt>,
}

/// Legally enforceable, formally recorded unilateral or bilateral directive i.e., a
/// policy or agreement.
#[derive(Debug, Clone, PartialEq)]
pub struct ContractSigner {
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
    /// Party which is a signator to this Contract.
    pub party: super::reference::Reference,
    /// Legally binding Contract DSIG signature contents in Base64.
    pub signature: Vec<super::signature::Signature>,
    /// Role of this Contract signer, e.g. notary, grantee.
    pub r#type: super::coding::Coding,
}

/// Legally enforceable, formally recorded unilateral or bilateral directive i.e., a
/// policy or agreement.
#[derive(Debug, Clone, PartialEq)]
pub struct ContractSubject {
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
    /// The entity the action is performed or not performed on or for.
    pub reference: Vec<super::reference::Reference>,
    /// Role type of agent assigned roles in this Contract.
    pub role: super::codeable_concept::CodeableConcept,
}

/// Legally enforceable, formally recorded unilateral or bilateral directive i.e., a
/// policy or agreement.
#[derive(Debug, Clone, PartialEq)]
pub struct ContractTerm {
    /// An actor taking a role in an activity for which it can be assigned some degree
    /// of responsibility for the activity taking place.
    pub action: Vec<super::contract::ContractAction>,
    /// Relevant time or time-period when this Contract Provision is applicable.
    pub applies: super::period::Period,
    /// Contract Term Asset List.
    pub asset: Vec<super::contract::ContractAsset>,
    /// Nested group of Contract Provisions.
    pub group: Vec<super::contract::ContractTerm>,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// Unique identifier for this particular Contract Provision.
    pub identifier: super::identifier::Identifier,
    /// When this Contract Provision was issued.
    pub issued: super::date_time::DateTime,
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
    /// The matter of concern in the context of this provision of the agrement.
    pub offer: super::contract::ContractOffer,
    /// Security labels that protect the handling of information about the term and its
    /// elements, which may be specifically identified.
    pub security_label: Vec<super::contract::ContractSecurityLabel>,
    /// A specialized legal clause or condition based on overarching contract type.
    pub sub_type: super::codeable_concept::CodeableConcept,
    /// Statement of a provision in a policy or a contract.
    pub text: super::string::String,
    /// The entity that the term applies to.
    pub topic_codeable_concept: super::codeable_concept::CodeableConcept,
    /// The entity that the term applies to.
    pub topic_reference: super::reference::Reference,
    /// A legal clause or condition contained within a contract that requires one or
    /// both parties to perform a particular requirement by some specified time or
    /// prevents one or both parties from performing a particular requirement by some
    /// specified time.
    pub r#type: super::codeable_concept::CodeableConcept,
}

/// Legally enforceable, formally recorded unilateral or bilateral directive i.e., a
/// policy or agreement.
#[derive(Debug, Clone, PartialEq)]
pub struct ContractValuedItem {
    /// Indicates the time during which this Contract ValuedItem information is
    /// effective.
    pub effective_time: super::date_time::DateTime,
    /// Specific type of Contract Valued Item that may be priced.
    pub entity_codeable_concept: super::codeable_concept::CodeableConcept,
    /// Specific type of Contract Valued Item that may be priced.
    pub entity_reference: super::reference::Reference,
    /// A real number that represents a multiplier used in determining the overall value
    /// of the Contract Valued Item delivered. The concept of a Factor allows for a
    /// discount or surcharge multiplier to be applied to a monetary amount.
    pub factor: super::decimal::Decimal,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// Identifies a Contract Valued Item instance.
    pub identifier: super::identifier::Identifier,
    /// Id  of the clause or question text related to the context of this valuedItem in
    /// the referenced form or QuestionnaireResponse.
    pub link_id: Vec<super::string::String>,
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
    /// Expresses the product of the Contract Valued Item unitQuantity and the
    /// unitPriceAmt. For example, the formula: unit Quantity * unit Price (Cost per
    /// Point) * factor Number  * points = net Amount. Quantity, factor and points are
    /// assumed to be 1 if not supplied.
    pub net: super::money::Money,
    /// Terms of valuation.
    pub payment: super::string::String,
    /// When payment is due.
    pub payment_date: super::date_time::DateTime,
    /// An amount that expresses the weighting (based on difficulty, cost and/or
    /// resource intensiveness) associated with the Contract Valued Item delivered. The
    /// concept of Points allows for assignment of point values for a Contract Valued
    /// Item, such that a monetary amount can be assigned to each point.
    pub points: super::decimal::Decimal,
    /// Specifies the units by which the Contract Valued Item is measured or counted,
    /// and quantifies the countable or measurable Contract Valued Item instances.
    pub quantity: super::quantity::Quantity,
    /// Who will receive payment.
    pub recipient: super::reference::Reference,
    /// Who will make payment.
    pub responsible: super::reference::Reference,
    /// A set of security labels that define which terms are controlled by this
    /// condition.
    pub security_label_number: Vec<super::unsigned_int::UnsignedInt>,
    /// A Contract Valued Item unit valuation measure.
    pub unit_price: super::money::Money,
}

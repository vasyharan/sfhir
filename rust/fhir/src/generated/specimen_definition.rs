/// A kind of specimen with associated set of requirements.
#[derive(Debug, Clone, PartialEq)]
pub struct SpecimenDefinition {
    /// The date on which the asset content was approved by the publisher. Approval
    /// happens once when the content is officially approved for usage.
    pub approval_date: super::date::Date,
    /// An individiual or organization primarily involved in the creation and
    /// maintenance of the {{title}}.
    pub author: Vec<super::contact_detail::ContactDetail>,
    /// The action to be performed for collecting the specimen.
    pub collection: Vec<super::codeable_concept::CodeableConcept>,
    /// Contact details to assist a user in finding and communicating with the
    /// publisher.
    pub contact: Vec<super::contact_detail::ContactDetail>,
    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, nor can they have
    /// their own independent transaction scope. This is allowed to be a Parameters
    /// resource if and only if it is referenced by a resource that provides context/
    /// meaning.
    pub contained: Vec<super::resource_list::ResourceList>,
    /// Copyright statement relating to the SpecimenDefinition and/or its contents.
    /// Copyright statements are generally legal restrictions on the use and publishing
    /// of the SpecimenDefinition.
    pub copyright: super::markdown::Markdown,
    /// A short string (<50 characters), suitable for inclusion in a page footer that
    /// identifies the copyright holder, effective period, and optionally whether rights
    /// are resctricted. (e.g. 'All rights reserved', 'Some rights reserved').
    pub copyright_label: super::string::String,
    /// For draft definitions, indicates the date of initial creation. For active
    /// definitions, represents the date of activation. For withdrawn definitions,
    /// indicates the date of withdrawal.
    pub date: super::date_time::DateTime,
    /// The canonical URL pointing to another FHIR-defined SpecimenDefinition that is
    /// adhered to in whole or in part by this definition.
    pub derived_from_canonical: Vec<super::canonical::Canonical>,
    /// The URL pointing to an externally-defined type of specimen, guideline or other
    /// definition that is adhered to in whole or in part by this definition.
    pub derived_from_uri: Vec<super::uri::Uri>,
    /// A free text natural language description of the SpecimenDefinition from the
    /// consumer's perspective.
    pub description: super::markdown::Markdown,
    /// An individual or organization primarily responsible for internal coherence of
    /// the {{title}}.
    pub editor: Vec<super::contact_detail::ContactDetail>,
    /// The period during which the SpecimenDefinition content was or is planned to
    /// be effective.
    pub effective_period: super::period::Period,
    /// An individual or organization asserted by the publisher to be responsible for
    /// officially endorsing the {{title}} for use in some setting.
    pub endorser: Vec<super::contact_detail::ContactDetail>,
    /// A flag to indicate that this SpecimenDefinition is not authored for  genuine
    /// usage.
    pub experimental: super::boolean::Boolean,
    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub id: super::id::Id,
    /// A business identifier assigned to this SpecimenDefinition.
    pub identifier: super::identifier::Identifier,
    /// A reference to a set of rules that were followed when the resource was
    /// constructed, and which must be understood when processing the content. Often,
    /// this is a reference to an implementation guide that defines the special rules
    /// along with other profiles etc.
    pub implicit_rules: super::uri::Uri,
    /// A jurisdiction in which the SpecimenDefinition is intended to be used.
    pub jurisdiction: Vec<super::codeable_concept::CodeableConcept>,
    /// The base language in which the resource is written.
    pub language: super::code::Code,
    /// The date on which the asset content was last reviewed. Review happens
    /// periodically after that, but doesn't change the original approval date.
    pub last_review_date: super::date::Date,
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
    /// A natural language name identifying the {{title}}. This name should be usable
    /// as an identifier for the module by machine processing applications such as code
    /// generation.
    pub name: super::string::String,
    /// Preparation of the patient for specimen collection.
    pub patient_preparation: Vec<super::codeable_concept::CodeableConcept>,
    /// Helps establish the "authority/credibility" of the SpecimenDefinition. May also
    /// allow for contact.
    pub publisher: super::string::String,
    /// Explains why this SpecimeDefinition is needed and why it has been designed as
    /// it has.
    pub purpose: super::markdown::Markdown,
    /// Related artifacts such as additional documentation, justification, dependencies,
    /// bibliographic references, and predecessor and successor artifacts.
    pub related_artifact: Vec<super::related_artifact::RelatedArtifact>,
    /// This is a SpecimenDefinition resource
    pub resource_type: String,
    /// An individual or organization asserted by the publisher to be primarily
    /// responsible for review of some aspect of the {{title}}.
    pub reviewer: Vec<super::contact_detail::ContactDetail>,
    /// The current state of theSpecimenDefinition.
    pub status: super::code::Code,
    /// A code or group definition that describes the intended subject  from which this
    /// kind of specimen is to be collected.
    pub subject_codeable_concept: super::codeable_concept::CodeableConcept,
    /// A code or group definition that describes the intended subject  from which this
    /// kind of specimen is to be collected.
    pub subject_reference: super::reference::Reference,
    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need
    /// not encode all the structured data, but is required to contain sufficient detail
    /// to make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub text: super::narrative::Narrative,
    /// Time aspect of specimen collection (duration or offset).
    pub time_aspect: super::string::String,
    /// A short, descriptive, user-friendly title for the SpecimenDefinition.
    pub title: super::string::String,
    /// Descriptive topics related to the content of the {{title}}. Topics provide a
    /// high-level categorization as well as keywords for the {{title}} that can be
    /// useful for filtering and searching.
    pub topic: Vec<super::codeable_concept::CodeableConcept>,
    /// The kind of material to be collected.
    pub type_collected: super::codeable_concept::CodeableConcept,
    /// Specimen conditioned in a container as expected by the testing laboratory.
    pub type_tested: Vec<super::specimen_definition::SpecimenDefinitionTypeTested>,
    /// An absolute URL that is used to identify this SpecimenDefinition when it
    /// is referenced in a specification, model, design or an instance. This SHALL
    /// be a URL, SHOULD be globally unique, and SHOULD be an address at which this
    /// SpecimenDefinition is (or will be) published. The URL SHOULD include the major
    /// version of the SpecimenDefinition. For more information see Technical and
    /// Business Versions.
    pub url: super::uri::Uri,
    /// The content was developed with a focus and intent of supporting the contexts
    /// that are listed. These terms may be used to assist with indexing and searching
    /// of specimen definitions.
    pub use_context: Vec<super::usage_context::UsageContext>,
    /// The identifier that is used to identify this version of the SpecimenDefinition
    /// when it is referenced in a specification, model, design or instance. This is an
    /// arbitrary value managed by the SpecimenDefinition author and is not expected to
    /// be globally unique.
    pub version: super::string::String,
    /// Indicates the mechanism used to compare versions to determine which is more
    /// current.
    pub version_algorithm_coding: super::coding::Coding,
    /// Indicates the mechanism used to compare versions to determine which is more
    /// current.
    pub version_algorithm_string: String,
}

/// A kind of specimen with associated set of requirements.
#[derive(Debug, Clone, PartialEq)]
pub struct SpecimenDefinitionAdditive {
    /// Substance introduced in the kind of container to preserve, maintain or enhance
    /// the specimen. Examples: Formalin, Citrate, EDTA.
    pub additive_codeable_concept: super::codeable_concept::CodeableConcept,
    /// Substance introduced in the kind of container to preserve, maintain or enhance
    /// the specimen. Examples: Formalin, Citrate, EDTA.
    pub additive_reference: super::reference::Reference,
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

/// A kind of specimen with associated set of requirements.
#[derive(Debug, Clone, PartialEq)]
pub struct SpecimenDefinitionContainer {
    /// Substance introduced in the kind of container to preserve, maintain or enhance
    /// the specimen. Examples: Formalin, Citrate, EDTA.
    pub additive: Vec<super::specimen_definition::SpecimenDefinitionAdditive>,
    /// Color of container cap.
    pub cap: super::codeable_concept::CodeableConcept,
    /// The capacity (volume or other measure) of this kind of container.
    pub capacity: super::quantity::Quantity,
    /// The textual description of the kind of container.
    pub description: super::markdown::Markdown,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// The type of material of the container.
    pub material: super::codeable_concept::CodeableConcept,
    /// The minimum volume to be conditioned in the container.
    pub minimum_volume_quantity: super::quantity::Quantity,
    /// The minimum volume to be conditioned in the container.
    pub minimum_volume_string: String,
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
    /// Special processing that should be applied to the container for this kind of
    /// specimen.
    pub preparation: super::markdown::Markdown,
    /// The type of container used to contain this kind of specimen.
    pub r#type: super::codeable_concept::CodeableConcept,
}

/// A kind of specimen with associated set of requirements.
#[derive(Debug, Clone, PartialEq)]
pub struct SpecimenDefinitionHandling {
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// Additional textual instructions for the preservation or transport of the
    /// specimen. For instance, 'Protect from light exposure'.
    pub instruction: super::markdown::Markdown,
    /// The maximum time interval of preservation of the specimen with these conditions.
    pub max_duration: super::duration::Duration,
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
    /// It qualifies the interval of temperature, which characterizes an occurrence of
    /// handling. Conditions that are not related to temperature may be handled in the
    /// instruction element.
    pub temperature_qualifier: super::codeable_concept::CodeableConcept,
    /// The temperature interval for this set of handling instructions.
    pub temperature_range: super::range::Range,
}

/// A kind of specimen with associated set of requirements.
#[derive(Debug, Clone, PartialEq)]
pub struct SpecimenDefinitionTypeTested {
    /// The specimen's container.
    pub container: super::specimen_definition::SpecimenDefinitionContainer,
    /// Set of instructions for preservation/transport of the specimen at a defined
    /// temperature interval, prior the testing process.
    pub handling: Vec<super::specimen_definition::SpecimenDefinitionHandling>,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// Primary of secondary specimen.
    pub is_derived: super::boolean::Boolean,
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
    /// The preference for this type of conditioned specimen.
    pub preference: super::code::Code,
    /// Criterion for rejection of the specimen in its container by the laboratory.
    pub rejection_criterion: Vec<super::codeable_concept::CodeableConcept>,
    /// Requirements for delivery and special handling of this kind of conditioned
    /// specimen.
    pub requirement: super::markdown::Markdown,
    /// The usual time that a specimen of this kind is retained after the ordered tests
    /// are completed, for the purpose of additional testing.
    pub retention_time: super::duration::Duration,
    /// Specimen can be used by only one test or panel if the value is "true".
    pub single_use: super::boolean::Boolean,
    /// Where the specimen will be tested: e.g., lab, sector, device or any combination
    /// of these.
    pub testing_destination: Vec<super::codeable_concept::CodeableConcept>,
    /// The kind of specimen conditioned for testing expected by lab.
    pub r#type: super::codeable_concept::CodeableConcept,
}

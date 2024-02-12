/// A single issue - either an indication, contraindication, interaction or an
/// undesirable effect for a medicinal product, medication, device or procedure.
#[derive(Debug, Clone, PartialEq)]
pub struct ClinicalUseDefinition {
    /// A categorisation of the issue, primarily for dividing warnings into subject
    /// heading areas such as "Pregnancy and Lactation", "Overdose", "Effects on Ability
    /// to Drive and Use Machines".
    pub category: Vec<super::codeable_concept::CodeableConcept>,
    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, nor can they have
    /// their own independent transaction scope. This is allowed to be a Parameters
    /// resource if and only if it is referenced by a resource that provides context/
    /// meaning.
    pub contained: Vec<super::resource_list::ResourceList>,
    /// Specifics for when this is a contraindication.
    pub contraindication: super::clinical_use_definition::ClinicalUseDefinitionContraindication,
    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub id: super::id::Id,
    /// Business identifier for this issue.
    pub identifier: Vec<super::identifier::Identifier>,
    /// A reference to a set of rules that were followed when the resource was
    /// constructed, and which must be understood when processing the content. Often,
    /// this is a reference to an implementation guide that defines the special rules
    /// along with other profiles etc.
    pub implicit_rules: super::uri::Uri,
    /// Specifics for when this is an indication.
    pub indication: super::clinical_use_definition::ClinicalUseDefinitionIndication,
    /// Specifics for when this is an interaction.
    pub interaction: super::clinical_use_definition::ClinicalUseDefinitionInteraction,
    /// The base language in which the resource is written.
    pub language: super::code::Code,
    /// Logic used by the clinical use definition.
    pub library: Vec<super::canonical::Canonical>,
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
    /// The population group to which this applies.
    pub population: Vec<super::reference::Reference>,
    /// This is a ClinicalUseDefinition resource
    pub resource_type: String,
    /// Whether this is a current issue or one that has been retired etc.
    pub status: super::codeable_concept::CodeableConcept,
    /// The medication, product, substance, device, procedure etc. for which this is
    /// an indication.
    pub subject: Vec<super::reference::Reference>,
    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need
    /// not encode all the structured data, but is required to contain sufficient detail
    /// to make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub text: super::narrative::Narrative,
    /// indication | contraindication | interaction | undesirable-effect | warning.
    pub r#type: super::code::Code,
    /// Describe the possible undesirable effects (negative outcomes) from the use of
    /// the medicinal product as treatment.
    pub undesirable_effect: super::clinical_use_definition::ClinicalUseDefinitionUndesirableEffect,
    /// A critical piece of information about environmental, health or physical risks
    /// or hazards that serve as caution to the user. For example 'Do not operate heavy
    /// machinery', 'May cause drowsiness', or 'Get medical advice/attention if you
    /// feel unwell'.
    pub warning: super::clinical_use_definition::ClinicalUseDefinitionWarning,
}

/// A single issue - either an indication, contraindication, interaction or an
/// undesirable effect for a medicinal product, medication, device or procedure.
#[derive(Debug, Clone, PartialEq)]
pub struct ClinicalUseDefinitionContraindication {
    /// An expression that returns true or false, indicating whether the indication is
    /// applicable or not, after having applied its other elements.
    pub applicability: super::expression::Expression,
    /// A comorbidity (concurrent condition) or coinfection.
    pub comorbidity: Vec<super::codeable_reference::CodeableReference>,
    /// The status of the disease or symptom for the contraindication, for example
    /// "chronic" or "metastatic".
    pub disease_status: super::codeable_reference::CodeableReference,
    /// The situation that is being documented as contraindicating against this item.
    pub disease_symptom_procedure: super::codeable_reference::CodeableReference,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// The indication which this is a contraidication for.
    pub indication: Vec<super::reference::Reference>,
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
    /// Information about the use of the medicinal product in relation to other
    /// therapies described as part of the contraindication.
    pub other_therapy: Vec<super::clinical_use_definition::ClinicalUseDefinitionOtherTherapy>,
}

/// A single issue - either an indication, contraindication, interaction or an
/// undesirable effect for a medicinal product, medication, device or procedure.
#[derive(Debug, Clone, PartialEq)]
pub struct ClinicalUseDefinitionIndication {
    /// An expression that returns true or false, indicating whether the indication is
    /// applicable or not, after having applied its other elements.
    pub applicability: super::expression::Expression,
    /// A comorbidity (concurrent condition) or coinfection as part of the indication.
    pub comorbidity: Vec<super::codeable_reference::CodeableReference>,
    /// The status of the disease or symptom for the indication, for example "chronic"
    /// or "metastatic".
    pub disease_status: super::codeable_reference::CodeableReference,
    /// The situation that is being documented as an indicaton for this item.
    pub disease_symptom_procedure: super::codeable_reference::CodeableReference,
    /// Timing or duration information, that may be associated with use with the
    /// indicated condition e.g. Adult patients suffering from myocardial infarction
    /// (from a few days until less than 35 days), ischaemic stroke (from 7 days until
    /// less than 6 months).
    pub duration_range: super::range::Range,
    /// Timing or duration information, that may be associated with use with the
    /// indicated condition e.g. Adult patients suffering from myocardial infarction
    /// (from a few days until less than 35 days), ischaemic stroke (from 7 days until
    /// less than 6 months).
    pub duration_string: String,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// The intended effect, aim or strategy to be achieved.
    pub intended_effect: super::codeable_reference::CodeableReference,
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
    /// Information about the use of the medicinal product in relation to other
    /// therapies described as part of the indication.
    pub other_therapy: Vec<super::clinical_use_definition::ClinicalUseDefinitionOtherTherapy>,
    /// An unwanted side effect or negative outcome that may happen if you use the drug
    /// (or other subject of this resource) for this indication.
    pub undesirable_effect: Vec<super::reference::Reference>,
}

/// A single issue - either an indication, contraindication, interaction or an
/// undesirable effect for a medicinal product, medication, device or procedure.
#[derive(Debug, Clone, PartialEq)]
pub struct ClinicalUseDefinitionInteractant {
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// The specific medication, product, food, substance etc. or laboratory test that
    /// interacts.
    pub item_codeable_concept: super::codeable_concept::CodeableConcept,
    /// The specific medication, product, food, substance etc. or laboratory test that
    /// interacts.
    pub item_reference: super::reference::Reference,
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

/// A single issue - either an indication, contraindication, interaction or an
/// undesirable effect for a medicinal product, medication, device or procedure.
#[derive(Debug, Clone, PartialEq)]
pub struct ClinicalUseDefinitionInteraction {
    /// The effect of the interaction, for example "reduced gastric absorption of
    /// primary medication".
    pub effect: super::codeable_reference::CodeableReference,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// The incidence of the interaction, e.g. theoretical, observed.
    pub incidence: super::codeable_concept::CodeableConcept,
    /// The specific medication, product, food, substance etc. or laboratory test that
    /// interacts.
    pub interactant: Vec<super::clinical_use_definition::ClinicalUseDefinitionInteractant>,
    /// Actions for managing the interaction.
    pub management: Vec<super::codeable_concept::CodeableConcept>,
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
    /// The type of the interaction e.g. drug-drug interaction, drug-food interaction,
    /// drug-lab test interaction.
    pub r#type: super::codeable_concept::CodeableConcept,
}

/// A single issue - either an indication, contraindication, interaction or an
/// undesirable effect for a medicinal product, medication, device or procedure.
#[derive(Debug, Clone, PartialEq)]
pub struct ClinicalUseDefinitionOtherTherapy {
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
    /// The type of relationship between the medicinal product indication or
    /// contraindication and another therapy.
    pub relationship_type: super::codeable_concept::CodeableConcept,
    /// Reference to a specific medication (active substance, medicinal product
    /// or class of products, biological, food etc.) as part of an indication or
    /// contraindication.
    pub treatment: super::codeable_reference::CodeableReference,
}

/// A single issue - either an indication, contraindication, interaction or an
/// undesirable effect for a medicinal product, medication, device or procedure.
#[derive(Debug, Clone, PartialEq)]
pub struct ClinicalUseDefinitionUndesirableEffect {
    /// High level classification of the effect.
    pub classification: super::codeable_concept::CodeableConcept,
    /// How often the effect is seen.
    pub frequency_of_occurrence: super::codeable_concept::CodeableConcept,
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
    /// The situation in which the undesirable effect may manifest.
    pub symptom_condition_effect: super::codeable_reference::CodeableReference,
}

/// A single issue - either an indication, contraindication, interaction or an
/// undesirable effect for a medicinal product, medication, device or procedure.
#[derive(Debug, Clone, PartialEq)]
pub struct ClinicalUseDefinitionWarning {
    /// A coded or unformatted textual definition of this warning.
    pub code: super::codeable_concept::CodeableConcept,
    /// A textual definition of this warning, with formatting.
    pub description: super::markdown::Markdown,
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

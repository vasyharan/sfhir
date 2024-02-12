/// Information about a medication that is used to support knowledge.
#[derive(Debug, Clone, PartialEq)]
pub struct MedicationKnowledge {
    /// The date on which the resource content was approved by the publisher. Approval
    /// happens once when the content is officially approved for usage.
    pub approval_date: super::date::Date,
    /// Links to associated medications that could be prescribed, dispensed or
    /// administered.
    pub associated_medication: Vec<super::reference::Reference>,
    /// The creator or owner of the knowledge or information about the medication.
    pub author: super::reference::Reference,
    /// Potential clinical issue with or between medication(s) (for example, drug-drug
    /// interaction, drug-disease contraindication, drug-allergy interaction, etc.).
    pub clinical_use_issue: Vec<super::reference::Reference>,
    /// A code that specifies this medication, or a textual description if no code is
    /// available. Usage note: This could be a standard medication code such as a code
    /// from RxNorm, SNOMED CT, IDMP etc. It could also be a national or local formulary
    /// code, optionally with translations to other code systems.
    pub code: super::codeable_concept::CodeableConcept,
    /// Contact details to assist a user in finding and communicating with the
    /// publisher.
    pub contact: Vec<super::contact_detail::ContactDetail>,
    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, nor can they have
    /// their own independent transaction scope. This is allowed to be a Parameters
    /// resource if and only if it is referenced by a resource that provides context/
    /// meaning.
    pub contained: Vec<super::resource_list::ResourceList>,
    /// A copyright statement relating to the {{title}} and/or its contents. Copyright
    /// statements are generally legal restrictions on the use and publishing of the
    /// {{title}}.
    pub copyright: super::markdown::Markdown,
    /// A short string (<50 characters), suitable for inclusion in a page footer that
    /// identifies the copyright holder, effective period, and optionally whether rights
    /// are resctricted. (e.g. 'All rights reserved', 'Some rights reserved').
    pub copyright_label: super::string::String,
    /// The price of the medication.
    pub cost: Vec<super::medication_knowledge::MedicationKnowledgeCost>,
    /// The date (and optionally time) when the {{title}} was last significantly
    /// changed. The date must change when the business version changes and it must
    /// change if the status code changes. In addition, it should change when the
    /// substantive content of the {{title}} changes.
    pub date: super::date_time::DateTime,
    /// Along with the link to a Medicinal Product Definition resource, this information
    /// provides common definitional elements that are needed to understand the specific
    /// medication that is being described.
    pub definitional: super::medication_knowledge::MedicationKnowledgeDefinitional,
    /// A free text natural language description of the {{title}} from a consumer's
    /// perspective.
    pub description: super::markdown::Markdown,
    /// An individual or organization primarily responsible for internal coherence of
    /// the {{title}}.
    pub editor: Vec<super::contact_detail::ContactDetail>,
    /// The period during which the {{title}} content was or is planned to be in active
    /// use.
    pub effective_period: super::period::Period,
    /// An individual or organization asserted by the publisher to be responsible for
    /// officially endorsing the {{title}} for use in some setting.
    pub endorser: Vec<super::contact_detail::ContactDetail>,
    /// A Boolean value to indicate that this {{title}} is authored for testing purposes
    /// (or education/evaluation/marketing) and is not intended for genuine usage.
    pub experimental: super::boolean::Boolean,
    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub id: super::id::Id,
    /// Business identifier for this medication.
    pub identifier: Vec<super::identifier::Identifier>,
    /// A reference to a set of rules that were followed when the resource was
    /// constructed, and which must be understood when processing the content. Often,
    /// this is a reference to an implementation guide that defines the special rules
    /// along with other profiles etc.
    pub implicit_rules: super::uri::Uri,
    /// Guidelines or protocols that are applicable for the administration of the
    /// medication based on indication.
    pub indication_guideline:
        Vec<super::medication_knowledge::MedicationKnowledgeIndicationGuideline>,
    /// Lists the jurisdictions that this medication knowledge was written for.
    pub intended_jurisdiction: Vec<super::codeable_concept::CodeableConcept>,
    /// A legal or geographic region in which the {{title}} is intended to be used.
    pub jurisdiction: Vec<super::codeable_concept::CodeableConcept>,
    /// The base language in which the resource is written.
    pub language: super::code::Code,
    /// The date on which the resource content was last reviewed. Review happens
    /// periodically after approval but does not change the original approval date.
    pub last_review_date: super::date::Date,
    /// Categorization of the medication within a formulary or classification system.
    pub medicine_classification:
        Vec<super::medication_knowledge::MedicationKnowledgeMedicineClassification>,
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
    /// The program under which the medication is reviewed.
    pub monitoring_program: Vec<super::medication_knowledge::MedicationKnowledgeMonitoringProgram>,
    /// Associated documentation about the medication.
    pub monograph: Vec<super::medication_knowledge::MedicationKnowledgeMonograph>,
    /// All of the names for a medication, for example, the name(s) given to a
    /// medication in different countries.  For example, acetaminophen and paracetamol
    /// or salbutamol and albuterol.
    pub name: Vec<super::string::String>,
    /// Information that only applies to packages (not products).
    pub packaging: Vec<super::medication_knowledge::MedicationKnowledgePackaging>,
    /// The instructions for preparing the medication.
    pub preparation_instruction: super::markdown::Markdown,
    /// Category of the medication or product (e.g. branded product, therapeutic moeity,
    /// generic product, innovator product, etc.).
    pub product_type: Vec<super::codeable_concept::CodeableConcept>,
    /// The name of the organization or individual responsible for the release and
    /// ongoing maintenance of the {{title}}.
    pub publisher: super::string::String,
    /// Explanation of why this {{title}} is needed and why it has been designed as
    /// it has.
    pub purpose: super::markdown::Markdown,
    /// Regulatory information about a medication.
    pub regulatory: Vec<super::medication_knowledge::MedicationKnowledgeRegulatory>,
    /// Related artifacts such as additional documentation, justification, dependencies,
    /// bibliographic references, and predecessor and successor artifacts.
    pub related_artifact: Vec<super::related_artifact::RelatedArtifact>,
    /// Associated or related medications. For example, if the medication is a branded
    /// product (e.g. Crestor), this is the Therapeutic Moeity (e.g. Rosuvastatin) or if
    /// this is a generic medication (e.g. Rosuvastatin), this would link to a branded
    /// product (e.g. Crestor.
    pub related_medication_knowledge:
        Vec<super::medication_knowledge::MedicationKnowledgeRelatedMedicationKnowledge>,
    /// This is a MedicationKnowledge resource
    pub resource_type: String,
    /// An individual or organization asserted by the publisher to be primarily
    /// responsible for review of some aspect of the {{title}}.
    pub reviewer: Vec<super::contact_detail::ContactDetail>,
    /// A code to indicate if the medication referred to by this MedicationKnowledge is
    /// in active use within the drug database or inventory system. The status refers
    /// to the validity about the information of the medication and not to its medicinal
    /// properties.
    pub status: super::code::Code,
    /// Information on how the medication should be stored, for example, refrigeration
    /// temperatures and length of stability at a given temperature.
    pub storage_guideline: Vec<super::medication_knowledge::MedicationKnowledgeStorageGuideline>,
    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need
    /// not encode all the structured data, but is required to contain sufficient detail
    /// to make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub text: super::narrative::Narrative,
    /// A short, descriptive, user-friendly title for the {{title}}.
    pub title: super::string::String,
    /// Descriptive topics related to the content of the {{title}}. Topics provide a
    /// high-level categorization as well as keywords for the {{title}} that can be
    /// useful for filtering and searching.
    pub topic: Vec<super::codeable_concept::CodeableConcept>,
    /// An absolute URI that is used to identify this {{title}} when it is referenced
    /// in a specification, model, design or an instance; also called its canonical
    /// identifier. This SHOULD be globally unique and SHOULD be a literal address at
    /// which an authoritative instance of this {{title}} is (or will be) published.
    /// This URL can be the target of a canonical reference. It SHALL remain the same
    /// when the {{title}} is stored on different servers.
    pub url: super::uri::Uri,
    /// The content was developed with a focus and intent of supporting the contexts
    /// that are listed. These contexts may be general categories (gender, age, ...) or
    /// may be references to specific programs (insurance plans, studies, ...) and may
    /// be used to assist with indexing and searching for appropriate {{title}}s.
    pub use_context: Vec<super::usage_context::UsageContext>,
    /// The identifier that is used to identify this version of the {{title}} when
    /// it is referenced in a specification, model, design or instance. This is an
    /// arbitrary value managed by the {{title}} author and is not expected to be
    /// globally unique. For example, it might be a timestamp (e.g. yyyymmdd) if a
    /// managed version is not available. There is also no expectation that versions can
    /// be placed in a lexicographical sequence without additional knowledge.  (See the
    /// versionAlgorithm element.)
    pub version: super::string::String,
    /// Indicates the mechanism used to compare versions to determine which is more
    /// current.
    pub version_algorithm_coding: super::coding::Coding,
    /// Indicates the mechanism used to compare versions to determine which is more
    /// current.
    pub version_algorithm_string: String,
}

/// Information about a medication that is used to support knowledge.
#[derive(Debug, Clone, PartialEq)]
pub struct MedicationKnowledgeCost {
    /// The price or representation of the cost (for example, Band A, Band B or $, $$)
    /// of the medication.
    pub cost_codeable_concept: super::codeable_concept::CodeableConcept,
    /// The price or representation of the cost (for example, Band A, Band B or $, $$)
    /// of the medication.
    pub cost_money: super::money::Money,
    /// The date range for which the cost information of the medication is effective.
    pub effective_date: Vec<super::period::Period>,
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
    /// The source or owner that assigns the price to the medication.
    pub source: super::string::String,
    /// The category of the cost information.  For example, manufacturers' cost, patient
    /// cost, claim reimbursement cost, actual acquisition cost.
    pub r#type: super::codeable_concept::CodeableConcept,
}

/// Information about a medication that is used to support knowledge.
#[derive(Debug, Clone, PartialEq)]
pub struct MedicationKnowledgeDefinitional {
    /// Associated definitions for this medication.
    pub definition: Vec<super::reference::Reference>,
    /// Describes the form of the item.  Powder; tablets; capsule.
    pub dose_form: super::codeable_concept::CodeableConcept,
    /// Specifies descriptive properties of the medicine, such as color, shape,
    /// imprints, etc.
    pub drug_characteristic:
        Vec<super::medication_knowledge::MedicationKnowledgeDrugCharacteristic>,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// Identifies a particular constituent of interest in the product.
    pub ingredient: Vec<super::medication_knowledge::MedicationKnowledgeIngredient>,
    /// The intended or approved route of administration.
    pub intended_route: Vec<super::codeable_concept::CodeableConcept>,
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

/// Information about a medication that is used to support knowledge.
#[derive(Debug, Clone, PartialEq)]
pub struct MedicationKnowledgeDosage {
    /// Dosage for the medication for the specific guidelines.
    pub dosage: Vec<super::dosage::Dosage>,
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
    /// The type or category of dosage for a given medication (for example, prophylaxis,
    /// maintenance, therapeutic, etc.).
    pub r#type: super::codeable_concept::CodeableConcept,
}

/// Information about a medication that is used to support knowledge.
#[derive(Debug, Clone, PartialEq)]
pub struct MedicationKnowledgeDosingGuideline {
    /// The type of the treatment that the guideline applies to, for example, long term
    /// therapy, first line treatment, etc.
    pub administration_treatment: super::codeable_concept::CodeableConcept,
    /// Dosage for the medication for the specific guidelines.
    pub dosage: Vec<super::medication_knowledge::MedicationKnowledgeDosage>,
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
    /// Characteristics of the patient that are relevant to the administration
    /// guidelines (for example, height, weight, gender, etc.).
    pub patient_characteristic:
        Vec<super::medication_knowledge::MedicationKnowledgePatientCharacteristic>,
    /// The overall intention of the treatment, for example, prophylactic, supporative,
    /// curative, etc.
    pub treatment_intent: super::codeable_concept::CodeableConcept,
}

/// Information about a medication that is used to support knowledge.
#[derive(Debug, Clone, PartialEq)]
pub struct MedicationKnowledgeDrugCharacteristic {
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
    /// A code specifying which characteristic of the medicine is being described (for
    /// example, colour, shape, imprint).
    pub r#type: super::codeable_concept::CodeableConcept,
    /// Description of the characteristic.
    pub value_attachment: super::attachment::Attachment,
    /// Description of the characteristic.
    pub value_base_64_binary: String,
    /// Description of the characteristic.
    pub value_codeable_concept: super::codeable_concept::CodeableConcept,
    /// Description of the characteristic.
    pub value_quantity: super::quantity::Quantity,
    /// Description of the characteristic.
    pub value_string: String,
}

/// Information about a medication that is used to support knowledge.
#[derive(Debug, Clone, PartialEq)]
pub struct MedicationKnowledgeEnvironmentalSetting {
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
    /// Identifies the category or type of setting (e.g., type of location, temperature,
    /// humidity).
    pub r#type: super::codeable_concept::CodeableConcept,
    /// Value associated to the setting. E.g., 40° – 50°F for temperature.
    pub value_codeable_concept: super::codeable_concept::CodeableConcept,
    /// Value associated to the setting. E.g., 40° – 50°F for temperature.
    pub value_quantity: super::quantity::Quantity,
    /// Value associated to the setting. E.g., 40° – 50°F for temperature.
    pub value_range: super::range::Range,
}

/// Information about a medication that is used to support knowledge.
#[derive(Debug, Clone, PartialEq)]
pub struct MedicationKnowledgeIndicationGuideline {
    /// The guidelines for the dosage of the medication for the indication.
    pub dosing_guideline: Vec<super::medication_knowledge::MedicationKnowledgeDosingGuideline>,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// Indication or reason for use of the medication that applies to the specific
    /// administration guideline.
    pub indication: Vec<super::codeable_reference::CodeableReference>,
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

/// Information about a medication that is used to support knowledge.
#[derive(Debug, Clone, PartialEq)]
pub struct MedicationKnowledgeIngredient {
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// A reference to the resource that provides information about the ingredient.
    pub item: super::codeable_reference::CodeableReference,
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
    /// Specifies how many (or how much) of the items there are in this Medication.  For
    /// example, 250 mg per tablet.  This is expressed as a ratio where the numerator is
    /// 250mg and the denominator is 1 tablet but can also be expressed a quantity when
    /// the denominator is assumed to be 1 tablet.
    pub strength_codeable_concept: super::codeable_concept::CodeableConcept,
    /// Specifies how many (or how much) of the items there are in this Medication.  For
    /// example, 250 mg per tablet.  This is expressed as a ratio where the numerator is
    /// 250mg and the denominator is 1 tablet but can also be expressed a quantity when
    /// the denominator is assumed to be 1 tablet.
    pub strength_quantity: super::quantity::Quantity,
    /// Specifies how many (or how much) of the items there are in this Medication.  For
    /// example, 250 mg per tablet.  This is expressed as a ratio where the numerator is
    /// 250mg and the denominator is 1 tablet but can also be expressed a quantity when
    /// the denominator is assumed to be 1 tablet.
    pub strength_ratio: super::ratio::Ratio,
    /// Indication of whether this ingredient affects the therapeutic action of the
    /// drug.
    pub r#type: super::codeable_concept::CodeableConcept,
}

/// Information about a medication that is used to support knowledge.
#[derive(Debug, Clone, PartialEq)]
pub struct MedicationKnowledgeMaxDispense {
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
    /// The period that applies to the maximum number of units.
    pub period: super::duration::Duration,
    /// The maximum number of units of the medication that can be dispensed.
    pub quantity: super::quantity::Quantity,
}

/// Information about a medication that is used to support knowledge.
#[derive(Debug, Clone, PartialEq)]
pub struct MedicationKnowledgeMedicineClassification {
    /// Specific category assigned to the medication (e.g. anti-infective, anti-
    /// hypertensive, antibiotic, etc.).
    pub classification: Vec<super::codeable_concept::CodeableConcept>,
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
    /// Either a textual source of the classification or a reference to an online
    /// source.
    pub source_string: String,
    /// Either a textual source of the classification or a reference to an online
    /// source.
    pub source_uri: String,
    /// The type of category for the medication (for example, therapeutic
    /// classification, therapeutic sub-classification).
    pub r#type: super::codeable_concept::CodeableConcept,
}

/// Information about a medication that is used to support knowledge.
#[derive(Debug, Clone, PartialEq)]
pub struct MedicationKnowledgeMonitoringProgram {
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
    /// Name of the reviewing program.
    pub name: super::string::String,
    /// Type of program under which the medication is monitored.
    pub r#type: super::codeable_concept::CodeableConcept,
}

/// Information about a medication that is used to support knowledge.
#[derive(Debug, Clone, PartialEq)]
pub struct MedicationKnowledgeMonograph {
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
    /// Associated documentation about the medication.
    pub source: super::reference::Reference,
    /// The category of documentation about the medication. (e.g. professional
    /// monograph, patient education monograph).
    pub r#type: super::codeable_concept::CodeableConcept,
}

/// Information about a medication that is used to support knowledge.
#[derive(Debug, Clone, PartialEq)]
pub struct MedicationKnowledgePackaging {
    /// The cost of the packaged medication.
    pub cost: Vec<super::medication_knowledge::MedicationKnowledgeCost>,
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
    /// A reference to a PackagedProductDefinition that provides the details of the
    /// product that is in the packaging and is being priced.
    pub packaged_product: super::reference::Reference,
}

/// Information about a medication that is used to support knowledge.
#[derive(Debug, Clone, PartialEq)]
pub struct MedicationKnowledgePatientCharacteristic {
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
    /// The categorization of the specific characteristic that is relevant to the
    /// administration guideline (e.g. height, weight, gender).
    pub r#type: super::codeable_concept::CodeableConcept,
    /// The specific characteristic (e.g. height, weight, gender, etc.).
    pub value_codeable_concept: super::codeable_concept::CodeableConcept,
    /// The specific characteristic (e.g. height, weight, gender, etc.).
    pub value_quantity: super::quantity::Quantity,
    /// The specific characteristic (e.g. height, weight, gender, etc.).
    pub value_range: super::range::Range,
}

/// Information about a medication that is used to support knowledge.
#[derive(Debug, Clone, PartialEq)]
pub struct MedicationKnowledgeRegulatory {
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// The maximum number of units of the medication that can be dispensed in a period.
    pub max_dispense: super::medication_knowledge::MedicationKnowledgeMaxDispense,
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
    /// The authority that is specifying the regulations.
    pub regulatory_authority: super::reference::Reference,
    /// Specifies the schedule of a medication in jurisdiction.
    pub schedule: Vec<super::codeable_concept::CodeableConcept>,
    /// Specifies if changes are allowed when dispensing a medication from a regulatory
    /// perspective.
    pub substitution: Vec<super::medication_knowledge::MedicationKnowledgeSubstitution>,
}

/// Information about a medication that is used to support knowledge.
#[derive(Debug, Clone, PartialEq)]
pub struct MedicationKnowledgeRelatedMedicationKnowledge {
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
    /// Associated documentation about the associated medication knowledge.
    pub reference: Vec<super::reference::Reference>,
    /// The category of the associated medication knowledge reference.
    pub r#type: super::codeable_concept::CodeableConcept,
}

/// Information about a medication that is used to support knowledge.
#[derive(Debug, Clone, PartialEq)]
pub struct MedicationKnowledgeStorageGuideline {
    /// Describes a setting/value on the environment for the adequate storage of the
    /// medication and other substances.  Environment settings may involve temperature,
    /// humidity, or exposure to light.
    pub environmental_setting:
        Vec<super::medication_knowledge::MedicationKnowledgeEnvironmentalSetting>,
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
    /// Additional notes about the storage.
    pub note: Vec<super::annotation::Annotation>,
    /// Reference to additional information about the storage guidelines.
    pub reference: super::uri::Uri,
    /// Duration that the medication remains stable if the environmentalSetting is
    /// respected.
    pub stability_duration: super::duration::Duration,
}

/// Information about a medication that is used to support knowledge.
#[derive(Debug, Clone, PartialEq)]
pub struct MedicationKnowledgeSubstitution {
    /// Specifies if regulation allows for changes in the medication when dispensing.
    pub allowed: super::boolean::Boolean,
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
    /// Specifies the type of substitution allowed.
    pub r#type: super::codeable_concept::CodeableConcept,
}

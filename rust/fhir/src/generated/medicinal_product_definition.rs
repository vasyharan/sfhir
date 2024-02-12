/// A medicinal product, being a substance or combination of substances that is
/// intended to treat, prevent or diagnose a disease, or to restore, correct or
/// modify physiological functions by exerting a pharmacological, immunological or
/// metabolic action. This resource is intended to define and detail such products
/// and their properties, for uses other than direct patient care (e.g. regulatory
/// use, or drug catalogs).
#[derive(Debug, Clone, PartialEq)]
pub struct MedicinalProductDefinition {
    /// Whether the Medicinal Product is subject to additional monitoring for regulatory
    /// reasons, such as heightened reporting requirements.
    pub additional_monitoring_indicator: super::codeable_concept::CodeableConcept,
    /// Additional information or supporting documentation about the medicinal product.
    pub attached_document: Vec<super::reference::Reference>,
    /// Allows the key product features to be recorded, such as "sugar free", "modified
    /// release", "parallel import".
    pub characteristic:
        Vec<super::medicinal_product_definition::MedicinalProductDefinitionCharacteristic>,
    /// Allows the product to be classified by various systems, commonly WHO ATC.
    pub classification: Vec<super::codeable_concept::CodeableConcept>,
    /// Clinical trials or studies that this product is involved in.
    pub clinical_trial: Vec<super::reference::Reference>,
    /// A code that this product is known by, usually within some formal terminology,
    /// perhaps assigned by a third party (i.e. not the manufacturer or regulator).
    /// Products (types of medications) tend to be known by identifiers during
    /// development and within regulatory process. However when they are prescribed they
    /// tend to be identified by codes. The same product may be have multiple codes,
    /// applied to it by multiple organizations.
    pub code: Vec<super::coding::Coding>,
    /// The dose form for a single part product, or combined form of a multiple part
    /// product. This is one concept that describes all the components. It does not
    /// represent the form with components physically mixed, if that might be necessary,
    /// for which see (AdministrableProductDefinition.administrableDoseForm).
    pub combined_pharmaceutical_dose_form: super::codeable_concept::CodeableConcept,
    /// Types of medicinal manufactured items and/or devices that this
    /// product consists of, such as tablets, capsule, or syringes. Used as a
    /// direct link when the item's packaging is not being recorded (see also
    /// PackagedProductDefinition.package.containedItem.item).
    pub comprised_of: Vec<super::reference::Reference>,
    /// A product specific contact, person (in a role), or an organization.
    pub contact: Vec<super::medicinal_product_definition::MedicinalProductDefinitionContact>,
    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, nor can they have
    /// their own independent transaction scope. This is allowed to be a Parameters
    /// resource if and only if it is referenced by a resource that provides context/
    /// meaning.
    pub contained: Vec<super::resource_list::ResourceList>,
    /// Reference to another product, e.g. for linking authorised to investigational
    /// product, or a virtual product.
    pub cross_reference:
        Vec<super::medicinal_product_definition::MedicinalProductDefinitionCrossReference>,
    /// General description of this product.
    pub description: super::markdown::Markdown,
    /// If this medicine applies to human or veterinary uses.
    pub domain: super::codeable_concept::CodeableConcept,
    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub id: super::id::Id,
    /// Business identifier for this product. Could be an MPID. When in development
    /// or being regulated, products are typically referenced by official identifiers,
    /// assigned by a manufacturer or regulator, and unique to a product (which, when
    /// compared to a product instance being prescribed, is actually a product type).
    /// See also MedicinalProductDefinition.code.
    pub identifier: Vec<super::identifier::Identifier>,
    /// A reference to a set of rules that were followed when the resource was
    /// constructed, and which must be understood when processing the content. Often,
    /// this is a reference to an implementation guide that defines the special rules
    /// along with other profiles etc.
    pub implicit_rules: super::uri::Uri,
    /// Any component of the drug product which is not the chemical entity defined as
    /// the drug substance, or an excipient in the drug product. This includes process-
    /// related impurities and contaminants, product-related impurities including
    /// degradation products.
    pub impurity: Vec<super::codeable_reference::CodeableReference>,
    /// Description of indication(s) for this product, used when structured indications
    /// are not required. In cases where structured indications are required, they are
    /// captured using the ClinicalUseDefinition resource. An indication is a medical
    /// situation for which using the product is appropriate.
    pub indication: super::markdown::Markdown,
    /// The ingredients of this medicinal product - when not detailed in other
    /// resources. This is only needed if the ingredients are not specified
    /// by incoming references from the Ingredient resource, or indirectly via
    /// incoming AdministrableProductDefinition, PackagedProductDefinition or
    /// ManufacturedItemDefinition references. In cases where those levels of detail are
    /// not used, the ingredients may be specified directly here as codes.
    pub ingredient: Vec<super::codeable_concept::CodeableConcept>,
    /// The base language in which the resource is written.
    pub language: super::code::Code,
    /// The legal status of supply of the medicinal product as classified by the
    /// regulator.
    pub legal_status_of_supply: super::codeable_concept::CodeableConcept,
    /// Marketing status of the medicinal product, in contrast to marketing
    /// authorization. This refers to the product being actually 'on the market' as
    /// opposed to being allowed to be on the market (which is an authorization).
    pub marketing_status: Vec<super::marketing_status::MarketingStatus>,
    /// A master file for the medicinal product (e.g. Pharmacovigilance System Master
    /// File). Drug master files (DMFs) are documents submitted to regulatory agencies
    /// to provide confidential detailed information about facilities, processes or
    /// articles used in the manufacturing, processing, packaging and storing of drug
    /// products.
    pub master_file: Vec<super::reference::Reference>,
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
    /// The product's name, including full name and possibly coded parts.
    pub name: Vec<super::medicinal_product_definition::MedicinalProductDefinitionName>,
    /// A manufacturing or administrative process or step associated with (or performed
    /// on) the medicinal product.
    pub operation: Vec<super::medicinal_product_definition::MedicinalProductDefinitionOperation>,
    /// Package type for the product. See also the PackagedProductDefinition resource.
    pub packaged_medicinal_product: Vec<super::codeable_concept::CodeableConcept>,
    /// If authorised for use in children, or infants, neonates etc.
    pub pediatric_use_indicator: super::codeable_concept::CodeableConcept,
    /// This is a MedicinalProductDefinition resource
    pub resource_type: String,
    /// The path by which the product is taken into or makes contact with the body. In
    /// some regions this is referred to as the licenced or approved route. See also
    /// AdministrableProductDefinition resource. MedicinalProductDefinition.route is the
    /// same concept as AdministrableProductDefinition.routeOfAdministration.code, and
    /// they cannot be used together.
    pub route: Vec<super::codeable_concept::CodeableConcept>,
    /// Whether the Medicinal Product is subject to special measures for regulatory
    /// reasons, such as a requirement to conduct post-authorization studies.
    pub special_measures: Vec<super::codeable_concept::CodeableConcept>,
    /// The status within the lifecycle of this product record. A high-level status,
    /// this is not intended to duplicate details carried elsewhere such as legal
    /// status, or authorization status.
    pub status: super::codeable_concept::CodeableConcept,
    /// The date at which the given status became applicable.
    pub status_date: super::date_time::DateTime,
    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need
    /// not encode all the structured data, but is required to contain sufficient detail
    /// to make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub text: super::narrative::Narrative,
    /// Regulatory type, e.g. Investigational or Authorized.
    pub r#type: super::codeable_concept::CodeableConcept,
    /// A business identifier relating to a specific version of the product, this is
    /// commonly used to support revisions to an existing product.
    pub version: super::string::String,
}

/// A medicinal product, being a substance or combination of substances that is
/// intended to treat, prevent or diagnose a disease, or to restore, correct or
/// modify physiological functions by exerting a pharmacological, immunological or
/// metabolic action. This resource is intended to define and detail such products
/// and their properties, for uses other than direct patient care (e.g. regulatory
/// use, or drug catalogs).
#[derive(Debug, Clone, PartialEq)]
pub struct MedicinalProductDefinitionCharacteristic {
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
    /// A code expressing the type of characteristic.
    pub r#type: super::codeable_concept::CodeableConcept,
    /// A value for the characteristic.text.
    pub value_attachment: super::attachment::Attachment,
    /// A value for the characteristic.text.
    pub value_boolean: bool,
    /// A value for the characteristic.text.
    pub value_codeable_concept: super::codeable_concept::CodeableConcept,
    /// A value for the characteristic.text.
    pub value_date: String,
    /// A value for the characteristic.text.
    pub value_integer: i64,
    /// A value for the characteristic.text.
    pub value_markdown: String,
    /// A value for the characteristic.text.
    pub value_quantity: super::quantity::Quantity,
}

/// A medicinal product, being a substance or combination of substances that is
/// intended to treat, prevent or diagnose a disease, or to restore, correct or
/// modify physiological functions by exerting a pharmacological, immunological or
/// metabolic action. This resource is intended to define and detail such products
/// and their properties, for uses other than direct patient care (e.g. regulatory
/// use, or drug catalogs).
#[derive(Debug, Clone, PartialEq)]
pub struct MedicinalProductDefinitionContact {
    /// A product specific contact, person (in a role), or an organization.
    pub contact: super::reference::Reference,
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
    /// Allows the contact to be classified, for example QPPV, Pharmacovigilance Enquiry
    /// Information.
    pub r#type: super::codeable_concept::CodeableConcept,
}

/// A medicinal product, being a substance or combination of substances that is
/// intended to treat, prevent or diagnose a disease, or to restore, correct or
/// modify physiological functions by exerting a pharmacological, immunological or
/// metabolic action. This resource is intended to define and detail such products
/// and their properties, for uses other than direct patient care (e.g. regulatory
/// use, or drug catalogs).
#[derive(Debug, Clone, PartialEq)]
pub struct MedicinalProductDefinitionCrossReference {
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
    /// Reference to another product, e.g. for linking authorised to investigational
    /// product.
    pub product: super::codeable_reference::CodeableReference,
    /// The type of relationship, for instance branded to generic, virtual to actual
    /// product, product to development product (investigational), parallel import
    /// version.
    pub r#type: super::codeable_concept::CodeableConcept,
}

/// A medicinal product, being a substance or combination of substances that is
/// intended to treat, prevent or diagnose a disease, or to restore, correct or
/// modify physiological functions by exerting a pharmacological, immunological or
/// metabolic action. This resource is intended to define and detail such products
/// and their properties, for uses other than direct patient care (e.g. regulatory
/// use, or drug catalogs).
#[derive(Debug, Clone, PartialEq)]
pub struct MedicinalProductDefinitionName {
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
    /// Coding words or phrases of the name.
    pub part: Vec<super::medicinal_product_definition::MedicinalProductDefinitionPart>,
    /// The full product name.
    pub product_name: super::string::String,
    /// Type of product name, such as rINN, BAN, Proprietary, Non-Proprietary.
    pub r#type: super::codeable_concept::CodeableConcept,
    /// Country and jurisdiction where the name applies, and associated language.
    pub usage: Vec<super::medicinal_product_definition::MedicinalProductDefinitionUsage>,
}

/// A medicinal product, being a substance or combination of substances that is
/// intended to treat, prevent or diagnose a disease, or to restore, correct or
/// modify physiological functions by exerting a pharmacological, immunological or
/// metabolic action. This resource is intended to define and detail such products
/// and their properties, for uses other than direct patient care (e.g. regulatory
/// use, or drug catalogs).
#[derive(Debug, Clone, PartialEq)]
pub struct MedicinalProductDefinitionOperation {
    /// Specifies whether this particular business or manufacturing process is
    /// considered proprietary or confidential.
    pub confidentiality_indicator: super::codeable_concept::CodeableConcept,
    /// Date range of applicability.
    pub effective_date: super::period::Period,
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
    /// The organization or establishment responsible for (or associated with) the
    /// particular process or step, examples include the manufacturer, importer, agent.
    pub organization: Vec<super::reference::Reference>,
    /// The type of manufacturing operation e.g. manufacturing itself, re-packaging. For
    /// the authorization of this, a RegulatedAuthorization would point to the same plan
    /// or activity referenced here.
    pub r#type: super::codeable_reference::CodeableReference,
}

/// A medicinal product, being a substance or combination of substances that is
/// intended to treat, prevent or diagnose a disease, or to restore, correct or
/// modify physiological functions by exerting a pharmacological, immunological or
/// metabolic action. This resource is intended to define and detail such products
/// and their properties, for uses other than direct patient care (e.g. regulatory
/// use, or drug catalogs).
#[derive(Debug, Clone, PartialEq)]
pub struct MedicinalProductDefinitionPart {
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
    /// A fragment of a product name.
    pub part: super::string::String,
    /// Identifying type for this part of the name (e.g. strength part).
    pub r#type: super::codeable_concept::CodeableConcept,
}

/// A medicinal product, being a substance or combination of substances that is
/// intended to treat, prevent or diagnose a disease, or to restore, correct or
/// modify physiological functions by exerting a pharmacological, immunological or
/// metabolic action. This resource is intended to define and detail such products
/// and their properties, for uses other than direct patient care (e.g. regulatory
/// use, or drug catalogs).
#[derive(Debug, Clone, PartialEq)]
pub struct MedicinalProductDefinitionUsage {
    /// Country code for where this name applies.
    pub country: super::codeable_concept::CodeableConcept,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// Jurisdiction code for where this name applies. A jurisdiction may be a sub- or
    /// supra-national entity (e.g. a state or a geographic region).
    pub jurisdiction: super::codeable_concept::CodeableConcept,
    /// Language code for this name.
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
}

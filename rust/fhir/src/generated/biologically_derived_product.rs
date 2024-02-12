/// This resource reflects an instance of a biologically derived product. A material
/// substance originating from a biological entity intended to be transplanted or
/// infused
/// into another (possibly the same) biological entity.
#[derive(Debug, Clone, PartialEq)]
pub struct BiologicallyDerivedProduct {
    /// An identifier that supports traceability to the event during which material in
    /// this product from one or more biological entities was obtained or pooled.
    pub biological_source_event: super::identifier::Identifier,
    /// How this product was collected.
    pub collection: super::biologically_derived_product::BiologicallyDerivedProductCollection,
    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, nor can they have
    /// their own independent transaction scope. This is allowed to be a Parameters
    /// resource if and only if it is referenced by a resource that provides context/
    /// meaning.
    pub contained: Vec<super::resource_list::ResourceList>,
    /// A unique identifier for an aliquot of a product.  Used to distinguish individual
    /// aliquots of a product carrying the same biologicalSource and productCode
    /// identifiers.
    pub division: super::string::String,
    /// Date, and where relevant time, of expiration.
    pub expiration_date: super::date_time::DateTime,
    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub id: super::id::Id,
    /// Unique instance identifiers assigned to a biologically derived product. Note:
    /// This is a business identifier, not a resource identifier.
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
    /// Parent product (if any) for this biologically-derived product.
    pub parent: Vec<super::reference::Reference>,
    /// Processing facilities responsible for the labeling and distribution of this
    /// biologically derived product.
    pub processing_facility: Vec<super::reference::Reference>,
    /// Broad category of this product.
    pub product_category: super::coding::Coding,
    /// A codified value that systematically supports characterization and
    /// classification of medical products of human origin inclusive of processing
    /// conditions such as additives, volumes and handling conditions.
    pub product_code: super::codeable_concept::CodeableConcept,
    /// Whether the product is currently available.
    pub product_status: super::coding::Coding,
    /// A property that is specific to this BiologicallyDerviedProduct instance.
    pub property: Vec<super::biologically_derived_product::BiologicallyDerivedProductProperty>,
    /// Request to obtain and/or infuse this biologically derived product.
    pub request: Vec<super::reference::Reference>,
    /// This is a BiologicallyDerivedProduct resource
    pub resource_type: String,
    /// The temperature requirements for storage of the biologically-derived product.
    pub storage_temp_requirements: super::range::Range,
    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need
    /// not encode all the structured data, but is required to contain sufficient detail
    /// to make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub text: super::narrative::Narrative,
}

/// This resource reflects an instance of a biologically derived product. A material
/// substance originating from a biological entity intended to be transplanted or
/// infused
/// into another (possibly the same) biological entity.
#[derive(Debug, Clone, PartialEq)]
pub struct BiologicallyDerivedProductCollection {
    /// Time of product collection.
    pub collected_date_time: String,
    /// Time of product collection.
    pub collected_period: super::period::Period,
    /// Healthcare professional who is performing the collection.
    pub collector: super::reference::Reference,
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
    /// The patient or entity, such as a hospital or vendor in the case of a processed/
    /// manipulated/manufactured product, providing the product.
    pub source: super::reference::Reference,
}

/// This resource reflects an instance of a biologically derived product. A material
/// substance originating from a biological entity intended to be transplanted or
/// infused
/// into another (possibly the same) biological entity.
#[derive(Debug, Clone, PartialEq)]
pub struct BiologicallyDerivedProductProperty {
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
    /// Code that specifies the property. It should reference an established coding
    /// system.
    pub r#type: super::codeable_concept::CodeableConcept,
    /// Property values.
    pub value_attachment: super::attachment::Attachment,
    /// Property values.
    pub value_boolean: bool,
    /// Property values.
    pub value_codeable_concept: super::codeable_concept::CodeableConcept,
    /// Property values.
    pub value_integer: i64,
    /// Property values.
    pub value_period: super::period::Period,
    /// Property values.
    pub value_quantity: super::quantity::Quantity,
    /// Property values.
    pub value_range: super::range::Range,
    /// Property values.
    pub value_ratio: super::ratio::Ratio,
    /// Property values.
    pub value_string: String,
}

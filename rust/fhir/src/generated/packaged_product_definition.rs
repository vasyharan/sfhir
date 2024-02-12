/// A medically related item or items, in a container or package.
#[derive(Debug, Clone, PartialEq)]
pub struct PackagedProductDefinition {
    /// Additional information or supporting documentation about the packaged product.
    pub attached_document: Vec<super::reference::Reference>,
    /// Allows the key features to be recorded, such as "hospital pack", "nurse
    /// prescribable", "calendar pack".
    pub characteristic: Vec<super::packaged_product_definition::PackagedProductDefinitionProperty>,
    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, nor can they have
    /// their own independent transaction scope. This is allowed to be a Parameters
    /// resource if and only if it is referenced by a resource that provides context/
    /// meaning.
    pub contained: Vec<super::resource_list::ResourceList>,
    /// A total of the complete count of contained items of a particular type/form,
    /// independent of sub-packaging or organization. This can be considered as the
    /// pack size. This attribute differs from containedItem.amount in that it can give
    /// a single aggregated count of all tablet types in a pack, even when these are
    /// different manufactured items. For example a pill pack of 21 tablets plus 7 sugar
    /// tablets, can be denoted here as '28 tablets'. This attribute is repeatable so
    /// that the different item types in one pack type can be counted (e.g. a count of
    /// vials and count of syringes). Each repeat must have different units, so that it
    /// is clear what the different sets of counted items are, and it is not intended
    /// to allow different counts of similar items (e.g. not '2 tubes and 3 tubes').
    /// Repeats are not to be used to represent different pack sizes (e.g. 20 pack vs.
    /// 50 pack) - which would be different instances of this resource.
    pub contained_item_quantity: Vec<super::quantity::Quantity>,
    /// Identifies if the package contains different items, such as when a drug product
    /// is supplied with another item e.g. a diluent or adjuvant.
    pub copackaged_indicator: super::boolean::Boolean,
    /// Textual description. Note that this is not the name of the package or product.
    pub description: super::markdown::Markdown,
    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub id: super::id::Id,
    /// A unique identifier for this package as whole - not the the content of the
    /// package. Unique instance identifiers assigned to a package by manufacturers,
    /// regulators, drug catalogue custodians or other organizations.
    pub identifier: Vec<super::identifier::Identifier>,
    /// A reference to a set of rules that were followed when the resource was
    /// constructed, and which must be understood when processing the content. Often,
    /// this is a reference to an implementation guide that defines the special rules
    /// along with other profiles etc.
    pub implicit_rules: super::uri::Uri,
    /// The base language in which the resource is written.
    pub language: super::code::Code,
    /// The legal status of supply of the packaged item as classified by the regulator.
    pub legal_status_of_supply:
        Vec<super::packaged_product_definition::PackagedProductDefinitionLegalStatusOfSupply>,
    /// Manufacturer of this package type. When there are multiple it means these are
    /// all possible manufacturers.
    pub manufacturer: Vec<super::reference::Reference>,
    /// Allows specifying that an item is on the market for sale, or that it is not
    /// available, and the dates and locations associated.
    pub marketing_status: Vec<super::marketing_status::MarketingStatus>,
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
    /// A name for this package. Typically what it would be listed as in a drug
    /// formulary or catalogue, inventory etc.
    pub name: super::string::String,
    /// The product this package model relates to, not the contents of the package (for
    /// which see package.containedItem).
    pub package_for: Vec<super::reference::Reference>,
    /// A packaging item, as a container for medically related items, possibly with
    /// other packaging items within, or a packaging component, such as bottle cap
    /// (which is not a device or a medication manufactured item).
    pub packaging: super::packaged_product_definition::PackagedProductDefinitionPackaging,
    /// This is a PackagedProductDefinition resource
    pub resource_type: String,
    /// The status within the lifecycle of this item. A high level status, this is
    /// not intended to duplicate details carried elsewhere such as legal status, or
    /// authorization or marketing status.
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
    /// A high level category e.g. medicinal product, raw material, shipping/transport
    /// container, etc.
    pub r#type: super::codeable_concept::CodeableConcept,
}

/// A medically related item or items, in a container or package.
#[derive(Debug, Clone, PartialEq)]
pub struct PackagedProductDefinitionContainedItem {
    /// The number of this type of item within this packaging or for continuous
    /// items such as liquids it is the quantity (for example 25ml). See also
    /// PackagedProductDefinition.containedItemQuantity (especially the long
    /// definition).
    pub amount: super::quantity::Quantity,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// The actual item(s) of medication, as manufactured, or a device (typically, but
    /// not necessarily, a co-packaged one), or other medically related item (such as
    /// food, biologicals, raw materials, medical fluids, gases etc.), as contained in
    /// the package. This also allows another whole packaged product to be included,
    /// which is solely for the case where a package of other entire packages is wanted
    /// - such as a wholesale or distribution pack (for layers within one package, use
    /// PackagedProductDefinition.packaging.packaging).
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
}

/// A medically related item or items, in a container or package.
#[derive(Debug, Clone, PartialEq)]
pub struct PackagedProductDefinitionLegalStatusOfSupply {
    /// The actual status of supply. Conveys in what situation this package type may be
    /// supplied for use.
    pub code: super::codeable_concept::CodeableConcept,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// The place where the legal status of supply applies. When not specified, this
    /// indicates it is unknown in this context.
    pub jurisdiction: super::codeable_concept::CodeableConcept,
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

/// A medically related item or items, in a container or package.
#[derive(Debug, Clone, PartialEq)]
pub struct PackagedProductDefinitionPackaging {
    /// A possible alternate material for this part of the packaging, that is allowed
    /// to be used instead of the usual material (e.g. different types of plastic for a
    /// blister sleeve).
    pub alternate_material: Vec<super::codeable_concept::CodeableConcept>,
    /// Is this a part of the packaging (e.g. a cap or bottle stopper), rather than
    /// the packaging itself (e.g. a bottle or vial). The latter type are designed be a
    /// container, but the former are not.
    pub component_part: super::boolean::Boolean,
    /// The item(s) within the packaging.
    pub contained_item:
        Vec<super::packaged_product_definition::PackagedProductDefinitionContainedItem>,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// A business identifier that is specific to this particular part of the packaging,
    /// often assigned by the manufacturer. Including possibly Data Carrier Identifier
    /// (a GS1 barcode).
    pub identifier: Vec<super::identifier::Identifier>,
    /// Manufacturer of this packaging item. When there are multiple values each one is
    /// a potential manufacturer of this packaging item.
    pub manufacturer: Vec<super::reference::Reference>,
    /// Material type of the package item.
    pub material: Vec<super::codeable_concept::CodeableConcept>,
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
    /// Allows containers (and parts of containers) within containers,
    /// still as a part of a single packaged product. See also
    /// PackagedProductDefinition.packaging.containedItem.item(PackagedProductDefinition
    /// ).
    pub packaging: Vec<super::packaged_product_definition::PackagedProductDefinitionPackaging>,
    /// General characteristics of this item.
    pub property: Vec<super::packaged_product_definition::PackagedProductDefinitionProperty>,
    /// The quantity of packaging items contained at this layer of the package. This
    /// does not relate to the number of contained items but relates solely to the
    /// number of packaging items. When looking at the outermost layer it is always 1.
    /// If there are two boxes within, at the next layer it would be 2.
    pub quantity: super::integer::Integer,
    /// Shelf Life and storage information.
    pub shelf_life_storage: Vec<super::product_shelf_life::ProductShelfLife>,
    /// The physical type of the container of the items.
    pub r#type: super::codeable_concept::CodeableConcept,
}

/// A medically related item or items, in a container or package.
#[derive(Debug, Clone, PartialEq)]
pub struct PackagedProductDefinitionProperty {
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
    /// A value for the characteristic.
    pub value_attachment: super::attachment::Attachment,
    /// A value for the characteristic.
    pub value_boolean: bool,
    /// A value for the characteristic.
    pub value_codeable_concept: super::codeable_concept::CodeableConcept,
    /// A value for the characteristic.
    pub value_date: String,
    /// A value for the characteristic.
    pub value_quantity: super::quantity::Quantity,
}

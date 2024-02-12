/// A functional description of an inventory item used in inventory and supply-
/// related workflows.
#[derive(Debug, Clone, PartialEq)]
pub struct InventoryItem {
    /// Association with other items or products.
    pub association: Vec<super::inventory_item::InventoryItemAssociation>,
    /// The base unit of measure - the unit in which the product is used or counted.
    pub base_unit: super::codeable_concept::CodeableConcept,
    /// Category or class of the item.
    pub category: Vec<super::codeable_concept::CodeableConcept>,
    /// The descriptive or identifying characteristics of the item.
    pub characteristic: Vec<super::inventory_item::InventoryItemCharacteristic>,
    /// Code designating the specific type of item.
    pub code: Vec<super::codeable_concept::CodeableConcept>,
    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, nor can they have
    /// their own independent transaction scope. This is allowed to be a Parameters
    /// resource if and only if it is referenced by a resource that provides context/
    /// meaning.
    pub contained: Vec<super::resource_list::ResourceList>,
    /// The descriptive characteristics of the inventory item.
    pub description: super::inventory_item::InventoryItemDescription,
    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub id: super::id::Id,
    /// Business identifier for the inventory item.
    pub identifier: Vec<super::identifier::Identifier>,
    /// A reference to a set of rules that were followed when the resource was
    /// constructed, and which must be understood when processing the content. Often,
    /// this is a reference to an implementation guide that defines the special rules
    /// along with other profiles etc.
    pub implicit_rules: super::uri::Uri,
    /// Instances or occurrences of the product.
    pub instance: super::inventory_item::InventoryItemInstance,
    /// The usage status e.g. recalled, in use, discarded... This can be used to
    /// indicate that the items have been taken out of inventory, or are in use, etc.
    pub inventory_status: Vec<super::codeable_concept::CodeableConcept>,
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
    /// The item name(s) - the brand name, or common name, functional name, generic
    /// name.
    pub name: Vec<super::inventory_item::InventoryItemName>,
    /// Net content or amount present in the item.
    pub net_content: super::quantity::Quantity,
    /// Link to a product resource used in clinical workflows.
    pub product_reference: super::reference::Reference,
    /// This is a InventoryItem resource
    pub resource_type: String,
    /// Organization(s) responsible for the product.
    pub responsible_organization: Vec<super::inventory_item::InventoryItemResponsibleOrganization>,
    /// Status of the item entry.
    pub status: super::code::Code,
    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need
    /// not encode all the structured data, but is required to contain sufficient detail
    /// to make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub text: super::narrative::Narrative,
}

/// A functional description of an inventory item used in inventory and supply-
/// related workflows.
#[derive(Debug, Clone, PartialEq)]
pub struct InventoryItemAssociation {
    /// This attribute defined the type of association when establishing associations or
    /// relations between items, e.g. 'packaged within' or 'used with' or 'to be mixed
    /// with.
    pub association_type: super::codeable_concept::CodeableConcept,
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
    /// The quantity of the related product in this product - Numerator is the quantity
    /// of the related product. Denominator is the quantity of the present product. For
    /// example a value of 20 means that this product contains 20 units of the related
    /// product; a value of 1:20 means the inverse - that the contained product contains
    /// 20 units of the present product.
    pub quantity: super::ratio::Ratio,
    /// The related item or product.
    pub related_item: super::reference::Reference,
}

/// A functional description of an inventory item used in inventory and supply-
/// related workflows.
#[derive(Debug, Clone, PartialEq)]
pub struct InventoryItemCharacteristic {
    /// The type of characteristic that is being defined.
    pub characteristic_type: super::codeable_concept::CodeableConcept,
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
    /// The value of the attribute.
    pub value_address: super::address::Address,
    /// The value of the attribute.
    pub value_annotation: super::annotation::Annotation,
    /// The value of the attribute.
    pub value_boolean: bool,
    /// The value of the attribute.
    pub value_codeable_concept: super::codeable_concept::CodeableConcept,
    /// The value of the attribute.
    pub value_date_time: String,
    /// The value of the attribute.
    pub value_decimal: f64,
    /// The value of the attribute.
    pub value_duration: super::duration::Duration,
    /// The value of the attribute.
    pub value_integer: i64,
    /// The value of the attribute.
    pub value_quantity: super::quantity::Quantity,
    /// The value of the attribute.
    pub value_range: super::range::Range,
    /// The value of the attribute.
    pub value_ratio: super::ratio::Ratio,
    /// The value of the attribute.
    pub value_string: String,
    /// The value of the attribute.
    pub value_url: String,
}

/// A functional description of an inventory item used in inventory and supply-
/// related workflows.
#[derive(Debug, Clone, PartialEq)]
pub struct InventoryItemDescription {
    /// Textual description of the item.
    pub description: super::string::String,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// The language for the item description, when an item must be described
    /// in different languages and those languages may be authoritative and not
    /// translations of a 'main' language.
    pub language: super::code::Code,
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

/// A functional description of an inventory item used in inventory and supply-
/// related workflows.
#[derive(Debug, Clone, PartialEq)]
pub struct InventoryItemInstance {
    /// The expiry date or date and time for the product.
    pub expiry: super::date_time::DateTime,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// The identifier for the physical instance, typically a serial number.
    pub identifier: Vec<super::identifier::Identifier>,
    /// The location that the item is associated with.
    pub location: super::reference::Reference,
    /// The lot or batch number of the item.
    pub lot_number: super::string::String,
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
    /// The subject that the item is associated with.
    pub subject: super::reference::Reference,
}

/// A functional description of an inventory item used in inventory and supply-
/// related workflows.
#[derive(Debug, Clone, PartialEq)]
pub struct InventoryItemName {
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// The language that the item name is expressed in.
    pub language: super::code::Code,
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
    /// The name or designation that the item is given.
    pub name: super::string::String,
    /// The type of name e.g. 'brand-name', 'functional-name', 'common-name'.
    pub name_type: super::coding::Coding,
}

/// A functional description of an inventory item used in inventory and supply-
/// related workflows.
#[derive(Debug, Clone, PartialEq)]
pub struct InventoryItemResponsibleOrganization {
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
    /// An organization that has an association with the item, e.g. manufacturer,
    /// distributor, responsible, etc.
    pub organization: super::reference::Reference,
    /// The role of the organization e.g. manufacturer, distributor, etc.
    pub role: super::codeable_concept::CodeableConcept,
}

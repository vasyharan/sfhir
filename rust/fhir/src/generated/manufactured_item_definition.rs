/// The definition and characteristics of a medicinal manufactured item, such as a
/// tablet or capsule, as contained in a packaged medicinal product.
#[derive(Debug, Clone, PartialEq)]
pub struct ManufacturedItemDefinition {
    /// Physical parts of the manufactured item, that it is intrisically made from. This
    /// is distinct from the ingredients that are part of its chemical makeup.
    pub component: Vec<super::manufactured_item_definition::ManufacturedItemDefinitionComponent>,
    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, nor can they have
    /// their own independent transaction scope. This is allowed to be a Parameters
    /// resource if and only if it is referenced by a resource that provides context/
    /// meaning.
    pub contained: Vec<super::resource_list::ResourceList>,
    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub id: super::id::Id,
    /// Unique identifier.
    pub identifier: Vec<super::identifier::Identifier>,
    /// A reference to a set of rules that were followed when the resource was
    /// constructed, and which must be understood when processing the content. Often,
    /// this is a reference to an implementation guide that defines the special rules
    /// along with other profiles etc.
    pub implicit_rules: super::uri::Uri,
    /// The ingredients of this manufactured item. This is only needed if the
    /// ingredients are not specified by incoming references from the Ingredient
    /// resource.
    pub ingredient: Vec<super::codeable_concept::CodeableConcept>,
    /// The base language in which the resource is written.
    pub language: super::code::Code,
    /// Dose form as manufactured and before any transformation into the pharmaceutical
    /// product.
    pub manufactured_dose_form: super::codeable_concept::CodeableConcept,
    /// Manufacturer of the item, one of several possible.
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
    /// A descriptive name applied to this item.
    pub name: super::string::String,
    /// General characteristics of this item.
    pub property: Vec<super::manufactured_item_definition::ManufacturedItemDefinitionProperty>,
    /// This is a ManufacturedItemDefinition resource
    pub resource_type: String,
    /// The status of this item. Enables tracking the life-cycle of the content.
    pub status: super::code::Code,
    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need
    /// not encode all the structured data, but is required to contain sufficient detail
    /// to make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub text: super::narrative::Narrative,
    /// The “real-world” units in which the quantity of the manufactured item is
    /// described.
    pub unit_of_presentation: super::codeable_concept::CodeableConcept,
}

/// The definition and characteristics of a medicinal manufactured item, such as a
/// tablet or capsule, as contained in a packaged medicinal product.
#[derive(Debug, Clone, PartialEq)]
pub struct ManufacturedItemDefinitionComponent {
    /// The measurable amount of total quantity of all substances in the component,
    /// expressable in different ways (e.g. by mass or volume).
    pub amount: Vec<super::quantity::Quantity>,
    /// A component that this component contains or is made from.
    pub component: Vec<super::manufactured_item_definition::ManufacturedItemDefinitionComponent>,
    /// A reference to a constituent of the manufactured item as a whole, linked here
    /// so that its component location within the item can be indicated. This not where
    /// the item's ingredient are primarily stated (for which see Ingredient.for or
    /// ManufacturedItemDefinition.ingredient).
    pub constituent:
        Vec<super::manufactured_item_definition::ManufacturedItemDefinitionConstituent>,
    /// The function of this component within the item e.g. delivers active ingredient,
    /// masks taste.
    pub function: Vec<super::codeable_concept::CodeableConcept>,
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
    /// General characteristics of this component.
    pub property: Vec<super::manufactured_item_definition::ManufacturedItemDefinitionProperty>,
    /// Defining type of the component e.g. shell, layer, ink.
    pub r#type: super::codeable_concept::CodeableConcept,
}

/// The definition and characteristics of a medicinal manufactured item, such as a
/// tablet or capsule, as contained in a packaged medicinal product.
#[derive(Debug, Clone, PartialEq)]
pub struct ManufacturedItemDefinitionConstituent {
    /// The measurable amount of the substance, expressable in different ways (e.g. by
    /// mass or volume).
    pub amount: Vec<super::quantity::Quantity>,
    /// The function of this constituent within the component e.g. binder.
    pub function: Vec<super::codeable_concept::CodeableConcept>,
    /// The ingredient that is the constituent of the given component.
    pub has_ingredient: Vec<super::codeable_reference::CodeableReference>,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// The physical location of the constituent/ingredient within the component.
    /// Example – if the component is the bead in the capsule, then the location would
    /// be where the ingredient resides within the product part – intragranular, extra-
    /// granular, etc.
    pub location: Vec<super::codeable_concept::CodeableConcept>,
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

/// The definition and characteristics of a medicinal manufactured item, such as a
/// tablet or capsule, as contained in a packaged medicinal product.
#[derive(Debug, Clone, PartialEq)]
pub struct ManufacturedItemDefinitionProperty {
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
    pub value_markdown: String,
    /// A value for the characteristic.
    pub value_quantity: super::quantity::Quantity,
    /// A value for the characteristic.
    pub value_reference: super::reference::Reference,
}

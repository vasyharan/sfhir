/// A food or supplement that is consumed by patients.
#[derive(Debug, Clone, PartialEq)]
pub struct NutritionProduct {
    /// Nutrition products can have different classifications - according to its
    /// nutritional properties, preparation methods, etc.
    pub category: Vec<super::codeable_concept::CodeableConcept>,
    /// Specifies descriptive properties of the nutrition product.
    pub characteristic: Vec<super::nutrition_product::NutritionProductCharacteristic>,
    /// The code assigned to the product, for example a USDA NDB number, a USDA FDC ID
    /// number, or a Langual code.
    pub code: super::codeable_concept::CodeableConcept,
    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, nor can they have
    /// their own independent transaction scope. This is allowed to be a Parameters
    /// resource if and only if it is referenced by a resource that provides context/
    /// meaning.
    pub contained: Vec<super::resource_list::ResourceList>,
    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub id: super::id::Id,
    /// A reference to a set of rules that were followed when the resource was
    /// constructed, and which must be understood when processing the content. Often,
    /// this is a reference to an implementation guide that defines the special rules
    /// along with other profiles etc.
    pub implicit_rules: super::uri::Uri,
    /// Ingredients contained in this product.
    pub ingredient: Vec<super::nutrition_product::NutritionProductIngredient>,
    /// Conveys instance-level information about this product item. One or several
    /// physical, countable instances or occurrences of the product.
    pub instance: Vec<super::nutrition_product::NutritionProductInstance>,
    /// Allergens that are known or suspected to be a part of this nutrition product.
    pub known_allergen: Vec<super::codeable_reference::CodeableReference>,
    /// The base language in which the resource is written.
    pub language: super::code::Code,
    /// The organisation (manufacturer, representative or legal authorization holder)
    /// that is responsible for the device.
    pub manufacturer: Vec<super::reference::Reference>,
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
    /// Comments made about the product.
    pub note: Vec<super::annotation::Annotation>,
    /// The product's nutritional information expressed by the nutrients.
    pub nutrient: Vec<super::nutrition_product::NutritionProductNutrient>,
    /// This is a NutritionProduct resource
    pub resource_type: String,
    /// The current state of the product.
    pub status: super::code::Code,
    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need
    /// not encode all the structured data, but is required to contain sufficient detail
    /// to make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub text: super::narrative::Narrative,
}

/// A food or supplement that is consumed by patients.
#[derive(Debug, Clone, PartialEq)]
pub struct NutritionProductCharacteristic {
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
    /// A code specifying which characteristic of the product is being described (for
    /// example, colour, shape).
    pub r#type: super::codeable_concept::CodeableConcept,
    /// The actual characteristic value corresponding to the type.
    pub value_attachment: super::attachment::Attachment,
    /// The actual characteristic value corresponding to the type.
    pub value_base_64_binary: String,
    /// The actual characteristic value corresponding to the type.
    pub value_boolean: bool,
    /// The actual characteristic value corresponding to the type.
    pub value_codeable_concept: super::codeable_concept::CodeableConcept,
    /// The actual characteristic value corresponding to the type.
    pub value_quantity: super::quantity::Quantity,
    /// The actual characteristic value corresponding to the type.
    pub value_string: String,
}

/// A food or supplement that is consumed by patients.
#[derive(Debug, Clone, PartialEq)]
pub struct NutritionProductIngredient {
    /// The amount of ingredient that is in the product.
    pub amount: Vec<super::ratio::Ratio>,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// The ingredient contained in the product.
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

/// A food or supplement that is consumed by patients.
#[derive(Debug, Clone, PartialEq)]
pub struct NutritionProductInstance {
    /// An identifier that supports traceability to the event during which material in
    /// this product from one or more biological entities was obtained or pooled.
    pub biological_source_event: super::identifier::Identifier,
    /// The time after which the product is no longer expected to be in proper
    /// condition, or its use is not advised or not allowed.
    pub expiry: super::date_time::DateTime,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// The identifier for the physical instance, typically a serial number or
    /// manufacturer number.
    pub identifier: Vec<super::identifier::Identifier>,
    /// The identification of the batch or lot of the product.
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
    /// The name for the specific product.
    pub name: super::string::String,
    /// The amount of items or instances that the resource considers, for instance when
    /// referring to 2 identical units together.
    pub quantity: super::quantity::Quantity,
    /// The time after which the product is no longer expected to be in proper
    /// condition, or its use is not advised or not allowed.
    pub use_by: super::date_time::DateTime,
}

/// A food or supplement that is consumed by patients.
#[derive(Debug, Clone, PartialEq)]
pub struct NutritionProductNutrient {
    /// The amount of nutrient expressed in one or more units: X per pack / per
    /// serving / per dose.
    pub amount: Vec<super::ratio::Ratio>,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// The (relevant) nutrients in the product.
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

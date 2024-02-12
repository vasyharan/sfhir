/// An ingredient of a manufactured item or pharmaceutical product.
#[derive(Debug, Clone, PartialEq)]
pub struct Ingredient {
    /// If the ingredient is a known or suspected allergen. Note that this is a property
    /// of the substance, so if a reference to a SubstanceDefinition is used to decribe
    /// that (rather than just a code), the allergen information should go there, not
    /// here.
    pub allergenic_indicator: super::boolean::Boolean,
    /// A place for providing any notes that are relevant to the component, e.g. removed
    /// during process, adjusted for loss on drying.
    pub comment: super::markdown::Markdown,
    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, nor can they have
    /// their own independent transaction scope. This is allowed to be a Parameters
    /// resource if and only if it is referenced by a resource that provides context/
    /// meaning.
    pub contained: Vec<super::resource_list::ResourceList>,
    /// The product which this ingredient is a constituent part of.
    pub r#for: Vec<super::reference::Reference>,
    /// A classification of the ingredient identifying its precise purpose(s) in the
    /// drug product. This extends the Ingredient.role to add more detail. Example:
    /// antioxidant, alkalizing agent.
    pub function: Vec<super::codeable_concept::CodeableConcept>,
    /// A classification of the ingredient according to where in the physical item it
    /// tends to be used, such the outer shell of a tablet, inner body or ink.
    pub group: super::codeable_concept::CodeableConcept,
    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub id: super::id::Id,
    /// The identifier(s) of this Ingredient that are assigned by business processes
    /// and/or used to refer to it when a direct URL reference to the resource itself is
    /// not appropriate.
    pub identifier: super::identifier::Identifier,
    /// A reference to a set of rules that were followed when the resource was
    /// constructed, and which must be understood when processing the content. Often,
    /// this is a reference to an implementation guide that defines the special rules
    /// along with other profiles etc.
    pub implicit_rules: super::uri::Uri,
    /// The base language in which the resource is written.
    pub language: super::code::Code,
    /// The organization(s) that manufacture this ingredient. Can be used to indicate:
    /// 1) Organizations we are aware of that manufacture this ingredient         2)
    /// Specific Manufacturer(s) currently being used         3) Set of organisations
    /// allowed to manufacture this ingredient for this product         Users must be
    /// clear on the application of context relevant to their use case.
    pub manufacturer: Vec<super::ingredient::IngredientManufacturer>,
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
    /// This is a Ingredient resource
    pub resource_type: String,
    /// A classification of the ingredient identifying its purpose within the product,
    /// e.g. active, inactive.
    pub role: super::codeable_concept::CodeableConcept,
    /// The status of this ingredient. Enables tracking the life-cycle of the content.
    pub status: super::code::Code,
    /// The substance that comprises this ingredient.
    pub substance: super::ingredient::IngredientSubstance,
    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need
    /// not encode all the structured data, but is required to contain sufficient detail
    /// to make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub text: super::narrative::Narrative,
}

/// An ingredient of a manufactured item or pharmaceutical product.
#[derive(Debug, Clone, PartialEq)]
pub struct IngredientManufacturer {
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// An organization that manufactures this ingredient.
    pub manufacturer: super::reference::Reference,
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
    /// The way in which this manufacturer is associated with the ingredient. For
    /// example whether it is a possible one (others allowed), or an exclusive
    /// authorized one for this ingredient. Note that this is not the manufacturing
    /// process role.
    pub role: super::code::Code,
}

/// An ingredient of a manufactured item or pharmaceutical product.
#[derive(Debug, Clone, PartialEq)]
pub struct IngredientReferenceStrength {
    /// The country or countries for which the strength range applies.
    pub country: Vec<super::codeable_concept::CodeableConcept>,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// For when strength is measured at a particular point or distance.
    pub measurement_point: super::string::String,
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
    /// Strength expressed in terms of a reference substance.
    pub strength_quantity: super::quantity::Quantity,
    /// Strength expressed in terms of a reference substance.
    pub strength_ratio: super::ratio::Ratio,
    /// Strength expressed in terms of a reference substance.
    pub strength_ratio_range: super::ratio_range::RatioRange,
    /// Relevant reference substance.
    pub substance: super::codeable_reference::CodeableReference,
}

/// An ingredient of a manufactured item or pharmaceutical product.
#[derive(Debug, Clone, PartialEq)]
pub struct IngredientStrength {
    /// A code that indicates if the strength is, for example, based on the ingredient
    /// substance as stated or on the substance base (when the ingredient is a salt).
    pub basis: super::codeable_concept::CodeableConcept,
    /// The strength per unitary volume (or mass).
    pub concentration_codeable_concept: super::codeable_concept::CodeableConcept,
    /// The strength per unitary volume (or mass).
    pub concentration_quantity: super::quantity::Quantity,
    /// The strength per unitary volume (or mass).
    pub concentration_ratio: super::ratio::Ratio,
    /// The strength per unitary volume (or mass).
    pub concentration_ratio_range: super::ratio_range::RatioRange,
    /// The country or countries for which the strength range applies.
    pub country: Vec<super::codeable_concept::CodeableConcept>,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// For when strength is measured at a particular point or distance. There are
    /// products where strength is measured at a particular point. For example, the
    /// strength of the ingredient in some inhalers is measured at a particular position
    /// relative to the point of aerosolization.
    pub measurement_point: super::string::String,
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
    /// The quantity of substance in the unit of presentation, or in the volume (or
    /// mass) of the single pharmaceutical product or manufactured item. Unit of
    /// presentation refers to the quantity that the item occurs in e.g. a strength per
    /// tablet size, perhaps 'per 20mg' (the size of the tablet). It is not generally
    /// normalized as a unitary unit, which would be 'per mg').
    pub presentation_codeable_concept: super::codeable_concept::CodeableConcept,
    /// The quantity of substance in the unit of presentation, or in the volume (or
    /// mass) of the single pharmaceutical product or manufactured item. Unit of
    /// presentation refers to the quantity that the item occurs in e.g. a strength per
    /// tablet size, perhaps 'per 20mg' (the size of the tablet). It is not generally
    /// normalized as a unitary unit, which would be 'per mg').
    pub presentation_quantity: super::quantity::Quantity,
    /// The quantity of substance in the unit of presentation, or in the volume (or
    /// mass) of the single pharmaceutical product or manufactured item. Unit of
    /// presentation refers to the quantity that the item occurs in e.g. a strength per
    /// tablet size, perhaps 'per 20mg' (the size of the tablet). It is not generally
    /// normalized as a unitary unit, which would be 'per mg').
    pub presentation_ratio: super::ratio::Ratio,
    /// The quantity of substance in the unit of presentation, or in the volume (or
    /// mass) of the single pharmaceutical product or manufactured item. Unit of
    /// presentation refers to the quantity that the item occurs in e.g. a strength per
    /// tablet size, perhaps 'per 20mg' (the size of the tablet). It is not generally
    /// normalized as a unitary unit, which would be 'per mg').
    pub presentation_ratio_range: super::ratio_range::RatioRange,
    /// Strength expressed in terms of a reference substance. For when the ingredient
    /// strength is additionally expressed as equivalent to the strength of some other
    /// closely related substance (e.g. salt vs. base). Reference strength represents
    /// the strength (quantitative composition) of the active moiety of the active
    /// substance. There are situations when the active substance and active moiety are
    /// different, therefore both a strength and a reference strength are needed.
    pub reference_strength: Vec<super::ingredient::IngredientReferenceStrength>,
    /// A textual represention of either the whole of the concentration strength or a
    /// part of it - with the rest being in Strength.concentration as a ratio.
    pub text_concentration: super::string::String,
    /// A textual represention of either the whole of the presentation strength or a
    /// part of it - with the rest being in Strength.presentation as a ratio.
    pub text_presentation: super::string::String,
}

/// An ingredient of a manufactured item or pharmaceutical product.
#[derive(Debug, Clone, PartialEq)]
pub struct IngredientSubstance {
    /// A code or full resource that represents the ingredient's substance.
    pub code: super::codeable_reference::CodeableReference,
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
    /// The quantity of substance in the unit of presentation, or in the volume
    /// (or mass) of the single pharmaceutical product or manufactured item. The
    /// allowed repetitions do not represent different strengths, but are different
    /// representations - mathematically equivalent - of a single strength.
    pub strength: Vec<super::ingredient::IngredientStrength>,
}

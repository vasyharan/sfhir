/// A request to supply a diet, formula feeding (enteral) or oral nutritional
/// supplement to a patient/resident.
#[derive(Debug, Clone, PartialEq)]
pub struct NutritionOrder {
    /// A link to a record of allergies or intolerances  which should be included in the
    /// nutrition order.
    pub allergy_intolerance: Vec<super::reference::Reference>,
    /// A plan or request that is fulfilled in whole or in part by this nutrition order.
    pub based_on: Vec<super::reference::Reference>,
    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, nor can they have
    /// their own independent transaction scope. This is allowed to be a Parameters
    /// resource if and only if it is referenced by a resource that provides context/
    /// meaning.
    pub contained: Vec<super::resource_list::ResourceList>,
    /// The date and time that this nutrition order was requested.
    pub date_time: super::date_time::DateTime,
    /// An encounter that provides additional information about the healthcare context
    /// in which this request is made.
    pub encounter: super::reference::Reference,
    /// Feeding provided through the gastrointestinal tract via a tube, catheter, or
    /// stoma that delivers nutrition distal to the oral cavity.
    pub enteral_formula: super::nutrition_order::NutritionOrderEnteralFormula,
    /// This modifier is used to convey Order-specific modifier about the type of oral
    /// food or oral fluids that should not be given. These can be derived from patient
    /// allergies, intolerances, or preferences such as No Red Meat, No Soy or No
    /// Wheat or  Gluten-Free.  While it should not be necessary to repeat allergy or
    /// intolerance information captured in the referenced AllergyIntolerance resource
    /// in the excludeFoodModifier, this element may be used to convey additional
    /// specificity related to foods that should be eliminated from the patient’s diet
    /// for any reason.  This modifier applies to the entire nutrition order inclusive
    /// of the oral diet, nutritional supplements and enteral formula feedings.
    pub exclude_food_modifier: Vec<super::codeable_concept::CodeableConcept>,
    /// This modifier is used to convey order-specific modifiers about the type of food
    /// that should be given. These can be derived from patient allergies, intolerances,
    /// or preferences such as Halal, Vegan or Kosher. This modifier applies to the
    /// entire nutrition order inclusive of the oral diet, nutritional supplements and
    /// enteral formula feedings.
    pub food_preference_modifier: Vec<super::codeable_concept::CodeableConcept>,
    /// A shared identifier common to all nutrition orders that were authorized more
    /// or less simultaneously by a single author, representing the composite or group
    /// identifier.
    pub group_identifier: super::identifier::Identifier,
    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub id: super::id::Id,
    /// Identifiers assigned to this order by the order sender or by the order receiver.
    pub identifier: Vec<super::identifier::Identifier>,
    /// A reference to a set of rules that were followed when the resource was
    /// constructed, and which must be understood when processing the content. Often,
    /// this is a reference to an implementation guide that defines the special rules
    /// along with other profiles etc.
    pub implicit_rules: super::uri::Uri,
    /// The URL pointing to a protocol, guideline, orderset or other definition that is
    /// adhered to in whole or in part by this NutritionOrder.
    pub instantiates: Vec<super::uri::Uri>,
    /// The URL pointing to a FHIR-defined protocol, guideline, orderset or other
    /// definition that is adhered to in whole or in part by this NutritionOrder.
    pub instantiates_canonical: Vec<super::canonical::Canonical>,
    /// The URL pointing to an externally maintained protocol, guideline, orderset or
    /// other definition that is adhered to in whole or in part by this NutritionOrder.
    pub instantiates_uri: Vec<super::uri::Uri>,
    /// Indicates the level of authority/intentionality associated with the NutrionOrder
    /// and where the request fits into the workflow chain.
    pub intent: super::code::Code,
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
    /// Comments made about the {{title}} by the requester, performer, subject or other
    /// participants.
    pub note: Vec<super::annotation::Annotation>,
    /// Diet given orally in contrast to enteral (tube) feeding.
    pub oral_diet: super::nutrition_order::NutritionOrderOralDiet,
    /// The practitioner that holds legal responsibility for ordering the diet,
    /// nutritional supplement, or formula feedings.
    pub orderer: super::reference::Reference,
    /// This modifier is used to convey whether a food item is allowed to be brought
    /// in by the patient and/or family.  If set to true, indicates that the receiving
    /// system does not need to supply the food item.
    pub outside_food_allowed: super::boolean::Boolean,
    /// The specified desired performer of the nutrition order.
    pub performer: Vec<super::codeable_reference::CodeableReference>,
    /// Indicates how quickly the Nutrition Order should be addressed with respect to
    /// other        requests.
    pub priority: super::code::Code,
    /// This is a NutritionOrder resource
    pub resource_type: String,
    /// The workflow status of the nutrition order/request.
    pub status: super::code::Code,
    /// The person or set of individuals who needs the nutrition order for an oral diet,
    /// nutritional supplement and/or enteral or formula feeding.
    pub subject: super::reference::Reference,
    /// Oral nutritional products given in order to add further nutritional value to the
    /// patient's diet.
    pub supplement: Vec<super::nutrition_order::NutritionOrderSupplement>,
    /// Information to support fulfilling (i.e. dispensing or administering) of the
    /// nutrition,        for example, patient height and weight).
    pub supporting_information: Vec<super::reference::Reference>,
    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need
    /// not encode all the structured data, but is required to contain sufficient detail
    /// to make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub text: super::narrative::Narrative,
}

/// A request to supply a diet, formula feeding (enteral) or oral nutritional
/// supplement to a patient/resident.
#[derive(Debug, Clone, PartialEq)]
pub struct NutritionOrderAdditive {
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
    /// The product or brand name of the type of modular component to be added to the
    /// formula.
    pub product_name: super::string::String,
    /// The amount of additive to be given in addition or to be mixed in with the base
    /// formula.
    pub quantity: super::quantity::Quantity,
    /// Indicates the type of modular component such as protein, carbohydrate, fat or
    /// fiber to be provided in addition to or mixed with the base formula.
    pub r#type: super::codeable_reference::CodeableReference,
}

/// A request to supply a diet, formula feeding (enteral) or oral nutritional
/// supplement to a patient/resident.
#[derive(Debug, Clone, PartialEq)]
pub struct NutritionOrderAdministration {
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
    /// The volume of formula to provide to the patient per the specified administration
    /// schedule.
    pub quantity: super::quantity::Quantity,
    /// The rate of administration of formula via a feeding pump, e.g. 60 mL per hour,
    /// according to the specified schedule.
    pub rate_quantity: super::quantity::Quantity,
    /// The rate of administration of formula via a feeding pump, e.g. 60 mL per hour,
    /// according to the specified schedule.
    pub rate_ratio: super::ratio::Ratio,
    /// Schedule information for an enteral formula.
    pub schedule: super::nutrition_order::NutritionOrderSchedule2,
}

/// A request to supply a diet, formula feeding (enteral) or oral nutritional
/// supplement to a patient/resident.
#[derive(Debug, Clone, PartialEq)]
pub struct NutritionOrderEnteralFormula {
    /// Indicates modular components to be provided in addition or mixed with the base
    /// formula.
    pub additive: Vec<super::nutrition_order::NutritionOrderAdditive>,
    /// Formula administration instructions as structured data.  This repeating
    /// structure allows for changing the administration rate or volume over time for
    /// both bolus and continuous feeding.  An example of this would be an instruction
    /// to increase the rate of continuous feeding every 2 hours.
    pub administration: Vec<super::nutrition_order::NutritionOrderAdministration>,
    /// Free text formula administration, feeding instructions or additional
    /// instructions or information.
    pub administration_instruction: super::markdown::Markdown,
    /// The product or brand name of the enteral or infant formula product such as "ACME
    /// Adult Standard Formula".
    pub base_formula_product_name: super::string::String,
    /// The type of enteral or infant formula such as an adult standard formula with
    /// fiber or a soy-based infant formula.
    pub base_formula_type: super::codeable_reference::CodeableReference,
    /// The amount of energy (calories) that the formula should provide per specified
    /// volume, typically per mL or fluid oz.  For example, an infant may require a
    /// formula that provides 24 calories per fluid ounce or an adult may require an
    /// enteral formula that provides 1.5 calorie/mL.
    pub caloric_density: super::quantity::Quantity,
    /// The intended type of device that is to be used for the administration of the
    /// enteral formula.
    pub delivery_device: Vec<super::codeable_reference::CodeableReference>,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// The maximum total quantity of formula that may be administered to a subject over
    /// the period of time, e.g. 1440 mL over 24 hours.
    pub max_volume_to_deliver: super::quantity::Quantity,
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
    /// The route or physiological path of administration into the patient's
    /// gastrointestinal  tract for purposes of providing the formula feeding, e.g.
    /// nasogastric tube.
    pub route_of_administration: super::codeable_concept::CodeableConcept,
}

/// A request to supply a diet, formula feeding (enteral) or oral nutritional
/// supplement to a patient/resident.
#[derive(Debug, Clone, PartialEq)]
pub struct NutritionOrderNutrient {
    /// The quantity of the specified nutrient to include in diet.
    pub amount: super::quantity::Quantity,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// The nutrient that is being modified such as carbohydrate or sodium.
    pub modifier: super::codeable_concept::CodeableConcept,
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

/// A request to supply a diet, formula feeding (enteral) or oral nutritional
/// supplement to a patient/resident.
#[derive(Debug, Clone, PartialEq)]
pub struct NutritionOrderOralDiet {
    /// The required consistency (e.g. honey-thick, nectar-thick, thin, thickened.) of
    /// liquids or fluids served to the patient.
    pub fluid_consistency_type: Vec<super::codeable_concept::CodeableConcept>,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// Free text or additional instructions or information pertaining to the oral diet.
    pub instruction: super::string::String,
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
    /// Class that defines the quantity and type of nutrient modifications (for example
    /// carbohydrate, fiber or sodium) required for the oral diet.
    pub nutrient: Vec<super::nutrition_order::NutritionOrderNutrient>,
    /// Schedule information for an oral diet.
    pub schedule: super::nutrition_order::NutritionOrderSchedule,
    /// Class that describes any texture modifications required for the patient to
    /// safely consume various types of solid foods.
    pub texture: Vec<super::nutrition_order::NutritionOrderTexture>,
    /// The kind of diet or dietary restriction such as fiber restricted diet or
    /// diabetic diet.
    pub r#type: Vec<super::codeable_concept::CodeableConcept>,
}

/// A request to supply a diet, formula feeding (enteral) or oral nutritional
/// supplement to a patient/resident.
#[derive(Debug, Clone, PartialEq)]
pub struct NutritionOrderSchedule {
    /// Indicates whether the product is only taken when needed within a specific dosing
    /// schedule.
    pub as_needed: super::boolean::Boolean,
    /// Indicates whether the product is only taken based on a precondition for taking
    /// the product.
    pub as_needed_for: super::codeable_concept::CodeableConcept,
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
    /// The time period and frequency at which the diet should be given.  The diet
    /// should be given for the combination of all schedules if more than one schedule
    /// is present.
    pub timing: Vec<super::timing::Timing>,
}

/// A request to supply a diet, formula feeding (enteral) or oral nutritional
/// supplement to a patient/resident.
#[derive(Debug, Clone, PartialEq)]
pub struct NutritionOrderSchedule1 {
    /// Indicates whether the supplement is only taken when needed within a specific
    /// dosing schedule.
    pub as_needed: super::boolean::Boolean,
    /// Indicates whether the supplement is only taken based on a precondition for
    /// taking the supplement.
    pub as_needed_for: super::codeable_concept::CodeableConcept,
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
    /// The time period and frequency at which the supplement should be given.  The
    /// supplement should be given for the combination of all schedules if more than one
    /// schedule is present.
    pub timing: Vec<super::timing::Timing>,
}

/// A request to supply a diet, formula feeding (enteral) or oral nutritional
/// supplement to a patient/resident.
#[derive(Debug, Clone, PartialEq)]
pub struct NutritionOrderSchedule2 {
    /// Indicates whether the enteral formula is only taken when needed within a
    /// specific dosing schedule.
    pub as_needed: super::boolean::Boolean,
    /// Indicates whether the enteral formula is only taken based on a precondition for
    /// taking the enteral formula.
    pub as_needed_for: super::codeable_concept::CodeableConcept,
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
    /// The time period and frequency at which the enteral formula should be given.  The
    /// enteral formula should be given for the combination of all schedules if more
    /// than one schedule is present.
    pub timing: Vec<super::timing::Timing>,
}

/// A request to supply a diet, formula feeding (enteral) or oral nutritional
/// supplement to a patient/resident.
#[derive(Debug, Clone, PartialEq)]
pub struct NutritionOrderSupplement {
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// Free text or additional instructions or information pertaining to the oral
    /// supplement.
    pub instruction: super::string::String,
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
    /// The product or brand name of the nutritional supplement such as "Acme Protein
    /// Shake".
    pub product_name: super::string::String,
    /// The amount of the nutritional supplement to be given.
    pub quantity: super::quantity::Quantity,
    /// Schedule information for a supplement.
    pub schedule: super::nutrition_order::NutritionOrderSchedule1,
    /// The kind of nutritional supplement product required such as a high protein or
    /// pediatric clear liquid supplement.
    pub r#type: super::codeable_reference::CodeableReference,
}

/// A request to supply a diet, formula feeding (enteral) or oral nutritional
/// supplement to a patient/resident.
#[derive(Debug, Clone, PartialEq)]
pub struct NutritionOrderTexture {
    /// The food type(s) (e.g. meats, all foods)  that the texture modification applies
    /// to.  This could be all foods types.
    pub food_type: super::codeable_concept::CodeableConcept,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// Any texture modifications (for solid foods) that should be made, e.g. easy to
    /// chew, chopped, ground, and pureed.
    pub modifier: super::codeable_concept::CodeableConcept,
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

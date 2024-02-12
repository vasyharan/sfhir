/// A record of food or fluid that is being consumed by a patient.   A
/// NutritionIntake may indicate that the patient may be consuming the food or
/// fluid now or has consumed the food or fluid in the past.  The source of this
/// information can be the patient, significant other (such as a family member or
/// spouse), or a clinician.  A common scenario where this information is captured
/// is during the history taking process during a patient visit or stay or through
/// an app that tracks food or fluids consumed.   The consumption information may
/// come from sources such as the patient's memory, from a nutrition label,  or from
/// a clinician documenting observed intake.
#[derive(Debug, Clone, PartialEq)]
pub struct NutritionIntake {
    /// A plan, proposal or order that is fulfilled in whole or in part by this event.
    pub based_on: Vec<super::reference::Reference>,
    /// Overall type of nutrition intake.
    pub code: super::codeable_concept::CodeableConcept,
    /// What food or fluid product or item was consumed.
    pub consumed_item: Vec<super::nutrition_intake::NutritionIntakeConsumedItem>,
    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, nor can they have
    /// their own independent transaction scope. This is allowed to be a Parameters
    /// resource if and only if it is referenced by a resource that provides context/
    /// meaning.
    pub contained: Vec<super::resource_list::ResourceList>,
    /// Allows linking the NutritionIntake to the underlying NutritionOrder, or to other
    /// information, such as AllergyIntolerance, that supports or is used to derive
    /// the NutritionIntake.
    pub derived_from: Vec<super::reference::Reference>,
    /// The encounter that establishes the context for this NutritionIntake.
    pub encounter: super::reference::Reference,
    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub id: super::id::Id,
    /// Identifiers associated with this Nutrition Intake that are defined by business
    /// processes and/or used to refer to it when a direct URL reference to the resource
    /// itself is not appropriate. They are business identifiers assigned to this
    /// resource by the performer or other systems and remain constant as the resource
    /// is updated and propagates from server to server.
    pub identifier: Vec<super::identifier::Identifier>,
    /// A reference to a set of rules that were followed when the resource was
    /// constructed, and which must be understood when processing the content. Often,
    /// this is a reference to an implementation guide that defines the special rules
    /// along with other profiles etc.
    pub implicit_rules: super::uri::Uri,
    /// Total nutrient amounts for the whole meal, product, serving, etc.
    pub ingredient_label: Vec<super::nutrition_intake::NutritionIntakeIngredientLabel>,
    /// Instantiates FHIR protocol or definition.
    pub instantiates_canonical: Vec<super::canonical::Canonical>,
    /// Instantiates external protocol or definition.
    pub instantiates_uri: Vec<super::uri::Uri>,
    /// The base language in which the resource is written.
    pub language: super::code::Code,
    /// Where the intake occurred.
    pub location: super::reference::Reference,
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
    /// Provides extra information about the Nutrition Intake that is not conveyed by
    /// the other attributes.
    pub note: Vec<super::annotation::Annotation>,
    /// The interval of time during which it is being asserted that the patient is/was
    /// consuming the food or fluid.
    pub occurrence_date_time: String,
    /// The interval of time during which it is being asserted that the patient is/was
    /// consuming the food or fluid.
    pub occurrence_period: super::period::Period,
    /// A larger event of which this particular event is a component or step.
    pub part_of: Vec<super::reference::Reference>,
    /// Who performed the intake and how they were involved.
    pub performer: Vec<super::nutrition_intake::NutritionIntakePerformer>,
    /// A reason, Condition or observation for why the food or fluid is /was consumed.
    pub reason: Vec<super::codeable_reference::CodeableReference>,
    /// The date when the Nutrition Intake was asserted by the information source.
    pub recorded: super::date_time::DateTime,
    /// The person or organization that provided the information about the consumption
    /// of this food or fluid. Note: Use derivedFrom when a NutritionIntake is derived
    /// from other resources.
    pub reported_boolean: bool,
    /// The person or organization that provided the information about the consumption
    /// of this food or fluid. Note: Use derivedFrom when a NutritionIntake is derived
    /// from other resources.
    pub reported_reference: super::reference::Reference,
    /// This is a NutritionIntake resource
    pub resource_type: String,
    /// A code representing the patient or other source's judgment about the state of
    /// the intake that this assertion is about.  Generally, this will be active or
    /// completed.
    pub status: super::code::Code,
    /// Captures the reason for the current state of the NutritionIntake.
    pub status_reason: Vec<super::codeable_concept::CodeableConcept>,
    /// The person, animal or group who is/was consuming the food or fluid.
    pub subject: super::reference::Reference,
    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need
    /// not encode all the structured data, but is required to contain sufficient detail
    /// to make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub text: super::narrative::Narrative,
}

/// A record of food or fluid that is being consumed by a patient.   A
/// NutritionIntake may indicate that the patient may be consuming the food or
/// fluid now or has consumed the food or fluid in the past.  The source of this
/// information can be the patient, significant other (such as a family member or
/// spouse), or a clinician.  A common scenario where this information is captured
/// is during the history taking process during a patient visit or stay or through
/// an app that tracks food or fluids consumed.   The consumption information may
/// come from sources such as the patient's memory, from a nutrition label,  or from
/// a clinician documenting observed intake.
#[derive(Debug, Clone, PartialEq)]
pub struct NutritionIntakeConsumedItem {
    /// Quantity of the specified food.
    pub amount: super::quantity::Quantity,
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
    /// Indicator when a patient is in a setting where it is helpful to know if food
    /// was not consumed, such as it was refused, held (as in tube feedings), or
    /// otherwise not provided. If a consumption is being recorded from an app, such as
    /// MyFitnessPal, this indicator will likely not be used.
    pub not_consumed: super::boolean::Boolean,
    /// Document the reason the food or fluid was not consumed, such as refused, held,
    /// etc.
    pub not_consumed_reason: super::codeable_concept::CodeableConcept,
    /// Identifies the food or fluid product that was consumed. This is potentially
    /// a link to a resource representing the details of the food product (TBD) or a
    /// simple attribute carrying a code that identifies the food from a known list
    /// of foods.
    pub nutrition_product: super::codeable_reference::CodeableReference,
    /// Rate at which enteral feeding was administered.
    pub rate: super::quantity::Quantity,
    /// Scheduled frequency of consumption.
    pub schedule: super::timing::Timing,
    /// Indicates what a category of item that was consumed: e.g., food, fluid, enteral,
    /// etc.
    pub r#type: super::codeable_concept::CodeableConcept,
}

/// A record of food or fluid that is being consumed by a patient.   A
/// NutritionIntake may indicate that the patient may be consuming the food or
/// fluid now or has consumed the food or fluid in the past.  The source of this
/// information can be the patient, significant other (such as a family member or
/// spouse), or a clinician.  A common scenario where this information is captured
/// is during the history taking process during a patient visit or stay or through
/// an app that tracks food or fluids consumed.   The consumption information may
/// come from sources such as the patient's memory, from a nutrition label,  or from
/// a clinician documenting observed intake.
#[derive(Debug, Clone, PartialEq)]
pub struct NutritionIntakeIngredientLabel {
    /// Total amount of nutrient consumed.
    pub amount: super::quantity::Quantity,
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
    /// Total nutrient consumed. This could be a macronutrient (protein, fat,
    /// carbohydrate), or a vitamin and mineral.
    pub nutrient: super::codeable_reference::CodeableReference,
}

/// A record of food or fluid that is being consumed by a patient.   A
/// NutritionIntake may indicate that the patient may be consuming the food or
/// fluid now or has consumed the food or fluid in the past.  The source of this
/// information can be the patient, significant other (such as a family member or
/// spouse), or a clinician.  A common scenario where this information is captured
/// is during the history taking process during a patient visit or stay or through
/// an app that tracks food or fluids consumed.   The consumption information may
/// come from sources such as the patient's memory, from a nutrition label,  or from
/// a clinician documenting observed intake.
#[derive(Debug, Clone, PartialEq)]
pub struct NutritionIntakePerformer {
    /// Who performed the intake.
    pub actor: super::reference::Reference,
    /// Type of performer.
    pub function: super::codeable_concept::CodeableConcept,
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

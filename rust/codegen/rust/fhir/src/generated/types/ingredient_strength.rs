use super::*;
/// An ingredient of a manufactured item or pharmaceutical product.
#[derive(Debug,Clone,PartialEq)]
pub struct IngredientStrength {
/// A code that indicates if the strength is, for example, based on the ingredient
/// substance as stated or on the substance base (when the ingredient is a salt).
pub basis: CodeableConcept,
/// The strength per unitary volume (or mass).
pub concentration_codeable_concept: CodeableConcept,
/// The strength per unitary volume (or mass).
pub concentration_quantity: Quantity,
/// The strength per unitary volume (or mass).
pub concentration_ratio: Ratio,
/// The strength per unitary volume (or mass).
pub concentration_ratio_range: RatioRange,
/// The country or countries for which the strength range applies.
pub country: Vec<CodeableConcept>,
/// Unique id for the element within a resource (for internal references). This may
/// be any string value that does not contain spaces.
pub id: String,
/// For when strength is measured at a particular point or distance. There are
/// products where strength is measured at a particular point. For example, the
/// strength of the ingredient in some inhalers is measured at a particular position
/// relative to the point of aerosolization.
pub measurement_point: String,
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
pub modifier_extension: Vec<Extension>,
/// The quantity of substance in the unit of presentation, or in the volume (or
/// mass) of the single pharmaceutical product or manufactured item. Unit of
/// presentation refers to the quantity that the item occurs in e.g. a strength per
/// tablet size, perhaps 'per 20mg' (the size of the tablet). It is not generally
/// normalized as a unitary unit, which would be 'per mg').
pub presentation_codeable_concept: CodeableConcept,
/// The quantity of substance in the unit of presentation, or in the volume (or
/// mass) of the single pharmaceutical product or manufactured item. Unit of
/// presentation refers to the quantity that the item occurs in e.g. a strength per
/// tablet size, perhaps 'per 20mg' (the size of the tablet). It is not generally
/// normalized as a unitary unit, which would be 'per mg').
pub presentation_quantity: Quantity,
/// The quantity of substance in the unit of presentation, or in the volume (or
/// mass) of the single pharmaceutical product or manufactured item. Unit of
/// presentation refers to the quantity that the item occurs in e.g. a strength per
/// tablet size, perhaps 'per 20mg' (the size of the tablet). It is not generally
/// normalized as a unitary unit, which would be 'per mg').
pub presentation_ratio: Ratio,
/// The quantity of substance in the unit of presentation, or in the volume (or
/// mass) of the single pharmaceutical product or manufactured item. Unit of
/// presentation refers to the quantity that the item occurs in e.g. a strength per
/// tablet size, perhaps 'per 20mg' (the size of the tablet). It is not generally
/// normalized as a unitary unit, which would be 'per mg').
pub presentation_ratio_range: RatioRange,
/// Strength expressed in terms of a reference substance. For when the ingredient
/// strength is additionally expressed as equivalent to the strength of some other
/// closely related substance (e.g. salt vs. base). Reference strength represents
/// the strength (quantitative composition) of the active moiety of the active
/// substance. There are situations when the active substance and active moiety are
/// different, therefore both a strength and a reference strength are needed.
pub reference_strength: Vec<IngredientReferenceStrength>,
/// A textual represention of either the whole of the concentration strength or a
/// part of it - with the rest being in Strength.concentration as a ratio.
pub text_concentration: String,
/// A textual represention of either the whole of the presentation strength or a
/// part of it - with the rest being in Strength.presentation as a ratio.
pub text_presentation: String,
}

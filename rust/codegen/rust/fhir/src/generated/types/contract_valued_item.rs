use super::*;
/// Legally enforceable, formally recorded unilateral or bilateral directive i.e., a
/// policy or agreement.
#[derive(Debug,Clone,PartialEq)]
pub struct ContractValuedItem {
/// Indicates the time during which this Contract ValuedItem information is
/// effective.
pub effective_time: DateTime,
/// Specific type of Contract Valued Item that may be priced.
pub entity_codeable_concept: CodeableConcept,
/// Specific type of Contract Valued Item that may be priced.
pub entity_reference: Reference,
/// A real number that represents a multiplier used in determining the overall value
/// of the Contract Valued Item delivered. The concept of a Factor allows for a
/// discount or surcharge multiplier to be applied to a monetary amount.
pub factor: Decimal,
/// Unique id for the element within a resource (for internal references). This may
/// be any string value that does not contain spaces.
pub id: String,
/// Identifies a Contract Valued Item instance.
pub identifier: Identifier,
/// Id  of the clause or question text related to the context of this valuedItem in
/// the referenced form or QuestionnaireResponse.
pub link_id: Vec<String>,
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
/// Expresses the product of the Contract Valued Item unitQuantity and the
/// unitPriceAmt. For example, the formula: unit Quantity * unit Price (Cost per
/// Point) * factor Number  * points = net Amount. Quantity, factor and points are
/// assumed to be 1 if not supplied.
pub net: Money,
/// Terms of valuation.
pub payment: String,
/// When payment is due.
pub payment_date: DateTime,
/// An amount that expresses the weighting (based on difficulty, cost and/or
/// resource intensiveness) associated with the Contract Valued Item delivered. The
/// concept of Points allows for assignment of point values for a Contract Valued
/// Item, such that a monetary amount can be assigned to each point.
pub points: Decimal,
/// Specifies the units by which the Contract Valued Item is measured or counted,
/// and quantifies the countable or measurable Contract Valued Item instances.
pub quantity: Quantity,
/// Who will receive payment.
pub recipient: Reference,
/// Who will make payment.
pub responsible: Reference,
/// A set of security labels that define which terms are controlled by this
/// condition.
pub security_label_number: Vec<UnsignedInt>,
/// A Contract Valued Item unit valuation measure.
pub unit_price: Money,
}

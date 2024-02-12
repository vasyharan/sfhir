use super::*;
/// Legally enforceable, formally recorded unilateral or bilateral directive i.e., a
/// policy or agreement.
#[derive(Debug,Clone,PartialEq)]
pub struct ContractAnswer {
/// Unique id for the element within a resource (for internal references). This may
/// be any string value that does not contain spaces.
pub id: String,
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
/// Response to an offer clause or question text,  which enables selection of values
/// to be agreed to, e.g., the period of participation, the date of occupancy of
/// a rental, warranty duration, or whether biospecimen may be used for further
/// research.
pub value_attachment: Attachment,
/// Response to an offer clause or question text,  which enables selection of values
/// to be agreed to, e.g., the period of participation, the date of occupancy of
/// a rental, warranty duration, or whether biospecimen may be used for further
/// research.
pub value_boolean: bool,
/// Response to an offer clause or question text,  which enables selection of values
/// to be agreed to, e.g., the period of participation, the date of occupancy of
/// a rental, warranty duration, or whether biospecimen may be used for further
/// research.
pub value_coding: Coding,
/// Response to an offer clause or question text,  which enables selection of values
/// to be agreed to, e.g., the period of participation, the date of occupancy of
/// a rental, warranty duration, or whether biospecimen may be used for further
/// research.
pub value_date: String,
/// Response to an offer clause or question text,  which enables selection of values
/// to be agreed to, e.g., the period of participation, the date of occupancy of
/// a rental, warranty duration, or whether biospecimen may be used for further
/// research.
pub value_date_time: String,
/// Response to an offer clause or question text,  which enables selection of values
/// to be agreed to, e.g., the period of participation, the date of occupancy of
/// a rental, warranty duration, or whether biospecimen may be used for further
/// research.
pub value_decimal: f64,
/// Response to an offer clause or question text,  which enables selection of values
/// to be agreed to, e.g., the period of participation, the date of occupancy of
/// a rental, warranty duration, or whether biospecimen may be used for further
/// research.
pub value_integer: i64,
/// Response to an offer clause or question text,  which enables selection of values
/// to be agreed to, e.g., the period of participation, the date of occupancy of
/// a rental, warranty duration, or whether biospecimen may be used for further
/// research.
pub value_quantity: Quantity,
/// Response to an offer clause or question text,  which enables selection of values
/// to be agreed to, e.g., the period of participation, the date of occupancy of
/// a rental, warranty duration, or whether biospecimen may be used for further
/// research.
pub value_reference: Reference,
/// Response to an offer clause or question text,  which enables selection of values
/// to be agreed to, e.g., the period of participation, the date of occupancy of
/// a rental, warranty duration, or whether biospecimen may be used for further
/// research.
pub value_string: String,
/// Response to an offer clause or question text,  which enables selection of values
/// to be agreed to, e.g., the period of participation, the date of occupancy of
/// a rental, warranty duration, or whether biospecimen may be used for further
/// research.
pub value_time: String,
/// Response to an offer clause or question text,  which enables selection of values
/// to be agreed to, e.g., the period of participation, the date of occupancy of
/// a rental, warranty duration, or whether biospecimen may be used for further
/// research.
pub value_uri: String,
}

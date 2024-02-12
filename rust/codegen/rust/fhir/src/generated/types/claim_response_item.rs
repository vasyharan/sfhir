use super::*;
/// This resource provides the adjudication details from the processing of a Claim
/// resource.
#[derive(Debug,Clone,PartialEq)]
pub struct ClaimResponseItem {
/// If this item is a group then the values here are a summary of the adjudication
/// of the detail items. If this item is a simple product or service then this is
/// the result of the adjudication of this item.
pub adjudication: Vec<ClaimResponseAdjudication>,
/// A claim detail. Either a simple (a product or service) or a 'group' of sub-
/// details which are simple items.
pub detail: Vec<ClaimResponseDetail>,
/// Unique id for the element within a resource (for internal references). This may
/// be any string value that does not contain spaces.
pub id: String,
/// A number to uniquely reference the claim item entries.
pub item_sequence: PositiveInt,
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
/// The numbers associated with notes below which apply to the adjudication of this
/// item.
pub note_number: Vec<PositiveInt>,
/// The high-level results of the adjudication if adjudication has been performed.
pub review_outcome: ClaimResponseReviewOutcome,
/// Trace number for tracking purposes. May be defined at the jurisdiction level or
/// between trading partners.
pub trace_number: Vec<Identifier>,
}

use super::*;
/// A record of a healthcare consumer’s  choices  or choices made on their behalf
/// by a third party, which permits or denies identified recipient(s) or recipient
/// role(s) to perform one or more actions within a given policy context, for
/// specific purposes and periods of time.
#[derive(Debug,Clone,PartialEq)]
pub struct ConsentProvision {
/// Actions controlled by this provision.
pub action: Vec<CodeableConcept>,
/// Who or what is controlled by this provision. Use group to identify a set of
/// actors by some property they share (e.g. 'admitting officers').
pub actor: Vec<ConsentActor>,
/// If this code is found in an instance, then the provision applies.
pub code: Vec<CodeableConcept>,
/// The resources controlled by this provision if specific resources are referenced.
pub data: Vec<ConsentData>,
/// Clinical or Operational Relevant period of time that bounds the data controlled
/// by this provision.
pub data_period: Period,
/// The documentType(s) covered by this provision. The type can be a CDA document,
/// or some other type that indicates what sort of information the consent relates
/// to.
pub document_type: Vec<Coding>,
/// A computable (FHIRPath or other) definition of what is controlled by this
/// consent.
pub expression: Expression,
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
/// Timeframe for this provision.
pub period: Period,
/// Provisions which provide exceptions to the base provision or subprovisions.
pub provision: Vec<ConsentProvision>,
/// The context of the activities a user is taking - why the user is accessing the
/// data - that are controlled by this provision.
pub purpose: Vec<Coding>,
/// The resourceType(s) covered by this provision. The type can be a FHIR resource
/// type or a profile on a type that indicates what information the consent relates
/// to.
pub resource_type: Vec<Coding>,
/// A security label, comprised of 0..* security label fields (Privacy tags), which
/// define which resources are controlled by this exception.
pub security_label: Vec<Coding>,
}

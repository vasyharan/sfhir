use super::super::types::*;
/// A financial tool for tracking value accrued for a particular purpose.  In the
/// healthcare field, used to track charges for a patient, cost centers, etc.
#[derive(Debug,Clone,PartialEq)]
pub struct Account {
/// The calculated account balances - these are calculated and processed by the
/// finance system.
///
/// The balances with a `term` that is not current are usually generated/updated by
/// an invoicing or similar process.
pub balance: Vec<AccountBalance>,
/// The BillingStatus tracks the lifecycle of the account through the billing
/// process. It indicates how transactions are treated when they are allocated to
/// the account.
pub billing_status: CodeableConcept,
/// Time the balance amount was calculated.
pub calculated_at: Instant,
/// These resources do not have an independent existence apart from the resource
/// that contains them - they cannot be identified independently, nor can they have
/// their own independent transaction scope. This is allowed to be a Parameters
/// resource if and only if it is referenced by a resource that provides context/
/// meaning.
pub contained: Vec<ResourceList>,
/// The party(s) that are responsible for covering the payment of this account, and
/// what order should they be applied to the account.
pub coverage: Vec<AccountCoverage>,
/// The default currency for the account.
pub currency: CodeableConcept,
/// Provides additional information about what the account tracks and how it is
/// used.
pub description: Markdown,
/// When using an account for billing a specific Encounter the set of diagnoses that
/// are relevant for billing are stored here on the account where they are able to
/// be sequenced appropriately prior to processing to produce claim(s).
pub diagnosis: Vec<AccountDiagnosis>,
/// The parties responsible for balancing the account if other payment options fall
/// short.
pub guarantor: Vec<AccountGuarantor>,
/// The logical id of the resource, as used in the URL for the resource. Once
/// assigned, this value never changes.
pub id: Id,
/// Unique identifier used to reference the account.  Might or might not be intended
/// for human use (e.g. credit card number).
pub identifier: Vec<Identifier>,
/// A reference to a set of rules that were followed when the resource was
/// constructed, and which must be understood when processing the content. Often,
/// this is a reference to an implementation guide that defines the special rules
/// along with other profiles etc.
pub implicit_rules: Uri,
/// The base language in which the resource is written.
pub language: Code,
/// The metadata about the resource. This is content that is maintained by the
/// infrastructure. Changes to the content might not always be associated with
/// version changes to the resource.
pub meta: Meta,
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
pub modifier_extension: Vec<Extension>,
/// Name used for the account when displaying it to humans in reports, etc.
pub name: String,
/// Indicates the service area, hospital, department, etc. with responsibility for
/// managing the Account.
pub owner: Reference,
/// When using an account for billing a specific Encounter the set of procedures
/// that are relevant for billing are stored here on the account where they are able
/// to be sequenced appropriately prior to processing to produce claim(s).
pub procedure: Vec<AccountProcedure>,
/// Other associated accounts related to this account.
pub related_account: Vec<AccountRelatedAccount>,
/// This is a Account resource
pub resource_type: String,
/// The date range of services associated with this account.
pub service_period: Period,
/// Indicates whether the account is presently used/usable or not.
pub status: Code,
/// Identifies the entity which incurs the expenses. While the immediate recipients
/// of services or goods might be entities related to the subject, the expenses were
/// ultimately incurred by the subject of the Account.
pub subject: Vec<Reference>,
/// A human-readable narrative that contains a summary of the resource and can be
/// used to represent the content of the resource to a human. The narrative need
/// not encode all the structured data, but is required to contain sufficient detail
/// to make it "clinically safe" for a human to just read the narrative. Resource
/// definitions may define what content should be represented in the narrative to
/// ensure clinical safety.
pub text: Narrative,
/// Categorizes the account for reporting and searching purposes.
pub r#type: CodeableConcept,
}

/// A financial tool for tracking value accrued for a particular purpose.  In the
/// healthcare field, used to track charges for a patient, cost centers, etc.
#[derive(Debug,Clone,PartialEq)]
pub struct AccountBalance {
/// Who is expected to pay this part of the balance.
pub aggregate: CodeableConcept,
/// The actual balance value calculated for the age defined in the term property.
pub amount: Money,
/// The amount is only an estimated value - this is likely common for `current` term
/// balances, but not with known terms (that were generated by a backend process).
pub estimate: Boolean,
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
/// The term of the account balances - The balance value is the amount that was
/// outstanding for this age.
pub term: CodeableConcept,
}

/// A financial tool for tracking value accrued for a particular purpose.  In the
/// healthcare field, used to track charges for a patient, cost centers, etc.
#[derive(Debug,Clone,PartialEq)]
pub struct AccountCoverage {
/// The party(s) that contribute to payment (or part of) of the charges applied to
/// this account (including self-pay).
///
/// A coverage may only be responsible for specific types of charges, and the
/// sequence of the coverages in the account could be important when processing
/// billing.
pub coverage: Reference,
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
/// The priority of the coverage in the context of this account.
pub priority: PositiveInt,
}

/// A financial tool for tracking value accrued for a particular purpose.  In the
/// healthcare field, used to track charges for a patient, cost centers, etc.
#[derive(Debug,Clone,PartialEq)]
pub struct AccountDiagnosis {
/// The diagnosis relevant to the account.
pub condition: CodeableReference,
/// Ranking of the diagnosis (for each type).
pub date_of_diagnosis: DateTime,
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
/// Was the Diagnosis present on Admission in the related Encounter.
pub on_admission: Boolean,
/// The package code can be used to group diagnoses that may be priced or delivered
/// as a single product. Such as DRGs.
pub package_code: Vec<CodeableConcept>,
/// Ranking of the diagnosis (for each type).
pub sequence: PositiveInt,
/// Type that this diagnosis has relevant to the account (e.g. admission, billing,
/// discharge …).
pub r#type: Vec<CodeableConcept>,
}

/// A financial tool for tracking value accrued for a particular purpose.  In the
/// healthcare field, used to track charges for a patient, cost centers, etc.
#[derive(Debug,Clone,PartialEq)]
pub struct AccountGuarantor {
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
/// A guarantor may be placed on credit hold or otherwise have their role
/// temporarily suspended.
pub on_hold: Boolean,
/// The entity who is responsible.
pub party: Reference,
/// The timeframe during which the guarantor accepts responsibility for the account.
pub period: Period,
}

/// A financial tool for tracking value accrued for a particular purpose.  In the
/// healthcare field, used to track charges for a patient, cost centers, etc.
#[derive(Debug,Clone,PartialEq)]
pub struct AccountProcedure {
/// The procedure relevant to the account.
pub code: CodeableReference,
/// Date of the procedure when using a coded procedure. If using a reference to a
/// procedure, then the date on the procedure should be used.
pub date_of_service: DateTime,
/// Any devices that were associated with the procedure relevant to the account.
pub device: Vec<Reference>,
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
/// The package code can be used to group procedures that may be priced or delivered
/// as a single product. Such as DRGs.
pub package_code: Vec<CodeableConcept>,
/// Ranking of the procedure (for each type).
pub sequence: PositiveInt,
/// How this procedure value should be used in charging the account.
pub r#type: Vec<CodeableConcept>,
}

/// A financial tool for tracking value accrued for a particular purpose.  In the
/// healthcare field, used to track charges for a patient, cost centers, etc.
#[derive(Debug,Clone,PartialEq)]
pub struct AccountRelatedAccount {
/// Reference to an associated Account.
pub account: Reference,
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
/// Relationship of the associated Account.
pub relationship: CodeableConcept,
}

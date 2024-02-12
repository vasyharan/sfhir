/// This resource provides the status of the payment for goods and services
/// rendered, and the request and response resource references.
#[derive(Debug, Clone, PartialEq)]
pub struct PaymentNotice {
    /// The amount sent to the payee.
    pub amount: super::money::Money,
    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, nor can they have
    /// their own independent transaction scope. This is allowed to be a Parameters
    /// resource if and only if it is referenced by a resource that provides context/
    /// meaning.
    pub contained: Vec<super::resource_list::ResourceList>,
    /// The date when this resource was created.
    pub created: super::date_time::DateTime,
    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub id: super::id::Id,
    /// A unique identifier assigned to this payment notice.
    pub identifier: Vec<super::identifier::Identifier>,
    /// A reference to a set of rules that were followed when the resource was
    /// constructed, and which must be understood when processing the content. Often,
    /// this is a reference to an implementation guide that defines the special rules
    /// along with other profiles etc.
    pub implicit_rules: super::uri::Uri,
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
    /// The party who will receive or has received payment that is the subject of this
    /// notification.
    pub payee: super::reference::Reference,
    /// A reference to the payment which is the subject of this notice.
    pub payment: super::reference::Reference,
    /// The date when the above payment action occurred.
    pub payment_date: super::date::Date,
    /// A code indicating whether payment has been sent or cleared.
    pub payment_status: super::codeable_concept::CodeableConcept,
    /// The party who is notified of the payment status.
    pub recipient: super::reference::Reference,
    /// The party who reports the payment notice.
    pub reporter: super::reference::Reference,
    /// Reference of resource for which payment is being made.
    pub request: super::reference::Reference,
    /// This is a PaymentNotice resource
    pub resource_type: String,
    /// Reference of response to resource for which payment is being made.
    pub response: super::reference::Reference,
    /// The status of the resource instance.
    pub status: super::code::Code,
    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need
    /// not encode all the structured data, but is required to contain sufficient detail
    /// to make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub text: super::narrative::Narrative,
}

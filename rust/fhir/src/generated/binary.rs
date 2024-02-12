/// A resource that represents the data of a single raw artifact as digital content
/// accessible in its native format.  A Binary resource can contain any content,
/// whether text, image, pdf, zip archive, etc.
#[derive(Debug, Clone, PartialEq)]
pub struct Binary {
    /// MimeType of the binary content represented as a standard MimeType (BCP 13).
    pub content_type: super::code::Code,
    /// The actual content, base64 encoded.
    pub data: super::base_64_binary::Base64Binary,
    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub id: super::id::Id,
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
    /// This is a Binary resource
    pub resource_type: String,
    /// This element identifies another resource that can be used as a proxy of the
    /// security sensitivity to use when deciding and enforcing access control rules for
    /// the Binary resource. Given that the Binary resource contains very few elements
    /// that can be used to determine the sensitivity of the data and relationships
    /// to individuals, the referenced resource stands in as a proxy equivalent for
    /// this purpose. This referenced resource may be related to the Binary (e.g.
    /// DocumentReference), or may be some non-related Resource purely as a security
    /// proxy. E.g. to identify that the binary resource relates to a patient, and
    /// access should only be granted to applications that have access to the patient.
    pub security_context: super::reference::Reference,
}

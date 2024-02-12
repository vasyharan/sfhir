/// A container for a collection of resources.
#[derive(Debug, Clone, PartialEq)]
pub struct Bundle {
    /// An entry in a bundle resource - will either contain a resource or information
    /// about a resource (transactions and history only).
    pub entry: Vec<super::bundle::BundleEntry>,
    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub id: super::id::Id,
    /// A persistent identifier for the bundle that won't change as a bundle is copied
    /// from server to server.
    pub identifier: super::identifier::Identifier,
    /// A reference to a set of rules that were followed when the resource was
    /// constructed, and which must be understood when processing the content. Often,
    /// this is a reference to an implementation guide that defines the special rules
    /// along with other profiles etc.
    pub implicit_rules: super::uri::Uri,
    /// Captures issues and warnings that relate to the construction of the Bundle and
    /// the content within it.
    pub issues: super::resource_list::ResourceList,
    /// The base language in which the resource is written.
    pub language: super::code::Code,
    /// A series of links that provide context to this bundle.
    pub link: Vec<super::bundle::BundleLink>,
    /// The metadata about the resource. This is content that is maintained by the
    /// infrastructure. Changes to the content might not always be associated with
    /// version changes to the resource.
    pub meta: super::meta::Meta,
    /// This is a Bundle resource
    pub resource_type: String,
    /// Digital Signature - base64 encoded. XML-DSig or a JWS.
    pub signature: super::signature::Signature,
    /// The date/time that the bundle was assembled - i.e. when the resources were
    /// placed in the bundle.
    pub timestamp: super::instant::Instant,
    /// If a set of search matches, this is the (potentially estimated) total number
    /// of entries of type 'match' across all pages in the search.  It does not include
    /// search.mode = 'include' or 'outcome' entries and it does not provide a count of
    /// the number of entries in the Bundle.
    pub total: super::unsigned_int::UnsignedInt,
    /// Indicates the purpose of this bundle - how it is intended to be used.
    pub r#type: super::code::Code,
}

/// A container for a collection of resources.
#[derive(Debug, Clone, PartialEq)]
pub struct BundleEntry {
    /// The Absolute URL for the resource. Except for transactions and batches, each
    /// entry in a Bundle must have a fullUrl. The fullUrl SHALL NOT disagree with
    /// the id in the resource - i.e. if the fullUrl is not a urn:uuid, the URL shall
    /// be version-independent URL consistent with the Resource.id. The fullUrl is a
    /// version independent reference to the resource. Even when not required, fullUrl
    /// MAY be set to a urn:uuid to allow referencing entries in a transaction. The
    /// fullUrl can be an arbitrary URI and is not limited to urn:uuid, urn:oid, http,
    /// and https. The fullUrl element SHALL have a value except when:
    /// * invoking a create
    /// * invoking or responding to an operation where the body is not a single
    /// identified resource
    /// * invoking or returning the results of a search or history operation.
    pub full_url: super::uri::Uri,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// A series of links that provide context to this entry.
    pub link: Vec<super::bundle::BundleLink>,
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
    /// Additional information about how this entry should be processed as part of a
    /// transaction or batch.  For history, it shows how the entry was processed to
    /// create the version contained in the entry.
    pub request: super::bundle::BundleRequest,
    /// The Resource for the entry. The purpose/meaning of the resource is determined by
    /// the Bundle.type. This is allowed to be a Parameters resource if and only if it
    /// is referenced by something else within the Bundle that provides context/meaning.
    pub resource: super::resource_list::ResourceList,
    /// Indicates the results of processing the corresponding 'request' entry in the
    /// batch or transaction being responded to or what the results of an operation
    /// where when returning history.
    pub response: super::bundle::BundleResponse,
    /// Information about the search process that lead to the creation of this entry.
    pub search: super::bundle::BundleSearch,
}

/// A container for a collection of resources.
#[derive(Debug, Clone, PartialEq)]
pub struct BundleLink {
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
    /// A name which details the functional use for this link - see [http://
    /// www.iana.org/assignments/link-relations/link-relations.xhtml#link-relations-
    /// 1](http://www.iana.org/assignments/link-relations/link-relations.xhtml#link-
    /// relations-1).
    pub relation: super::code::Code,
    /// The reference details for the link.
    pub url: super::uri::Uri,
}

/// A container for a collection of resources.
#[derive(Debug, Clone, PartialEq)]
pub struct BundleRequest {
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// Only perform the operation if the Etag value matches. For more information, see
    /// the API section ["Managing Resource Contention"](http.html#concurrency).
    pub if_match: super::string::String,
    /// Only perform the operation if the last updated date matches. See the API
    /// documentation for ["Conditional Read"](http.html#cread).
    pub if_modified_since: super::instant::Instant,
    /// Instruct the server not to perform the create if a specified resource already
    /// exists. For further information, see the API documentation for ["Conditional
    /// Create"](http.html#ccreate). This is just the query portion of the URL - what
    /// follows the "?" (not including the "?").
    pub if_none_exist: super::string::String,
    /// If the ETag values match, return a 304 Not Modified status. See the API
    /// documentation for ["Conditional Read"](http.html#cread).
    pub if_none_match: super::string::String,
    /// In a transaction or batch, this is the HTTP action to be executed for this
    /// entry. In a history bundle, this indicates the HTTP action that occurred.
    pub method: super::code::Code,
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
    /// The URL for this entry, relative to the root (the address to which the request
    /// is posted).
    pub url: super::uri::Uri,
}

/// A container for a collection of resources.
#[derive(Debug, Clone, PartialEq)]
pub struct BundleResponse {
    /// The Etag for the resource, if the operation for the entry produced a versioned
    /// resource (see [Resource Metadata and Versioning](http.html#versioning) and
    /// [Managing Resource Contention](http.html#concurrency)).
    pub etag: super::string::String,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// The date/time that the resource was modified on the server.
    pub last_modified: super::instant::Instant,
    /// The location header created by processing this operation, populated if the
    /// operation returns a location.
    pub location: super::uri::Uri,
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
    /// An OperationOutcome containing hints and warnings produced as part of processing
    /// this entry in a batch or transaction.
    pub outcome: super::resource_list::ResourceList,
    /// The status code returned by processing this entry. The status SHALL start with
    /// a 3 digit HTTP code (e.g. 404) and may contain the standard HTTP description
    /// associated with the status code.
    pub status: super::string::String,
}

/// A container for a collection of resources.
#[derive(Debug, Clone, PartialEq)]
pub struct BundleSearch {
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// Why this entry is in the result set - whether it's included as a match or
    /// because of an _include requirement, or to convey information or warning
    /// information about the search process.
    pub mode: super::code::Code,
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
    /// When searching, the server's search ranking score for the entry.
    pub score: super::decimal::Decimal,
}

/// Virtual Service Contact Details.
#[derive(Debug, Clone, PartialEq)]
pub struct VirtualServiceDetail {
    /// Address to see alternative connection details.
    pub additional_info: Vec<super::url::Url>,
    /// What address or number needs to be used for a user to connect to the virtual
    /// service to join. The channelType informs as to which datatype is appropriate to
    /// use (requires knowledge of the specific type).
    pub address_contact_point: super::contact_point::ContactPoint,
    /// What address or number needs to be used for a user to connect to the virtual
    /// service to join. The channelType informs as to which datatype is appropriate to
    /// use (requires knowledge of the specific type).
    pub address_extended_contact_detail: super::extended_contact_detail::ExtendedContactDetail,
    /// What address or number needs to be used for a user to connect to the virtual
    /// service to join. The channelType informs as to which datatype is appropriate to
    /// use (requires knowledge of the specific type).
    pub address_string: String,
    /// What address or number needs to be used for a user to connect to the virtual
    /// service to join. The channelType informs as to which datatype is appropriate to
    /// use (requires knowledge of the specific type).
    pub address_url: String,
    /// The type of virtual service to connect to (i.e. Teams, Zoom, Specific VMR
    /// technology, WhatsApp).
    pub channel_type: super::coding::Coding,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// Maximum number of participants supported by the virtual service.
    pub max_participants: super::positive_int::PositiveInt,
    /// Session Key required by the virtual service.
    pub session_key: super::string::String,
}

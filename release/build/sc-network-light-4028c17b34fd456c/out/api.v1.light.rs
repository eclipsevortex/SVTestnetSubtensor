/// Enumerate all possible light client request messages.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Request {
    #[prost(oneof = "request::Request", tags = "1, 2, 4")]
    pub request: ::core::option::Option<request::Request>,
}
/// Nested message and enum types in `Request`.
pub mod request {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Request {
        #[prost(message, tag = "1")]
        RemoteCallRequest(super::RemoteCallRequest),
        #[prost(message, tag = "2")]
        RemoteReadRequest(super::RemoteReadRequest),
        /// Note: ids 3 and 5 were used in the past. It would be preferable to not re-use them.
        #[prost(message, tag = "4")]
        RemoteReadChildRequest(super::RemoteReadChildRequest),
    }
}
/// Enumerate all possible light client response messages.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Response {
    #[prost(oneof = "response::Response", tags = "1, 2")]
    pub response: ::core::option::Option<response::Response>,
}
/// Nested message and enum types in `Response`.
pub mod response {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Response {
        #[prost(message, tag = "1")]
        RemoteCallResponse(super::RemoteCallResponse),
        /// Note: ids 3 and 4 were used in the past. It would be preferable to not re-use them.
        #[prost(message, tag = "2")]
        RemoteReadResponse(super::RemoteReadResponse),
    }
}
/// Remote call request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoteCallRequest {
    /// Block at which to perform call.
    #[prost(bytes = "vec", required, tag = "2")]
    pub block: ::prost::alloc::vec::Vec<u8>,
    /// Method name.
    #[prost(string, required, tag = "3")]
    pub method: ::prost::alloc::string::String,
    /// Call data.
    #[prost(bytes = "vec", required, tag = "4")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
/// Remote call response.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoteCallResponse {
    /// Execution proof. If missing, indicates that the remote couldn't answer, for example because
    /// the block is pruned.
    #[prost(bytes = "vec", optional, tag = "2")]
    pub proof: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
/// Remote storage read request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoteReadRequest {
    /// Block at which to perform call.
    #[prost(bytes = "vec", required, tag = "2")]
    pub block: ::prost::alloc::vec::Vec<u8>,
    /// Storage keys.
    #[prost(bytes = "vec", repeated, tag = "3")]
    pub keys: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
/// Remote read response.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoteReadResponse {
    /// Read proof. If missing, indicates that the remote couldn't answer, for example because
    /// the block is pruned.
    #[prost(bytes = "vec", optional, tag = "2")]
    pub proof: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
/// Remote storage read child request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoteReadChildRequest {
    /// Block at which to perform call.
    #[prost(bytes = "vec", required, tag = "2")]
    pub block: ::prost::alloc::vec::Vec<u8>,
    /// Child Storage key, this is relative
    /// to the child type storage location.
    #[prost(bytes = "vec", required, tag = "3")]
    pub storage_key: ::prost::alloc::vec::Vec<u8>,
    /// Storage keys.
    #[prost(bytes = "vec", repeated, tag = "6")]
    pub keys: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}

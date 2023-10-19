// @generated
/// Contains a batch of serialized Key Packages
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidateKeyPackagesRequest {
    #[prost(message, repeated, tag="1")]
    pub key_packages: ::prost::alloc::vec::Vec<validate_key_packages_request::KeyPackage>,
}
/// Nested message and enum types in `ValidateKeyPackagesRequest`.
pub mod validate_key_packages_request {
    /// Wrapper for each key package
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct KeyPackage {
        #[prost(bytes="vec", tag="1")]
        pub key_package_bytes_tls_serialized: ::prost::alloc::vec::Vec<u8>,
    }
}
/// Response to ValidateKeyPackagesRequest
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidateKeyPackagesResponse {
    #[prost(message, repeated, tag="1")]
    pub responses: ::prost::alloc::vec::Vec<validate_key_packages_response::ValidationResponse>,
}
/// Nested message and enum types in `ValidateKeyPackagesResponse`.
pub mod validate_key_packages_response {
    /// An individual response to one key package
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ValidationResponse {
        #[prost(bool, tag="1")]
        pub is_ok: bool,
        #[prost(string, tag="2")]
        pub error_message: ::prost::alloc::string::String,
        #[prost(string, tag="3")]
        pub installation_id: ::prost::alloc::string::String,
        #[prost(string, tag="4")]
        pub wallet_address: ::prost::alloc::string::String,
    }
}
/// Contains a batch of serialized Group Messages
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidateGroupMessagesRequest {
    #[prost(message, repeated, tag="1")]
    pub group_messages: ::prost::alloc::vec::Vec<validate_group_messages_request::GroupMessage>,
}
/// Nested message and enum types in `ValidateGroupMessagesRequest`.
pub mod validate_group_messages_request {
    /// Wrapper for each message
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GroupMessage {
        #[prost(bytes="vec", tag="1")]
        pub group_message_bytes_tls_serialized: ::prost::alloc::vec::Vec<u8>,
    }
}
/// Response to ValidateGroupMessagesRequest
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidateGroupMessagesResponse {
    #[prost(message, repeated, tag="1")]
    pub responses: ::prost::alloc::vec::Vec<validate_group_messages_response::ValidationResponse>,
}
/// Nested message and enum types in `ValidateGroupMessagesResponse`.
pub mod validate_group_messages_response {
    /// An individual response to one message
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ValidationResponse {
        #[prost(bool, tag="1")]
        pub is_ok: bool,
        #[prost(string, tag="2")]
        pub error_message: ::prost::alloc::string::String,
        #[prost(string, tag="3")]
        pub group_id: ::prost::alloc::string::String,
        #[prost(uint64, tag="4")]
        pub epoch: u64,
    }
}
/// Contains a batch of serialized credentials
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidateBasicIdentitiesRequest {
    #[prost(message, repeated, tag="1")]
    pub credentials: ::prost::alloc::vec::Vec<validate_basic_identities_request::Credential>,
}
/// Nested message and enum types in `ValidateBasicIdentitiesRequest`.
pub mod validate_basic_identities_request {
    /// Wrapper for each credential and public key
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Credential {
        #[prost(bytes="vec", tag="1")]
        pub identity_bytes_tls_serialized: ::prost::alloc::vec::Vec<u8>,
        #[prost(bytes="vec", tag="2")]
        pub signing_public_key_bytes: ::prost::alloc::vec::Vec<u8>,
    }
}
/// Response to ValidateBasicIdentitiesRequest
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidateBasicIdentitiesResponse {
    #[prost(message, repeated, tag="1")]
    pub responses: ::prost::alloc::vec::Vec<validate_basic_identities_response::ValidationResponse>,
}
/// Nested message and enum types in `ValidateBasicIdentitiesResponse`.
pub mod validate_basic_identities_response {
    /// An individual response to one credential
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ValidationResponse {
        #[prost(bool, tag="1")]
        pub is_ok: bool,
        #[prost(string, tag="2")]
        pub error_message: ::prost::alloc::string::String,
        #[prost(string, tag="3")]
        pub installation_id: ::prost::alloc::string::String,
        #[prost(string, tag="4")]
        pub wallet_address: ::prost::alloc::string::String,
    }
}
/// Encoded file descriptor set for the `xmtp.mls_validation.v1` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0x91, 0x27, 0x0a, 0x1f, 0x6d, 0x6c, 0x73, 0x5f, 0x76, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74,
    0x69, 0x6f, 0x6e, 0x2f, 0x76, 0x31, 0x2f, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x2e, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x12, 0x16, 0x78, 0x6d, 0x74, 0x70, 0x2e, 0x6d, 0x6c, 0x73, 0x5f, 0x76,
    0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x76, 0x31, 0x22, 0xd4, 0x01, 0x0a,
    0x1a, 0x56, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x65, 0x4b, 0x65, 0x79, 0x50, 0x61, 0x63, 0x6b,
    0x61, 0x67, 0x65, 0x73, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x60, 0x0a, 0x0c, 0x6b,
    0x65, 0x79, 0x5f, 0x70, 0x61, 0x63, 0x6b, 0x61, 0x67, 0x65, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28,
    0x0b, 0x32, 0x3d, 0x2e, 0x78, 0x6d, 0x74, 0x70, 0x2e, 0x6d, 0x6c, 0x73, 0x5f, 0x76, 0x61, 0x6c,
    0x69, 0x64, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x76, 0x31, 0x2e, 0x56, 0x61, 0x6c, 0x69, 0x64,
    0x61, 0x74, 0x65, 0x4b, 0x65, 0x79, 0x50, 0x61, 0x63, 0x6b, 0x61, 0x67, 0x65, 0x73, 0x52, 0x65,
    0x71, 0x75, 0x65, 0x73, 0x74, 0x2e, 0x4b, 0x65, 0x79, 0x50, 0x61, 0x63, 0x6b, 0x61, 0x67, 0x65,
    0x52, 0x0b, 0x6b, 0x65, 0x79, 0x50, 0x61, 0x63, 0x6b, 0x61, 0x67, 0x65, 0x73, 0x1a, 0x54, 0x0a,
    0x0a, 0x4b, 0x65, 0x79, 0x50, 0x61, 0x63, 0x6b, 0x61, 0x67, 0x65, 0x12, 0x46, 0x0a, 0x20, 0x6b,
    0x65, 0x79, 0x5f, 0x70, 0x61, 0x63, 0x6b, 0x61, 0x67, 0x65, 0x5f, 0x62, 0x79, 0x74, 0x65, 0x73,
    0x5f, 0x74, 0x6c, 0x73, 0x5f, 0x73, 0x65, 0x72, 0x69, 0x61, 0x6c, 0x69, 0x7a, 0x65, 0x64, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x1c, 0x6b, 0x65, 0x79, 0x50, 0x61, 0x63, 0x6b, 0x61, 0x67,
    0x65, 0x42, 0x79, 0x74, 0x65, 0x73, 0x54, 0x6c, 0x73, 0x53, 0x65, 0x72, 0x69, 0x61, 0x6c, 0x69,
    0x7a, 0x65, 0x64, 0x22, 0xa4, 0x02, 0x0a, 0x1b, 0x56, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x65,
    0x4b, 0x65, 0x79, 0x50, 0x61, 0x63, 0x6b, 0x61, 0x67, 0x65, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f,
    0x6e, 0x73, 0x65, 0x12, 0x64, 0x0a, 0x09, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x73,
    0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x46, 0x2e, 0x78, 0x6d, 0x74, 0x70, 0x2e, 0x6d, 0x6c,
    0x73, 0x5f, 0x76, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x76, 0x31, 0x2e,
    0x56, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x65, 0x4b, 0x65, 0x79, 0x50, 0x61, 0x63, 0x6b, 0x61,
    0x67, 0x65, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x2e, 0x56, 0x61, 0x6c, 0x69,
    0x64, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x52, 0x09,
    0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x73, 0x1a, 0x9e, 0x01, 0x0a, 0x12, 0x56, 0x61,
    0x6c, 0x69, 0x64, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65,
    0x12, 0x13, 0x0a, 0x05, 0x69, 0x73, 0x5f, 0x6f, 0x6b, 0x18, 0x01, 0x20, 0x01, 0x28, 0x08, 0x52,
    0x04, 0x69, 0x73, 0x4f, 0x6b, 0x12, 0x23, 0x0a, 0x0d, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x5f, 0x6d,
    0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0c, 0x65, 0x72,
    0x72, 0x6f, 0x72, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x12, 0x27, 0x0a, 0x0f, 0x69, 0x6e,
    0x73, 0x74, 0x61, 0x6c, 0x6c, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x69, 0x64, 0x18, 0x03, 0x20,
    0x01, 0x28, 0x09, 0x52, 0x0e, 0x69, 0x6e, 0x73, 0x74, 0x61, 0x6c, 0x6c, 0x61, 0x74, 0x69, 0x6f,
    0x6e, 0x49, 0x64, 0x12, 0x25, 0x0a, 0x0e, 0x77, 0x61, 0x6c, 0x6c, 0x65, 0x74, 0x5f, 0x61, 0x64,
    0x64, 0x72, 0x65, 0x73, 0x73, 0x18, 0x04, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0d, 0x77, 0x61, 0x6c,
    0x6c, 0x65, 0x74, 0x41, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x22, 0xe4, 0x01, 0x0a, 0x1c, 0x56,
    0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x65, 0x47, 0x72, 0x6f, 0x75, 0x70, 0x4d, 0x65, 0x73, 0x73,
    0x61, 0x67, 0x65, 0x73, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x68, 0x0a, 0x0e, 0x67,
    0x72, 0x6f, 0x75, 0x70, 0x5f, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x18, 0x01, 0x20,
    0x03, 0x28, 0x0b, 0x32, 0x41, 0x2e, 0x78, 0x6d, 0x74, 0x70, 0x2e, 0x6d, 0x6c, 0x73, 0x5f, 0x76,
    0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x76, 0x31, 0x2e, 0x56, 0x61, 0x6c,
    0x69, 0x64, 0x61, 0x74, 0x65, 0x47, 0x72, 0x6f, 0x75, 0x70, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67,
    0x65, 0x73, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x2e, 0x47, 0x72, 0x6f, 0x75, 0x70, 0x4d,
    0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x52, 0x0d, 0x67, 0x72, 0x6f, 0x75, 0x70, 0x4d, 0x65, 0x73,
    0x73, 0x61, 0x67, 0x65, 0x73, 0x1a, 0x5a, 0x0a, 0x0c, 0x47, 0x72, 0x6f, 0x75, 0x70, 0x4d, 0x65,
    0x73, 0x73, 0x61, 0x67, 0x65, 0x12, 0x4a, 0x0a, 0x22, 0x67, 0x72, 0x6f, 0x75, 0x70, 0x5f, 0x6d,
    0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x5f, 0x62, 0x79, 0x74, 0x65, 0x73, 0x5f, 0x74, 0x6c, 0x73,
    0x5f, 0x73, 0x65, 0x72, 0x69, 0x61, 0x6c, 0x69, 0x7a, 0x65, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x0c, 0x52, 0x1e, 0x67, 0x72, 0x6f, 0x75, 0x70, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x42,
    0x79, 0x74, 0x65, 0x73, 0x54, 0x6c, 0x73, 0x53, 0x65, 0x72, 0x69, 0x61, 0x6c, 0x69, 0x7a, 0x65,
    0x64, 0x22, 0x88, 0x02, 0x0a, 0x1d, 0x56, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x65, 0x47, 0x72,
    0x6f, 0x75, 0x70, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f,
    0x6e, 0x73, 0x65, 0x12, 0x66, 0x0a, 0x09, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x73,
    0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x48, 0x2e, 0x78, 0x6d, 0x74, 0x70, 0x2e, 0x6d, 0x6c,
    0x73, 0x5f, 0x76, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x76, 0x31, 0x2e,
    0x56, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x65, 0x47, 0x72, 0x6f, 0x75, 0x70, 0x4d, 0x65, 0x73,
    0x73, 0x61, 0x67, 0x65, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x2e, 0x56, 0x61,
    0x6c, 0x69, 0x64, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65,
    0x52, 0x09, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x73, 0x1a, 0x7f, 0x0a, 0x12, 0x56,
    0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73,
    0x65, 0x12, 0x13, 0x0a, 0x05, 0x69, 0x73, 0x5f, 0x6f, 0x6b, 0x18, 0x01, 0x20, 0x01, 0x28, 0x08,
    0x52, 0x04, 0x69, 0x73, 0x4f, 0x6b, 0x12, 0x23, 0x0a, 0x0d, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x5f,
    0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0c, 0x65,
    0x72, 0x72, 0x6f, 0x72, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x12, 0x19, 0x0a, 0x08, 0x67,
    0x72, 0x6f, 0x75, 0x70, 0x5f, 0x69, 0x64, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x52, 0x07, 0x67,
    0x72, 0x6f, 0x75, 0x70, 0x49, 0x64, 0x12, 0x14, 0x0a, 0x05, 0x65, 0x70, 0x6f, 0x63, 0x68, 0x18,
    0x04, 0x20, 0x01, 0x28, 0x04, 0x52, 0x05, 0x65, 0x70, 0x6f, 0x63, 0x68, 0x22, 0x90, 0x02, 0x0a,
    0x1e, 0x56, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x65, 0x42, 0x61, 0x73, 0x69, 0x63, 0x49, 0x64,
    0x65, 0x6e, 0x74, 0x69, 0x74, 0x69, 0x65, 0x73, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12,
    0x63, 0x0a, 0x0b, 0x63, 0x72, 0x65, 0x64, 0x65, 0x6e, 0x74, 0x69, 0x61, 0x6c, 0x73, 0x18, 0x01,
    0x20, 0x03, 0x28, 0x0b, 0x32, 0x41, 0x2e, 0x78, 0x6d, 0x74, 0x70, 0x2e, 0x6d, 0x6c, 0x73, 0x5f,
    0x76, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x76, 0x31, 0x2e, 0x56, 0x61,
    0x6c, 0x69, 0x64, 0x61, 0x74, 0x65, 0x42, 0x61, 0x73, 0x69, 0x63, 0x49, 0x64, 0x65, 0x6e, 0x74,
    0x69, 0x74, 0x69, 0x65, 0x73, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x2e, 0x43, 0x72, 0x65,
    0x64, 0x65, 0x6e, 0x74, 0x69, 0x61, 0x6c, 0x52, 0x0b, 0x63, 0x72, 0x65, 0x64, 0x65, 0x6e, 0x74,
    0x69, 0x61, 0x6c, 0x73, 0x1a, 0x88, 0x01, 0x0a, 0x0a, 0x43, 0x72, 0x65, 0x64, 0x65, 0x6e, 0x74,
    0x69, 0x61, 0x6c, 0x12, 0x41, 0x0a, 0x1d, 0x69, 0x64, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x5f,
    0x62, 0x79, 0x74, 0x65, 0x73, 0x5f, 0x74, 0x6c, 0x73, 0x5f, 0x73, 0x65, 0x72, 0x69, 0x61, 0x6c,
    0x69, 0x7a, 0x65, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x1a, 0x69, 0x64, 0x65, 0x6e,
    0x74, 0x69, 0x74, 0x79, 0x42, 0x79, 0x74, 0x65, 0x73, 0x54, 0x6c, 0x73, 0x53, 0x65, 0x72, 0x69,
    0x61, 0x6c, 0x69, 0x7a, 0x65, 0x64, 0x12, 0x37, 0x0a, 0x18, 0x73, 0x69, 0x67, 0x6e, 0x69, 0x6e,
    0x67, 0x5f, 0x70, 0x75, 0x62, 0x6c, 0x69, 0x63, 0x5f, 0x6b, 0x65, 0x79, 0x5f, 0x62, 0x79, 0x74,
    0x65, 0x73, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x15, 0x73, 0x69, 0x67, 0x6e, 0x69, 0x6e,
    0x67, 0x50, 0x75, 0x62, 0x6c, 0x69, 0x63, 0x4b, 0x65, 0x79, 0x42, 0x79, 0x74, 0x65, 0x73, 0x22,
    0xac, 0x02, 0x0a, 0x1f, 0x56, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x65, 0x42, 0x61, 0x73, 0x69,
    0x63, 0x49, 0x64, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x69, 0x65, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f,
    0x6e, 0x73, 0x65, 0x12, 0x68, 0x0a, 0x09, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x73,
    0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x4a, 0x2e, 0x78, 0x6d, 0x74, 0x70, 0x2e, 0x6d, 0x6c,
    0x73, 0x5f, 0x76, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x76, 0x31, 0x2e,
    0x56, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x65, 0x42, 0x61, 0x73, 0x69, 0x63, 0x49, 0x64, 0x65,
    0x6e, 0x74, 0x69, 0x74, 0x69, 0x65, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x2e,
    0x56, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e,
    0x73, 0x65, 0x52, 0x09, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x73, 0x1a, 0x9e, 0x01,
    0x0a, 0x12, 0x56, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x65, 0x73, 0x70,
    0x6f, 0x6e, 0x73, 0x65, 0x12, 0x13, 0x0a, 0x05, 0x69, 0x73, 0x5f, 0x6f, 0x6b, 0x18, 0x01, 0x20,
    0x01, 0x28, 0x08, 0x52, 0x04, 0x69, 0x73, 0x4f, 0x6b, 0x12, 0x23, 0x0a, 0x0d, 0x65, 0x72, 0x72,
    0x6f, 0x72, 0x5f, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09,
    0x52, 0x0c, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x12, 0x27,
    0x0a, 0x0f, 0x69, 0x6e, 0x73, 0x74, 0x61, 0x6c, 0x6c, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x69,
    0x64, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0e, 0x69, 0x6e, 0x73, 0x74, 0x61, 0x6c, 0x6c,
    0x61, 0x74, 0x69, 0x6f, 0x6e, 0x49, 0x64, 0x12, 0x25, 0x0a, 0x0e, 0x77, 0x61, 0x6c, 0x6c, 0x65,
    0x74, 0x5f, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x18, 0x04, 0x20, 0x01, 0x28, 0x09, 0x52,
    0x0d, 0x77, 0x61, 0x6c, 0x6c, 0x65, 0x74, 0x41, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x32, 0xaa,
    0x03, 0x0a, 0x0d, 0x56, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x41, 0x70, 0x69,
    0x12, 0x80, 0x01, 0x0a, 0x13, 0x56, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x65, 0x4b, 0x65, 0x79,
    0x50, 0x61, 0x63, 0x6b, 0x61, 0x67, 0x65, 0x73, 0x12, 0x32, 0x2e, 0x78, 0x6d, 0x74, 0x70, 0x2e,
    0x6d, 0x6c, 0x73, 0x5f, 0x76, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x76,
    0x31, 0x2e, 0x56, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x65, 0x4b, 0x65, 0x79, 0x50, 0x61, 0x63,
    0x6b, 0x61, 0x67, 0x65, 0x73, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x1a, 0x33, 0x2e, 0x78,
    0x6d, 0x74, 0x70, 0x2e, 0x6d, 0x6c, 0x73, 0x5f, 0x76, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x69,
    0x6f, 0x6e, 0x2e, 0x76, 0x31, 0x2e, 0x56, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x65, 0x4b, 0x65,
    0x79, 0x50, 0x61, 0x63, 0x6b, 0x61, 0x67, 0x65, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73,
    0x65, 0x22, 0x00, 0x12, 0x86, 0x01, 0x0a, 0x15, 0x56, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x65,
    0x47, 0x72, 0x6f, 0x75, 0x70, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x12, 0x34, 0x2e,
    0x78, 0x6d, 0x74, 0x70, 0x2e, 0x6d, 0x6c, 0x73, 0x5f, 0x76, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74,
    0x69, 0x6f, 0x6e, 0x2e, 0x76, 0x31, 0x2e, 0x56, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x65, 0x47,
    0x72, 0x6f, 0x75, 0x70, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x52, 0x65, 0x71, 0x75,
    0x65, 0x73, 0x74, 0x1a, 0x35, 0x2e, 0x78, 0x6d, 0x74, 0x70, 0x2e, 0x6d, 0x6c, 0x73, 0x5f, 0x76,
    0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x76, 0x31, 0x2e, 0x56, 0x61, 0x6c,
    0x69, 0x64, 0x61, 0x74, 0x65, 0x47, 0x72, 0x6f, 0x75, 0x70, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67,
    0x65, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x22, 0x00, 0x12, 0x8c, 0x01, 0x0a,
    0x17, 0x56, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x65, 0x42, 0x61, 0x73, 0x69, 0x63, 0x49, 0x64,
    0x65, 0x6e, 0x74, 0x69, 0x74, 0x69, 0x65, 0x73, 0x12, 0x36, 0x2e, 0x78, 0x6d, 0x74, 0x70, 0x2e,
    0x6d, 0x6c, 0x73, 0x5f, 0x76, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x76,
    0x31, 0x2e, 0x56, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x65, 0x42, 0x61, 0x73, 0x69, 0x63, 0x49,
    0x64, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x69, 0x65, 0x73, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74,
    0x1a, 0x37, 0x2e, 0x78, 0x6d, 0x74, 0x70, 0x2e, 0x6d, 0x6c, 0x73, 0x5f, 0x76, 0x61, 0x6c, 0x69,
    0x64, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x76, 0x31, 0x2e, 0x56, 0x61, 0x6c, 0x69, 0x64, 0x61,
    0x74, 0x65, 0x42, 0x61, 0x73, 0x69, 0x63, 0x49, 0x64, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x69, 0x65,
    0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x22, 0x00, 0x42, 0xcf, 0x01, 0x0a, 0x1a,
    0x63, 0x6f, 0x6d, 0x2e, 0x78, 0x6d, 0x74, 0x70, 0x2e, 0x6d, 0x6c, 0x73, 0x5f, 0x76, 0x61, 0x6c,
    0x69, 0x64, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x76, 0x31, 0x42, 0x0c, 0x53, 0x65, 0x72, 0x76,
    0x69, 0x63, 0x65, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x50, 0x01, 0x5a, 0x2d, 0x67, 0x69, 0x74, 0x68,
    0x75, 0x62, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x78, 0x6d, 0x74, 0x70, 0x2f, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x2f, 0x76, 0x33, 0x2f, 0x67, 0x6f, 0x2f, 0x6d, 0x6c, 0x73, 0x5f, 0x76, 0x61, 0x6c, 0x69,
    0x64, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x2f, 0x76, 0x31, 0xa2, 0x02, 0x03, 0x58, 0x4d, 0x58, 0xaa,
    0x02, 0x15, 0x58, 0x6d, 0x74, 0x70, 0x2e, 0x4d, 0x6c, 0x73, 0x56, 0x61, 0x6c, 0x69, 0x64, 0x61,
    0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x56, 0x31, 0xca, 0x02, 0x15, 0x58, 0x6d, 0x74, 0x70, 0x5c, 0x4d,
    0x6c, 0x73, 0x56, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x5c, 0x56, 0x31, 0xe2,
    0x02, 0x21, 0x58, 0x6d, 0x74, 0x70, 0x5c, 0x4d, 0x6c, 0x73, 0x56, 0x61, 0x6c, 0x69, 0x64, 0x61,
    0x74, 0x69, 0x6f, 0x6e, 0x5c, 0x56, 0x31, 0x5c, 0x47, 0x50, 0x42, 0x4d, 0x65, 0x74, 0x61, 0x64,
    0x61, 0x74, 0x61, 0xea, 0x02, 0x17, 0x58, 0x6d, 0x74, 0x70, 0x3a, 0x3a, 0x4d, 0x6c, 0x73, 0x56,
    0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x3a, 0x3a, 0x56, 0x31, 0x4a, 0x9c, 0x15,
    0x0a, 0x06, 0x12, 0x04, 0x01, 0x00, 0x56, 0x01, 0x0a, 0x17, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x01,
    0x00, 0x12, 0x1a, 0x0d, 0x20, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x20, 0x41, 0x50, 0x49,
    0x0a, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x02, 0x00, 0x1f, 0x0a, 0x08, 0x0a, 0x01, 0x08,
    0x12, 0x03, 0x04, 0x00, 0x44, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x0b, 0x12, 0x03, 0x04, 0x00, 0x44,
    0x0a, 0x26, 0x0a, 0x02, 0x06, 0x00, 0x12, 0x04, 0x07, 0x00, 0x10, 0x01, 0x1a, 0x1a, 0x20, 0x52,
    0x50, 0x43, 0x73, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6e, 0x65, 0x77, 0x20,
    0x4d, 0x4c, 0x53, 0x20, 0x41, 0x50, 0x49, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x06, 0x00, 0x01, 0x12,
    0x03, 0x07, 0x08, 0x15, 0x0a, 0x58, 0x0a, 0x04, 0x06, 0x00, 0x02, 0x00, 0x12, 0x03, 0x09, 0x02,
    0x5e, 0x1a, 0x4b, 0x20, 0x56, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x65, 0x73, 0x20, 0x61, 0x6e,
    0x64, 0x20, 0x70, 0x61, 0x72, 0x73, 0x65, 0x73, 0x20, 0x61, 0x20, 0x62, 0x61, 0x74, 0x63, 0x68,
    0x20, 0x6f, 0x66, 0x20, 0x6b, 0x65, 0x79, 0x20, 0x70, 0x61, 0x63, 0x6b, 0x61, 0x67, 0x65, 0x73,
    0x20, 0x61, 0x6e, 0x64, 0x20, 0x72, 0x65, 0x74, 0x75, 0x72, 0x6e, 0x73, 0x20, 0x72, 0x65, 0x6c,
    0x65, 0x76, 0x61, 0x6e, 0x74, 0x20, 0x64, 0x65, 0x74, 0x61, 0x69, 0x6c, 0x73, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x06, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x09, 0x06, 0x19, 0x0a, 0x0c, 0x0a, 0x05,
    0x06, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x09, 0x1a, 0x34, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x09, 0x3f, 0x5a, 0x0a, 0x50, 0x0a, 0x04, 0x06, 0x00, 0x02, 0x01,
    0x12, 0x03, 0x0c, 0x02, 0x64, 0x1a, 0x43, 0x20, 0x56, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x65,
    0x73, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x70, 0x61, 0x72, 0x73, 0x65, 0x73, 0x20, 0x61, 0x20, 0x67,
    0x72, 0x6f, 0x75, 0x70, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x20, 0x61, 0x6e, 0x64,
    0x20, 0x72, 0x65, 0x74, 0x75, 0x72, 0x6e, 0x73, 0x20, 0x72, 0x65, 0x6c, 0x65, 0x76, 0x61, 0x6e,
    0x74, 0x20, 0x64, 0x65, 0x74, 0x61, 0x69, 0x6c, 0x73, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00,
    0x02, 0x01, 0x01, 0x12, 0x03, 0x0c, 0x06, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x01,
    0x02, 0x12, 0x03, 0x0c, 0x1c, 0x38, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x01, 0x03, 0x12,
    0x03, 0x0c, 0x43, 0x60, 0x0a, 0x38, 0x0a, 0x04, 0x06, 0x00, 0x02, 0x02, 0x12, 0x03, 0x0f, 0x02,
    0x6a, 0x1a, 0x2b, 0x20, 0x56, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x65, 0x20, 0x61, 0x6e, 0x64,
    0x20, 0x70, 0x61, 0x72, 0x73, 0x65, 0x20, 0x61, 0x20, 0x62, 0x61, 0x74, 0x63, 0x68, 0x20, 0x6f,
    0x66, 0x20, 0x63, 0x72, 0x65, 0x64, 0x65, 0x6e, 0x74, 0x69, 0x61, 0x6c, 0x73, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x06, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x0f, 0x06, 0x1d, 0x0a, 0x0c, 0x0a, 0x05,
    0x06, 0x00, 0x02, 0x02, 0x02, 0x12, 0x03, 0x0f, 0x1e, 0x3c, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00,
    0x02, 0x02, 0x03, 0x12, 0x03, 0x0f, 0x47, 0x66, 0x0a, 0x39, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04,
    0x13, 0x00, 0x1a, 0x01, 0x1a, 0x2d, 0x20, 0x43, 0x6f, 0x6e, 0x74, 0x61, 0x69, 0x6e, 0x73, 0x20,
    0x61, 0x20, 0x62, 0x61, 0x74, 0x63, 0x68, 0x20, 0x6f, 0x66, 0x20, 0x73, 0x65, 0x72, 0x69, 0x61,
    0x6c, 0x69, 0x7a, 0x65, 0x64, 0x20, 0x4b, 0x65, 0x79, 0x20, 0x50, 0x61, 0x63, 0x6b, 0x61, 0x67,
    0x65, 0x73, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x13, 0x08, 0x22, 0x0a,
    0x2c, 0x0a, 0x04, 0x04, 0x00, 0x03, 0x00, 0x12, 0x04, 0x15, 0x02, 0x17, 0x03, 0x1a, 0x1e, 0x20,
    0x57, 0x72, 0x61, 0x70, 0x70, 0x65, 0x72, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x65, 0x61, 0x63, 0x68,
    0x20, 0x6b, 0x65, 0x79, 0x20, 0x70, 0x61, 0x63, 0x6b, 0x61, 0x67, 0x65, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x03, 0x00, 0x01, 0x12, 0x03, 0x15, 0x0a, 0x14, 0x0a, 0x0d, 0x0a, 0x06, 0x04,
    0x00, 0x03, 0x00, 0x02, 0x00, 0x12, 0x03, 0x16, 0x04, 0x2f, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00,
    0x03, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x16, 0x04, 0x09, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00,
    0x03, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x16, 0x0a, 0x2a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00,
    0x03, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x16, 0x2d, 0x2e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00,
    0x02, 0x00, 0x12, 0x03, 0x19, 0x02, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04,
    0x12, 0x03, 0x19, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x06, 0x12, 0x03,
    0x19, 0x0b, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x19, 0x16,
    0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x19, 0x25, 0x26, 0x0a,
    0x34, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x1d, 0x00, 0x27, 0x01, 0x1a, 0x28, 0x20, 0x52, 0x65,
    0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x20, 0x74, 0x6f, 0x20, 0x56, 0x61, 0x6c, 0x69, 0x64, 0x61,
    0x74, 0x65, 0x4b, 0x65, 0x79, 0x50, 0x61, 0x63, 0x6b, 0x61, 0x67, 0x65, 0x73, 0x52, 0x65, 0x71,
    0x75, 0x65, 0x73, 0x74, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x1d, 0x08,
    0x23, 0x0a, 0x39, 0x0a, 0x04, 0x04, 0x01, 0x03, 0x00, 0x12, 0x04, 0x1f, 0x02, 0x24, 0x03, 0x1a,
    0x2b, 0x20, 0x41, 0x6e, 0x20, 0x69, 0x6e, 0x64, 0x69, 0x76, 0x69, 0x64, 0x75, 0x61, 0x6c, 0x20,
    0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x20, 0x74, 0x6f, 0x20, 0x6f, 0x6e, 0x65, 0x20,
    0x6b, 0x65, 0x79, 0x20, 0x70, 0x61, 0x63, 0x6b, 0x61, 0x67, 0x65, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x03, 0x00, 0x01, 0x12, 0x03, 0x1f, 0x0a, 0x1c, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x01,
    0x03, 0x00, 0x02, 0x00, 0x12, 0x03, 0x20, 0x04, 0x13, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03,
    0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x20, 0x04, 0x08, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03,
    0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x20, 0x09, 0x0e, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03,
    0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x20, 0x11, 0x12, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x01, 0x03,
    0x00, 0x02, 0x01, 0x12, 0x03, 0x21, 0x04, 0x1d, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00,
    0x02, 0x01, 0x05, 0x12, 0x03, 0x21, 0x04, 0x0a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00,
    0x02, 0x01, 0x01, 0x12, 0x03, 0x21, 0x0b, 0x18, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00,
    0x02, 0x01, 0x03, 0x12, 0x03, 0x21, 0x1b, 0x1c, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x01, 0x03, 0x00,
    0x02, 0x02, 0x12, 0x03, 0x22, 0x04, 0x1f, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02,
    0x02, 0x05, 0x12, 0x03, 0x22, 0x04, 0x0a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02,
    0x02, 0x01, 0x12, 0x03, 0x22, 0x0b, 0x1a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02,
    0x02, 0x03, 0x12, 0x03, 0x22, 0x1d, 0x1e, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x01, 0x03, 0x00, 0x02,
    0x03, 0x12, 0x03, 0x23, 0x04, 0x1e, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x03,
    0x05, 0x12, 0x03, 0x23, 0x04, 0x0a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x03,
    0x01, 0x12, 0x03, 0x23, 0x0b, 0x19, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x03,
    0x03, 0x12, 0x03, 0x23, 0x1c, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03,
    0x26, 0x02, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x03, 0x26, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x06, 0x12, 0x03, 0x26, 0x0b, 0x1d, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x26, 0x1e, 0x27, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x26, 0x2a, 0x2b, 0x0a, 0x3b, 0x0a, 0x02, 0x04,
    0x02, 0x12, 0x04, 0x2a, 0x00, 0x31, 0x01, 0x1a, 0x2f, 0x20, 0x43, 0x6f, 0x6e, 0x74, 0x61, 0x69,
    0x6e, 0x73, 0x20, 0x61, 0x20, 0x62, 0x61, 0x74, 0x63, 0x68, 0x20, 0x6f, 0x66, 0x20, 0x73, 0x65,
    0x72, 0x69, 0x61, 0x6c, 0x69, 0x7a, 0x65, 0x64, 0x20, 0x47, 0x72, 0x6f, 0x75, 0x70, 0x20, 0x4d,
    0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12,
    0x03, 0x2a, 0x08, 0x24, 0x0a, 0x28, 0x0a, 0x04, 0x04, 0x02, 0x03, 0x00, 0x12, 0x04, 0x2c, 0x02,
    0x2e, 0x03, 0x1a, 0x1a, 0x20, 0x57, 0x72, 0x61, 0x70, 0x70, 0x65, 0x72, 0x20, 0x66, 0x6f, 0x72,
    0x20, 0x65, 0x61, 0x63, 0x68, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x03, 0x00, 0x01, 0x12, 0x03, 0x2c, 0x0a, 0x16, 0x0a, 0x0d, 0x0a, 0x06,
    0x04, 0x02, 0x03, 0x00, 0x02, 0x00, 0x12, 0x03, 0x2d, 0x04, 0x31, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x02, 0x03, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x2d, 0x04, 0x09, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x02, 0x03, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x2d, 0x0a, 0x2c, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x02, 0x03, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x2d, 0x2f, 0x30, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x02, 0x02, 0x00, 0x12, 0x03, 0x30, 0x02, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00,
    0x04, 0x12, 0x03, 0x30, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x06, 0x12,
    0x03, 0x30, 0x0b, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x30,
    0x18, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x30, 0x29, 0x2a,
    0x0a, 0x36, 0x0a, 0x02, 0x04, 0x03, 0x12, 0x04, 0x34, 0x00, 0x3e, 0x01, 0x1a, 0x2a, 0x20, 0x52,
    0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x20, 0x74, 0x6f, 0x20, 0x56, 0x61, 0x6c, 0x69, 0x64,
    0x61, 0x74, 0x65, 0x47, 0x72, 0x6f, 0x75, 0x70, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73,
    0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12,
    0x03, 0x34, 0x08, 0x25, 0x0a, 0x35, 0x0a, 0x04, 0x04, 0x03, 0x03, 0x00, 0x12, 0x04, 0x36, 0x02,
    0x3b, 0x03, 0x1a, 0x27, 0x20, 0x41, 0x6e, 0x20, 0x69, 0x6e, 0x64, 0x69, 0x76, 0x69, 0x64, 0x75,
    0x61, 0x6c, 0x20, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x20, 0x74, 0x6f, 0x20, 0x6f,
    0x6e, 0x65, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x03, 0x00, 0x01, 0x12, 0x03, 0x36, 0x0a, 0x1c, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x03, 0x03,
    0x00, 0x02, 0x00, 0x12, 0x03, 0x37, 0x04, 0x13, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x03, 0x00,
    0x02, 0x00, 0x05, 0x12, 0x03, 0x37, 0x04, 0x08, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x03, 0x00,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x37, 0x09, 0x0e, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x03, 0x00,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x37, 0x11, 0x12, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x03, 0x03, 0x00,
    0x02, 0x01, 0x12, 0x03, 0x38, 0x04, 0x1d, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x03, 0x00, 0x02,
    0x01, 0x05, 0x12, 0x03, 0x38, 0x04, 0x0a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x03, 0x00, 0x02,
    0x01, 0x01, 0x12, 0x03, 0x38, 0x0b, 0x18, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x03, 0x00, 0x02,
    0x01, 0x03, 0x12, 0x03, 0x38, 0x1b, 0x1c, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x03, 0x03, 0x00, 0x02,
    0x02, 0x12, 0x03, 0x39, 0x04, 0x18, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x03, 0x00, 0x02, 0x02,
    0x05, 0x12, 0x03, 0x39, 0x04, 0x0a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x03, 0x00, 0x02, 0x02,
    0x01, 0x12, 0x03, 0x39, 0x0b, 0x13, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x03, 0x00, 0x02, 0x02,
    0x03, 0x12, 0x03, 0x39, 0x16, 0x17, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x03, 0x03, 0x00, 0x02, 0x03,
    0x12, 0x03, 0x3a, 0x04, 0x15, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x03, 0x00, 0x02, 0x03, 0x05,
    0x12, 0x03, 0x3a, 0x04, 0x0a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x03, 0x00, 0x02, 0x03, 0x01,
    0x12, 0x03, 0x3a, 0x0b, 0x10, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x03, 0x00, 0x02, 0x03, 0x03,
    0x12, 0x03, 0x3a, 0x13, 0x14, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x00, 0x12, 0x03, 0x3d,
    0x02, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x04, 0x12, 0x03, 0x3d, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x06, 0x12, 0x03, 0x3d, 0x0b, 0x1d, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x01, 0x12, 0x03, 0x3d, 0x1e, 0x27, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x00, 0x03, 0x12, 0x03, 0x3d, 0x2a, 0x2b, 0x0a, 0x38, 0x0a, 0x02, 0x04, 0x04,
    0x12, 0x04, 0x41, 0x00, 0x49, 0x01, 0x1a, 0x2c, 0x20, 0x43, 0x6f, 0x6e, 0x74, 0x61, 0x69, 0x6e,
    0x73, 0x20, 0x61, 0x20, 0x62, 0x61, 0x74, 0x63, 0x68, 0x20, 0x6f, 0x66, 0x20, 0x73, 0x65, 0x72,
    0x69, 0x61, 0x6c, 0x69, 0x7a, 0x65, 0x64, 0x20, 0x63, 0x72, 0x65, 0x64, 0x65, 0x6e, 0x74, 0x69,
    0x61, 0x6c, 0x73, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x04, 0x01, 0x12, 0x03, 0x41, 0x08, 0x26,
    0x0a, 0x3a, 0x0a, 0x04, 0x04, 0x04, 0x03, 0x00, 0x12, 0x04, 0x43, 0x02, 0x46, 0x03, 0x1a, 0x2c,
    0x20, 0x57, 0x72, 0x61, 0x70, 0x70, 0x65, 0x72, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x65, 0x61, 0x63,
    0x68, 0x20, 0x63, 0x72, 0x65, 0x64, 0x65, 0x6e, 0x74, 0x69, 0x61, 0x6c, 0x20, 0x61, 0x6e, 0x64,
    0x20, 0x70, 0x75, 0x62, 0x6c, 0x69, 0x63, 0x20, 0x6b, 0x65, 0x79, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x04, 0x03, 0x00, 0x01, 0x12, 0x03, 0x43, 0x0a, 0x14, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x04,
    0x03, 0x00, 0x02, 0x00, 0x12, 0x03, 0x44, 0x04, 0x2c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x03,
    0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x44, 0x04, 0x09, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x03,
    0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x44, 0x0a, 0x27, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x03,
    0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x44, 0x2a, 0x2b, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x04, 0x03,
    0x00, 0x02, 0x01, 0x12, 0x03, 0x45, 0x04, 0x27, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x03, 0x00,
    0x02, 0x01, 0x05, 0x12, 0x03, 0x45, 0x04, 0x09, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x03, 0x00,
    0x02, 0x01, 0x01, 0x12, 0x03, 0x45, 0x0a, 0x22, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x03, 0x00,
    0x02, 0x01, 0x03, 0x12, 0x03, 0x45, 0x25, 0x26, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x00,
    0x12, 0x03, 0x48, 0x02, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x04, 0x12, 0x03,
    0x48, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x06, 0x12, 0x03, 0x48, 0x0b,
    0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x01, 0x12, 0x03, 0x48, 0x16, 0x21, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x03, 0x12, 0x03, 0x48, 0x24, 0x25, 0x0a, 0x38, 0x0a,
    0x02, 0x04, 0x05, 0x12, 0x04, 0x4c, 0x00, 0x56, 0x01, 0x1a, 0x2c, 0x20, 0x52, 0x65, 0x73, 0x70,
    0x6f, 0x6e, 0x73, 0x65, 0x20, 0x74, 0x6f, 0x20, 0x56, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x65,
    0x42, 0x61, 0x73, 0x69, 0x63, 0x49, 0x64, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x69, 0x65, 0x73, 0x52,
    0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x05, 0x01, 0x12, 0x03,
    0x4c, 0x08, 0x27, 0x0a, 0x38, 0x0a, 0x04, 0x04, 0x05, 0x03, 0x00, 0x12, 0x04, 0x4e, 0x02, 0x53,
    0x03, 0x1a, 0x2a, 0x20, 0x41, 0x6e, 0x20, 0x69, 0x6e, 0x64, 0x69, 0x76, 0x69, 0x64, 0x75, 0x61,
    0x6c, 0x20, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x20, 0x74, 0x6f, 0x20, 0x6f, 0x6e,
    0x65, 0x20, 0x63, 0x72, 0x65, 0x64, 0x65, 0x6e, 0x74, 0x69, 0x61, 0x6c, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x05, 0x03, 0x00, 0x01, 0x12, 0x03, 0x4e, 0x0a, 0x1c, 0x0a, 0x0d, 0x0a, 0x06, 0x04,
    0x05, 0x03, 0x00, 0x02, 0x00, 0x12, 0x03, 0x4f, 0x04, 0x13, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x05,
    0x03, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x4f, 0x04, 0x08, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x05,
    0x03, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x4f, 0x09, 0x0e, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x05,
    0x03, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x4f, 0x11, 0x12, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x05,
    0x03, 0x00, 0x02, 0x01, 0x12, 0x03, 0x50, 0x04, 0x1d, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x05, 0x03,
    0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x50, 0x04, 0x0a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x05, 0x03,
    0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x50, 0x0b, 0x18, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x05, 0x03,
    0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x50, 0x1b, 0x1c, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x05, 0x03,
    0x00, 0x02, 0x02, 0x12, 0x03, 0x51, 0x04, 0x1f, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x05, 0x03, 0x00,
    0x02, 0x02, 0x05, 0x12, 0x03, 0x51, 0x04, 0x0a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x05, 0x03, 0x00,
    0x02, 0x02, 0x01, 0x12, 0x03, 0x51, 0x0b, 0x1a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x05, 0x03, 0x00,
    0x02, 0x02, 0x03, 0x12, 0x03, 0x51, 0x1d, 0x1e, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x05, 0x03, 0x00,
    0x02, 0x03, 0x12, 0x03, 0x52, 0x04, 0x1e, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x05, 0x03, 0x00, 0x02,
    0x03, 0x05, 0x12, 0x03, 0x52, 0x04, 0x0a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x05, 0x03, 0x00, 0x02,
    0x03, 0x01, 0x12, 0x03, 0x52, 0x0b, 0x19, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x05, 0x03, 0x00, 0x02,
    0x03, 0x03, 0x12, 0x03, 0x52, 0x1c, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x00, 0x12,
    0x03, 0x55, 0x02, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x04, 0x12, 0x03, 0x55,
    0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x06, 0x12, 0x03, 0x55, 0x0b, 0x1d,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x01, 0x12, 0x03, 0x55, 0x1e, 0x27, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x03, 0x12, 0x03, 0x55, 0x2a, 0x2b, 0x62, 0x06, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x33,
];
include!("xmtp.mls_validation.v1.serde.rs");
include!("xmtp.mls_validation.v1.tonic.rs");
// @@protoc_insertion_point(module)
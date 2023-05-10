use crate::authenticator::{client_pin::AuthProtocolVersion, Sha256Hash};
use bounded_vec::BoundedVec;
use fido_common::credential::public_key;
use std::{collections::HashMap, usize};

pub enum Error {
    OperationDenied,
    PinNotSet,
    PinInvalid,
    InvalidParameter,
    MissingParameter,
    InvalidOption,
    UnsupportedOption,
    PinUvAuthTokenRequired,
    PinAuthInvalid,
    UserActionTimeout,
    PinBlocked,
    NoCredentials,
}

/// > The following option keys are defined for use in
/// > [`assertion::get::Request`]'s `options` parameter.
pub enum OptionKey {
    /// > user presence: Instructs the authenticator to require user consent
    /// > to complete the operation.
    UserPrecense,
    /// > user verification: If true, instructs the authenticator to require
    /// > a user-verifying gesture in order to complete the request.
    /// > Examples of such gestures are fingerprint scan or a PIN.
    UserVerification,
}

/// Request parameters for [`Ctap2Device::get_assertion`] operation.
#[derive(Clone, Copy)]
pub struct Request<'a> {
    /// > relying party identifier
    pub relying_party_id: &'a str,
    /// > Hash of the serialized client data collected by the host.
    pub client_data_hash: &'a Sha256Hash,
    /// > An array of [`public_key::Descriptor`] structures, each denoting a
    /// > credential, as specified in `WebAuthn`. A platform MUST NOT send
    /// > an empty `allowList`—if it would be empty it MUST be omitted. If
    /// > this parameter is present the authenticator MUST only generate an
    /// > assertion using one of the denoted credentials.
    // TODO: Is there an equivalent of Vec1 but for a slice?
    pub allow_list: Option<&'a BoundedVec<public_key::Descriptor, 1, { usize::MAX }>>,
    /// > Parameters to influence authenticator operation. These parameters
    /// > might be authenticator specific.
    pub extensions: Option<&'a HashMap<fido_common::extension::Identifier, Vec<u8>>>,
    /// > Parameters to influence authenticator operation.
    pub options: Option<&'a HashMap<OptionKey, bool>>,
    pub pin_uv_auth_param: Option<&'a [u8]>,
    /// > PIN/UV protocol version selected by platform.
    pub pin_uv_auth_protocol_version: Option<AuthProtocolVersion>,
}

/// Response structure for [`Ctap2Device::get_assertion`] operation.
pub struct Response {
    /// > PublicKeyCredentialDescriptor structure containing the credential
    /// > identifier whose private key was used to generate the assertion.
    pub credential: public_key::Descriptor,
    /// > The signed-over contextual bindings made by the authenticator, as
    /// > specified in [WebAuthn].
    pub auth_data: Vec<u8>,
    /// > The assertion signature produced by the authenticator, as
    /// > specified in [WebAuthn].
    pub signature: Vec<u8>,
    /// > [`public_key::UserEntity`] structure containing the user account
    /// > information
    pub user: Option<public_key::UserEntity>,
    /// > Total number of account credentials for the RP. Optional; defaults
    /// > to one. This member is required when more than one credential is
    /// > found for an RP, and the authenticator does not have a display or
    /// > the UV & UP flags are false.
    pub number_of_credentials: Option<usize>,
    /// > Indicates that a credential was selected by the user via
    /// > interaction directly with the authenticator, and thus the platform
    /// > does not need to confirm the credential.
    pub user_selected: Option<bool>,
    /// > The contents of the associated `largeBlobKey` if present for the
    /// > asserted credential, and if `largeBlobKey` was true in the
    /// > extensions input.
    pub large_blob_key: Option<Vec<u8>>,
}
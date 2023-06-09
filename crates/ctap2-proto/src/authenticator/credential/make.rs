use crate::{
    authenticator::{self, client_pin},
    extensions, Sha256Hash,
};
use fido_common::{attestation, credential::public_key};
use std::collections::BTreeMap;
use typed_builder::TypedBuilder;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "serde")]
use serde_with::{serde_as, skip_serializing_none, Bytes};

#[derive(Debug)]
pub enum Error {
    OperationDenied,
    PinNotSet,
    PinInvalid,
    InvalidParameter,
    MissingParameter,
    UnsupportedAlgorithm,
    InvalidOption,
    UnsupportedOption,
    PinUvAuthTokenRequired,
    PinAuthInvalid,
    UserActionTimeout,
    PinBlocked,
    CredentialExcluded,
    KeyStoreFull,
}

/// > The following option keys are defined for use in
/// > `authenticatorMakeCredential`'s `options` parameter.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum OptionKey {
    /// > Specifies whether this credential is to be discoverable or
    /// > not.
    #[cfg_attr(feature = "serde", serde(rename = "rk"))]
    Discoverable,
    /// > user presence: Instructs the authenticator to require user
    /// > consent
    /// > to complete the operation.
    #[cfg_attr(feature = "serde", serde(rename = "up"))]
    UserPresence,
    /// > user verification: If true, instructs the authenticator to require a
    /// > user-verifying gesture in order to complete the request. Examples of
    /// > such gestures are fingerprint scan or a PIN.
    #[cfg_attr(feature = "serde", serde(rename = "uv"))]
    UserVerification,
}

#[cfg_eval]
/// Input parameters for [`Ctap2Device::make_credential`] operation.
#[cfg_attr(feature = "serde", serde_as, skip_serializing_none, derive(Serialize))]
#[derive(Debug, Clone, Copy, TypedBuilder)]
pub struct Request<'a> {
    /// > Hash of the ClientData contextual binding specified by host.
    #[cfg_attr(feature = "serde", serde(rename = 0x01), serde_as(as = "Bytes"))]
    pub client_data_hash: &'a Sha256Hash,
    /// > This PublicKeyCredentialRpEntity data structure describes a
    /// > Relying Party with which the new public key credential will be
    /// > associated.
    #[cfg_attr(feature = "serde", serde(rename = 0x02))]
    pub relying_party: &'a public_key::RelyingPartyEntity,
    /// > ... describes the user account to which the new public key
    /// > credential will be associated at the RP.
    #[cfg_attr(feature = "serde", serde(rename = 0x03))]
    pub user: &'a public_key::UserEntity,
    /// > List of supported algorithms for credential generation, as
    /// > specified in [WebAuthn]. The array is ordered from most preferred
    /// > to least preferred and MUST NOT include duplicate entries.
    #[cfg_attr(feature = "serde", serde(rename = 0x04))]
    pub public_key_credential_params: &'a [public_key::Parameters], // TODO: BTreeSet? BTreeMap
    // with preference as key?
    /// > An array of PublicKeyCredentialDescriptor structures, as specified
    /// > in [WebAuthn]. The authenticator returns an error if the
    /// > authenticator already contains one of the credentials enumerated
    /// > in this array. This allows RPs to limit the creation of multiple
    /// > credentials for the same account on a single authenticator.
    #[builder(default, setter(strip_option))]
    #[cfg_attr(feature = "serde", serde(rename = 0x05))]
    pub exclude_list: Option<&'a [&'a public_key::Descriptor]>,
    /// > Parameters to influence authenticator operation, as specified in
    /// > [WebAuthn]. These parameters might be authenticator specific.
    #[builder(default, setter(strip_option))]
    #[cfg_attr(feature = "serde", serde(rename = 0x06))]
    pub extensions: Option<&'a BTreeMap<extensions::Identifier, Vec<u8>>>,
    #[builder(default, setter(strip_option))]
    #[cfg_attr(feature = "serde", serde(rename = 0x07))]
    pub options: Option<&'a BTreeMap<OptionKey, bool>>,
    #[builder(default, setter(strip_option))]
    #[cfg_attr(
        feature = "serde",
        serde(rename = 0x08),
        serde_as(as = "Option<Bytes>")
    )]
    pub pin_uv_auth_param: Option<&'a [u8]>,
    /// > PIN/UV protocol version selected by platform.
    #[builder(default, setter(strip_option))]
    #[cfg_attr(feature = "serde", serde(rename = 0x09))]
    pub pin_uv_auth_protocol_version: Option<client_pin::auth_protocol::Version>,
    /// > An authenticator supporting this enterprise attestation feature is
    /// > enterprise attestation capable and signals its support via the `ep`
    /// > Option ID in the `authenticatorGetInfo` command response.
    /// >
    /// > If the `enterpriseAttestation` parameter is absent, attestation’s
    /// > privacy characteristics are unaffected, regardless of whether the
    /// > enterprise attestation feature is presently enabled.
    /// >
    /// > If present with a valid value, the usual privacy concerns around
    /// > attestation batching may not apply to the results of this operation
    /// > and the platform is requesting an enterprise attestation that includes
    /// > uniquely identifying information.
    #[builder(default, setter(strip_option))]
    #[cfg_attr(feature = "serde", serde(rename = 0x0A))]
    pub enterprise_attestation: Option<attestation::enterprise::Kind>,
}

#[cfg_attr(feature = "serde", derive(Deserialize))]
#[derive(Debug)]
pub struct Response {
    #[cfg_attr(feature = "serde", serde(rename = 0x01))]
    pub format: fido_common::attestation::FormatIdentifier,
    #[cfg_attr(feature = "serde", serde(rename = 0x02))]
    pub authenticator_data: authenticator::Data,
    #[cfg_attr(feature = "serde", serde(rename = 0x03))]
    pub attestation_statement: Option<attestation::Statement>,
    /// > Indicates whether an enterprise attestation was returned for this
    /// > credential. If `epAtt` is absent or present and set to false, then an
    /// > enterprise attestation was not returned. If `epAtt` is present and set
    /// > to true, then an enterprise attestation was returned.
    #[cfg_attr(feature = "serde", serde(rename = 0x04))]
    pub enterprise_attestation: Option<bool>,
    /// > Contains the `largeBlobKey` for the credential, if requested with the
    /// > `largeBlobKey` extension.
    #[cfg_attr(feature = "serde", serde(rename = 0x05))]
    pub large_blob_key: Option<Vec<u8>>,
    /// > A map, keyed by extension identifiers, to unsigned outputs of
    /// > extensions, if any.
    #[cfg_attr(feature = "serde", serde(rename = 0x06))]
    pub unsigned_extension_outputs: Option<BTreeMap<extensions::Identifier, Vec<u8>>>,
}

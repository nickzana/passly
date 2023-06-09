use bounded_integer::BoundedUsize;
use serde_with::{Bytes, DeserializeAs, SerializeAs};
use std::{borrow::Cow, collections::BTreeSet};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub mod auth_protocol;

#[cfg(feature = "serde")]
mod raw;

#[cfg(feature = "serde")]
use raw::{RawRequest, RawResponse};

pub type PinUvAuthParam = [u8; 16];

#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum PinUvAuthToken {
    Short([u8; 16]),
    Long([u8; 32]),
}

impl AsRef<[u8]> for PinUvAuthToken {
    fn as_ref(&self) -> &[u8] {
        match self {
            PinUvAuthToken::Short(bytes) => bytes.as_ref(),
            PinUvAuthToken::Long(bytes) => bytes.as_ref(),
        }
    }
}

#[cfg(feature = "serde")]
impl TryFrom<&[u8]> for PinUvAuthToken {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        match value.len() {
            16 => {
                let mut short = [0; 16];
                short.copy_from_slice(value);
                Ok(Self::Short(short))
            }
            32 => {
                let mut long = [0; 32];
                long.copy_from_slice(value);
                Ok(Self::Long(long))
            }
            _ => Err(Error::InvalidParameter),
        }
    }
}

#[cfg(feature = "serde")]
impl<'de> DeserializeAs<'de, PinUvAuthToken> for Bytes {
    fn deserialize_as<D>(deserializer: D) -> Result<PinUvAuthToken, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let bytes: Vec<u8> = Bytes::deserialize_as(deserializer)?;
        PinUvAuthToken::try_from(bytes.as_ref()).map_err(serde::de::Error::custom)
    }
}

#[cfg(feature = "serde")]
impl SerializeAs<PinUvAuthToken> for Bytes {
    fn serialize_as<S>(source: &PinUvAuthToken, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match source {
            PinUvAuthToken::Short(bytes) => Bytes::serialize_as(bytes, serializer),
            PinUvAuthToken::Long(bytes) => Bytes::serialize_as(bytes, serializer),
        }
    }
}
#[derive(Clone, Debug)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(into = "RawRequest", try_from = "RawRequest")
)]
pub enum Request<'a> {
    GetPinRetries,
    GetKeyAgreement {
        version: auth_protocol::Version,
    },
    SetPin {
        version: auth_protocol::Version,
        key_agreement: cosey::PublicKey,
        new_pin_encrypted: [u8; 64],
        pin_uv_auth_param: PinUvAuthParam,
    },
    ChangePin {
        version: auth_protocol::Version,
        key_agreement: cosey::PublicKey,
        pin_hash_encrypted: [u8; 16],
        new_pin_encrypted: [u8; 64],
        pin_uv_auth_param: PinUvAuthParam,
    },
    GetPinToken {
        version: auth_protocol::Version,
        key_agreement: cosey::PublicKey,
        pin_hash_encrypted: [u8; 16],
    },
    GetPinUvAuthTokenUsingUvWithPermissions {
        version: auth_protocol::Version,
        key_agreement: cosey::PublicKey,
        permissions: &'a BTreeSet<Permission>, // TODO: Enforce non-empty set?
        relying_party_id: Option<Cow<'a, str>>,
    },
    GetUvRetries,
    GetPinUvAuthTokenUsingPinWithPermissions {
        version: auth_protocol::Version,
        key_agreement: cosey::PublicKey,
        pin_hash_encrypted: [u8; 16],
        permissions: &'a BTreeSet<Permission>, // TODO: Enforce non-empty set?
        relying_party_id: Option<Cow<'a, str>>,
    },
}

#[derive(Clone, Debug)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(into = "RawResponse", try_from = "RawResponse")
)]
pub enum Response {
    GetPinRetries {
        pin_retries: usize,
        power_cycle_state: Option<usize>,
    },
    GetKeyAgreement {
        key_agreement: cosey::PublicKey,
    },
    SetPin,
    ChangePin,
    GetPinToken {
        pin_uv_auth_token: PinUvAuthToken,
    },
    GetPinUvAuthTokenUsingUvWithPermissions {
        /// > The pinUvAuthToken, encrypted by calling encrypt with the shared
        /// > secret as the key.
        pin_uv_auth_token: PinUvAuthToken,
    },
    GetUvRetries {
        /// > Number of uv attempts remaining before lockout.
        ///
        /// > The `uv_retries` counter represents the number of user
        /// > verification attempts left before built-in user verification is
        /// > disabled.
        uv_retries: BoundedUsize<1, 25>,
    },
    GetPinUvAuthTokenUsingPinWithPermissions {
        /// > The pinUvAuthToken, encrypted by calling encrypt with the shared
        /// > secret as the key.
        pin_uv_auth_token: PinUvAuthToken,
    },
}

#[derive(Debug, Clone, Copy)]
pub enum Error {
    MissingParameter,
    InvalidParameter,
    PinAuthInvalid,
    PinPolicyViolation,
    PinBlocked,
    PinAuthBlocked,
    PinInvalid,
    OperationDenied,
    UnauthorizedPermission,
    NotAllowed,
    UserVerificationBlocked,
    UserActionTimeout,
    UserVerificationInvalid,
}

/// > When obtaining a `pinUvAuthToken`, the platform requests permissions
/// > appropriate for the operations it intends to perform. Consequently, the
/// > `pinUvAuthToken` can only be used for those operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Permission {
    /// > This allows the `pinUvAuthToken` to be used for
    /// > `authenticatorMakeCredential` operations with the provided `rpId`
    /// > parameter.
    MakeCredential,
    /// > This allows the `pinUvAuthToken` to be used for
    /// > `authenticatorGetAssertion` operations with the provided `rpId`
    /// > parameter.
    GetAssertion,
    /// > This allows the `pinUvAuthToken` to be used with the
    /// > `authenticatorCredentialManagement` command. The `rpId` parameter is
    /// > optional, if it is present, the `pinUvAuthToken` can only be used for
    /// > Credential Management operations on Credentials associated with that
    /// > RP ID.
    CredentialManagement,
    /// > This allows the `pinUvAuthToken` to be used with the
    /// > `authenticatorBioEnrollment` command.
    BiometricEnrollment,
    /// > This allows the `pinUvAuthToken` to be used with the
    /// > `authenticatorLargeBlobs` command.
    LargeBlobWrite,
    /// > This allows the `pinUvAuthToken` to be used with the
    /// > `authenticatorConfig` command.
    AuthenticatorConfiguration,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::MissingParameter => write!(f, "Missing parameter"),
            Error::InvalidParameter => write!(f, "Invalid parameter"),
            Error::PinAuthInvalid => write!(f, "PIN auth invalid"),
            Error::PinPolicyViolation => write!(f, "PIN policy violation"),
            Error::PinBlocked => write!(f, "PIN blocked"),
            Error::PinAuthBlocked => write!(f, "PIN auth blocked"),
            Error::PinInvalid => write!(f, "PIN invalid"),
            Error::OperationDenied => write!(f, "Operation denied"),
            Error::UnauthorizedPermission => write!(f, "Unauthorized permission"),
            Error::NotAllowed => write!(f, "Not allowed"),
            Error::UserVerificationBlocked => write!(f, "User verification blocked"),
            Error::UserActionTimeout => write!(f, "User action timeout"),
            Error::UserVerificationInvalid => write!(f, "User verification invalid"),
        }
    }
}

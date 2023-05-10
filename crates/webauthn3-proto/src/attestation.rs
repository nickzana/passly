#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Debug)]
/// > WebAuthn Relying Parties may use AttestationConveyancePreference to
/// > specify their preference regarding attestation conveyance during
/// > credential generation.
/// >
/// > <https://w3c.github.io/webauthn/#enum-attestation-convey/>
pub enum ConveyancePreference {
    /// > The Relying Party is not interested in authenticator attestation.
    #[cfg_attr(feature = "serde", serde(rename = "none"))]
    None,
    /// > The Relying Party wants to receive a verifiable attestation statement,
    /// > but allows the client to decide how to obtain such an attestation
    /// > statement.
    #[cfg_attr(feature = "serde", serde(rename = "indirect"))]
    Indirect,
    /// > The Relying Party wants to receive the attestation statement as
    /// > generated by the authenticator.
    #[cfg_attr(feature = "serde", serde(rename = "direct"))]
    Direct,
    /// > The Relying Party wants to receive an attestation statement that may
    /// > include uniquely identifying information.
    #[cfg_attr(feature = "serde", serde(rename = "enterprise"))]
    Enterprise,
}

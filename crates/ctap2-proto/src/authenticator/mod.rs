use crate::attestation;

pub mod assertion;
pub mod bio_enrollment;
pub mod client_pin;
pub mod config;
pub mod credential;
pub mod device;
pub mod reset;
pub mod selection;

/// SHA 256 hash values are 32 bytes long.
pub type Sha256Hash = [u8; 32];

/// > The authenticator data structure encodes contextual bindings made by the
/// > authenticator. These bindings are controlled by the authenticator itself,
/// > and derive their trust from the `WebAuthn` Relying Party's assessment of
/// > the security properties of the authenticator. In one extreme case, the
/// > authenticator may be embedded in the client, and its bindings may be no
/// > more trustworthy than the client data. At the other extreme, the
/// > authenticator may be a discrete entity with high-security hardware and
/// > software, connected to the client over a secure channel. In both cases,
/// > the Relying Party receives the authenticator data in the same format, and
/// > uses its knowledge of the authenticator to make trust decisions.
/// >
/// > The authenticator data has a compact but extensible encoding. This is
/// > desired since authenticators can be devices with limited capabilities and
/// > low power requirements, with much simpler software stacks than the client
/// > platform.
pub struct Data {
    /// > SHA-256 hash of the RP ID the credential is scoped to.
    pub relying_party_id_hash: Sha256Hash,
    pub user_is_present: bool,
    pub user_is_verified: bool,
    pub signature_counter: u32,
    pub attested_credential_data: attestation::CredentialData,
    // TODO: extensions
}
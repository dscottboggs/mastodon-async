/// Represents a canonical email block (hashed).
///
/// See also [the API documentation](https://docs.joinmastodon.org/entities/Admin_CanonicalEmailBlock/)
pub struct CanonicalEmailBlock {
    /// The ID of the email block in the database.
    pub id: CanonicalEmailBlockId,
    /// The SHA256 hash of the canonical email address.
    pub canonical_email_hash: String,
}

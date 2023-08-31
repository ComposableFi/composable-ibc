use jsonwebtoken::{encode, get_current_timestamp, Algorithm, EncodingKey, Header};
use serde::{Deserialize, Serialize};

/// Default algorithm used for JWT token signing.
const DEFAULT_ALGORITHM: Algorithm = Algorithm::HS256;

/// JWT secret length in bytes.
pub const JWT_SECRET_LENGTH: usize = 32;

/// Generates a bearer token from a JWT secret
pub struct JwtKey([u8; JWT_SECRET_LENGTH]);

impl JwtKey {
	/// Wrap given slice in `Self`. Returns an error if slice.len() != `JWT_SECRET_LENGTH`.
	pub fn from_slice(key: &[u8]) -> Result<Self, String> {
		if key.len() != JWT_SECRET_LENGTH {
			return Err(format!(
				"Invalid key length. Expected {} got {}",
				JWT_SECRET_LENGTH,
				key.len()
			))
		}
		let mut res = [0; JWT_SECRET_LENGTH];
		res.copy_from_slice(key);
		Ok(Self(res))
	}

	/// Decode the given string from hex (no 0x prefix), and attempt to create a key from it.
	pub fn from_hex(hex: &str) -> Result<Self, String> {
		let bytes = hex::decode(hex).map_err(|e| format!("Invalid hex: {}", e))?;
		Self::from_slice(&bytes)
	}

	/// Returns a reference to the underlying byte array.
	pub fn as_bytes(&self) -> &[u8; JWT_SECRET_LENGTH] {
		&self.0
	}

	/// Consumes the key, returning its underlying byte array.
	pub fn into_bytes(self) -> [u8; JWT_SECRET_LENGTH] {
		self.0
	}
}

/// Contains the JWT secret and claims parameters.
pub struct JwtAuth {
	key: EncodingKey,
	id: Option<String>,
	clv: Option<String>,
}

impl JwtAuth {
	/// Create a new [JwtAuth] from a secret key, and optional `id` and `clv` claims.
	pub fn new(secret: JwtKey, id: Option<String>, clv: Option<String>) -> Self {
		Self { key: EncodingKey::from_secret(secret.as_bytes()), id, clv }
	}

	/// Generate a JWT token with `claims.iat` set to current time.
	pub fn generate_token(&self) -> Result<String, anyhow::Error> {
		let claims = self.generate_claims_at_timestamp();
		self.generate_token_with_claims(&claims)
	}

	/// Generate a JWT token with the given claims.
	fn generate_token_with_claims(&self, claims: &Claims) -> Result<String, anyhow::Error> {
		let header = Header::new(DEFAULT_ALGORITHM);
		Ok(encode(&header, claims, &self.key)?)
	}

	/// Generate a `Claims` struct with `iat` set to current time
	fn generate_claims_at_timestamp(&self) -> Claims {
		Claims { iat: get_current_timestamp(), id: self.id.clone(), clv: self.clv.clone() }
	}

	/// Validate a JWT token given the secret key and return the originally signed `TokenData`.
	pub fn validate_token(
		token: &str,
		secret: &JwtKey,
	) -> Result<jsonwebtoken::TokenData<Claims>, anyhow::Error> {
		let mut validation = jsonwebtoken::Validation::new(DEFAULT_ALGORITHM);
		validation.validate_exp = false;
		validation.required_spec_claims.remove("exp");

		Ok(jsonwebtoken::decode::<Claims>(
			token,
			&jsonwebtoken::DecodingKey::from_secret(secret.as_bytes()),
			&validation,
		)?)
	}
}

/// Claims struct as defined in <https://github.com/ethereum/execution-apis/blob/main/src/engine/authentication.md#jwt-claims>
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Claims {
	/// issued-at claim. Represented as seconds passed since UNIX_EPOCH.
	iat: u64,
	/// Optional unique identifier for the CL node.
	id: Option<String>,
	/// Optional client version for the CL node.
	clv: Option<String>,
}

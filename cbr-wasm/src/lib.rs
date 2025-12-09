extern crate challenge_bypass_ristretto;
extern crate hmac;
extern crate cbr_macros;
extern crate rand;
extern crate sha2;
extern crate wasm_bindgen;

use hmac::Hmac;
use rand::rngs::OsRng;
use sha2::Sha512;
use wasm_bindgen::prelude::*;

use challenge_bypass_ristretto::errors::TokenError;
use challenge_bypass_ristretto::voprf::{
    BatchDLEQProof as _BatchDLEQProof, BlindedToken as _BlindedToken, PublicKey as _PublicKey,
    SignedToken as _SignedToken, SigningKey as _SigningKey, Token as _Token,
    TokenPreimage as _TokenPreimage, UnblindedToken as _UnblindedToken,
    VerificationKey as _VerificationKey, VerificationSignature as _VerificationSignature,
};

use cbr_macros::Base64;

type HmacSha512 = Hmac<Sha512>;

fn convert_error(e: TokenError) -> JsValue {
    JsValue::from_str(&e.to_string())
}

#[wasm_bindgen]
#[derive(Base64)]
pub struct TokenPreimage(_TokenPreimage);

#[wasm_bindgen]
#[derive(Base64)]
pub struct Token(_Token);

#[wasm_bindgen]
#[derive(Base64)]
pub struct BlindedToken(_BlindedToken);

#[wasm_bindgen]
#[derive(Base64)]
pub struct PublicKey(_PublicKey);

#[wasm_bindgen]
#[derive(Base64)]
pub struct SigningKey(_SigningKey);

#[wasm_bindgen]
#[derive(Base64)]
pub struct SignedToken(_SignedToken);

#[wasm_bindgen]
#[derive(Base64)]
pub struct UnblindedToken(_UnblindedToken);

#[wasm_bindgen]
pub struct VerificationKey(_VerificationKey);

#[wasm_bindgen]
#[derive(Base64)]
pub struct VerificationSignature(_VerificationSignature);

#[wasm_bindgen]
#[derive(Base64)]
pub struct BatchDLEQProof(_BatchDLEQProof);

#[wasm_bindgen]
impl Token {
    /// Generates a new random `Token` using the provided random number generator.
    pub fn random() -> Token {
        let mut rng = OsRng;
        Token(_Token::random::<Sha512, OsRng>(&mut rng))
    }

    /// Blinds the `Token`, returning a `BlindedToken` to be sent to the server.
    pub fn blind(&self) -> BlindedToken {
        BlindedToken(self.0.blind())
    }
}

#[wasm_bindgen]
impl SigningKey {
    /// Generates a new random `SigningKey` using the provided random number generator.
    pub fn random() -> SigningKey {
        let mut rng = OsRng;
        SigningKey(_SigningKey::random::<OsRng>(&mut rng))
    }

    /// Signs the provided `BlindedToken`
    ///
    /// Returns None if the `BlindedToken` point is not valid.
    #[allow(non_snake_case)]
    pub fn sign(&self, P: &BlindedToken) -> Result<SignedToken, JsValue> {
        self.0
            .sign(&P.0)
            .map(|p| p.into())
            .map_err(|e| convert_error(e))
    }

    /// Rederives an `UnblindedToken` via the token preimage of the provided `UnblindedToken`
    ///
    /// W' = T^k = H_1(t)^k
    pub fn rederive_unblinded_token(&self, t: &TokenPreimage) -> UnblindedToken {
        UnblindedToken(self.0.rederive_unblinded_token(&t.0))
    }

    /// Return the `PublicKey` for this `SigningKey`
    pub fn public_key(&self) -> PublicKey {
        PublicKey(self.0.public_key)
    }
}

#[wasm_bindgen]
impl UnblindedToken {
    /// Derive the `VerificationKey` for this particular `UnblindedToken`
    pub fn derive_verification_key_sha512(&self) -> VerificationKey {
        VerificationKey(self.0.derive_verification_key::<Sha512>())
    }

    /// Return the `TokenPreimage` for this particular `UnblindedToken`
    pub fn preimage(&self) -> TokenPreimage {
        TokenPreimage(self.0.t)
    }
}

#[wasm_bindgen]
impl VerificationKey {
    /// Use the `VerificationKey` to "sign" a message, producing a `VerificationSignature`
    pub fn sign_sha512(&self, message: &[u8]) -> VerificationSignature {
        VerificationSignature(self.0.sign::<HmacSha512>(message))
    }

    /// Use the `VerificationKey` to check that the signature of a message matches the
    /// provided `VerificationSignature`
    pub fn verify_sha512(&self, sig: &VerificationSignature, message: &[u8]) -> bool {
        self.0.verify::<HmacSha512>(&sig.0, message)
    }
}

#[wasm_bindgen]
impl BatchDLEQProof {
    /// Construct a new `BatchDLEQProof`, takes tokens as a comma separated list of b64 encoded strings
    pub fn create_from_encoded(
        blinded_tokens: String,
        signed_tokens: String,
        signing_key: &SigningKey,
    ) -> Result<BatchDLEQProof, JsValue> {
        let mut rng = OsRng;
        let blinded_tokens: Vec<_BlindedToken> = blinded_tokens
            .split(",")
            .map(|s| _BlindedToken::decode_base64(s).unwrap())
            .collect();
        let signed_tokens: Vec<_SignedToken> = signed_tokens
            .split(",")
            .map(|s| _SignedToken::decode_base64(s).unwrap())
            .collect();
        _BatchDLEQProof::new::<Sha512, _>(
            &mut rng,
            &blinded_tokens,
            &signed_tokens,
            &signing_key.0,
        )
        .map(|p| p.into())
        .map_err(|e| convert_error(e))
    }

    /// Verify the `BatchDLEQProof` returning a comma separated list of b64 encoded unblinded tokens,
    /// takes tokens as a comma separated list of b64 encoded strings
    pub fn verify_and_unblind_from_encoded(
        &self,
        tokens: String,
        blinded_tokens: String,
        signed_tokens: String,
        public_key: &PublicKey,
    ) -> Result<String, JsValue> {
        let tokens: Vec<_Token> = tokens
            .split(",")
            .map(|s| _Token::decode_base64(s).unwrap())
            .collect();
        let blinded_tokens: Vec<_BlindedToken> = blinded_tokens
            .split(",")
            .map(|s| _BlindedToken::decode_base64(s).unwrap())
            .collect();
        let signed_tokens: Vec<_SignedToken> = signed_tokens
            .split(",")
            .map(|s| _SignedToken::decode_base64(s).unwrap())
            .collect();
        match self.0.verify_and_unblind::<Sha512, _>(
            &tokens,
            &blinded_tokens,
            &signed_tokens,
            &public_key.0,
        ) {
            Ok(unblinded_tokens) => {
                let unblinded_tokens: Vec<String> =
                    unblinded_tokens.iter().map(|t| t.encode_base64()).collect();
                Ok(unblinded_tokens.join(","))
            }
            Err(err) => Err(convert_error(err)),
        }
    }
}

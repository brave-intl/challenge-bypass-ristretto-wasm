const { SigningKey, Token, BatchDLEQProof, UnblindedToken } = require('./pkg')

const msg = Buffer.from('test message')

const sKey = SigningKey.random()
const pKey = sKey.public_key()

const token1 = Token.random()
const token2 = Token.random()

const tokens = token1.encode_base64() + ',' + token2.encode_base64()

const blindedToken1 = token1.blind()
const blindedToken2 = token2.blind()

const blindedTokens = blindedToken1.encode_base64() + ',' + blindedToken2.encode_base64()

const signedToken1 = sKey.sign(blindedToken1)
const signedToken2 = sKey.sign(blindedToken2)

const signedTokens = signedToken1.encode_base64() + ',' + signedToken2.encode_base64()

const proof = BatchDLEQProof.create_from_encoded(blindedTokens, signedTokens, sKey)

const unblindedTokens = proof.verify_and_unblind_from_encoded(tokens, blindedTokens, signedTokens, pKey)

const unblindedToken1 = UnblindedToken.decode_base64(unblindedTokens.split(',')[0])

const clientVerificationKey1 = unblindedToken1.derive_verification_key_sha512()

const verificationSignature1 = clientVerificationKey1.sign_sha512(msg)

const serverUnblindedToken1 = sKey.rederive_unblinded_token(unblindedToken1.preimage())

const serverVerificationKey1 = serverUnblindedToken1.derive_verification_key_sha512()

if (!serverVerificationKey1.verify_sha512(verificationSignature1, msg)) {
  console.error('Expected verification signatures to match')
  process.exit(1)
}

if (serverVerificationKey1.verify_sha512(verificationSignature1, Buffer.from('asdf'))) {
  console.error('Expected verification signatures to NOT match')
  process.exit(1)
}

console.log('Success!')

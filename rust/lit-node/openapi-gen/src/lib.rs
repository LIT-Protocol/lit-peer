//! OpenAPI specification generator for Lit Protocol Node API.
//!
//! This crate generates OpenAPI 3.1 specifications from the lit-node-core models.

use utoipa::OpenApi;

mod wrappers;
pub use wrappers::*;

// Re-export types from lit-node-core for schema registration
use lit_node_core::request::{
    EncryptionSignRequest, JsonExecutionRequest, JsonPKPClaimKeyRequest, JsonPKPSigningRequest,
    JsonSignSessionKeyRequestV2, SDKHandshakeRequest,
};
use lit_node_core::response::{
    EncryptionSignResponse, JsonExecutionResponse, JsonPKPClaimKeyResponse, JsonPKPSigningResponse,
    JsonSignSessionKeyResponseV2, SDKHandshakeResponseV0,
};
use lit_node_core::{
    AccessControlBooleanOperator, AuthMaterialType, AuthMethod, AuthSigItem, BlsSignedMessageShare,
    CosmosCondition, CurveType, DynamicPaymentItem, EVMContractCondition, EcdsaSignedMessageShare,
    FrostSignedMessageShare, Invocation, JsonAccessControlCondition,
    JsonAccessControlConditionOperator, JsonAuthSig, JsonReturnValueTest, JsonReturnValueTestV2,
    LitActionPriceComponent, MultipleAuthSigs, NodeSet, SignableOutput, SignedData, SigningScheme,
    SolPdaInterface, SolRpcConditionV2, SolRpcConditionV2Options,
};

/// OpenAPI document for the Lit Protocol Node API.
#[derive(OpenApi)]
#[openapi(
    info(
        title = "Lit Protocol Node API",
        version = "2.0.1",
        description = "API for interacting with Lit Protocol nodes. Provides endpoints for PKP signing, Lit Actions execution, encryption, and session key management.",
        license(name = "Apache-2.0"),
        contact(
            name = "Lit Protocol",
            url = "https://litprotocol.com"
        )
    ),
    servers(
        (url = "https://{node_address}", description = "Lit Protocol Node", variables(
            ("node_address" = (default = "localhost:7470", description = "Node address"))
        ))
    ),
    tags(
        (name = "handshake", description = "Node handshake and connection setup"),
        (name = "encryption", description = "Encryption and decryption operations"),
        (name = "pkp", description = "Programmable Key Pair signing operations"),
        (name = "session", description = "Session key management"),
        (name = "execution", description = "Lit Actions execution")
    ),
    paths(
        handshake,
        encryption_sign,
        pkp_sign,
        sign_session_key,
        execute,
        pkp_claim,
    ),
    components(schemas(
        // Request types
        SDKHandshakeRequest,
        EncryptionSignRequest,
        JsonSignSessionKeyRequestV2,
        JsonPKPSigningRequest,
        JsonExecutionRequest,
        JsonPKPClaimKeyRequest,
        // Response types
        SDKHandshakeResponseV0,
        EncryptionSignResponse,
        JsonSignSessionKeyResponseV2,
        JsonPKPSigningResponse,
        JsonExecutionResponse,
        JsonPKPClaimKeyResponse,
        // Auth types
        AuthMethod,
        AuthMaterialType,
        AuthSigItem,
        MultipleAuthSigs,
        JsonAuthSig,
        // Signature types
        SignableOutput,
        EcdsaSignedMessageShare,
        FrostSignedMessageShare,
        BlsSignedMessageShare,
        SignedData,
        // Access control types
        AccessControlBooleanOperator,
        JsonAccessControlCondition,
        JsonAccessControlConditionOperator,
        JsonReturnValueTest,
        JsonReturnValueTestV2,
        EVMContractCondition,
        SolRpcConditionV2,
        SolRpcConditionV2Options,
        SolPdaInterface,
        CosmosCondition,
        // Other types
        CurveType,
        Invocation,
        NodeSet,
        DynamicPaymentItem,
        LitActionPriceComponent,
        SigningScheme,
        // Wrapper types for external dependencies
        wrappers::FunctionAbiSchema,
        wrappers::AbiParam,
    ))
)]
pub struct ApiDoc;

/// POST /web/handshake - Establish connection with a Lit node
#[utoipa::path(
    post,
    path = "/web/handshake",
    request_body = SDKHandshakeRequest,
    responses(
        (status = 200, description = "Handshake successful", body = SDKHandshakeResponseV0),
        (status = 400, description = "Bad request"),
        (status = 500, description = "Internal server error"),
    ),
    tag = "handshake"
)]
pub async fn handshake() {}

/// POST /web/encryption/sign/v2 - Request encryption key shares
#[utoipa::path(
    post,
    path = "/web/encryption/sign/v2",
    request_body = EncryptionSignRequest,
    responses(
        (status = 200, description = "Encryption sign successful", body = EncryptionSignResponse),
        (status = 400, description = "Bad request - invalid access control conditions"),
        (status = 401, description = "Unauthorized - invalid auth signature"),
        (status = 500, description = "Internal server error"),
    ),
    tag = "encryption"
)]
pub async fn encryption_sign() {}

/// POST /web/pkp/sign/v2 - Sign data with a Programmable Key Pair
#[utoipa::path(
    post,
    path = "/web/pkp/sign/v2",
    request_body = JsonPKPSigningRequest,
    responses(
        (status = 200, description = "PKP signing successful", body = JsonPKPSigningResponse),
        (status = 400, description = "Bad request - invalid signing parameters"),
        (status = 401, description = "Unauthorized - invalid auth signature"),
        (status = 500, description = "Internal server error"),
    ),
    tag = "pkp"
)]
pub async fn pkp_sign() {}

/// POST /web/sign_session_key/v2 - Sign a session key for subsequent requests
#[utoipa::path(
    post,
    path = "/web/sign_session_key/v2",
    request_body = JsonSignSessionKeyRequestV2,
    responses(
        (status = 200, description = "Session key signed successfully", body = JsonSignSessionKeyResponseV2),
        (status = 400, description = "Bad request - invalid session parameters"),
        (status = 401, description = "Unauthorized - invalid auth methods"),
        (status = 500, description = "Internal server error"),
    ),
    tag = "session"
)]
pub async fn sign_session_key() {}

/// POST /web/execute/v2 - Execute a Lit Action
#[utoipa::path(
    post,
    path = "/web/execute/v2",
    request_body = JsonExecutionRequest,
    responses(
        (status = 200, description = "Execution successful", body = JsonExecutionResponse),
        (status = 400, description = "Bad request - invalid code or parameters"),
        (status = 401, description = "Unauthorized - invalid auth signature"),
        (status = 500, description = "Internal server error"),
    ),
    tag = "execution"
)]
pub async fn execute() {}

/// POST /web/pkp/claim - Claim a PKP key
#[utoipa::path(
    post,
    path = "/web/pkp/claim",
    request_body = JsonPKPClaimKeyRequest,
    responses(
        (status = 200, description = "PKP claim successful", body = JsonPKPClaimKeyResponse),
        (status = 400, description = "Bad request - invalid claim parameters"),
        (status = 401, description = "Unauthorized - invalid auth method"),
        (status = 500, description = "Internal server error"),
    ),
    tag = "pkp"
)]
pub async fn pkp_claim() {}

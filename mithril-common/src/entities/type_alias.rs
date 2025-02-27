use std::collections::HashMap;

/// ImmutableFileNumber represents the id of immutable files in the Cardano node database
pub type ImmutableFileNumber = u64;

/// PartyId represents a signing party in Mithril protocol
pub type PartyId = String;

/// Stake represents the stakes of a participant in the Cardano chain
pub type Stake = u64;

/// StakeDistribution represents the stakes of multiple participants in the Cardano chain
pub type StakeDistribution = HashMap<PartyId, Stake>;

/// LotteryIndex represents the index of a Mithril single signature lottery
pub type LotteryIndex = u64;

/// Cardano Network magic identifier
pub type MagicId = u64;

/// Protocol version
pub type ProtocolVersion = String;

/// Hex encoded key
pub type HexEncodedKey = String;

/// Hex encoded Single Signature
pub type HexEncodedSingleSignature = HexEncodedKey;

/// Hex encoded Multi Signature
pub type HexEncodedMultiSignature = HexEncodedKey;

/// Hex encoded Aggregate Verification Key
pub type HexEncodedAgregateVerificationKey = HexEncodedKey;

/// Hex encoded Verification Key
pub type HexEncodedVerificationKey = HexEncodedKey;

/// Hex encoded Verification Key Signature
pub type HexEncodedVerificationKeySignature = HexEncodedKey;

/// Hex encoded Operational Certificate
pub type HexEncodedOpCert = HexEncodedKey;

/// Hex encoded Genesis Secret Key
pub type HexEncodedGenesisSecretKey = HexEncodedKey;

/// Hex encoded Genesis Verification Key
pub type HexEncodedGenesisVerificationKey = HexEncodedKey;

/// Hex encoded Genesis Signature
pub type HexEncodedGenesisSignature = HexEncodedKey;

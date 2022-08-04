//! Fake data builders for testing.

use std::time::{SystemTime, UNIX_EPOCH};

use crate::entities::{
    CertificateMetadata, LotteryIndex, ProtocolMessage, ProtocolMessagePartKey, SingleSignatures,
};
use crate::{crypto_helper, entities};

/// Fake Beacon
pub fn beacon() -> entities::Beacon {
    let network = "testnet".to_string();
    let seconds_since_unix_epoch = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let immutable_file_number = (seconds_since_unix_epoch / (20)) as u64; // 1 immutable_file_number every 20s
    let epoch = (immutable_file_number / 9) as u64; // 1 epoch every 9 immutable_file_number

    entities::Beacon::new(network, epoch, immutable_file_number)
}

/// Fake Digest
pub fn digest(beacon: &entities::Beacon) -> Vec<u8> {
    format!(
        "digest-{}-{}-{}",
        beacon.network, beacon.epoch, beacon.immutable_file_number
    )
    .as_bytes()
    .to_vec()
}

/// Fake ProtocolParameters
pub fn protocol_parameters() -> entities::ProtocolParameters {
    let k = 5;
    let m = 100;
    let phi_f = 0.65;
    entities::ProtocolParameters::new(k, m, phi_f)
}

/// Fake CertificatePending
pub fn certificate_pending() -> entities::CertificatePending {
    // Beacon
    let beacon = beacon();

    // Protocol parameters
    let protocol_parameters = protocol_parameters();

    // Signers
    let signers = signers(5);

    // Certificate pending
    entities::CertificatePending::new(beacon, protocol_parameters, signers)
}

/// Fake Certificate
pub fn certificate(certificate_hash: String) -> entities::Certificate {
    // Beacon
    let beacon = beacon();

    // Protocol parameters
    let protocol_parameters = protocol_parameters();

    // Signers with stakes
    let signers = signers_with_stakes(5);

    // Certificate metadata
    let protocol_version = crypto_helper::PROTOCOL_VERSION.to_string();
    let initiated_at = "2006-01-02T15:04:05Z".to_string();
    let sealed_at = "2006-01-02T15:04:05Z".to_string();
    let metadata = CertificateMetadata::new(
        protocol_version,
        protocol_parameters,
        initiated_at,
        sealed_at,
        signers,
    );

    // Protocol message
    let next_aggregate_verification_key = "next-avk-123".to_string();
    let mut protocol_message = ProtocolMessage::new();
    let snapshot_digest = format!("1{}", beacon.immutable_file_number).repeat(20);
    protocol_message.set_message_part(ProtocolMessagePartKey::SnapshotDigest, snapshot_digest);
    protocol_message.set_message_part(
        ProtocolMessagePartKey::NextAggregateVerificationKey,
        next_aggregate_verification_key,
    );

    // Certificate
    let previous_hash = format!("{}0", certificate_hash);
    let aggregate_verification_key = format!("AVK{}", beacon.immutable_file_number).repeat(5);
    let multi_signature = format!("MSIG{}", beacon.immutable_file_number).repeat(200);
    let genesis_signature = "".to_string();
    let mut certificate = entities::Certificate::new(
        previous_hash,
        beacon,
        metadata,
        protocol_message,
        aggregate_verification_key,
        multi_signature,
        genesis_signature,
    );
    certificate.hash = certificate_hash;
    certificate
}

/// Fake SignersWithStake
pub fn signers_with_stakes(total: u64) -> Vec<entities::SignerWithStake> {
    let total = total as usize;
    let signers_with_stakes =
        serde_json::from_str::<Vec<entities::SignerWithStake>>(SIGNERS_WITH_STAKE_JSON).unwrap();
    assert!(signers_with_stakes.len() >= total);
    signers_with_stakes[0..total].to_vec()
}

/// Fake Signers
pub fn signers(total: u64) -> Vec<entities::Signer> {
    signers_with_stakes(total)
        .iter()
        .map(|signer| {
            entities::Signer::new(signer.party_id.clone(), signer.verification_key.clone())
        })
        .collect::<Vec<entities::Signer>>()
}

/// Fake SingleSignatures
pub fn single_signatures(won_indexes: Vec<LotteryIndex>) -> SingleSignatures {
    let party_id = "party_id".to_string();
    let signature = "7b227369676d61223a5b3137312c3139352c3234322c3235332c3137392c32352c3138372c3135312c3132322c3133302c3230372c3132322c38342c3132352c3134322c3132332c3233352c3134312c3230362c3136392c382c3136302c3138382c36382c35312c3232302c3232342c3231312c3137312c3230372c3231362c3139332c3230352c3139312c3233372c3131312c3232392c3132392c3131392c36362c3134342c3234382c3235322c39322c3234372c35382c37312c39355d2c22706b223a7b22766b223a5b3134332c3136312c3235352c34382c37382c35372c3230342c3232302c32352c3232312c3136342c3235322c3234382c31342c35362c3132362c3138362c3133352c3232382c3138382c3134352c3138312c35322c3230302c39372c39392c3231332c34362c302c3139392c3139332c38392c3138372c38382c32392c3133352c3137332c3234342c38362c33362c38332c35342c36372c3136342c362c3133372c39342c37322c362c3130352c3132382c3132382c39332c34382c3137362c31312c342c3234362c3133382c34382c3138302c3133332c39302c3134322c3139322c32342c3139332c3131312c3134322c33312c37362c3131312c3131302c3233342c3135332c39302c3230382c3139322c33312c3132342c39352c3130322c34392c3135382c39392c35322c3232302c3136352c39342c3235312c36382c36392c3132312c31362c3232342c3139345d2c22706f70223a5b3136382c35302c3233332c3139332c31352c3133362c36352c37322c3132332c3134382c3132392c3137362c33382c3139382c3230392c34372c32382c3230342c3137362c3134342c35372c3235312c34322c32382c36362c37362c38392c39372c3135382c36332c35342c3139382c3139342c3137362c3133352c3232312c31342c3138352c3139372c3232352c3230322c39382c3234332c37342c3233332c3232352c3134332c3135312c3134372c3137372c3137302c3131372c36362c3136352c36362c36322c33332c3231362c3233322c37352c36382c3131342c3139352c32322c3130302c36352c34342c3139382c342c3136362c3130322c3233332c3235332c3234302c35392c3137352c36302c3131372c3134322c3131342c3134302c3132322c31372c38372c3131302c3138372c312c31372c31302c3139352c3135342c31332c3234392c38362c35342c3232365d7d2c227061727479223a302c227374616b65223a3832362c22696e646578223a332c2270617468223a7b2276616c756573223a5b5b3235332c3135382c3135382c3232322c3233352c3137302c3137362c3139392c33332c3230302c36362c32362c3231312c3137392c3132362c3232362c35352c3234302c3138322c33302c3234362c3231352c37362c3135382c31362c3131342c342c3231392c36322c3131352c3235332c322c3139322c3231392c3135322c3137352c3131322c34352c36392c3131322c36382c3139352c31372c34342c3230352c3230342c37382c3233342c3130362c3234362c3230392c33312c3230302c3137312c3130382c32372c3136352c35382c3232392c37342c35382c3139312c3132352c3231385d2c5b3130382c39382c35322c3137372c3131332c3132392c3139342c37312c3133352c3137342c37342c3230352c38392c3137352c312c3230382c3234362c3136312c3132322c3233312c33302c3137382c32362c3234312c39382c35322c3133322c31342c33372c302c3138312c3232342c3130332c34362c3130312c3232322c3139392c36312c3231372c31322c39322c3231362c3139302c3131352c3233362c3134322c3138322c3235332c38312c32352c3138392c342c3235302c35382c34352c3234332c38302c37332c3130322c38332c32342c3130392c3131312c3138305d2c5b3138302c3230382c3234392c35312c3231362c3133352c34342c3134342c372c3132392c36302c36332c3234342c3130342c33362c3232392c34332c31312c3132332c38362c3131352c3131322c3138342c3230382c3135392c3138352c37332c31302c3136392c32372c39362c3231382c39392c3135322c3138312c36362c3230312c36302c3135342c31342c3231312c39342c3232392c3135382c3230382c3136362c3233302c37362c32332c3131382c3137382c3230382c38372c3131372c3233302c31392c3233312c32392c3230362c35382c3232352c32322c39352c3130335d5d2c22696e646578223a302c22686173686572223a6e756c6c7d7d".to_string();
    entities::SingleSignatures::new(party_id, signature, won_indexes)
}

/// Fake Snapshots
pub fn snapshots(total: u64) -> Vec<entities::Snapshot> {
    (1..total + 1)
        .map(|snapshot_id| {
            let digest = format!("1{}", snapshot_id).repeat(20);
            let beacon = beacon();
            let certificate_hash = "123".to_string();
            let size = snapshot_id * 100000;
            let created_at = "2006-01-02T15:04:05Z".to_string();
            let mut locations = Vec::new();
            locations.push(format!("http://{}", certificate_hash));
            locations.push(format!("http2://{}", certificate_hash));
            entities::Snapshot::new(
                digest,
                beacon,
                certificate_hash,
                size,
                created_at,
                locations,
            )
        })
        .collect::<Vec<entities::Snapshot>>()
}

const SIGNERS_WITH_STAKE_JSON: &str = r###"
[
  {
    "party_id": "0",
    "stake": 826,
    "verification_key": "7b22766b223a5b3134332c3136312c3235352c34382c37382c35372c3230342c3232302c32352c3232312c3136342c3235322c3234382c31342c35362c3132362c3138362c3133352c3232382c3138382c3134352c3138312c35322c3230302c39372c39392c3231332c34362c302c3139392c3139332c38392c3138372c38382c32392c3133352c3137332c3234342c38362c33362c38332c35342c36372c3136342c362c3133372c39342c37322c362c3130352c3132382c3132382c39332c34382c3137362c31312c342c3234362c3133382c34382c3138302c3133332c39302c3134322c3139322c32342c3139332c3131312c3134322c33312c37362c3131312c3131302c3233342c3135332c39302c3230382c3139322c33312c3132342c39352c3130322c34392c3135382c39392c35322c3232302c3136352c39342c3235312c36382c36392c3132312c31362c3232342c3139345d2c22706f70223a5b3136382c35302c3233332c3139332c31352c3133362c36352c37322c3132332c3134382c3132392c3137362c33382c3139382c3230392c34372c32382c3230342c3137362c3134342c35372c3235312c34322c32382c36362c37362c38392c39372c3135382c36332c35342c3139382c3139342c3137362c3133352c3232312c31342c3138352c3139372c3232352c3230322c39382c3234332c37342c3233332c3232352c3134332c3135312c3134372c3137372c3137302c3131372c36362c3136352c36362c36322c33332c3231362c3233322c37352c36382c3131342c3139352c32322c3130302c36352c34342c3139382c342c3136362c3130322c3233332c3235332c3234302c35392c3137352c36302c3131372c3134322c3131342c3134302c3132322c31372c38372c3131302c3138372c312c31372c31302c3139352c3135342c31332c3234392c38362c35342c3232365d7d"
  },
  {
    "party_id": "1",
    "stake": 741,
    "verification_key": "7b22766b223a5b3134352c35362c3137352c33322c3132322c3138372c3231342c3232362c3235312c3134382c38382c392c312c3130332c3135392c3134362c38302c3136362c3130372c3234332c3235312c3233362c34312c32382c3131312c3132382c3230372c3136342c3133322c3134372c3232382c38332c3234362c3232382c3137302c36382c38392c37382c36302c32382c3132332c3133302c38382c3233342c33382c39372c34322c36352c312c3130302c35332c31382c37382c3133312c382c36312c3132322c3133312c3233382c38342c3233332c3232332c3135342c3131382c3131382c37332c32382c32372c3130312c37382c38302c3233332c3132332c3230362c3232302c3137342c3133342c3230352c37312c3131302c3131322c3138302c39372c39382c302c3131332c36392c3134352c3233312c3136382c34332c3137332c3137322c35362c3130342c3230385d2c22706f70223a5b3133372c3231342c37352c37352c3134342c3136312c3133372c37392c39342c3134302c3138312c34372c33312c38312c3231332c33312c3137312c3231362c32342c3137342c37382c3234382c3133302c37352c3235352c31312c3134352c3132342c36312c38302c3139302c32372c3231362c3130352c3130362c3234382c39312c3134332c3230342c3130322c3230332c3136322c37362c3130372c31352c35322c36312c38322c3134362c3133302c3132342c37342c382c33342c3136342c3138372c3230332c38322c36342c3130382c3139312c3138352c3138382c37372c3132322c352c3234362c3235352c3130322c3131392c3234372c3139392c3131372c36372c3234312c3134332c32392c3136382c36372c39342c3135312c37382c3132392c3133312c33302c3130312c3137332c31302c36392c36382c3137352c39382c33372c3233392c3139342c32395d7d"
  },
  {
    "party_id": "2",
    "stake": 144,
    "verification_key": "7b22766b223a5b3135302c3134352c35332c3235302c3234372c3131372c3130312c3137352c332c3139342c3133342c32352c3134362c3138342c3136372c3134382c3139332c3139332c32352c302c3230372c3139322c31302c35382c3131332c3235342c312c3230372c34332c3136362c3139332c38322c37332c3139332c36322c3138362c35342c3138322c32342c3231302c3137312c39372c34382c37302c3234312c38342c3132382c31352c32322c35322c3131332c3137362c3235342c35372c3130392c35372c3137372c33332c38312c36312c33342c3232392c392c3136352c332c35322c31312c3139382c32322c3230332c3132382c36352c39362c382c35372c3233312c3130352c38382c32382c3137392c36312c35322c3134332c3139342c3134332c3230352c3235342c3233322c3230342c37322c3231312c39382c3132352c39312c3133312c3133345d2c22706f70223a5b3132382c312c3134302c3130322c35332c3131372c32312c3232382c3233322c3138332c3135302c36342c3134392c36332c3137382c3232352c3235322c3136382c3132342c3233322c3138382c36322c3130362c3130372c34382c31312c3135352c3131382c3138342c3139392c3138382c3233322c35352c302c35332c3132312c3136372c39362c3135322c39342c3131332c3131382c3231382c382c3135302c39302c3137362c3137362c3134342c31352c37392c32372c31372c3133312c34392c3232312c3133302c35322c3233322c3139332c39342c33372c32362c3136332c32362c3233372c3136332c3132312c3135392c362c3234392c3235312c3139352c3231322c3235352c3130322c39362c3230392c3139312c3230382c3135322c3233362c3135352c3137322c31332c37332c34342c3231342c31382c3232392c382c3133362c33372c3136312c33372c345d7d"
  },
  {
    "party_id": "3",
    "stake": 734,
    "verification_key": "7b22766b223a5b3136392c3139332c39392c3233362c3131302c36342c3132302c3232362c3138322c35312c3233392c39302c3133332c3134352c3137322c3132382c3137302c35302c3130352c35352c3130372c3230372c36302c3231362c3234302c3137342c3139302c3235322c3232342c31392c37332c3134302c32392c35322c3230312c32362c3135322c3131322c3135342c36352c3232332c392c3139342c372c36392c3134362c3135362c3138302c362c38332c3234302c3137362c3232372c3138302c31322c352c3230342c32392c3138362c3233352c36342c3232362c3234342c3138322c3137372c3135302c3130372c3132332c3138382c3234332c39322c3135372c3136332c3135352c3137362c3231332c32342c3130362c36392c34332c31392c3136302c3130382c31352c38312c3136322c3133392c3234322c3139332c3235312c3131372c39392c302c3135352c3139312c3132355d2c22706f70223a5b3134392c3134372c3135362c3137322c3132372c3230372c3231322c38362c3135362c322c33392c3131332c3139362c3235322c3233352c39382c3130302c3130342c31332c3137302c34312c3235302c3139352c36352c3232332c34382c3232302c3234332c38382c3136352c3139332c34332c34342c31362c35362c3230352c3132332c34392c3133382c38372c33352c32322c3132372c37322c32342c3133332c3136332c3139352c3138342c3232382c3136372c3131312c3132342c3138372c3132322c3231332c3134352c3136342c35352c34342c36322c37302c3130372c3132362c35352c35352c3233322c3230322c38362c39392c33392c34382c3135382c31322c3132342c32332c39382c3135382c3231352c3137372c35342c3233382c3131382c3134362c33302c39352c3235302c3133362c3136362c39352c38362c37372c3138332c3138342c37352c3133325d7d"
  },
  {
    "party_id": "4",
    "stake": 41,
    "verification_key": "7b22766b223a5b3138332c37392c3133302c3132302c38342c3135322c33322c3134342c3138302c3130342c3139322c3134302c3133392c38352c32332c302c3137312c3132312c3136342c38382c31382c3136392c3233332c3137382c39322c342c392c3134302c3230332c37352c39362c3232352c31302c31382c33302c38322c3132312c3230302c36382c33362c34352c3132302c3231322c35332c3232322c3135332c31322c3138382c31312c3131332c3231352c31382c3132352c36302c3231302c3130302c35322c3138392c34372c31312c3135322c33382c35332c38362c32342c3231322c34322c3131302c35352c3139372c36322c3137362c33392c35322c39372c3137322c31322c322c3234302c31342c322c3131332c3138362c3131382c39372c37312c34342c3233342c3133392c3234302c362c3232392c3136372c3231312c38302c3136305d2c22706f70223a5b3135312c3138312c3231332c3137382c3235342c3137312c3235352c3234332c3131342c34352c34342c3137352c3234332c3137302c3135372c3133382c3139382c38362c31362c36322c31392c3132342c3131392c37332c3230302c35332c3231372c3230322c382c3139332c3134352c3132312c3231362c34362c3134392c3233382c3232332c3233342c35382c32362c322c3131342c32302c3131372c38332c3234382c3138332c36302c3134372c38322c3230352c3139332c3136322c34352c3136312c3231302c31312c342c3230302c37352c382c3137382c3135342c3235332c37312c3234302c3132362c38332c32322c33342c3231372c3138362c3232312c3131302c3138362c3133372c352c3139372c3231382c35352c3132352c3137372c3136302c36332c32342c39382c3134322c3232362c3133322c3231312c3230392c3136372c32372c3137322c37362c3137375d7d"
  }
]
"###;
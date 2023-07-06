use anyhow::{anyhow, Context, Result};
use rand_chacha::ChaCha20Rng;
use rand_core::{CryptoRng, RngCore, SeedableRng};
use std::path::Path;
use thiserror::Error;

use crate::{
    crypto_helper::{
        key_decode_hex, OpCert, ProtocolClerk, ProtocolClosedKeyRegistration, ProtocolInitializer,
        ProtocolKeyRegistration, ProtocolSignerVerificationKey,
        ProtocolSignerVerificationKeySignature, ProtocolStakeDistribution,
    },
    entities::{PartyId, ProtocolParameters, SignerWithStake},
    protocol::MultiSigner,
};

use super::SingleSigner;

/// Allow to build Single Or Multi signers to generate a single signature or aggregate them
#[derive(Debug)]
pub struct SignerBuilder {
    protocol_parameters: ProtocolParameters,
    closed_key_registration: ProtocolClosedKeyRegistration,
}

/// [SignerBuilder] specific errors
#[derive(Debug, Error)]
pub enum SignerBuilderError {
    /// Error raised when the list of signers given to the builder is empty
    #[error("The list of signers must not be empty to create a signer builder.")]
    EmptySigners,
}

impl SignerBuilder {
    /// [SignerBuilder] constructor.
    pub fn new(
        registered_signers: &[SignerWithStake],
        protocol_parameters: &ProtocolParameters,
    ) -> Result<Self> {
        if registered_signers.is_empty() {
            return Err(SignerBuilderError::EmptySigners.into());
        }

        let stake_distribution = registered_signers
            .iter()
            .map(|s| s.into())
            .collect::<ProtocolStakeDistribution>();
        let mut key_registration = ProtocolKeyRegistration::init(&stake_distribution);

        for signer in registered_signers {
            let signer_keys = SignerCryptographicMaterial::decode_from_signer(signer)
                .with_context(|| {
                    format!(
                        "Invalid signers in stake distribution: '{}'",
                        signer.party_id
                    )
                })?;
            key_registration
                .register(
                    Some(signer.party_id.to_owned()),
                    signer_keys.operational_certificate,
                    signer_keys.kes_signature,
                    signer_keys.kes_period,
                    signer_keys.verification_key,
                )
                .with_context(|| {
                    format!("Registration failed for signer: '{}'", signer.party_id)
                })?;
        }

        let closed_registration = key_registration.close();
        Ok(Self {
            protocol_parameters: protocol_parameters.clone(),
            closed_key_registration: closed_registration,
        })
    }

    /// Build a [MultiSigner] based on the registered parties
    pub fn build_multi_signer(&self) -> MultiSigner {
        let stm_parameters = self.protocol_parameters.clone().into();
        let clerk =
            ProtocolClerk::from_registration(&stm_parameters, &self.closed_key_registration);

        MultiSigner::new(clerk, stm_parameters)
    }

    fn build_single_signer_with_rng<R: RngCore + CryptoRng>(
        &self,
        signer_with_stake: SignerWithStake,
        kes_secret_key_path: Option<&Path>,
        rng: &mut R,
    ) -> Result<(SingleSigner, ProtocolInitializer)> {
        let protocol_initializer = ProtocolInitializer::setup(
            self.protocol_parameters.clone().into(),
            kes_secret_key_path,
            signer_with_stake.kes_period,
            signer_with_stake.stake,
            rng,
        )
        .with_context(|| {
            format!(
                "Could not create a protocol initializer for party: '{}'",
                signer_with_stake.party_id
            )
        })?;

        let protocol_signer = protocol_initializer
            .clone()
            .new_signer(self.closed_key_registration.clone())
            .with_context(|| {
                format!(
                    "Could not create a protocol signer for party: '{}'",
                    signer_with_stake.party_id
                )
            })?;

        Ok((
            SingleSigner::new(signer_with_stake.party_id, protocol_signer),
            protocol_initializer,
        ))
    }

    /// Build non deterministic [SingleSigner] and [ProtocolInitializer] based on the registered parties.
    pub fn build_single_signer(
        &self,
        signer_with_stake: SignerWithStake,
        kes_secret_key_path: Option<&Path>,
    ) -> Result<(SingleSigner, ProtocolInitializer)> {
        self.build_single_signer_with_rng(
            signer_with_stake,
            kes_secret_key_path,
            &mut rand_core::OsRng,
        )
    }

    /// Build deterministic [SingleSigner] and [ProtocolInitializer] based on the registered parties.
    ///
    /// Use for **TEST ONLY**.
    pub fn build_test_single_signer(
        &self,
        signer_with_stake: SignerWithStake,
        kes_secret_key_path: Option<&Path>,
    ) -> Result<(SingleSigner, ProtocolInitializer)> {
        let protocol_initializer_seed: [u8; 32] = signer_with_stake.party_id.as_bytes()[..32]
            .try_into()
            .unwrap();

        self.build_single_signer_with_rng(
            signer_with_stake,
            kes_secret_key_path,
            &mut ChaCha20Rng::from_seed(protocol_initializer_seed),
        )
    }

    /// Restore a [SingleSigner] based on the registered parties and the given
    /// protocol_initializer.
    ///
    /// This is useful since each protocol initializer holds a unique secret key
    /// that corresponds to a registration key sent to an aggregator.
    ///
    /// The actual signing of message is done at a later epoch.
    ///
    /// The [SignerBuilder] used must be tied to the key registration, stake distribution
    /// and protocol parameters of the epoch during which the given protocol initializer
    /// was created.
    pub fn restore_signer_from_initializer(
        &self,
        party_id: PartyId,
        protocol_initializer: ProtocolInitializer,
    ) -> Result<SingleSigner> {
        let single_signer = protocol_initializer
            .new_signer(self.closed_key_registration.clone())
            .with_context(|| {
                "Could not create a single signer from protocol initializer".to_string()
            })?;

        Ok(SingleSigner::new(party_id, single_signer))
    }
}

struct SignerCryptographicMaterial {
    pub operational_certificate: Option<OpCert>,
    pub verification_key: ProtocolSignerVerificationKey,
    pub kes_signature: Option<ProtocolSignerVerificationKeySignature>,
    pub kes_period: Option<u32>,
}

impl SignerCryptographicMaterial {
    fn decode_from_signer(signer: &SignerWithStake) -> Result<Self> {
        let operational_certificate = match &signer.operational_certificate {
            Some(operational_certificate) => key_decode_hex(operational_certificate)
                .map_err(|e| anyhow!(e))
                .with_context(|| "Could not decode operational certificate".to_string())?,
            _ => None,
        };
        let verification_key = key_decode_hex(&signer.verification_key)
            .map_err(|e| anyhow!(e))
            .with_context(|| "Could not decode verification key".to_string())?;
        let kes_signature = match &signer.verification_key_signature {
            Some(verification_key_signature) => Some(
                key_decode_hex(verification_key_signature)
                    .map_err(|e| anyhow!(e))
                    .with_context(|| "Could not decode verification key signature".to_string())?,
            ),
            _ => None,
        };

        Ok(Self {
            operational_certificate,
            verification_key,
            kes_signature,
            kes_period: signer.kes_period,
        })
    }
}

#[cfg(test)]
mod test {
    use crate::test_utils::{fake_data, MithrilFixtureBuilder};

    use super::*;

    #[test]
    fn cant_construct_signer_builder_with_an_empty_signers_list() {
        let signers = vec![];
        let protocol_parameters = fake_data::protocol_parameters();

        let error = SignerBuilder::new(&signers, &protocol_parameters).expect_err(
            "We should not be able to construct a signer builder with an empty signers list",
        );

        match error.downcast_ref::<SignerBuilderError>() {
            Some(SignerBuilderError::EmptySigners) => (),
            None => panic!("Expected an EmptySigners error, got: {error}"),
        }
    }

    #[test]
    fn cant_construct_signer_builder_with_a_signer_with_invalid_keys() {
        let fixture = MithrilFixtureBuilder::default().with_signers(3).build();
        let mut signers = fixture.signers_with_stake();
        signers[2].operational_certificate = Some("invalid".to_string());

        let error = SignerBuilder::new(&signers, &fixture.protocol_parameters()).expect_err(
            "We should not be able to construct a signer builder with a signer with invalid keys",
        );

        assert!(
            error
                .to_string()
                .contains("Invalid signers in stake distribution"),
            "Expected Invalid signers error, got: {}, cause: {}",
            error,
            error.root_cause()
        );
    }

    #[test]
    fn cant_construct_signer_builder_if_a_signer_registration_fail() {
        // To make this test fail we try to build a SignerBuilder with signers from two
        // different stake distributions, this will pass the individual check but not the
        // register check.
        let fixture = MithrilFixtureBuilder::default().with_signers(3).build();
        let fixture_with_another_stake_distribution = MithrilFixtureBuilder::default()
            .with_signers(1)
            .with_stake_distribution(
                crate::test_utils::StakeDistributionGenerationMethod::RandomDistribution {
                    seed: [4u8; 32],
                },
            )
            .build();
        let mut signers = fixture.signers_with_stake();
        signers.append(&mut fixture_with_another_stake_distribution.signers_with_stake());

        let error = SignerBuilder::new(&signers, &fixture.protocol_parameters()).expect_err(
            "We should not be able to construct a signer builder if a signer registration fail",
        );

        assert!(
            error.to_string().contains("Registration failed for signer"),
            "Expected Registration error, got: {}, cause: {}",
            error,
            error.root_cause()
        );
    }

    #[test]
    fn can_construct_signer_builder_with_valid_signers() {
        let fixture = MithrilFixtureBuilder::default().with_signers(3).build();

        SignerBuilder::new(
            &fixture.signers_with_stake(),
            &fixture.protocol_parameters(),
        )
        .expect("We should be able to construct a signer builder with valid signers");
    }

    #[test]
    fn cant_build_single_signer_for_unregistered_party() {
        let fixture = MithrilFixtureBuilder::default().with_signers(3).build();
        let signers_from_another_fixture = MithrilFixtureBuilder::default()
            .with_signers(1)
            .with_party_id_seed([4u8; 32])
            .build()
            .signers_fixture();
        let non_registered_signer = signers_from_another_fixture.first().unwrap();

        let error = SignerBuilder::new(
            &fixture.signers_with_stake(),
            &fixture.protocol_parameters(),
        )
        .unwrap()
        .build_test_single_signer(
            non_registered_signer.signer_with_stake.clone(),
            non_registered_signer.kes_secret_key_path(),
        )
        .expect_err(
            "We should not be able to construct a single signer from a not registered party",
        );

        assert!(
            error
                .to_string()
                .contains("Could not create a protocol signer for party"),
            "Expected protocol signer creation error, got: {}, cause: {}",
            error,
            error.root_cause()
        );
    }

    #[test]
    fn should_build_single_signer_for_registered_party() {
        let fixture = MithrilFixtureBuilder::default().with_signers(3).build();
        let signers = fixture.signers_fixture();
        let signer = signers.first().unwrap();

        let builder = SignerBuilder::new(
            &fixture.signers_with_stake(),
            &fixture.protocol_parameters(),
        )
        .unwrap();

        builder
            .build_test_single_signer(
                signer.signer_with_stake.clone(),
                signer.kes_secret_key_path(),
            )
            .expect("Should be able to build test single signer for a registered party");
    }

    #[test]
    fn should_restore_single_signer_from_previous_initializer() {
        let fixture = MithrilFixtureBuilder::default().with_signers(3).build();
        let signers = fixture.signers_fixture();
        let signer = signers.first().unwrap();

        let first_builder = SignerBuilder::new(
            &fixture.signers_with_stake(),
            &fixture.protocol_parameters(),
        )
        .unwrap();

        let (_, initializer) = first_builder
            .build_test_single_signer(
                signer.signer_with_stake.clone(),
                signer.kes_secret_key_path(),
            )
            .unwrap();

        let second_builder = SignerBuilder::new(
            &fixture.signers_with_stake(),
            &fixture.protocol_parameters(),
        )
        .unwrap();

        second_builder
            .restore_signer_from_initializer(signer.party_id(), initializer)
            .expect("Should be able to restore a single signer from a protocol initialized");
    }
}

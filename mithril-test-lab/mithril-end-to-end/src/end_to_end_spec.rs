use crate::utils::AttemptResult;
use crate::{attempt, Aggregator, Client, ClientCommand, Devnet, MithrilInfrastructure};
use mithril_common::chain_observer::{CardanoCliChainObserver, ChainObserver};
use mithril_common::digesters::ImmutableFile;
use mithril_common::entities::{Certificate, Epoch, EpochSettings, Snapshot};
use reqwest::StatusCode;
use slog_scope::{info, warn};
use std::error::Error;
use std::path::Path;
use std::sync::Arc;
use std::time::Duration;

pub struct Spec {
    infrastructure: MithrilInfrastructure,
}

impl Spec {
    pub fn new(infrastructure: MithrilInfrastructure) -> Self {
        Self { infrastructure }
    }

    pub async fn run(&mut self) -> Result<(), Box<dyn Error>> {
        let aggregator_endpoint = self.infrastructure.aggregator().endpoint();
        wait_for_enough_immutable(self.infrastructure.aggregator().db_directory()).await?;
        let start_epoch = self
            .infrastructure
            .chain_observer()
            .get_current_epoch()
            .await?
            .unwrap_or_default();

        // Wait 1 epoch after start epoch for the aggregator to be able to bootstrap a genesis certificate
        let mut target_epoch = start_epoch + 1;
        wait_for_target_epoch(
            self.infrastructure.chain_observer(),
            target_epoch,
            "minimal epoch for the aggregator to be able to bootstrap genesis certificate"
                .to_string(),
        )
        .await?;
        bootstrap_genesis_certificate(self.infrastructure.aggregator_mut()).await?;
        wait_for_epoch_settings(&aggregator_endpoint).await?;

        // Wait 2 epochs before changing stake distribution, so that we use at least once original stake distribution
        target_epoch += 2;
        wait_for_target_epoch(
            self.infrastructure.chain_observer(),
            target_epoch,
            "epoch after which the stake distribution will change".to_string(),
        )
        .await?;
        delegate_stakes_to_pools(self.infrastructure.devnet()).await?;

        // Wait 5 epochs after stake delegation, so that we make sure that we use new stake distribution a few times
        target_epoch += 5;
        wait_for_target_epoch(
            self.infrastructure.chain_observer(),
            target_epoch,
            "epoch after which the certificate chain will be long enough to catch most common troubles".to_string(),
        )
        .await?;

        let digest = assert_node_producing_snapshot(&aggregator_endpoint).await?;
        let certificate_hash =
            assert_signer_is_signing_snapshot(&aggregator_endpoint, &digest, target_epoch - 2)
                .await?;
        assert_is_creating_certificate_with_enough_signers(
            &aggregator_endpoint,
            &certificate_hash,
            self.infrastructure.signers().len(),
        )
        .await?;

        let mut client = self.infrastructure.build_client()?;
        assert_client_can_verify_snapshot(&mut client, &digest).await?;

        Ok(())
    }

    pub async fn tail_logs(&self, number_of_line: u64) -> Result<(), String> {
        self.infrastructure
            .aggregator()
            .tail_logs(number_of_line)
            .await?;
        for signer in self.infrastructure.signers() {
            signer.tail_logs(number_of_line).await?;
        }

        Ok(())
    }
}

async fn wait_for_enough_immutable(db_directory: &Path) -> Result<(), String> {
    info!("Waiting that enough immutable have been written in the devnet");

    match attempt!(24, Duration::from_secs(5), {
        match ImmutableFile::list_completed_in_dir(db_directory)
            .map_err(|e| {
                format!(
                    "Immutable file listing failed in dir `{}`: {}",
                    db_directory.display(),
                    e
                )
            })?
            .last()
        {
            Some(_) => Ok(Some(())),
            None => Ok(None),
        }
    }) {
        AttemptResult::Ok(_) => Ok(()),
        AttemptResult::Err(error) => Err(error),
        AttemptResult::Timeout() => Err(format!(
            "Timeout exhausted for enough immutable to be written in `{}`",
            db_directory.display()
        )),
    }
}

async fn wait_for_epoch_settings(aggregator_endpoint: &str) -> Result<EpochSettings, String> {
    let url = format!("{}/epoch-settings", aggregator_endpoint);
    info!("Waiting for the aggregator to expose epoch settings");

    match attempt!(20, Duration::from_millis(1000), {
        match reqwest::get(url.clone()).await {
            Ok(response) => match response.status() {
                StatusCode::OK => {
                    let epoch_settings = response
                        .json::<EpochSettings>()
                        .await
                        .map_err(|e| format!("Invalid EpochSettings body : {}", e))?;
                    info!("Aggregator ready"; "epoch_settings"  => #?epoch_settings);
                    Ok(Some(epoch_settings))
                }
                s if s.is_server_error() => {
                    warn!(
                        "Server error while waiting for the Aggregator, http code: {}",
                        s
                    );
                    Ok(None)
                }
                _ => Ok(None),
            },
            Err(_) => Ok(None),
        }
    }) {
        AttemptResult::Ok(epoch_settings) => Ok(epoch_settings),
        AttemptResult::Err(error) => Err(error),
        AttemptResult::Timeout() => Err(format!(
            "Timeout exhausted for aggregator to be up, no response from `{}`",
            url
        )),
    }
}

async fn wait_for_target_epoch(
    chain_observer: Arc<CardanoCliChainObserver>,
    target_epoch: Epoch,
    wait_reason: String,
) -> Result<(), String> {
    info!(
        "Waiting for the cardano network to be at the target epoch: {}", wait_reason;
        "target_epoch" => ?target_epoch
    );

    match attempt!(90, Duration::from_millis(1000), {
        match chain_observer.get_current_epoch().await {
            Ok(Some(epoch)) => {
                if epoch >= target_epoch {
                    Ok(Some(()))
                } else {
                    Ok(None)
                }
            }
            Ok(None) => Ok(None),
            Err(err) => Err(format!("Could not query current epoch: {}", err)),
        }
    }) {
        AttemptResult::Ok(_) => {
            info!("Target epoch reached !"; "target_epoch" => ?target_epoch);
            Ok(())
        }
        AttemptResult::Err(error) => Err(error),
        AttemptResult::Timeout() => {
            Err("Timeout exhausted for target epoch to be reached".to_string())
        }
    }?;

    Ok(())
}

async fn bootstrap_genesis_certificate(aggregator: &mut Aggregator) -> Result<(), String> {
    info!("Bootstrap genesis certificate");

    info!("> stopping aggregator");
    aggregator.stop().await?;
    info!("> bootstrapping genesis using signers registered two epochs ago ...");
    aggregator.bootstrap_genesis().await?;
    info!("> done, restarting aggregator");
    aggregator.serve()?;

    Ok(())
}

async fn delegate_stakes_to_pools(devnet: &Devnet) -> Result<(), String> {
    info!("Delegate stakes to the cardano pools");

    devnet.delegate_stakes().await?;

    Ok(())
}

async fn assert_node_producing_snapshot(aggregator_endpoint: &str) -> Result<String, String> {
    let url = format!("{}/snapshots", aggregator_endpoint);
    info!("Waiting for the aggregator to produce a snapshot");

    // todo: reduce the number of attempts if we can reduce the delay between two immutables
    match attempt!(45, Duration::from_millis(2000), {
        match reqwest::get(url.clone()).await {
            Ok(response) => match response.status() {
                StatusCode::OK => match response.json::<Vec<Snapshot>>().await.as_deref() {
                    Ok([snapshot, ..]) => Ok(Some(snapshot.digest.clone())),
                    Ok(&[]) => Ok(None),
                    Err(err) => Err(format!("Invalid snapshot body : {}", err,)),
                },
                s => Err(format!("Unexpected status code from Aggregator: {}", s)),
            },
            Err(err) => Err(format!("Request to `{}` failed: {}", url, err)),
        }
    }) {
        AttemptResult::Ok(digest) => {
            info!("Aggregator produced a snapshot"; "digest" => &digest);
            Ok(digest)
        }
        AttemptResult::Err(error) => Err(error),
        AttemptResult::Timeout() => Err(format!(
            "Timeout exhausted assert_node_producing_snapshot, no response from `{}`",
            url
        )),
    }
}

async fn assert_signer_is_signing_snapshot(
    aggregator_endpoint: &str,
    digest: &str,
    expected_epoch_min: Epoch,
) -> Result<String, String> {
    let url = format!("{}/snapshot/{}", aggregator_endpoint, digest);
    info!(
        "Asserting the aggregator is signing the snapshot message `{}` with an expected min epoch of `{}`",
        digest,
        expected_epoch_min
    );

    match attempt!(10, Duration::from_millis(1000), {
        match reqwest::get(url.clone()).await {
            Ok(response) => match response.status() {
                StatusCode::OK => match response.json::<Snapshot>().await {
                    Ok(snapshot) => match snapshot.beacon.epoch {
                        epoch if epoch >= expected_epoch_min => Ok(Some(snapshot)),
                        epoch => Err(format!(
                            "Minimum expected snapshot epoch not reached : {} < {}",
                            epoch, expected_epoch_min
                        )),
                    },
                    Err(err) => Err(format!("Invalid snapshot body : {}", err,)),
                },
                StatusCode::NOT_FOUND => Ok(None),
                s => Err(format!("Unexpected status code from Aggregator: {}", s)),
            },
            Err(err) => Err(format!("Request to `{}` failed: {}", url, err)),
        }
    }) {
        AttemptResult::Ok(snapshot) => {
            // todo: assert that the snapshot is really signed
            info!("Signer signed a snapshot"; "certificate_hash" => &snapshot.certificate_hash);
            Ok(snapshot.certificate_hash)
        }
        AttemptResult::Err(error) => Err(error),
        AttemptResult::Timeout() => Err(format!(
            "Timeout exhausted assert_signer_is_signing_snapshot, no response from `{}`",
            url
        )),
    }
}

async fn assert_is_creating_certificate_with_enough_signers(
    aggregator_endpoint: &str,
    certificate_hash: &str,
    total_signers_expected: usize,
) -> Result<(), String> {
    let url = format!("{}/certificate/{}", aggregator_endpoint, certificate_hash);

    match attempt!(10, Duration::from_millis(1000), {
        match reqwest::get(url.clone()).await {
            Ok(response) => match response.status() {
                StatusCode::OK => match response.json::<Certificate>().await {
                    Ok(certificate) => Ok(Some(certificate)),
                    Err(err) => Err(format!("Invalid snapshot body : {}", err,)),
                },
                StatusCode::NOT_FOUND => Ok(None),
                s => Err(format!("Unexpected status code from Aggregator: {}", s)),
            },
            Err(err) => Err(format!("Request to `{}` failed: {}", url, err)),
        }
    }) {
        AttemptResult::Ok(certificate) => {
            info!("Aggregator produced a certificate"; "certificate" => #?certificate);
            if certificate.metadata.signers.len() == total_signers_expected {
                info!(
                    "Certificate is signed by expected number of signers: {} >= {} ",
                    certificate.metadata.signers.len(),
                    total_signers_expected
                );
                Ok(())
            } else {
                Err(format!(
                    "Certificate is not signed by expected number of signers: {} < {} ",
                    certificate.metadata.signers.len(),
                    total_signers_expected
                ))
            }
        }
        AttemptResult::Err(error) => Err(error),
        AttemptResult::Timeout() => Err(format!(
            "Timeout exhausted assert_is_creating_certificate, no response from `{}`",
            url
        )),
    }
}

async fn assert_client_can_verify_snapshot(
    client: &mut Client,
    digest: &str,
) -> Result<(), String> {
    client
        .run(ClientCommand::Download {
            digest: digest.to_string(),
        })
        .await?;
    info!("Client downloaded the snapshot"; "digest" => &digest);

    client
        .run(ClientCommand::Restore {
            digest: digest.to_string(),
        })
        .await?;
    info!("Client restored the snapshot"; "digest" => &digest);

    Ok(())
}

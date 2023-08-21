use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlite::{Connection, Value};
use std::{iter::repeat, sync::Arc};
use tokio::sync::Mutex;

use mithril_common::{
    certificate_chain::{CertificateRetriever, CertificateRetrieverError},
    entities::{
        Beacon, Certificate, CertificateMetadata, CertificateSignature, Epoch,
        HexEncodedAgregateVerificationKey, HexEncodedKey, ProtocolMessage, ProtocolParameters,
        ProtocolVersion, SignerWithStake,
    },
    sqlite::{
        EntityCursor, HydrationError, Projection, Provider, SourceAlias, SqLiteEntity,
        WhereCondition,
    },
    StdResult,
};

#[cfg(test)]
use mithril_common::test_utils::fake_keys;

/// Certificate record is the representation of a stored certificate.
#[derive(Debug, PartialEq, Clone)]
pub struct CertificateRecord {
    /// Certificate id.
    pub certificate_id: String,

    /// Parent Certificate id.
    pub parent_certificate_id: Option<String>,

    /// Message that is signed.
    pub message: String,

    /// Signature of the certificate.
    /// Note: multi-signature if parent certificate id is set, genesis signature otherwise.
    pub signature: HexEncodedKey,

    /// Aggregate verification key
    /// Note: used only if signature is a multi-signature
    pub aggregate_verification_key: HexEncodedAgregateVerificationKey,

    /// Epoch of creation of the certificate.
    pub epoch: Epoch,

    /// Beacon used to produce the signed message
    pub beacon: Beacon,

    /// Protocol Version (semver)
    pub protocol_version: ProtocolVersion,

    /// Protocol parameters.
    pub protocol_parameters: ProtocolParameters,

    /// Structured message that is used to create the signed message
    pub protocol_message: ProtocolMessage,

    /// The list of the active signers with their stakes and verification keys
    pub signers: Vec<SignerWithStake>,

    /// Date and time when the certificate was initiated
    pub initiated_at: DateTime<Utc>,

    /// Date and time when the certificate was sealed
    pub sealed_at: DateTime<Utc>,
}

impl CertificateRecord {
    #[cfg(test)]
    pub fn dummy_genesis(id: &str, beacon: Beacon) -> Self {
        let mut record = Self::dummy(id, "", beacon);
        record.parent_certificate_id = None;
        record.signature = fake_keys::genesis_signature()[0].to_owned();
        record
    }

    #[cfg(test)]
    pub fn dummy(id: &str, parent_id: &str, beacon: Beacon) -> Self {
        Self {
            certificate_id: id.to_string(),
            parent_certificate_id: Some(parent_id.to_string()),
            message: "message".to_string(),
            signature: fake_keys::multi_signature()[0].to_owned(),
            aggregate_verification_key: fake_keys::aggregate_verification_key()[0].to_owned(),
            epoch: beacon.epoch,
            beacon,
            protocol_version: "protocol_version".to_string(),
            protocol_parameters: Default::default(),
            protocol_message: Default::default(),
            signers: vec![],
            initiated_at: DateTime::parse_from_rfc3339("2024-02-12T13:11:47Z")
                .unwrap()
                .with_timezone(&Utc),
            sealed_at: DateTime::parse_from_rfc3339("2024-02-12T13:12:57Z")
                .unwrap()
                .with_timezone(&Utc),
        }
    }
}

impl From<Certificate> for CertificateRecord {
    fn from(other: Certificate) -> Self {
        let (signature, parent_certificate_id) = match other.signature {
            CertificateSignature::GenesisSignature(signature) => (signature.to_bytes_hex(), None),
            CertificateSignature::MultiSignature(signature) => {
                (signature.to_json_hex().unwrap(), Some(other.previous_hash))
            }
        };

        CertificateRecord {
            certificate_id: other.hash,
            parent_certificate_id,
            message: other.signed_message,
            signature,
            aggregate_verification_key: other.aggregate_verification_key,
            epoch: other.beacon.epoch,
            beacon: other.beacon,
            protocol_version: other.metadata.protocol_version,
            protocol_parameters: other.metadata.protocol_parameters,
            protocol_message: other.protocol_message,
            signers: other.metadata.signers,
            initiated_at: other.metadata.initiated_at,
            sealed_at: other.metadata.sealed_at,
        }
    }
}

impl From<CertificateRecord> for Certificate {
    fn from(other: CertificateRecord) -> Self {
        let certificate_metadata = CertificateMetadata::new(
            other.protocol_version,
            other.protocol_parameters,
            other.initiated_at,
            other.sealed_at,
            other.signers,
        );
        let (previous_hash, signature) = match other.parent_certificate_id {
            None => (
                String::new(),
                CertificateSignature::GenesisSignature(other.signature.try_into().unwrap()),
            ),
            Some(parent_certificate_id) => (
                parent_certificate_id,
                CertificateSignature::MultiSignature(other.signature.try_into().unwrap()),
            ),
        };

        Certificate {
            hash: other.certificate_id,
            previous_hash,
            beacon: other.beacon,
            metadata: certificate_metadata,
            signed_message: other.protocol_message.compute_hash(),
            protocol_message: other.protocol_message,
            aggregate_verification_key: other.aggregate_verification_key,
            signature,
        }
    }
}

impl SqLiteEntity for CertificateRecord {
    fn hydrate(row: sqlite::Row) -> Result<Self, HydrationError>
    where
        Self: Sized,
    {
        let certificate_id = row.read::<&str, _>(0).to_string();
        let parent_certificate_id = row.read::<Option<&str>, _>(1).map(|s| s.to_owned());
        let message = row.read::<&str, _>(2).to_string();
        let signature = row.read::<&str, _>(3).to_string();
        let aggregate_verification_key = row.read::<&str, _>(4).to_string();
        let epoch_int = row.read::<i64, _>(5);
        let beacon_string = row.read::<&str, _>(6);
        let protocol_version = row.read::<&str, _>(7).to_string();
        let protocol_parameters_string = row.read::<&str, _>(8);
        let protocol_message_string = row.read::<&str, _>(9);
        let signers_string = row.read::<&str, _>(10);
        let initiated_at = row.read::<&str, _>(11);
        let sealed_at = row.read::<&str, _>(12);

        let certificate_record = Self {
            certificate_id,
            parent_certificate_id,
            message,
            signature,
            aggregate_verification_key,
            epoch: Epoch(epoch_int.try_into().map_err(|e| {
                HydrationError::InvalidData(format!(
                    "Could not cast i64 ({epoch_int}) to u64. Error: '{e}'"
                ))
            })?),
            beacon: serde_json::from_str(beacon_string).map_err(
                |e| {
                    HydrationError::InvalidData(format!(
                        "Could not turn string '{beacon_string}' to Beacon. Error: {e}"
                    ))
                },
            )?,
            protocol_version,
            protocol_parameters: serde_json::from_str(protocol_parameters_string).map_err(
                |e| {
                    HydrationError::InvalidData(format!(
                        "Could not turn string '{protocol_parameters_string}' to ProtocolParameters. Error: {e}"
                    ))
                },
            )?,
            protocol_message: serde_json::from_str(protocol_message_string).map_err(
                |e| {
                    HydrationError::InvalidData(format!(
                        "Could not turn string '{protocol_message_string}' to ProtocolMessage. Error: {e}"
                    ))
                },
            )?,
            signers: serde_json::from_str(signers_string).map_err(
                |e| {
                    HydrationError::InvalidData(format!(
                        "Could not turn string '{signers_string}' to Vec<SignerWithStake>. Error: {e}"
                    ))
                },
            )?,
            initiated_at: DateTime::parse_from_rfc3339(initiated_at).map_err(
                |e| {
                  HydrationError::InvalidData(format!(
                      "Could not turn string '{initiated_at}' to rfc3339 Datetime. Error: {e}"
                  ))
              },
            )?.with_timezone(&Utc),
            sealed_at: DateTime::parse_from_rfc3339(sealed_at).map_err(
                |e| {
                    HydrationError::InvalidData(format!(
                        "Could not turn string '{sealed_at}' to rfc3339 Datetime. Error: {e}"
                    ))
                },
            )?.with_timezone(&Utc),
        };

        Ok(certificate_record)
    }

    fn get_projection() -> Projection {
        let mut projection = Projection::default();
        projection.add_field("certificate_id", "{:certificate:}.certificate_id", "text");
        projection.add_field(
            "parent_certificate_id",
            "{:certificate:}.parent_certificate_id",
            "text",
        );
        projection.add_field("message", "{:certificate:}.message", "text");
        projection.add_field("signature", "{:certificate:}.signature", "text");
        projection.add_field(
            "aggregate_verification_key",
            "{:certificate:}.aggregate_verification_key",
            "text",
        );
        projection.add_field("epoch", "{:certificate:}.epoch", "integer");
        projection.add_field("beacon", "{:certificate:}.beacon", "text");
        projection.add_field(
            "protocol_version",
            "{:certificate:}.protocol_version",
            "text",
        );
        projection.add_field(
            "protocol_parameters",
            "{:certificate:}.protocol_parameters",
            "text",
        );
        projection.add_field(
            "protocol_message",
            "{:certificate:}.protocol_message",
            "text",
        );
        projection.add_field("signers", "{:certificate:}.signers", "text");
        projection.add_field("initiated_at", "{:certificate:}.initiated_at", "text");
        projection.add_field("sealed_at", "{:certificate:}.sealed_at", "text");

        projection
    }
}

/// Simple [CertificateRecord] provider.
pub struct CertificateRecordProvider<'client> {
    client: &'client Connection,
}

impl<'client> CertificateRecordProvider<'client> {
    /// Create a new provider
    pub fn new(client: &'client Connection) -> Self {
        Self { client }
    }

    fn condition_by_certificate_id(&self, certificate_id: &str) -> StdResult<WhereCondition> {
        Ok(WhereCondition::new(
            "certificate_id = ?*",
            vec![Value::String(certificate_id.to_owned())],
        ))
    }

    fn condition_by_epoch(&self, epoch: &Epoch) -> StdResult<WhereCondition> {
        Ok(WhereCondition::new(
            "epoch = ?*",
            vec![Value::Integer(epoch.try_into()?)],
        ))
    }

    /// Get CertificateRecords for a given certificate id.
    pub fn get_by_certificate_id(
        &self,
        certificate_id: &str,
    ) -> StdResult<EntityCursor<CertificateRecord>> {
        let filters = self.condition_by_certificate_id(certificate_id)?;
        let certificate_record = self.find(filters)?;

        Ok(certificate_record)
    }

    /// Get CertificateRecords for a given Epoch.
    pub fn get_by_epoch(&self, epoch: &Epoch) -> StdResult<EntityCursor<CertificateRecord>> {
        let filters = self.condition_by_epoch(epoch)?;
        let certificate_record = self.find(filters)?;

        Ok(certificate_record)
    }

    /// Get all CertificateRecords.
    pub fn get_all(&self) -> StdResult<EntityCursor<CertificateRecord>> {
        let filters = WhereCondition::default();
        let certificate_record = self.find(filters)?;

        Ok(certificate_record)
    }
}

impl<'client> Provider<'client> for CertificateRecordProvider<'client> {
    type Entity = CertificateRecord;

    fn get_connection(&'client self) -> &'client Connection {
        self.client
    }

    fn get_definition(&self, condition: &str) -> String {
        let aliases = SourceAlias::new(&[("{:certificate:}", "c")]);
        let projection = Self::Entity::get_projection().expand(aliases);
        format!("select {projection} from certificate as c where {condition} order by ROWID desc")
    }
}

/// Query to insert the certificate record
pub struct InsertCertificateRecordProvider<'conn> {
    connection: &'conn Connection,
}

impl<'conn> InsertCertificateRecordProvider<'conn> {
    /// Create a new instance
    pub fn new(connection: &'conn Connection) -> Self {
        Self { connection }
    }

    fn get_insert_condition(&self, certificate_record: &CertificateRecord) -> WhereCondition {
        self.get_insert_many_condition(&vec![certificate_record.clone()])
    }

    fn get_insert_many_condition(
        &self,
        certificates_records: &[CertificateRecord],
    ) -> WhereCondition {
        let columns = "(certificate_id, parent_certificate_id, message, signature, \
aggregate_verification_key, epoch, beacon, protocol_version, protocol_parameters, \
protocol_message, signers, initiated_at, sealed_at)";
        let values_columns: Vec<&str> =
            repeat("(?*, ?*, ?*, ?*, ?*, ?*, ?*, ?*, ?*, ?*, ?*, ?*, ?*)")
                .take(certificates_records.len())
                .collect();

        let values: Vec<Value> = certificates_records
            .iter()
            .flat_map(|certificate_record| {
                vec![
                    Value::String(certificate_record.certificate_id.to_owned()),
                    match certificate_record.parent_certificate_id.to_owned() {
                        Some(parent_certificate_id) => Value::String(parent_certificate_id),
                        None => Value::Null,
                    },
                    Value::String(certificate_record.message.to_owned()),
                    Value::String(certificate_record.signature.to_owned()),
                    Value::String(certificate_record.aggregate_verification_key.to_owned()),
                    Value::Integer(certificate_record.epoch.try_into().unwrap()),
                    Value::String(serde_json::to_string(&certificate_record.beacon).unwrap()),
                    Value::String(certificate_record.protocol_version.to_owned()),
                    Value::String(
                        serde_json::to_string(&certificate_record.protocol_parameters).unwrap(),
                    ),
                    Value::String(
                        serde_json::to_string(&certificate_record.protocol_message).unwrap(),
                    ),
                    Value::String(serde_json::to_string(&certificate_record.signers).unwrap()),
                    Value::String(certificate_record.initiated_at.to_rfc3339()),
                    Value::String(certificate_record.sealed_at.to_rfc3339()),
                ]
            })
            .collect();

        WhereCondition::new(
            format!("{columns} values {}", values_columns.join(", ")).as_str(),
            values,
        )
    }

    fn persist(&self, certificate_record: CertificateRecord) -> StdResult<CertificateRecord> {
        let filters = self.get_insert_condition(&certificate_record);

        let entity = self.find(filters)?.next().unwrap_or_else(|| {
            panic!(
                "No entity returned by the persister, certificate_record = {certificate_record:#?}"
            )
        });

        Ok(entity)
    }

    fn persist_many(
        &self,
        certificates_records: Vec<CertificateRecord>,
    ) -> StdResult<Vec<CertificateRecord>> {
        let filters = self.get_insert_many_condition(&certificates_records);

        Ok(self.find(filters)?.collect())
    }
}

impl<'conn> Provider<'conn> for InsertCertificateRecordProvider<'conn> {
    type Entity = CertificateRecord;

    fn get_connection(&'conn self) -> &'conn Connection {
        self.connection
    }

    fn get_definition(&self, condition: &str) -> String {
        // it is important to alias the fields with the same name as the table
        // since the table cannot be aliased in a RETURNING statement in SQLite.
        let projection = Self::Entity::get_projection()
            .expand(SourceAlias::new(&[("{:certificate:}", "certificate")]));

        format!("insert into certificate {condition} returning {projection}")
    }
}

struct MasterCertificateProvider<'conn> {
    connection: &'conn Connection,
}

impl<'conn> MasterCertificateProvider<'conn> {
    pub fn new(connection: &'conn Connection) -> Self {
        Self { connection }
    }

    pub fn get_master_certificate_condition(&self, epoch: Epoch) -> WhereCondition {
        let epoch_i64: i64 = epoch.try_into().unwrap();
        WhereCondition::new(
            "certificate.epoch between ?* and ?*",
            vec![Value::Integer(epoch_i64 - 1), Value::Integer(epoch_i64)],
        )
        .and_where(
            WhereCondition::new("certificate.parent_certificate_id is null", vec![]).or_where(
                WhereCondition::new("certificate.epoch != parent_certificate.epoch", vec![]),
            ),
        )
    }
}

impl<'conn> Provider<'conn> for MasterCertificateProvider<'conn> {
    type Entity = CertificateRecord;

    fn get_connection(&'conn self) -> &'conn Connection {
        self.connection
    }

    fn get_definition(&self, condition: &str) -> String {
        // it is important to alias the fields with the same name as the table
        // since the table cannot be aliased in a RETURNING statement in SQLite.
        let projection = Self::Entity::get_projection().expand(SourceAlias::new(&[
            ("{:certificate:}", "certificate"),
            ("{:parent_certificate:}", "parent_certificate"),
        ]));

        format!(
            r#"
select {projection}
from certificate
    left join certificate as parent_certificate 
        on parent_certificate.certificate_id = certificate.parent_certificate_id
where {condition}
order by certificate.ROWID desc"#
        )
    }
}

/// Provider to remove old data from the `certificate` table
pub struct DeleteCertificateProvider<'conn> {
    connection: &'conn Connection,
}

impl<'conn> Provider<'conn> for DeleteCertificateProvider<'conn> {
    type Entity = CertificateRecord;

    fn get_connection(&'conn self) -> &'conn Connection {
        self.connection
    }

    fn get_definition(&self, condition: &str) -> String {
        // it is important to alias the fields with the same name as the table
        // since the table cannot be aliased in a RETURNING statement in SQLite.
        let projection = Self::Entity::get_projection()
            .expand(SourceAlias::new(&[("{:certificate:}", "certificate")]));

        format!("delete from certificate where {condition} returning {projection}")
    }
}

impl<'conn> DeleteCertificateProvider<'conn> {
    /// Create a new instance
    pub fn new(connection: &'conn Connection) -> Self {
        Self { connection }
    }

    /// Create the SQL condition to delete certificates with the given ids.
    fn get_delete_by_ids_condition(&self, ids: &[&str]) -> WhereCondition {
        let ids_values = ids.iter().map(|id| Value::String(id.to_string())).collect();

        WhereCondition::where_in("certificate_id", ids_values)
    }

    /// Delete the certificates with with the given ids.
    pub fn delete_by_ids(&self, ids: &[&str]) -> StdResult<EntityCursor<CertificateRecord>> {
        let filters = self.get_delete_by_ids_condition(ids);

        self.find(filters)
    }
}

/// Database frontend API for Certificate queries.
pub struct CertificateRepository {
    connection: Arc<Mutex<Connection>>,
}

impl CertificateRepository {
    /// Instantiate a new repository
    pub fn new(connection: Arc<Mutex<Connection>>) -> Self {
        Self { connection }
    }

    /// Return the certificate corresponding to the given hash if any.
    pub async fn get_certificate(&self, hash: &str) -> StdResult<Option<Certificate>> {
        let lock = self.connection.lock().await;
        let provider = CertificateRecordProvider::new(&lock);
        let mut cursor = provider.get_by_certificate_id(hash)?;

        Ok(cursor.next().map(|v| v.into()))
    }

    /// Return the latest certificates.
    pub async fn get_latest_certificates(&self, last_n: usize) -> StdResult<Vec<Certificate>> {
        let lock = self.connection.lock().await;
        let provider = CertificateRecordProvider::new(&lock);
        let cursor = provider.get_all()?;

        Ok(cursor.take(last_n).map(|v| v.into()).collect())
    }

    /// Return the first certificate signed per epoch as the reference
    /// certificate for this Epoch. This will be the parent certificate for all
    /// other certificates issued within this Epoch.
    pub async fn get_master_certificate_for_epoch(
        &self,
        epoch: Epoch,
    ) -> StdResult<Option<Certificate>> {
        let lock = self.connection.lock().await;
        let provider = MasterCertificateProvider::new(&lock);
        let mut cursor = provider.find(provider.get_master_certificate_condition(epoch))?;

        Ok(cursor.next().map(|c| c.into()))
    }

    /// Create a new certificate in the database.
    pub async fn create_certificate(&self, certificate: Certificate) -> StdResult<Certificate> {
        let lock = self.connection.lock().await;
        let provider = InsertCertificateRecordProvider::new(&lock);

        provider.persist(certificate.into()).map(|r| r.into())
    }

    /// Create many certificates at once in the database.
    pub async fn create_many_certificates(
        &self,
        certificates: Vec<Certificate>,
    ) -> StdResult<Vec<Certificate>> {
        let lock = self.connection.lock().await;
        let provider = InsertCertificateRecordProvider::new(&lock);
        let records: Vec<CertificateRecord> =
            certificates.into_iter().map(|cert| cert.into()).collect();
        let new_certificates = provider.persist_many(records)?;

        Ok(new_certificates
            .into_iter()
            .map(|cert| cert.into())
            .collect::<Vec<_>>())
    }

    /// Delete all the given certificates from the database
    pub async fn delete_certificates(&self, certificates: &[&Certificate]) -> StdResult<()> {
        let ids = certificates
            .iter()
            .map(|c| c.hash.as_str())
            .collect::<Vec<_>>();

        let connection = self.connection.lock().await;
        let provider = DeleteCertificateProvider::new(&connection);
        let _ = provider.delete_by_ids(&ids)?.collect::<Vec<_>>();

        Ok(())
    }
}

#[async_trait]
impl CertificateRetriever for CertificateRepository {
    async fn get_certificate_details(
        &self,
        certificate_hash: &str,
    ) -> Result<Certificate, CertificateRetrieverError> {
        self.get_certificate(certificate_hash)
            .await
            .map_err(|e| CertificateRetrieverError::General(e.to_string()))?
            .ok_or(CertificateRetrieverError::General(
                "certificate does not exist".to_string(),
            ))
    }
}

#[cfg(test)]
mod tests {
    use crate::database::provider::disable_foreign_key_support;
    use crate::{
        database::provider::apply_all_migrations_to_db, dependency_injection::DependenciesBuilder,
        Configuration,
    };
    use mithril_common::crypto_helper::tests_setup::setup_certificate_chain;

    use super::*;

    pub fn setup_certificate_db(
        connection: &Connection,
        certificates: Vec<Certificate>,
    ) -> StdResult<()> {
        apply_all_migrations_to_db(connection)?;
        disable_foreign_key_support(connection)?;

        if certificates.is_empty() {
            return Ok(());
        }

        let query = {
            // leverage the expanded parameter from this provider which is unit
            // tested on its own above.
            let update_provider = InsertCertificateRecordProvider::new(connection);
            let (sql_values, _) = update_provider
                .get_insert_condition(&(certificates.first().unwrap().to_owned().into()))
                .expand();
            format!("insert into certificate {sql_values}")
        };

        for certificate in certificates {
            let certificate_record: CertificateRecord = certificate.into();
            let mut statement = connection.prepare(&query)?;
            statement
                .bind::<&[(_, Value)]>(&[
                    (1, certificate_record.certificate_id.into()),
                    (
                        2,
                        match certificate_record.parent_certificate_id {
                            None => Value::Null,
                            Some(parent_certificate_id) => parent_certificate_id.into(),
                        },
                    ),
                    (3, certificate_record.message.into()),
                    (4, certificate_record.signature.into()),
                    (5, certificate_record.aggregate_verification_key.into()),
                    (6, Value::Integer(*certificate_record.epoch as i64)),
                    (
                        7,
                        serde_json::to_string(&certificate_record.beacon)
                            .unwrap()
                            .into(),
                    ),
                    (8, certificate_record.protocol_version.into()),
                    (
                        9,
                        serde_json::to_string(&certificate_record.protocol_parameters)
                            .unwrap()
                            .into(),
                    ),
                    (
                        10,
                        serde_json::to_string(&certificate_record.protocol_message)
                            .unwrap()
                            .into(),
                    ),
                    (
                        11,
                        serde_json::to_string(&certificate_record.signers)
                            .unwrap()
                            .into(),
                    ),
                    (12, certificate_record.initiated_at.to_rfc3339().into()),
                    (13, certificate_record.sealed_at.to_rfc3339().into()),
                ])
                .unwrap();

            statement.next().unwrap();
        }

        Ok(())
    }

    fn insert_golden_certificate(connection: &Connection) {
        connection
            .execute(r#"
            -- genesis certificate
            insert into certificate
            values(
                'bfb4efbd48d58f7677ddb7d5fe5b5b9e998e8ca549cbf7583873bdccfc70f194',
                null,
                '08420665c56dcf6981b7d8b64b5a584e148edbf7638f466cb36b278ce962439c',
                'b7944ddc7d728812f8e68abc93b668a84876e9867b97648bc937b20debdff15a8415470ee709599d1a12a50ac5a57a3a4955cf19307d04955fcad6931c3b9505',
                '7b226d745f636f6d6d69746d656e74223a7b22726f6f74223a5b37372c3230382c3138392c3138372c37362c3136322c36382c3233382c3134342c31372c3131342c3137352c36302c3136352c3230322c3134362c3139342c31332c37332c3233392c3233372c3232322c3136392c3230362c352c3130392c3132332c35322c3235342c39382c3133312c37395d2c226e725f6c6561766573223a332c22686173686572223a6e756c6c7d2c22746f74616c5f7374616b65223a32383439323639303636317d',
                241,
                '{"network":"preview","epoch":241,"immutable_file_number":4823}',
                '0.1.0',
                '{"k":2422,"m":20973,"phi_f":0.2}',
                '{"message_parts":{
                    "next_aggregate_verification_key":"7b226d745f636f6d6d69746d656e74223a7b22726f6f74223a5b37372c3230382c3138392c3138372c37362c3136322c36382c3233382c3134342c31372c3131342c3137352c36302c3136352c3230322c3134362c3139342c31332c37332c3233392c3233372c3232322c3136392c3230362c352c3130392c3132332c35322c3235342c39382c3133312c37395d2c226e725f6c6561766573223a332c22686173686572223a6e756c6c7d2c22746f74616c5f7374616b65223a32383439323639303636317d"
                }}',
                '[{
                    "party_id":"pool1vapqexnsx6hvc588yyysxpjecf3k43hcr5mvhmstutuvy085xpa",
                    "verification_key":"7b22766b223a5b3133382c33322c3133382c3135322c3134362c3235352c3130382c3139302c37302c34322c3132362c3137322c31392c3135312c3133392c3133392c3235352c33352c3134312c38322c3138372c33372c3133332c3235322c3139322c302c32362c32342c3134342c372c3235332c3136362c3135312c3139332c392c3230392c3131392c3230302c3134312c34312c38302c342c3231372c3132322c3132302c3235332c3230382c3131312c362c37382c3234362c3134362c3131382c352c3235312c31392c3234332c3138342c3233382c3139352c39392c3235312c3135312c342c39342c3133382c3234362c33362c33372c34382c3133362c3130302c3233352c3134312c3232382c392c39362c3131332c35392c3137352c3130322c3232392c39352c39332c3134332c3137312c3130302c32302c3133362c36372c33302c3133312c3135332c32362c35372c3132385d2c22706f70223a5b3137342c3233302c33382c3138312c3131332c38332c372c34332c3130312c38392c3133372c3133302c37302c3135382c3235342c31342c31362c36372c38332c362c3234322c39312c3136372c34352c3232392c3139382c3130312c37302c3232382c36312c3138302c3132302c3130332c3232302c3231312c3134362c3136322c37302c33382c3230352c3139312c3235322c3138342c3235322c39362c3134382c3130322c3133362c3136362c34322c3137382c3133352c3130302c33312c38392c3233342c3135392c3131382c33382c3133392c31362c3134342c3132382c3134382c3132382c3139312c31382c34382c38392c3136352c35342c3134362c36332c3136302c3138362c3139362c31392c3137312c3136302c31342c39322c35382c3232312c3138352c3132392c382c3133322c35352c3231382c3235302c39352c32312c3235302c3135312c36352c3231395d7d",
                    "verification_key_signature":"7b227369676d61223a7b227369676d61223a7b227369676d61223a7b227369676d61223a7b227369676d61223a7b227369676d61223a5b35342c35372c32332c3234302c3234342c3130352c3139322c3138312c3130362c3232312c3132302c3139382c3136392c3134372c3233362c34382c32342c35382c3233352c31332c36302c31352c3231382c33312c34352c3135322c3133302c3230382c36392c38312c34372c3135302c3234352c3234332c32352c39342c3134382c3136322c39322c3136392c3131352c37382c31352c38382c3139382c38342c3233322c3138342c3135372c3139352c35342c3136352c33352c382c3232342c3130312c3138392c38372c32392c3131342c3133322c33382c3132322c31305d2c226c68735f706b223a5b3139322c3135342c3230322c3233342c36352c3234332c3132392c3230302c3131382c3137352c3131342c3233352c3232322c3235342c3134322c3232332c3137372c3233342c31352c31382c34312c31362c38382c38352c37322c3130372c33322c3134382c33352c35312c3132352c34355d2c227268735f706b223a5b3137342c39352c3132342c31382c36322c3135312c3137302c3136382c3232332c36362c3132322c36312c3234322c3130372c3132352c3137372c3137302c3132332c35382c3231362c3137362c392c3234302c3131382c3131302c35362c3232372c3230302c3131322c3130352c32392c3230385d7d2c226c68735f706b223a5b36392c3138322c39392c382c34302c39332c3130382c3233312c382c312c3235322c3131302c3132322c37332c3133302c3230372c3231332c3137312c3130352c3232322c31352c3134322c3230362c3137392c33382c3132302c39322c362c32302c3133352c3130382c3138335d2c227268735f706b223a5b33342c36372c3134302c3132392c3231352c36392c3136302c3135362c3230302c31302c3232362c35382c3132322c36342c33382c3135362c3230362c3230362c302c3137382c3132302c3139332c362c3135332c3131322c3130392c3135372c3131322c3132322c3133372c3233372c38355d7d2c226c68735f706b223a5b37332c3131342c3136352c3137312c34322c3131372c3139322c3139342c3137342c32302c38312c392c3230392c31392c3134352c3233302c3233302c3130392c34382c3135302c31332c3232392c3139322c35342c3138362c3137372c32382c3133362c31352c3230342c3231342c3132305d2c227268735f706b223a5b3139302c32322c3131312c38362c38322c3138362c3231372c3134312c302c3136382c3130382c3230362c3130392c332c3138342c3230342c382c3138362c3136362c32312c39372c34342c3135352c332c3136352c3139392c3132372c39312c3233382c38362c3139302c35305d7d2c226c68735f706b223a5b3135392c37352c3131382c3132372c3139382c34342c3137392c34322c3231382c3131382c3235332c3139392c32342c37312c3133302c362c3136332c3131342c3133392c31332c3130392c31372c3132372c35312c39342c3133312c3132382c3230332c3131382c3231312c3137392c36365d2c227268735f706b223a5b3139312c3136342c33362c3131312c37362c3132372c3231382c3230352c3234322c3134322c3230312c3233322c3235322c3233322c35372c39362c3131372c3232362c37332c34322c3231372c3235342c3130382c3233342c3234372c3137362c3234372c3133302c32342c36332c31392c38355d7d2c226c68735f706b223a5b35342c33392c3235342c33322c3131392c39332c3138322c3132372c3136352c3134362c3230352c33392c36352c3139362c3134362c36392c36392c34332c3139382c3130322c3139342c35372c31332c3230302c3232332c39382c38322c3134312c3133362c35382c3235322c3130325d2c227268735f706b223a5b3137372c34322c33372c3133322c3133352c3130322c3135342c392c3233362c31392c3235302c3235312c39382c36352c3133302c3232352c3136382c3232362c3136352c34392c35302c35322c3134312c3136392c35312c3230342c3234362c3130302c3233372c3234362c39322c32345d7d2c226c68735f706b223a5b33302c38302c3232322c3233372c3139302c342c3130352c3230362c37302c31372c3234382c3134322c362c31332c3137352c3136332c38342c3231352c3132322c3235352c3232302c3131382c34382c33312c34352c33332c3233372c3234352c3235302c3234302c3132392c3131355d2c227268735f706b223a5b3132332c31302c31352c36332c3138312c3231382c31302c36362c3138382c3138312c3130302c3138302c3130302c3139352c3137382c38372c3233362c32382c3138322c35362c3232362c35382c3234302c3131322c392c3133322c39332c33302c33372c3136332c3134322c39315d7d",
                    "operational_certificate":"5b5b5b3131322c39352c34322c39372c382c3235322c31382c3231342c31392c3231382c3231372c3234322c3233302c3138372c3234302c3133392c31342c3135382c3137392c3234392c3231312c36332c3132332c342c32362c3132362c3132312c3234372c302c35372c31362c3136315d2c312c37312c5b3132392c3234382c3133342c3132342c3230372c3130332c3233312c37302c3130372c32382c3134322c3134312c38362c3234392c3230352c31312c33392c3232382c3130382c3132322c3233312c3138322c3132372c3130312c3234352c33332c3135322c3233342c35342c36372c3138312c39362c3137372c3234362c32382c322c3235322c3130382c35392c3231352c3232372c3230392c3131382c3130352c3135342c37312c36332c3134352c3132372c3137352c3133382c3131352c39362c3233352c3131382c31322c3234302c3232352c3130392c3130382c3231322c3232392c35372c31305d5d2c5b33302c3138312c32302c37382c33392c3232332c352c3133372c3134312c3138392c372c3132372c34352c3232372c3230362c3135372c39352c3131352c36312c3132382c3135392c3135362c34332c3132372c302c34302c3134332c3138332c3233302c32352c39312c3137305d5d",
                    "kes_period":22,
                    "stake":1009497432569
                }]',
                '2023-06-23T08:37:49.066Z',
                '2023-06-23T08:37:49.066Z'
            );
            
            -- multi-signature certificate
            insert into certificate
            values(
                '9a86b602d1eda6d3a48967e63f5b35885368795669d9293014e1c289ee0defa7',
                '3997f18bbbe706a77fbf464101a3e6c6476a9d1dd2e10f2ed614f028713b8f11',
                '33975e636d019513d93e9182e6a5e38092909620cd4b650e06a03e2c4cf2e65a',
                '7b227369676e617475726573223a5b5b7b227369676d61223a5b3138342c3133342c38392c3137382c3234312c3232362c34372c34372c34312c36382c3136392c36352c38362c3136302c39322c362c3130382c33382c39322c3134332c3131372c3231382c33382c39342c3131332c3232372c3133332c3231302c3131332c3134312c31382c3139322c3133332c3230312c3231382c3233392c33342c3231322c39302c382c34302c3132302c3233342c3136382c3135332c3137372c3133322c34335d2c22696e6465786573223a5b312c382c31322c31342c31372c32332c32382c33332c33392c38382c39332c39382c3131342c3131352c3131372c3132372c3133322c3133342c3133362c3133392c3134302c3134312c3135302c3135372c3136332c3136342c3137322c3137372c3137382c3138312c3139302c3139312c3139322c3230302c3230312c3230332c3230342c3231352c3231362c3231392c3233322c3233342c3233372c3235302c3235312c3235352c3235362c3236322c3236352c3236362c3237372c3238302c3238342c3238392c3239372c3330302c3331312c3332302c3332312c3332382c3333332c3333342c3333372c3334322c3334332c3334342c3335342c3335372c3336302c3336392c3337352c3337362c3338362c3339342c3339372c3339382c3339392c3430312c3430322c3430352c3431302c3431352c3431372c3432302c3432372c3433302c3433362c3434312c3435302c3435392c3436352c3436362c3437322c3437342c3438322c3438352c3438382c3438392c3439312c3530342c3531302c3531342c3531362c3531372c3532312c3532322c3532342c3532382c3533302c3534342c3534392c3535302c3535312c3535322c3535372c3536322c3536382c3537342c3537392c3538322c3538352c3538382c3538392c3539342c3630392c3631382c3632312c3632342c3632392c3633312c3633352c3633392c3634302c3634312c3634322c3634362c3634372c3635302c3635372c3636342c3637332c3637352c3637362c3638312c3638342c3638372c3730312c3730322c3731352c3731382c3732352c3732392c3733302c3733362c3733382c3734322c3736312c3736372c3737312c3737322c3737342c3737382c3738392c3739312c3830362c3831332c3832332c3832372c3833342c3833382c3833392c3834352c3834382c3835322c3835352c3835362c3836352c3836372c3837302c3837312c3837322c3837342c3838332c3838352c3839302c3839372c3839392c3930312c3930332c3930352c3931362c3931382c3932322c3933342c3933362c3933382c3934342c3934362c3934392c3935322c3936382c3937302c3937322c3937342c3937372c3938302c3938332c3938352c3939342c313030372c313030392c313031302c313032312c313032372c313033312c313033362c313034312c313034392c313035342c313035372c313035392c313036342c313036362c313036372c313037352c313037382c313037392c313038312c313038382c313130312c313130372c313130382c313131312c313131362c313132332c313133322c313133332c313134302c313134352c313135312c313135362c313136312c313136382c313137302c313137332c313138312c313138392c313139322c313139332c313230312c313232362c313232372c313232392c313233382c313234302c313234352c313235312c313235322c313235332c313236352c313236362c313237342c313237362c313238302c313238352c313239352c313239362c313239372c313239382c313331302c313331322c313331382c313333322c313334302c313334312c313334352c313334362c313335352c313335362c313335392c313337322c313337382c313337392c313338332c313338362c313339322c313430312c313430352c313430362c313431302c313431322c313431332c313433342c313433372c313433382c313434372c313435322c313435352c313436392c313437362c313437382c313438302c313438372c313438382c313439302c313439342c313439352c313439392c313530342c313531332c313531342c313532342c313532382c313534302c313534312c313534322c313535352c313535362c313536302c313536392c313537332c313537342c313539332c313539362c313630342c313630392c313631372c313631392c313632322c313632332c313632372c313632382c313633392c313635352c313636352c313636392c313638312c313639352c313639392c313730302c313731342c313731352c313731372c313731382c313732302c313732342c313732362c313732392c313733312c313733322c313733362c313734332c313735342c313735372c313736382c313737312c313737362c313737382c313738322c313739302c313739322c313739342c313830322c313830372c313831392c313832382c313833392c313835342c313836372c313837342c313837362c313838362c313839342c313839372c313930312c313930382c313931352c313932392c313933372c313934372c313935352c313935362c313935372c313935382c313936322c313938342c313938352c313939302c313939322c323030302c323030312c323031342c323031372c323032352c323032392c323034312c323034362c323034372c323035322c323035352c323035382c323036342c323036392c323037302c323037332c323037352c323037362c323038332c323038352c323038372c323039302c323039312c323039322c323130362c323131332c323131392c323132302c323132362c323132392c323133312c323133342c323133392c323136352c323136392c323137302c323137372c323138302c323138342c323139342c323139362c323139372c323230342c323231352c323232302c323233322c323233362c323233372c323233382c323234302c323234382c323235302c323235382c323235392c323236312c323236372c323237372c323237392c323238342c323238392c323330302c323330352c323331322c323331372c323332302c323332332c323333322c323333352c323334392c323335322c323335362c323336342c323336362c323336372c323337302c323337352c323337392c323338332c323338362c323339322c323339332c323339362c323339372c323430352c323430382c323432392c323434302c323434312c323434322c323434342c323434382c323435362c323436302c323438302c323530332c323530342c323530392c323532332c323533332c323533342c323533372c323534382c323535392c323536372c323537302c323537322c323538352c323538392c323539302c323539342c323539352c323630362c323631302c323631342c323631382c323632322c323632332c323633322c323635312c323635352c323636312c323637322c323637362c323637382c323638302c323638312c323639312c323639322c323639332c323639362c323730302c323730332c323730382c323731332c323731342c323731352c323731362c323732392c323734342c323735312c323736312c323736322c323736332c323736362c323737332c323737342c323737352c323737392c323738342c323738382c323739302c323739362c323739392c323830392c323831302c323831312c323831392c323832312c323832372c323832382c323833382c323833392c323834382c323835332c323835392c323836332c323837352c323838312c323839342c323930302c323931302c323931312c323931352c323931392c323932312c323932332c323932372c323933322c323933362c323934352c323935352c323935372c323936322c323936372c323936392c323937322c323937352c323937372c323938302c323938322c323938332c323938342c323938352c323939332c333030362c333031322c333031342c333031362c333033312c333034372c333034382c333035342c333036342c333036362c333036382c333037322c333037362c333038312c333039302c333039312c333039342c333130322c333130332c333130342c333130372c333130382c333131312c333132342c333133302c333134352c333134362c333135342c333135382c333136302c333136322c333136392c333137342c333137362c333137382c333138362c333138372c333139312c333139332c333230322c333230342c333230392c333231302c333231332c333231352c333233312c333234322c333234342c333234392c333235362c333236302c333236382c333236392c333237352c333238302c333238382c333239302c333239392c333330322c333330352c333331362c333331382c333333322c333333342c333335312c333335322c333335362c333335392c333336362c333337312c333337332c333338332c333338372c333339302c333339312c333339372c333430372c333431312c333431332c333431392c333432332c333432392c333433342c333434312c333434352c333435372c333435382c333435392c333436332c333436372c333438372c333438392c333439302c333439332c333439342c333530322c333530352c333530382c333530392c333531302c333531312c333531352c333532322c333532392c333533392c333534312c333534342c333535312c333535342c333536322c333536382c333537302c333537372c333537382c333538302c333538322c333630302c333630342c333631362c333631382c333632302c333632322c333632332c333633372c333634362c333635352c333635392c333637302c333638322c333638372c333639392c333730322c333730362c333731322c333731372c333731382c333732312c333732382c333735302c333735312c333735322c333735332c333736332c333736342c333736382c333737332c333737362c333737382c333739322c333739352c333830362c333830382c333831342c333832382c333832392c333833322c333833342c333834302c333835392c333837392c333838302c333838312c333838342c333838352c333839342c333839372c333839382c333930322c333931322c333931332c333932352c333933382c333934332c333934352c333934372c333935312c333935392c333936352c333937312c333937332c333938372c333939312c333939322c333939372c343030342c343030392c343031362c343032362c343033312c343033322c343033352c343033362c343033382c343034302c343034322c343034332c343034342c343034392c343035302c343035332c343035352c343036362c343037302c343037332c343039302c343039332c343039342c343039352c343130332c343130342c343130392c343131352c343131382c343132332c343132352c343133302c343133312c343134332c343135322c343135382c343136312c343137342c343137392c343138302c343138362c343139352c343139382c343230332c343230362c343230382c343231372c343232302c343232372c343232382c343234312c343234322c343235362c343235392c343236312c343236392c343237302c343237362c343238342c343238382c343239312c343330342c343330352c343331322c343331352c343331362c343332302c343332342c343332362c343333322c343333352c343334302c343334322c343334342c343334392c343335302c343335312c343335342c343335352c343336322c343336352c343337332c343338302c343338362c343430362c343431332c343432302c343432342c343432382c343433312c343433372c343434322c343434342c343434382c343435302c343435322c343435382c343437352c343437372c343438312c343439322c343439372c343530332c343530372c343531312c343531322c343532362c343534312c343535322c343535332c343536302c343536332c343536372c343537312c343537392c343538322c343538332c343630302c343630382c343630392c343631322c343631332c343631382c343632382c343633302c343634322c343634332c343634342c343634372c343635322c343636342c343636382c343637332c343637352c343638332c343638362c343638382c343639392c343730302c343730312c343731312c343731322c343732312c343733312c343733322c343733382c343734332c343734382c343735342c343735352c343735372c343736342c343738302c343739352c343739372c343830352c343831332c343831392c343832302c343832332c343832342c343833302c343833332c343834312c343834322c343834352c343834392c343835302c343835322c343835342c343835352c343835382c343836372c343837312c343837322c343837332c343838302c343838312c343838322c343838352c343839312c343839322c343839362c343930302c343930322c343930392c343931302c343931392c343932342c343932362c343933392c343934312c343934332c343934342c343934372c343935302c343935382c343936332c343936352c343937362c343938302c343938342c343938372c343939362c343939382c353030362c353030372c353031362c353032342c353034342c353034362c353035352c353035372c353036302c353036312c353036332c353037302c353037342c353037362c353037372c353038342c353039312c353039342c353039352c353131312c353131382c353132332c353132382c353133312c353134302c353134312c353134342c353134352c353135312c353135332c353135342c353136302c353137372c353137382c353139332c353230352c353230362c353231352c353231362c353231392c353232312c353232342c353232372c353233302c353233352c353234312c353234332c353234352c353234372c353236362c353236382c353237372c353238332c353238372c353238382c353239322c353239352c353239362c353330302c353330332c353330362c353330372c353330392c353331332c353331342c353331372c353332312c353332322c353332372c353332392c353334302c353334332c353335342c353335382c353336352c353337392c353338372c353339362c353430312c353430352c353430362c353430372c353431342c353432312c353432332c353433302c353433362c353434382c353435312c353435372c353435392c353436302c353436332c353437342c353437392c353438322c353438332c353438372c353438392c353439352c353530382c353531302c353531352c353532322c353532342c353533322c353533372c353533382c353535332c353535342c353535352c353536352c353537352c353538302c353538362c353538372c353538392c353539352c353630342c353630352c353630372c353631332c353632322c353632332c353632342c353633302c353633342c353633372c353634342c353634362c353635302c353635312c353635342c353635362c353636322c353636352c353637312c353637382c353638392c353730322c353730382c353731322c353731372c353732342c353733302c353733312c353733322c353733332c353733372c353733382c353734382c353735362c353736302c353736352c353736392c353737332c353738312c353738342c353739372c353739382c353830312c353830332c353830342c353830372c353830382c353831322c353831342c353832312c353832332c353832382c353834312c353835322c353836362c353836382c353838352c353839302c353839322c353930342c353930362c353930382c353931302c353931332c353932322c353932342c353933342c353933372c353934302c353934392c353936302c353936322c353936342c353936352c353936362c353937372c353937392c353938302c353938362c353939332c353939352c353939382c363030362c363030382c363031382c363032302c363032332c363033302c363033332c363033362c363033372c363034322c363034352c363035322c363035352c363035382c363035392c363036302c363036342c363036382c363036392c363037332c363037342c363037382c363038322c363038332c363038372c363039312c363039352c363039392c363131322c363131352c363131382c363133312c363134342c363135352c363135372c363136302c363136342c363136382c363137302c363137362c363137372c363138302c363138372c363139362c363230302c363230332c363230352c363231302c363231342c363231352c363232302c363232352c363232362c363233302c363233312c363233352c363234312c363234392c363235332c363235362c363236362c363236372c363236382c363237312c363237342c363238342c363238392c363239312c363330392c363331352c363332302c363332332c363332342c363333322c363333332c363333342c363333352c363333382c363334322c363334332c363334342c363334372c363335312c363335332c363335352c363335382c363335392c363337322c363337362c363337372c363338312c363338342c363338362c363339332c363339342c363339382c363339392c363430302c363430332c363430342c363430352c363430362c363431332c363431362c363432332c363432362c363432372c363433332c363434392c363435382c363436302c363436322c363436352c363437322c363437362c363437372c363438312c363438352c363438362c363439302c363530352c363531302c363531392c363532312c363532322c363532362c363533392c363534312c363534332c363534362c363535312c363535352c363536342c363536372c363537322c363538382c363538392c363539342c363539362c363630342c363630362c363631332c363631352c363631362c363632312c363632332c363633332c363633352c363634322c363634372c363634382c363635332c363635342c363636342c363636352c363637352c363637382c363638352c363638372c363639342c363730302c363730322c363730332c363730342c363730372c363731372c363732302c363733312c363733342c363733382c363734352c363734392c363735302c363735312c363735332c363735362c363735372c363736352c363736362c363737362c363738302c363738322c363738342c363738392c363739332c363830332c363830342c363830362c363830382c363831312c363831382c363832332c363832372c363833332c363834352c363835372c363836352c363836372c363837302c363837342c363838362c363839332c363839352c363930372c363930382c363931332c363932352c363932372c363932392c363934342c363934362c363935302c363935332c363936312c363936352c363936362c363936382c363937302c363937322c363937372c363938302c363938342c363938372c363939302c373030302c373030312c373030362c373031302c373031332c373031352c373032322c373032352c373032392c373033302c373033352c373033392c373034312c373034372c373034392c373035332c373036332c373036382c373037322c373037352c373037372c373038312c373038322c373038392c373039332c373039352c373039372c373039392c373130352c373131362c373131372c373133372c373133382c373133392c373134302c373134372c373134382c373135382c373136372c373136392c373137302c373137312c373137342c373137362c373137382c373138382c373139382c373139392c373230332c373230382c373231352c373231392c373232302c373232312c373232362c373232382c373233362c373234372c373234392c373235332c373237302c373237312c373237342c373238302c373238342c373238382c373239382c373239392c373330352c373330382c373331352c373331392c373332372c373333342c373333372c373333382c373334342c373334352c373335352c373335362c373335382c373336332c373336372c373337312c373337362c373337372c373338332c373339332c373339342c373339392c373430302c373430312c373431352c373432332c373432382c373433312c373433372c373434312c373434332c373434382c373435342c373435352c373435362c373436362c373437352c373438302c373438322c373438332c373439342c373439362c373439372c373530322c373530342c373530362c373531322c373531332c373531362c373533302c373533312c373533332c373533342c373533372c373533392c373534332c373534382c373535302c373535312c373535372c373536312c373536382c373537332c373538362c373538382c373539302c373539362c373539372c373539382c373539392c373630382c373631302c373631322c373632352c373633312c373633342c373633382c373634332c373634352c373634372c373636322c373636392c373637322c373637362c373638332c373638382c373639352c373730322c373731302c373731362c373732342c373732352c373732372c373732382c373733302c373734352c373735302c373735372c373736342c373736382c373736392c373737362c373737372c373738302c373738372c373738392c373739332c373830302c373831322c373832342c373833332c373833352c373834302c373834332c373834372c373835302c373836302c373836362c373837342c373838352c373838362c373839312c373839382c373930332c373930372c373930382c373932342c373933322c373933382c373934312c373935302c373935352c373936302c373937302c373937322c373937332c373937372c373938352c373938372c373939322c373939362c373939392c383030312c383031312c383031352c383031362c383032332c383033302c383033312c383033322c383033352c383033382c383033392c383034362c383035352c383036352c383037332c383037392c383038312c383038362c383039332c383039382c383039392c383131392c383132382c383133312c383133332c383133362c383133382c383134302c383134322c383135372c383135392c383136302c383136312c383137302c383137312c383137342c383137352c383137382c383138302c383139302c383139322c383230342c383230362c383233362c383234302c383234322c383234382c383235372c383236302c383236312c383236332c383236362c383238352c383238382c383238392c383239352c383330322c383330332c383330352c383330362c383330372c383331332c383331362c383332322c383332382c383332392c383333362c383334332c383334372c383335322c383335382c383336322c383336382c383337392c383338312c383338332c383338372c383339322c383430302c383430342c383431352c383432322c383433312c383433382c383433392c383434352c383434392c383435302c383435312c383435372c383435382c383435392c383437312c383437322c383437362c383438302c383438312c383438342c383439302c383439322c383439342c383439362c383530332c383531362c383531372c383532302c383532332c383533302c383533372c383533382c383535312c383535342c383535352c383536302c383536312c383536352c383538312c383538332c383538372c383539302c383539322c383630302c383632342c383632372c383632392c383633312c383634322c383634342c383634352c383634382c383636312c383637302c383637312c383639372c383730332c383730342c383730392c383731372c383732352c383732382c383733302c383733332c383734342c383735302c383735352c383736392c383737332c383737342c383737352c383737392c383738302c383738332c383738352c383739302c383739312c383739392c383830332c383830342c383830352c383831392c383833312c383833362c383834312c383834322c383834332c383834362c383835302c383835312c383835322c383835342c383836332c383836392c383837302c383838322c383838362c383838372c383838382c383839372c383930322c383931322c383932312c383932382c383932392c383933332c383933382c383935342c383935352c383935362c383936332c383936342c383937322c383937362c383938382c383939302c393030322c393030352c393030372c393032352c393033302c393033352c393035302c393036362c393036372c393036382c393037312c393037352c393037372c393038312c393038322c393038352c393038362c393038372c393039312c393130322c393131332c393131342c393131352c393131392c393132312c393133392c393134392c393135312c393135352c393136362c393136372c393136382c393137302c393137352c393137382c393138322c393139372c393230302c393231302c393231332c393232322c393233312c393233332c393233352c393234342c393235392c393236302c393237322c393238302c393238342c393238382c393238392c393239322c393239392c393330312c393330332c393330352c393331322c393331332c393331352c393331362c393331392c393332302c393333302c393333362c393334302c393334322c393335352c393336352c393337302c393338382c393339352c393339392c393430312c393430392c393431342c393433342c393433352c393433382c393435352c393435372c393437392c393438362c393439322c393530332c393530372c393530392c393531322c393531372c393532362c393533302c393533362c393533392c393534332c393534352c393535312c393535322c393535342c393536312c393536322c393537322c393537332c393539332c393539342c393539382c393630322c393630332c393630342c393630352c393630372c393631302c393631352c393632302c393632332c393633322c393633372c393634322c393634352c393634382c393635322c393635352c393635372c393636312c393636392c393637302c393637312c393637342c393638332c393638352c393638392c393639302c393730332c393730342c393730372c393730382c393734332c393735312c393737392c393738332c393738382c393830302c393830392c393831362c393832312c393832322c393832342c393832362c393833322c393833342c393833382c393836322c393836342c393837302c393837322c393837372c393838312c393838352c393838392c393839342c393839352c393930332c393930352c393930372c393930382c393931392c393932302c393932362c393933332c393933342c393933362c393933392c393934382c393934392c393935322c393935382c393935392c393936392c393937392c393938302c393939312c393939372c393939382c31303030332c31303031322c31303031352c31303031372c31303032322c31303032362c31303033382c31303034342c31303034362c31303034382c31303035312c31303035342c31303036332c31303036342c31303036352c31303037332c31303038302c31303038322c31303130302c31303130312c31303130322c31303131352c31303132312c31303132322c31303133302c31303133382c31303134332c31303135312c31303135362c31303135382c31303136302c31303136342c31303136382c31303137302c31303137352c31303137392c31303139312c31303139342c31303139372c31303139382c31303230312c31303230322c31303230392c31303231302c31303231332c31303232342c31303232382c31303233302c31303233362c31303234302c31303234312c31303234362c31303235372c31303236312c31303236322c31303237312c31303237322c31303237362c31303239352c31303239372c31303239382c31303330302c31303330382c31303331322c31303331342c31303331392c31303332312c31303332362c31303333352c31303333362c31303334312c31303334322c31303334392c31303335332c31303336312c31303336322c31303336362c31303338312c31303338322c31303338332c31303338342c31303338392c31303339342c31303430322c31303430372c31303430382c31303431302c31303431312c31303431352c31303431382c31303432302c31303432322c31303433302c31303433342c31303433392c31303434312c31303434352c31303434362c31303434392c31303435312c31303435322c31303435392c31303436302c31303438362c31303439342c31303439352c31303530302c31303530322c31303531342c31303531352c31303531392c31303532342c31303532362c31303533302c31303533332c31303534302c31303534312c31303534352c31303535342c31303535382c31303535392c31303536302c31303536332c31303537352c31303537392c31303538342c31303538392c31303539302c31303539312c31303539352c31303539392c31303630352c31303631312c31303632372c31303633312c31303634332c31303634372c31303634382c31303634392c31303635312c31303636322c31303637312c31303637352c31303638332c31303638362c31303639302c31303639332c31303730302c31303730322c31303730342c31303731392c31303732312c31303732382c31303732392c31303733352c31303733362c31303733382c31303733392c31303734352c31303735332c31303735382c31303736332c31303737372c31303738312c31303738382c31303739302c31303739322c31303739392c31303830302c31303830362c31303830392c31303831322c31303831362c31303832322c31303832332c31303832342c31303834362c31303835302c31303835332c31303835352c31303835382c31303835392c31303836382c31303837332c31303837392c31303838302c31303838322c31303838352c31303839352c31303839382c31303930322c31303930352c31303930362c31303932342c31303933362c31303935322c31303935382c31303936372c31303937312c31303937332c31303937392c31303938322c31303938342c31303939342c31313030322c31313030352c31313030382c31313031342c31313031392c31313032352c31313032382c31313032392c31313033302c31313034342c31313034352c31313035302c31313035332c31313036352c31313037372c31313037382c31313038332c31313038342c31313038392c31313039392c31313130312c31313130342c31313132312c31313132332c31313132372c31313133302c31313133352c31313133372c31313134332c31313134372c31313135312c31313135322c31313135332c31313135372c31313135382c31313136312c31313136332c31313137302c31313137362c31313138362c31313138382c31313139312c31313231362c31313231382c31313232302c31313232322c31313232342c31313232362c31313232392c31313233382c31313234352c31313234382c31313235382c31313236332c31313236382c31313237302c31313237372c31313238312c31313238322c31313238342c31313239302c31313239352c31313239372c31313239382c31313330312c31313330322c31313330332c31313331352c31313333302c31313333322c31313333332c31313334352c31313334392c31313335332c31313336332c31313337342c31313338312c31313338322c31313338352c31313338382c31313339342c31313339362c31313339372c31313430302c31313430332c31313430342c31313431322c31313431332c31313431342c31313432322c31313433322c31313433342c31313434332c31313434372c31313434392c31313435302c31313435312c31313435352c31313435362c31313435382c31313436322c31313436392c31313437332c31313437382c31313438342c31313439392c31313530332c31313530342c31313530352c31313530392c31313531362c31313532352c31313533322c31313533332c31313533362c31313533372c31313533392c31313534312c31313535382c31313536322c31313536332c31313536362c31313536382c31313537372c31313538302c31313538382c31313539322c31313539352c31313539372c31313539382c31313630322c31313630362c31313630382c31313630392c31313631372c31313632352c31313633352c31313633392c31313634302c31313634312c31313634322c31313634332c31313634372c31313635392c31313636302c31313636322c31313636342c31313636372c31313638342c31313638362c31313638382c31313639312c31313639362c31313639372c31313639382c31313639392c31313730312c31313730322c31313730372c31313730382c31313731352c31313732352c31313732362c31313732392c31313733352c31313734312c31313735312c31313736362c31313736382c31313737322c31313738302c31313738352c31313739302c31313830362c31313831302c31313833302c31313833312c31313833362c31313834322c31313834332c31313834392c31313835352c31313835392c31313836362c31313838342c31313838352c31313839302c31313839322c31313839342c31313839392c31313930362c31313931352c31313932312c31313932352c31313932362c31313933352c31313934352c31313934372c31313934382c31313937382c31313938362c31313938392c31323030312c31323030352c31323031352c31323031362c31323031382c31323032302c31323032312c31323032362c31323033312c31323033372c31323034342c31323034362c31323035312c31323035342c31323036332c31323036362c31323037362c31323038302c31323038342c31323039342c31323039362c31323130312c31323130342c31323131332c31323131352c31323132312c31323132362c31323133322c31323136332c31323136362c31323136382c31323139342c31323139392c31323231322c31323231342c31323232382c31323233362c31323233392c31323234362c31323235302c31323236302c31323236312c31323236372c31323237332c31323238382c31323239302c31323239342c31323239382c31323330332c31323330342c31323330352c31323331382c31323332322c31323332332c31323332342c31323333362c31323334302c31323334322c31323335352c31323335362c31323335382c31323336322c31323336342c31323336392c31323337362c31323337372c31323338302c31323338312c31323338392c31323339342c31323339382c31323339392c31323431352c31323432342c31323432362c31323432392c31323433332c31323435332c31323436302c31323436362c31323436382c31323437332c31323437342c31323437352c31323437382c31323438322c31323438342c31323438382c31323439332c31323530382c31323531302c31323531372c31323533372c31323534312c31323534342c31323534362c31323535312c31323535382c31323536302c31323536312c31323538302c31323538362c31323538372c31323539362c31323630342c31323630362c31323630392c31323631372c31323632392c31323633352c31323633362c31323633382c31323633392c31323634342c31323634392c31323636342c31323638332c31323639312c31323639322c31323639352c31323639392c31323730322c31323730342c31323731332c31323731372c31323732302c31323733312c31323733362c31323734352c31323734372c31323734392c31323736332c31323737372c31323737392c31323738332c31323738352c31323739332c31323739352c31323830312c31323830382c31323831352c31323832312c31323832322c31323833312c31323833382c31323833392c31323834332c31323834372c31323835392c31323836302c31323838312c31323838332c31323838392c31323839342c31323839372c31323839392c31323930322c31323930352c31323930362c31323931332c31323931382c31323932342c31323932352c31323932382c31323932392c31323936312c31323936332c31323936342c31323937312c31323938322c31323939322c31333030322c31333031332c31333031362c31333031382c31333031392c31333032322c31333033322c31333033382c31333034372c31333035392c31333036312c31333036392c31333037302c31333037362c31333037372c31333037382c31333039352c31333130322c31333130352c31333131312c31333131332c31333131362c31333133302c31333133312c31333134302c31333134382c31333136342c31333136372c31333137332c31333138312c31333138392c31333139312c31333139372c31333230312c31333230342c31333230352c31333230362c31333232302c31333232332c31333232382c31333233342c31333234302c31333234312c31333234352c31333234372c31333235312c31333236302c31333237302c31333237362c31333238322c31333238352c31333239352c31333239392c31333330302c31333330352c31333332342c31333333332c31333333342c31333333352c31333333362c31333335362c31333336312c31333336382c31333337352c31333337382c31333338362c31333338372c31333339312c31333339342c31333430302c31333430352c31333431372c31333432322c31333433302c31333433362c31333434312c31333434322c31333434382c31333434392c31333435342c31333435352c31333436302c31333436312c31333436332c31333436382c31333436392c31333438342c31333439392c31333530302c31333530332c31333530392c31333531302c31333531332c31333531382c31333531392c31333533322c31333533382c31333534302c31333534312c31333536302c31333536382c31333537332c31333537382c31333538302c31333539332c31333539352c31333539392c31333630312c31333630392c31333631372c31333632312c31333632342c31333634312c31333634362c31333634372c31333634382c31333636372c31333637302c31333637352c31333637382c31333730382c31333731312c31333731332c31333731342c31333731362c31333731372c31333732322c31333734382c31333735352c31333736362c31333736372c31333737302c31333737342c31333737382c31333738342c31333739322c31333739332c31333739392c31333830302c31333830322c31333830342c31333830352c31333831312c31333832302c31333832332c31333832352c31333832372c31333832392c31333834312c31333834322c31333835362c31333835372c31333836362c31333836372c31333836382c31333837362c31333837392c31333838332c31333838372c31333839312c31333839322c31333839362c31333930332c31333930352c31333930362c31333931332c31333931372c31333932362c31333932392c31333934302c31333934322c31333934382c31333935302c31333935312c31333935322c31333936302c31333936352c31333936372c31333937312c31333937342c31333937352c31333938302c31333939312c31333939322c31333939332c31333939382c31343030302c31343031342c31343031352c31343031362c31343031382c31343032342c31343033322c31343033342c31343035312c31343035362c31343035382c31343036302c31343036312c31343036322c31343036332c31343036372c31343036392c31343037352c31343038332c31343038382c31343039342c31343130342c31343131372c31343131382c31343132312c31343132322c31343132342c31343132372c31343134342c31343135312c31343135362c31343136362c31343136392c31343137312c31343137332c31343138312c31343138322c31343138362c31343139312c31343139322c31343230342c31343230362c31343231332c31343231362c31343232312c31343233332c31343233362c31343234322c31343234342c31343235372c31343236302c31343236322c31343236352c31343236392c31343237372c31343237382c31343238322c31343238332c31343238342c31343238352c31343239302c31343239312c31343239392c31343330312c31343330352c31343331312c31343331342c31343331362c31343332302c31343332392c31343333302c31343333322c31343333372c31343334302c31343334322c31343334362c31343334382c31343335362c31343336302c31343336322c31343336362c31343337322c31343337332c31343337372c31343338332c31343338342c31343338362c31343338392c31343430382c31343431312c31343431332c31343431392c31343432322c31343432342c31343433312c31343433372c31343433392c31343434302c31343434312c31343434342c31343434352c31343435302c31343436352c31343436362c31343436372c31343438392c31343439342c31343439392c31343530312c31343530352c31343531342c31343531352c31343531362c31343531382c31343532362c31343533312c31343533332c31343534342c31343535362c31343535372c31343536322c31343536352c31343536372c31343537322c31343537392c31343538322c31343538392c31343539352c31343539362c31343630322c31343630332c31343632372c31343633302c31343633362c31343634322c31343634352c31343634382c31343634392c31343635312c31343635372c31343635382c31343636392c31343637342c31343638312c31343638362c31343638382c31343639322c31343639342c31343639352c31343730332c31343730342c31343730362c31343730372c31343730392c31343731342c31343731382c31343732322c31343732352c31343733312c31343733322c31343733342c31343733362c31343733372c31343733382c31343733392c31343734342c31343734372c31343735302c31343735322c31343735362c31343736332c31343736342c31343736392c31343737302c31343737322c31343738302c31343738392c31343739312c31343739392c31343831342c31343831382c31343832302c31343832342c31343832372c31343834342c31343836312c31343836342c31343836362c31343836372c31343836382c31343837322c31343837352c31343837362c31343837392c31343838322c31343838362c31343838372c31343839352c31343839362c31343839372c31343839382c31343839392c31343930342c31343931332c31343931392c31343933302c31343933392c31343934302c31343934312c31343935372c31343936302c31343936312c31343936332c31343936362c31343936382c31343937332c31343937342c31343937392c31343938332c31343938342c31343939302c31343939352c31353031322c31353031342c31353032302c31353032382c31353033392c31353034302c31353034342c31353034362c31353035372c31353035382c31353036322c31353036352c31353037352c31353038342c31353038372c31353130302c31353130312c31353130322c31353130372c31353131302c31353131352c31353133312c31353133342c31353134342c31353134362c31353134372c31353135302c31353135362c31353135382c31353136302c31353136332c31353137322c31353137382c31353138302c31353138342c31353139352c31353230332c31353230392c31353231322c31353232342c31353232352c31353232392c31353233392c31353234302c31353234332c31353234382c31353236322c31353236382c31353236392c31353238362c31353239302c31353239342c31353239362c31353330342c31353330352c31353330382c31353331312c31353333352c31353333392c31353334352c31353336332c31353336352c31353336392c31353430312c31353431322c31353432352c31353432362c31353432372c31353433302c31353433332c31353433362c31353433372c31353434312c31353434332c31353435312c31353435322c31353435342c31353435392c31353436312c31353436362c31353436392c31353437302c31353437372c31353438362c31353438372c31353439302c31353439352c31353439382c31353530312c31353530352c31353530362c31353530372c31353530382c31353531342c31353531382c31353532312c31353532362c31353533372c31353534322c31353534362c31353535372c31353535382c31353536342c31353536362c31353630352c31353630362c31353631352c31353631362c31353632362c31353633332c31353633362c31353633382c31353634302c31353634312c31353634332c31353634352c31353635302c31353635342c31353635372c31353635392c31353636302c31353637302c31353637342c31353637352c31353638312c31353638372c31353639342c31353730322c31353730372c31353731312c31353731382c31353732312c31353732352c31353733312c31353733342c31353734342c31353734362c31353735322c31353735342c31353736302c31353736322c31353737362c31353738312c31353738352c31353738362c31353738382c31353739312c31353739322c31353739332c31353739352c31353830302c31353830312c31353830322c31353830382c31353831382c31353832332c31353832362c31353833342c31353833392c31353834392c31353835352c31353835362c31353835372c31353836312c31353836332c31353837352c31353839342c31353930322c31353930372c31353930392c31353933392c31353934302c31353934322c31353934362c31353934372c31353934392c31353935362c31353936332c31353937342c31353938302c31353938332c31353939302c31353939322c31363031322c31363031362c31363032332c31363032342c31363033302c31363034372c31363035302c31363035312c31363037302c31363037322c31363038332c31363038352c31363038372c31363039312c31363039352c31363039382c31363130302c31363130372c31363130382c31363130392c31363131302c31363131322c31363131392c31363132302c31363132312c31363133372c31363133392c31363134332c31363134372c31363135372c31363135382c31363136362c31363136392c31363137302c31363137312c31363137392c31363138302c31363138362c31363138382c31363139322c31363139342c31363139382c31363230302c31363230352c31363231392c31363232312c31363232322c31363232332c31363232382c31363233302c31363233312c31363233332c31363233392c31363234302c31363235322c31363235382c31363236302c31363236332c31363236382c31363237312c31363237352c31363237372c31363238322c31363238352c31363238372c31363238382c31363238392c31363239322c31363239352c31363239362c31363239372c31363330322c31363330332c31363331302c31363331352c31363332322c31363332342c31363332382c31363332392c31363333322c31363333392c31363334352c31363334372c31363335352c31363336302c31363336322c31363336332c31363336362c31363336392c31363337352c31363337382c31363338312c31363338352c31363338372c31363339362c31363430322c31363430352c31363430372c31363431322c31363431342c31363431372c31363432382c31363433312c31363433332c31363433362c31363435312c31363435362c31363436312c31363436372c31363437372c31363437382c31363437392c31363438362c31363439362c31363439382c31363531352c31363531362c31363531382c31363532302c31363533302c31363533372c31363534362c31363534382c31363535362c31363535372c31363535382c31363536302c31363536362c31363537302c31363537342c31363537362c31363538332c31363538342c31363538362c31363539302c31363539342c31363539392c31363630322c31363630372c31363631342c31363632332c31363632392c31363633302c31363634332c31363634342c31363634382c31363635342c31363636302c31363636312c31363636382c31363636392c31363637332c31363637362c31363638332c31363638372c31363638382c31363638392c31363639362c31363730362c31363730392c31363731332c31363731362c31363731382c31363733302c31363733362c31363734362c31363735382c31363736332c31363736362c31363737342c31363737362c31363737382c31363738322c31363738352c31363738362c31363739372c31363831312c31363831322c31363831342c31363831382c31363832312c31363832392c31363833342c31363833362c31363834312c31363834322c31363834352c31363835332c31363835372c31363836362c31363836372c31363837312c31363837382c31363838342c31363838382c31363839322c31363839352c31363930302c31363930332c31363931322c31363931352c31363932332c31363933312c31363936312c31363936342c31363936352c31363937342c31363938312c31363938332c31363938372c31363939302c31363939332c31373030342c31373030372c31373030392c31373031312c31373031372c31373032302c31373032352c31373033302c31373033352c31373033392c31373034362c31373035302c31373035322c31373035372c31373035392c31373036362c31373036392c31373037302c31373037312c31373038322c31373039362c31373039392c31373130322c31373131382c31373132302c31373132322c31373132332c31373132362c31373133362c31373134302c31373134312c31373134382c31373135312c31373135362c31373135372c31373135392c31373136302c31373136322c31373137312c31373137332c31373138382c31373139352c31373139362c31373139372c31373230312c31373230332c31373230342c31373231322c31373231352c31373231362c31373231372c31373232302c31373232352c31373232382c31373232392c31373234362c31373235312c31373235372c31373236312c31373236322c31373236332c31373236342c31373237302c31373238302c31373238372c31373238392c31373330322c31373330342c31373330372c31373331302c31373331312c31373331332c31373331372c31373332302c31373332382c31373333332c31373333392c31373335342c31373335362c31373336322c31373336332c31373336362c31373336392c31373337302c31373337322c31373337352c31373337372c31373337392c31373338322c31373338372c31373339322c31373339372c31373339392c31373430312c31373430392c31373431312c31373431392c31373432312c31373433352c31373434372c31373434382c31373434392c31373435362c31373435382c31373435392c31373437312c31373437362c31373438352c31373438372c31373438392c31373439372c31373439382c31373530342c31373530372c31373530382c31373532322c31373532352c31373532362c31373533302c31373533362c31373534312c31373534322c31373534342c31373535302c31373535322c31373535362c31373535382c31373535392c31373536302c31373536322c31373536382c31373538302c31373539382c31373539392c31373631322c31373631372c31373633332c31373634372c31373635312c31373635362c31373636302c31373636332c31373636372c31373637322c31373637362c31373637372c31373638392c31373639352c31373730302c31373730352c31373731302c31373731362c31373732392c31373733322c31373733382c31373733392c31373734362c31373735362c31373735372c31373736372c31373737312c31373737352c31373737362c31373738362c31373830372c31373830392c31373832332c31373832342c31373833332c31373833362c31373834302c31373835352c31373835372c31373837352c31373837382c31373838322c31373838342c31373838362c31373839302c31373839372c31373930302c31373930332c31373930352c31373930372c31373930392c31373933302c31373934312c31373934342c31373934382c31373934392c31373935332c31373935342c31373935382c31373935392c31373936392c31373937372c31373938342c31373938352c31373939352c31373939362c31373939392c31383030372c31383030382c31383031312c31383031322c31383031352c31383031382c31383031392c31383032362c31383032392c31383034332c31383034392c31383035332c31383035372c31383036332c31383037382c31383037392c31383038322c31383038362c31383038392c31383039362c31383130302c31383130322c31383130342c31383130362c31383130382c31383131302c31383131392c31383132372c31383132382c31383133332c31383133342c31383133372c31383134372c31383134392c31383135302c31383137382c31383138322c31383138362c31383230322c31383231302c31383231362c31383231392c31383232302c31383232332c31383232382c31383233322c31383233382c31383233392c31383234312c31383234322c31383234332c31383234382c31383235332c31383235372c31383236312c31383236342c31383237312c31383237362c31383238302c31383238372c31383239312c31383239392c31383330332c31383330352c31383330362c31383331332c31383331352c31383331372c31383332302c31383332342c31383332372c31383332382c31383333362c31383333382c31383334332c31383334362c31383335342c31383335392c31383336382c31383336392c31383337302c31383338382c31383339332c31383339382c31383431312c31383432382c31383433302c31383433342c31383434342c31383434362c31383434372c31383435302c31383435322c31383436302c31383436322c31383436342c31383438342c31383439322c31383439362c31383530322c31383530342c31383530372c31383531312c31383531332c31383532342c31383532352c31383532362c31383532372c31383532392c31383533322c31383534332c31383534372c31383534382c31383535332c31383535362c31383536322c31383536342c31383536362c31383537372c31383538342c31383538382c31383539302c31383539362c31383630372c31383631342c31383631362c31383632332c31383632352c31383632372c31383632392c31383633302c31383633332c31383633382c31383634362c31383634372c31383635352c31383635372c31383636312c31383636332c31383636382c31383637332c31383637342c31383637352c31383638312c31383638382c31383639302c31383639332c31383639352c31383639382c31383639392c31383730312c31383731372c31383732302c31383732332c31383733302c31383733312c31383733322c31383733362c31383734352c31383734392c31383736372c31383737352c31383737382c31383738302c31383738312c31383738392c31383739302c31383830312c31383830392c31383831332c31383831392c31383832302c31383832362c31383833322c31383833332c31383833362c31383834342c31383834372c31383835352c31383835362c31383835372c31383836302c31383836352c31383836372c31383837312c31383837362c31383837372c31383839322c31383839342c31383839372c31383931322c31383931352c31383931372c31383932322c31383932332c31383932382c31383933372c31383933392c31383934362c31383935362c31383935382c31383936332c31383936362c31383937332c31383938312c31383938372c31383939302c31383939342c31383939352c31383939362c31393030302c31393030382c31393031322c31393031342c31393031372c31393032302c31393032312c31393032382c31393032392c31393033312c31393033332c31393034382c31393035372c31393036332c31393036352c31393036362c31393036382c31393037382c31393037392c31393038312c31393038362c31393038372c31393039302c31393039312c31393039352c31393130342c31393130372c31393131352c31393132312c31393132352c31393132392c31393133302c31393133332c31393133342c31393133362c31393133382c31393134312c31393134392c31393135362c31393135382c31393136312c31393136332c31393137332c31393138352c31393139302c31393139312c31393139382c31393230302c31393230312c31393230332c31393230362c31393230372c31393230392c31393231342c31393232312c31393233302c31393233312c31393233322c31393233372c31393234312c31393235392c31393236312c31393236382c31393236392c31393237312c31393237332c31393237342c31393238312c31393238332c31393238372c31393238392c31393239322c31393239342c31393239352c31393330342c31393330372c31393332342c31393332352c31393332362c31393333312c31393333342c31393333372c31393334312c31393334352c31393334372c31393335372c31393336322c31393336382c31393337342c31393338332c31393338342c31393339362c31393431302c31393431332c31393431342c31393433352c31393434332c31393434342c31393435362c31393435382c31393437302c31393437382c31393437392c31393438302c31393530312c31393530322c31393530362c31393531332c31393531392c31393532302c31393532372c31393533382c31393534352c31393534362c31393535352c31393535372c31393536382c31393537342c31393539302c31393630392c31393631302c31393631322c31393632312c31393632342c31393634392c31393635312c31393635372c31393635382c31393635392c31393636302c31393636352c31393636382c31393637372c31393637392c31393638382c31393639322c31393639332c31393639342c31393639352c31393639362c31393639392c31393730342c31393730372c31393731302c31393731382c31393732302c31393732312c31393732332c31393732342c31393732362c31393733322c31393734302c31393734322c31393734342c31393735332c31393735352c31393735382c31393736312c31393736362c31393736392c31393737362c31393738322c31393738332c31393738342c31393738372c31393738392c31393739302c31393830352c31393830392c31393831352c31393832302c31393832382c31393834312c31393834342c31393835302c31393835332c31393835352c31393835392c31393836332c31393837372c31393838302c31393838352c31393839322c31393839332c31393839352c31393930382c31393931362c31393932302c31393932312c31393932362c31393933302c31393933322c31393934332c31393935332c31393935352c31393935362c31393936302c31393936372c31393938342c31393938382c31393938392c32303030352c32303031362c32303032362c32303032372c32303034352c32303034362c32303035312c32303035342c32303037372c32303037382c32303038322c32303038332c32303039322c32303039372c32303039382c32303130312c32303130342c32303130382c32303132312c32303132322c32303132382c32303134342c32303134352c32303134362c32303134382c32303134392c32303135302c32303135342c32303135362c32303136332c32303136342c32303136392c32303138342c32303138352c32303138382c32303230302c32303230352c32303230392c32303231312c32303231322c32303231332c32303231382c32303232302c32303232342c32303233312c32303233332c32303234302c32303234312c32303234322c32303234352c32303235362c32303235392c32303236312c32303238322c32303238342c32303239302c32303239342c32303239382c32303330332c32303330352c32303331312c32303331332c32303331392c32303332312c32303333312c32303333362c32303333382c32303334352c32303334362c32303335302c32303335342c32303336332c32303337312c32303338302c32303338332c32303338352c32303338362c32303338392c32303339332c32303339352c32303339362c32303430332c32303430352c32303432332c32303432382c32303433362c32303433392c32303434312c32303434352c32303435362c32303435392c32303436332c32303437302c32303437332c32303437342c32303437372c32303438312c32303438352c32303438392c32303530372c32303531342c32303531372c32303532342c32303532382c32303532392c32303533372c32303534322c32303534362c32303534382c32303535392c32303536302c32303536372c32303536382c32303536392c32303537362c32303538322c32303538392c32303539322c32303539342c32303631322c32303631372c32303632322c32303632372c32303634382c32303635332c32303635352c32303635362c32303636302c32303636322c32303636392c32303637302c32303637332c32303637372c32303638332c32303638362c32303639312c32303639322c32303730342c32303731332c32303731362c32303731382c32303732352c32303734392c32303735352c32303737302c32303738312c32303738332c32303738362c32303739322c32303739342c32303831342c32303832342c32303832362c32303832372c32303833352c32303834312c32303834352c32303834392c32303835382c32303836312c32303836332c32303838322c32303838382c32303839322c32303839342c32303930322c32303931332c32303931382c32303933382c32303934302c32303935362c32303936372c32303937315d2c227369676e65725f696e646578223a307d2c5b5b3133382c33322c3133382c3135322c3134362c3235352c3130382c3139302c37302c34322c3132362c3137322c31392c3135312c3133392c3133392c3235352c33352c3134312c38322c3138372c33372c3133332c3235322c3139322c302c32362c32342c3134342c372c3235332c3136362c3135312c3139332c392c3230392c3131392c3230302c3134312c34312c38302c342c3231372c3132322c3132302c3235332c3230382c3131312c362c37382c3234362c3134362c3131382c352c3235312c31392c3234332c3138342c3233382c3139352c39392c3235312c3135312c342c39342c3133382c3234362c33362c33372c34382c3133362c3130302c3233352c3134312c3232382c392c39362c3131332c35392c3137352c3130322c3232392c39352c39332c3134332c3137312c3130302c32302c3133362c36372c33302c3133312c3135332c32362c35372c3132385d2c313030393439373433323536395d5d5d2c2262617463685f70726f6f66223a7b2276616c756573223a5b5d2c22696e6469636573223a5b305d2c22686173686572223a6e756c6c7d7d',
                '7b226d745f636f6d6d69746d656e74223a7b22726f6f74223a5b3134302c31332c3135352c3134312c3136332c372c38362c3232372c34372c31392c3138302c3132372c3139362c3130382c3137312c3135382c3134302c37372c3137352c3135392c3133362c3139332c3130382c34322c3134322c3234342c38352c3131362c3235322c3135362c3233352c35305d2c226e725f6c6561766573223a312c22686173686572223a6e756c6c7d2c22746f74616c5f7374616b65223a313030393439373433323536397d',
                142,
                '{"network":"preview","epoch":142,"immutable_file_number":2838}','0.1.0',
                '{"k":2422,"m":20973,"phi_f":0.2}',
                '{"message_parts":{
                    "snapshot_digest":"cfed71151e42f8208b841531dc95477f10db25083db5eb9759e745155e83ca7c",
                    "next_aggregate_verification_key":"7b226d745f636f6d6d69746d656e74223a7b22726f6f74223a5b3132322c3131322c3131302c37332c3131352c3130302c33352c3131322c37312c3130372c3139392c3139322c3131352c37382c32312c38322c3131362c3136312c35312c34332c3233342c3134332c3139382c3138352c33342c3233302c3131332c3234352c3136392c3137332c3136322c37315d2c226e725f6c6561766573223a322c22686173686572223a6e756c6c7d2c22746f74616c5f7374616b65223a323031383939353036313631357d"
                 }}',
                '[{
                    "party_id":"pool1vapqexnsx6hvc588yyysxpjecf3k43hcr5mvhmstutuvy085xpa",
                    "verification_key":"7b22766b223a5b3133382c33322c3133382c3135322c3134362c3235352c3130382c3139302c37302c34322c3132362c3137322c31392c3135312c3133392c3133392c3235352c33352c3134312c38322c3138372c33372c3133332c3235322c3139322c302c32362c32342c3134342c372c3235332c3136362c3135312c3139332c392c3230392c3131392c3230302c3134312c34312c38302c342c3231372c3132322c3132302c3235332c3230382c3131312c362c37382c3234362c3134362c3131382c352c3235312c31392c3234332c3138342c3233382c3139352c39392c3235312c3135312c342c39342c3133382c3234362c33362c33372c34382c3133362c3130302c3233352c3134312c3232382c392c39362c3131332c35392c3137352c3130322c3232392c39352c39332c3134332c3137312c3130302c32302c3133362c36372c33302c3133312c3135332c32362c35372c3132385d2c22706f70223a5b3137342c3233302c33382c3138312c3131332c38332c372c34332c3130312c38392c3133372c3133302c37302c3135382c3235342c31342c31362c36372c38332c362c3234322c39312c3136372c34352c3232392c3139382c3130312c37302c3232382c36312c3138302c3132302c3130332c3232302c3231312c3134362c3136322c37302c33382c3230352c3139312c3235322c3138342c3235322c39362c3134382c3130322c3133362c3136362c34322c3137382c3133352c3130302c33312c38392c3233342c3135392c3131382c33382c3133392c31362c3134342c3132382c3134382c3132382c3139312c31382c34382c38392c3136352c35342c3134362c36332c3136302c3138362c3139362c31392c3137312c3136302c31342c39322c35382c3232312c3138352c3132392c382c3133322c35352c3231382c3235302c39352c32312c3235302c3135312c36352c3231395d7d",
                    "verification_key_signature":"7b227369676d61223a7b227369676d61223a7b227369676d61223a7b227369676d61223a7b227369676d61223a7b227369676d61223a5b35342c35372c32332c3234302c3234342c3130352c3139322c3138312c3130362c3232312c3132302c3139382c3136392c3134372c3233362c34382c32342c35382c3233352c31332c36302c31352c3231382c33312c34352c3135322c3133302c3230382c36392c38312c34372c3135302c3234352c3234332c32352c39342c3134382c3136322c39322c3136392c3131352c37382c31352c38382c3139382c38342c3233322c3138342c3135372c3139352c35342c3136352c33352c382c3232342c3130312c3138392c38372c32392c3131342c3133322c33382c3132322c31305d2c226c68735f706b223a5b3139322c3135342c3230322c3233342c36352c3234332c3132392c3230302c3131382c3137352c3131342c3233352c3232322c3235342c3134322c3232332c3137372c3233342c31352c31382c34312c31362c38382c38352c37322c3130372c33322c3134382c33352c35312c3132352c34355d2c227268735f706b223a5b3137342c39352c3132342c31382c36322c3135312c3137302c3136382c3232332c36362c3132322c36312c3234322c3130372c3132352c3137372c3137302c3132332c35382c3231362c3137362c392c3234302c3131382c3131302c35362c3232372c3230302c3131322c3130352c32392c3230385d7d2c226c68735f706b223a5b36392c3138322c39392c382c34302c39332c3130382c3233312c382c312c3235322c3131302c3132322c37332c3133302c3230372c3231332c3137312c3130352c3232322c31352c3134322c3230362c3137392c33382c3132302c39322c362c32302c3133352c3130382c3138335d2c227268735f706b223a5b33342c36372c3134302c3132392c3231352c36392c3136302c3135362c3230302c31302c3232362c35382c3132322c36342c33382c3135362c3230362c3230362c302c3137382c3132302c3139332c362c3135332c3131322c3130392c3135372c3131322c3132322c3133372c3233372c38355d7d2c226c68735f706b223a5b37332c3131342c3136352c3137312c34322c3131372c3139322c3139342c3137342c32302c38312c392c3230392c31392c3134352c3233302c3233302c3130392c34382c3135302c31332c3232392c3139322c35342c3138362c3137372c32382c3133362c31352c3230342c3231342c3132305d2c227268735f706b223a5b3139302c32322c3131312c38362c38322c3138362c3231372c3134312c302c3136382c3130382c3230362c3130392c332c3138342c3230342c382c3138362c3136362c32312c39372c34342c3135352c332c3136352c3139392c3132372c39312c3233382c38362c3139302c35305d7d2c226c68735f706b223a5b3135392c37352c3131382c3132372c3139382c34342c3137392c34322c3231382c3131382c3235332c3139392c32342c37312c3133302c362c3136332c3131342c3133392c31332c3130392c31372c3132372c35312c39342c3133312c3132382c3230332c3131382c3231312c3137392c36365d2c227268735f706b223a5b3139312c3136342c33362c3131312c37362c3132372c3231382c3230352c3234322c3134322c3230312c3233322c3235322c3233322c35372c39362c3131372c3232362c37332c34322c3231372c3235342c3130382c3233342c3234372c3137362c3234372c3133302c32342c36332c31392c38355d7d2c226c68735f706b223a5b35342c33392c3235342c33322c3131392c39332c3138322c3132372c3136352c3134362c3230352c33392c36352c3139362c3134362c36392c36392c34332c3139382c3130322c3139342c35372c31332c3230302c3232332c39382c38322c3134312c3133362c35382c3235322c3130325d2c227268735f706b223a5b3137372c34322c33372c3133322c3133352c3130322c3135342c392c3233362c31392c3235302c3235312c39382c36352c3133302c3232352c3136382c3232362c3136352c34392c35302c35322c3134312c3136392c35312c3230342c3234362c3130302c3233372c3234362c39322c32345d7d2c226c68735f706b223a5b33302c38302c3232322c3233372c3139302c342c3130352c3230362c37302c31372c3234382c3134322c362c31332c3137352c3136332c38342c3231352c3132322c3235352c3232302c3131382c34382c33312c34352c33332c3233372c3234352c3235302c3234302c3132392c3131355d2c227268735f706b223a5b3132332c31302c31352c36332c3138312c3231382c31302c36362c3138382c3138312c3130302c3138302c3130302c3139352c3137382c38372c3233362c32382c3138322c35362c3232362c35382c3234302c3131322c392c3133322c39332c33302c33372c3136332c3134322c39315d7d",
                    "operational_certificate":"5b5b5b3131322c39352c34322c39372c382c3235322c31382c3231342c31392c3231382c3231372c3234322c3233302c3138372c3234302c3133392c31342c3135382c3137392c3234392c3231312c36332c3132332c342c32362c3132362c3132312c3234372c302c35372c31362c3136315d2c312c37312c5b3132392c3234382c3133342c3132342c3230372c3130332c3233312c37302c3130372c32382c3134322c3134312c38362c3234392c3230352c31312c33392c3232382c3130382c3132322c3233312c3138322c3132372c3130312c3234352c33332c3135322c3233342c35342c36372c3138312c39362c3137372c3234362c32382c322c3235322c3130382c35392c3231352c3232372c3230392c3131382c3130352c3135342c37312c36332c3134352c3132372c3137352c3133382c3131352c39362c3233352c3131382c31322c3234302c3232352c3130392c3130382c3231322c3232392c35372c31305d5d2c5b33302c3138312c32302c37382c33392c3232332c352c3133372c3134312c3138392c372c3132372c34352c3232372c3230362c3135372c39352c3131352c36312c3132382c3135392c3135362c34332c3132372c302c34302c3134332c3138332c3233302c32352c39312c3137305d5d",
                    "kes_period":22,
                    "stake":1009497432569
                }]',
                '2023-03-16T01:51:00.880Z',
                '2023-03-16T02:07:22.145Z'
            );
            "#,
            )
            .unwrap();
    }

    #[test]
    fn test_golden_master() {
        let connection = Connection::open(":memory:").unwrap();
        setup_certificate_db(&connection, vec![]).unwrap();
        insert_golden_certificate(&connection);

        let provider = CertificateRecordProvider::new(&connection);
        let certificate_records: Vec<CertificateRecord> = provider
            .get_all()
            .expect("Getting Golden certificates should not fail")
            .collect();

        assert_eq!(certificate_records.len(), 2);
    }

    #[test]
    fn test_convert_certificates() {
        let (certificates, _) = setup_certificate_chain(20, 3);
        let mut certificate_records: Vec<CertificateRecord> = Vec::new();
        for certificate in certificates.clone() {
            certificate_records.push(certificate.into());
        }
        let mut certificates_new: Vec<Certificate> = Vec::new();
        for certificate_record in certificate_records {
            certificates_new.push(certificate_record.into());
        }
        assert_eq!(certificates, certificates_new);
    }

    #[test]
    fn converting_certificate_record_to_certificate_should_not_recompute_hash() {
        let expected_hash = "my_hash";
        let record =
            CertificateRecord::dummy_genesis(expected_hash, Beacon::new(String::new(), 1, 1));
        let certificate: Certificate = record.into();

        assert_eq!(expected_hash, &certificate.hash);
    }

    #[test]
    fn projection() {
        let projection = CertificateRecord::get_projection();
        let aliases = SourceAlias::new(&[("{:certificate:}", "c")]);

        assert_eq!(
            "c.certificate_id as certificate_id, c.parent_certificate_id as parent_certificate_id, c.message as message, c.signature as signature, c.aggregate_verification_key as aggregate_verification_key, c.epoch as epoch, c.beacon as beacon, c.protocol_version as protocol_version, c.protocol_parameters as protocol_parameters, c.protocol_message as protocol_message, c.signers as signers, c.initiated_at as initiated_at, c.sealed_at as sealed_at"
                .to_string(),
            projection.expand(aliases)
        );
    }

    #[test]
    fn get_certificate_record_by_epoch() {
        let connection = Connection::open(":memory:").unwrap();
        let provider = CertificateRecordProvider::new(&connection);
        let condition = provider.condition_by_epoch(&Epoch(17)).unwrap();
        let (filter, values) = condition.expand();

        assert_eq!("epoch = ?1".to_string(), filter);
        assert_eq!(vec![Value::Integer(17)], values);
    }

    #[test]
    fn get_certificate_record_by_certificate_id() {
        let connection = Connection::open(":memory:").unwrap();
        let provider = CertificateRecordProvider::new(&connection);
        let condition = provider
            .condition_by_certificate_id("certificate-123")
            .unwrap();
        let (filter, values) = condition.expand();

        assert_eq!("certificate_id = ?1".to_string(), filter);
        assert_eq!(vec![Value::String("certificate-123".to_string())], values);
    }

    #[test]
    fn insert_certificate_condition() {
        let (certificates, _) = setup_certificate_chain(2, 1);
        let certificate_record: CertificateRecord = certificates.first().unwrap().to_owned().into();
        let connection = Connection::open(":memory:").unwrap();
        let provider = InsertCertificateRecordProvider::new(&connection);
        let condition = provider.get_insert_condition(&certificate_record);
        let (values, params) = condition.expand();

        assert_eq!(
            "(certificate_id, parent_certificate_id, message, signature, aggregate_verification_key, epoch, beacon, protocol_version, protocol_parameters, protocol_message, signers, initiated_at, sealed_at) values (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)".to_string(),
            values
        );
        assert_eq!(
            vec![
                Value::String(certificate_record.certificate_id),
                Value::String(certificate_record.parent_certificate_id.unwrap()),
                Value::String(certificate_record.message),
                Value::String(certificate_record.signature),
                Value::String(certificate_record.aggregate_verification_key),
                Value::Integer(*certificate_record.epoch as i64),
                Value::String(serde_json::to_string(&certificate_record.beacon).unwrap()),
                Value::String(certificate_record.protocol_version),
                Value::String(
                    serde_json::to_string(&certificate_record.protocol_parameters).unwrap(),
                ),
                Value::String(serde_json::to_string(&certificate_record.protocol_message).unwrap()),
                Value::String(serde_json::to_string(&certificate_record.signers).unwrap()),
                Value::String(certificate_record.initiated_at.to_rfc3339()),
                Value::String(certificate_record.sealed_at.to_rfc3339()),
            ],
            params
        );
    }

    #[test]
    fn insert_many_certificates_condition() {
        let (certificates, _) = setup_certificate_chain(2, 1);
        let certificates_records: Vec<CertificateRecord> =
            certificates.into_iter().map(|c| c.into()).collect();
        let connection = Connection::open(":memory:").unwrap();
        let provider = InsertCertificateRecordProvider::new(&connection);
        let condition = provider.get_insert_many_condition(&certificates_records);
        let (values, params) = condition.expand();

        assert_eq!(
            "(certificate_id, parent_certificate_id, message, signature, \
aggregate_verification_key, epoch, beacon, protocol_version, protocol_parameters, \
protocol_message, signers, initiated_at, sealed_at) values \
(?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13), \
(?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21, ?22, ?23, ?24, ?25, ?26)"
                .to_string(),
            values
        );
        assert_eq!(
            certificates_records
                .into_iter()
                .flat_map(|certificate_record| {
                    vec![
                        Value::String(certificate_record.certificate_id),
                        match certificate_record.parent_certificate_id {
                            Some(id) => Value::String(id),
                            None => Value::Null,
                        },
                        Value::String(certificate_record.message),
                        Value::String(certificate_record.signature),
                        Value::String(certificate_record.aggregate_verification_key),
                        Value::Integer(*certificate_record.epoch as i64),
                        Value::String(serde_json::to_string(&certificate_record.beacon).unwrap()),
                        Value::String(certificate_record.protocol_version),
                        Value::String(
                            serde_json::to_string(&certificate_record.protocol_parameters).unwrap(),
                        ),
                        Value::String(
                            serde_json::to_string(&certificate_record.protocol_message).unwrap(),
                        ),
                        Value::String(serde_json::to_string(&certificate_record.signers).unwrap()),
                        Value::String(certificate_record.initiated_at.to_rfc3339()),
                        Value::String(certificate_record.sealed_at.to_rfc3339()),
                    ]
                })
                .collect::<Vec<_>>(),
            params
        );
    }

    #[test]
    fn test_get_certificate_records() {
        let (certificates, _) = setup_certificate_chain(20, 7);

        let connection = Connection::open(":memory:").unwrap();
        setup_certificate_db(&connection, certificates.clone()).unwrap();

        let provider = CertificateRecordProvider::new(&connection);

        let certificate_records: Vec<CertificateRecord> =
            provider.get_by_epoch(&Epoch(1)).unwrap().collect();
        let expected_certificate_records: Vec<CertificateRecord> = certificates
            .iter()
            .filter_map(|c| (c.beacon.epoch == Epoch(1)).then_some(c.to_owned().into()))
            .rev()
            .collect();
        assert_eq!(expected_certificate_records, certificate_records);

        let certificate_records: Vec<CertificateRecord> =
            provider.get_by_epoch(&Epoch(3)).unwrap().collect();
        let expected_certificate_records: Vec<CertificateRecord> = certificates
            .iter()
            .filter_map(|c| (c.beacon.epoch == Epoch(3)).then_some(c.to_owned().into()))
            .rev()
            .collect();
        assert_eq!(expected_certificate_records, certificate_records);

        let cursor = provider.get_by_epoch(&Epoch(5)).unwrap();
        assert_eq!(0, cursor.count());

        let certificate_records: Vec<CertificateRecord> = provider.get_all().unwrap().collect();
        let expected_certificate_records: Vec<CertificateRecord> = certificates
            .iter()
            .map(|c| c.to_owned().into())
            .rev()
            .collect();
        assert_eq!(expected_certificate_records, certificate_records);
    }

    #[test]
    fn test_insert_certificate_record() {
        let (certificates, _) = setup_certificate_chain(5, 2);

        let connection = Connection::open(":memory:").unwrap();
        setup_certificate_db(&connection, Vec::new()).unwrap();

        let provider = InsertCertificateRecordProvider::new(&connection);

        for certificate in certificates {
            let certificate_record: CertificateRecord = certificate.into();
            let certificate_record_saved = provider.persist(certificate_record.clone()).unwrap();
            assert_eq!(certificate_record, certificate_record_saved);
        }
    }

    #[test]
    fn test_insert_many_certificates_records() {
        let (certificates, _) = setup_certificate_chain(5, 2);
        let certificates_records: Vec<CertificateRecord> =
            certificates.into_iter().map(|cert| cert.into()).collect();

        let connection = Connection::open(":memory:").unwrap();
        setup_certificate_db(&connection, Vec::new()).unwrap();

        let provider = InsertCertificateRecordProvider::new(&connection);
        let certificates_records_saved = provider
            .persist_many(certificates_records.clone())
            .expect("saving many records should not fail");

        assert_eq!(certificates_records, certificates_records_saved);
    }

    #[tokio::test]
    async fn master_certificate_condition() {
        let connection = Connection::open(":memory:").unwrap();
        let provider = MasterCertificateProvider::new(&connection);
        let condition = provider.get_master_certificate_condition(Epoch(10));
        let (condition_str, parameters) = condition.expand();

        assert_eq!(
            "certificate.epoch between ?1 and ?2 and (certificate.parent_certificate_id is null or certificate.epoch != parent_certificate.epoch)".to_string(),
            condition_str
        );
        assert_eq!(vec![Value::Integer(9), Value::Integer(10)], parameters);
    }

    #[tokio::test]
    async fn repository_get_certificate() {
        let (certificates, _) = setup_certificate_chain(5, 2);
        let expected_hash = certificates[0].hash.clone();
        let mut deps = DependenciesBuilder::new(Configuration::new_sample());
        let connection = deps.get_sqlite_connection().await.unwrap();
        {
            let lock = connection.lock().await;
            let provider = InsertCertificateRecordProvider::new(&lock);

            for certificate in certificates.iter().rev() {
                provider.persist(certificate.to_owned().into()).unwrap();
            }
        }

        let repository = CertificateRepository::new(connection);
        let certificate = repository.get_certificate("whatever").await.unwrap();
        assert!(certificate.is_none());

        let certificate = repository
            .get_certificate(&expected_hash)
            .await
            .unwrap()
            .expect("The certificate exist and should be returned.");

        assert_eq!(expected_hash, certificate.hash);
    }

    #[tokio::test]
    async fn repository_get_latest_certificates() {
        let (certificates, _) = setup_certificate_chain(5, 2);
        let mut deps = DependenciesBuilder::new(Configuration::new_sample());
        let connection = deps.get_sqlite_connection().await.unwrap();
        {
            let lock = connection.lock().await;
            let provider = InsertCertificateRecordProvider::new(&lock);

            for certificate in certificates.iter().rev() {
                provider.persist(certificate.to_owned().into()).unwrap();
            }
        }

        let repository = CertificateRepository::new(connection);
        let latest_certificates = repository
            .get_latest_certificates(certificates.len())
            .await
            .unwrap();

        assert_eq!(certificates, latest_certificates);
    }

    async fn insert_certificate_records(
        connection: Arc<Mutex<Connection>>,
        records: Vec<CertificateRecord>,
    ) {
        let lock = connection.lock().await;
        let provider = InsertCertificateRecordProvider::new(&lock);

        for certificate in records {
            provider.persist(certificate).unwrap();
        }
    }

    #[tokio::test]
    async fn get_master_certificate_no_certificate_recorded_returns_none() {
        let mut deps = DependenciesBuilder::new(Configuration::new_sample());
        let connection = deps.get_sqlite_connection().await.unwrap();
        let certificates = vec![];
        insert_certificate_records(connection.clone(), certificates).await;

        let repository = CertificateRepository::new(connection);
        let certificate = repository
            .get_master_certificate_for_epoch(Epoch(1))
            .await
            .unwrap();

        assert_eq!(None, certificate);
    }

    #[tokio::test]
    async fn get_master_certificate_one_cert_in_current_epoch_recorded_returns_that_one() {
        let mut deps = DependenciesBuilder::new(Configuration::new_sample());
        let connection = deps.get_sqlite_connection().await.unwrap();
        let certificate = CertificateRecord::dummy_genesis("1", Beacon::new(String::new(), 1, 1));
        let expected_certificate: Certificate = certificate.clone().into();
        insert_certificate_records(connection.clone(), vec![certificate]).await;

        let repository = CertificateRepository::new(connection);
        let certificate = repository
            .get_master_certificate_for_epoch(Epoch(1))
            .await
            .unwrap()
            .expect("This should return a certificate.");

        assert_eq!(expected_certificate, certificate);
    }

    #[tokio::test]
    async fn get_master_certificate_multiple_cert_in_current_epoch_returns_first_of_current_epoch()
    {
        let mut deps = DependenciesBuilder::new(Configuration::new_sample());
        let connection = deps.get_sqlite_connection().await.unwrap();
        let certificates = vec![
            CertificateRecord::dummy_genesis("1", Beacon::new(String::new(), 1, 1)),
            CertificateRecord::dummy("2", "1", Beacon::new(String::new(), 1, 2)),
            CertificateRecord::dummy("3", "1", Beacon::new(String::new(), 1, 3)),
        ];
        let expected_certificate: Certificate = certificates.first().unwrap().clone().into();
        insert_certificate_records(connection.clone(), certificates).await;

        let repository = CertificateRepository::new(connection);
        let certificate = repository
            .get_master_certificate_for_epoch(Epoch(1))
            .await
            .unwrap()
            .expect("This should return a certificate.");

        assert_eq!(expected_certificate, certificate);
    }

    #[tokio::test]
    async fn get_master_certificate_multiple_cert_in_previous_epoch_none_in_the_current_returns_first_of_previous_epoch(
    ) {
        let mut deps = DependenciesBuilder::new(Configuration::new_sample());
        let connection = deps.get_sqlite_connection().await.unwrap();
        let certificates = vec![
            CertificateRecord::dummy_genesis("1", Beacon::new(String::new(), 1, 1)),
            CertificateRecord::dummy("2", "1", Beacon::new(String::new(), 1, 2)),
            CertificateRecord::dummy("3", "1", Beacon::new(String::new(), 1, 3)),
        ];
        let expected_certificate: Certificate = certificates.first().unwrap().clone().into();
        insert_certificate_records(connection.clone(), certificates).await;

        let repository = CertificateRepository::new(connection);
        let certificate = repository
            .get_master_certificate_for_epoch(Epoch(2))
            .await
            .unwrap()
            .expect("This should return a certificate.");

        assert_eq!(expected_certificate, certificate);
    }

    #[tokio::test]
    async fn get_master_certificate_multiple_cert_in_previous_one_cert_in_current_epoch_returns_one_in_current_epoch(
    ) {
        let mut deps = DependenciesBuilder::new(Configuration::new_sample());
        let connection = deps.get_sqlite_connection().await.unwrap();
        let certificates = vec![
            CertificateRecord::dummy_genesis("1", Beacon::new(String::new(), 1, 1)),
            CertificateRecord::dummy("2", "1", Beacon::new(String::new(), 1, 2)),
            CertificateRecord::dummy("3", "1", Beacon::new(String::new(), 1, 3)),
            CertificateRecord::dummy("4", "1", Beacon::new(String::new(), 2, 4)),
        ];
        let expected_certificate: Certificate = certificates.last().unwrap().clone().into();
        insert_certificate_records(connection.clone(), certificates).await;

        let repository = CertificateRepository::new(connection);
        let certificate = repository
            .get_master_certificate_for_epoch(Epoch(2))
            .await
            .unwrap()
            .expect("This should return a certificate.");

        assert_eq!(expected_certificate, certificate);
    }

    #[tokio::test]
    async fn get_master_certificate_multiple_cert_in_previous_multiple_in_current_epoch_returns_first_of_current_epoch(
    ) {
        let mut deps = DependenciesBuilder::new(Configuration::new_sample());
        let connection = deps.get_sqlite_connection().await.unwrap();
        let certificates = vec![
            CertificateRecord::dummy_genesis("1", Beacon::new(String::new(), 1, 1)),
            CertificateRecord::dummy("2", "1", Beacon::new(String::new(), 1, 2)),
            CertificateRecord::dummy("3", "1", Beacon::new(String::new(), 1, 3)),
            CertificateRecord::dummy("4", "1", Beacon::new(String::new(), 2, 4)),
            CertificateRecord::dummy("5", "4", Beacon::new(String::new(), 2, 5)),
            CertificateRecord::dummy("6", "4", Beacon::new(String::new(), 2, 6)),
        ];
        let expected_certificate: Certificate = certificates.get(3).unwrap().clone().into();
        insert_certificate_records(connection.clone(), certificates).await;

        let repository = CertificateRepository::new(connection);
        let certificate = repository
            .get_master_certificate_for_epoch(Epoch(2))
            .await
            .unwrap()
            .expect("This should return a certificate.");
        assert_eq!(expected_certificate, certificate);
    }

    #[tokio::test]
    async fn get_master_certificate_multiple_cert_in_penultimate_epoch_none_in_previous_returns_none(
    ) {
        let mut deps = DependenciesBuilder::new(Configuration::new_sample());
        let connection = deps.get_sqlite_connection().await.unwrap();
        let certificates = vec![
            CertificateRecord::dummy_genesis("1", Beacon::new(String::new(), 1, 1)),
            CertificateRecord::dummy("2", "1", Beacon::new(String::new(), 1, 2)),
            CertificateRecord::dummy("3", "1", Beacon::new(String::new(), 1, 3)),
        ];
        insert_certificate_records(connection.clone(), certificates).await;

        let repository = CertificateRepository::new(connection);
        let certificate = repository
            .get_master_certificate_for_epoch(Epoch(3))
            .await
            .unwrap();

        assert_eq!(None, certificate);
    }

    #[tokio::test]
    async fn get_master_certificate_second_genesis_after_multiple_cert_in_current_epoch_returns_last_genesis(
    ) {
        let mut deps = DependenciesBuilder::new(Configuration::new_sample());
        let connection = deps.get_sqlite_connection().await.unwrap();
        let certificates = vec![
            CertificateRecord::dummy_genesis("1", Beacon::new(String::new(), 1, 1)),
            CertificateRecord::dummy("2", "1", Beacon::new(String::new(), 1, 2)),
            CertificateRecord::dummy("3", "1", Beacon::new(String::new(), 1, 3)),
            CertificateRecord::dummy_genesis("4", Beacon::new(String::new(), 1, 3)),
        ];
        let expected_certificate: Certificate = certificates.last().unwrap().clone().into();
        insert_certificate_records(connection.clone(), certificates).await;

        let repository = CertificateRepository::new(connection);
        let certificate = repository
            .get_master_certificate_for_epoch(Epoch(2))
            .await
            .unwrap()
            .expect("This should return a certificate.");

        assert_eq!(expected_certificate, certificate);
    }

    #[tokio::test]
    async fn get_master_certificate_second_genesis_after_multiple_cert_in_multiple_epochs_returns_last_genesis(
    ) {
        let mut deps = DependenciesBuilder::new(Configuration::new_sample());
        let connection = deps.get_sqlite_connection().await.unwrap();
        let certificates = vec![
            CertificateRecord::dummy_genesis("1", Beacon::new(String::new(), 1, 1)),
            CertificateRecord::dummy("2", "1", Beacon::new(String::new(), 1, 2)),
            CertificateRecord::dummy("3", "1", Beacon::new(String::new(), 1, 2)),
            CertificateRecord::dummy("4", "1", Beacon::new(String::new(), 2, 4)),
            CertificateRecord::dummy("5", "1", Beacon::new(String::new(), 2, 5)),
            CertificateRecord::dummy_genesis("6", Beacon::new(String::new(), 2, 5)),
        ];
        let expected_certificate: Certificate = certificates.last().unwrap().clone().into();
        insert_certificate_records(connection.clone(), certificates).await;

        let repository = CertificateRepository::new(connection);
        let certificate = repository
            .get_master_certificate_for_epoch(Epoch(2))
            .await
            .unwrap()
            .expect("This should return a certificate.");

        assert_eq!(expected_certificate, certificate);
    }

    #[tokio::test]
    async fn get_master_certificate_new_genesis_after_multiple_cert_in_previous_epoch_returns_last_genesis(
    ) {
        let mut deps = DependenciesBuilder::new(Configuration::new_sample());
        let connection = deps.get_sqlite_connection().await.unwrap();
        let certificates = vec![
            CertificateRecord::dummy_genesis("1", Beacon::new(String::new(), 1, 1)),
            CertificateRecord::dummy("2", "1", Beacon::new(String::new(), 1, 2)),
            CertificateRecord::dummy("3", "1", Beacon::new(String::new(), 1, 3)),
            CertificateRecord::dummy_genesis("4", Beacon::new(String::new(), 2, 3)),
        ];
        let expected_certificate: Certificate = certificates.last().unwrap().clone().into();
        insert_certificate_records(connection.clone(), certificates).await;

        let repository = CertificateRepository::new(connection);
        let certificate = repository
            .get_master_certificate_for_epoch(Epoch(2))
            .await
            .unwrap()
            .expect("This should return a certificate.");

        assert_eq!(expected_certificate, certificate);
    }

    #[tokio::test]
    async fn get_master_certificate_for_epoch() {
        let (certificates, _) = setup_certificate_chain(3, 1);
        let expected_certificate_id = &certificates[2].hash;
        let epoch = &certificates[2].beacon.epoch;
        let mut deps = DependenciesBuilder::new(Configuration::new_sample());
        let connection = deps.get_sqlite_connection().await.unwrap();
        {
            let lock = connection.lock().await;
            let provider = InsertCertificateRecordProvider::new(&lock);

            for certificate in certificates.iter().rev() {
                provider.persist(certificate.to_owned().into()).unwrap();
            }
        }

        let repository = CertificateRepository::new(connection);
        let certificate = repository
            .get_master_certificate_for_epoch(*epoch)
            .await
            .unwrap()
            .expect("This should return a certificate.");

        assert_eq!(expected_certificate_id.to_string(), certificate.hash);
    }

    #[tokio::test]
    async fn save_certificate() {
        let (certificates, _) = setup_certificate_chain(5, 3);
        let mut deps = DependenciesBuilder::new(Configuration::new_sample());
        let connection = deps.get_sqlite_connection().await.unwrap();
        let repository = CertificateRepository::new(connection);
        let certificate = repository
            .create_certificate(certificates[4].clone())
            .await
            .unwrap();

        assert_eq!(certificates[4].hash, certificate.hash);
        {
            let connection = deps.get_sqlite_connection().await.unwrap();
            let lock = connection.lock().await;
            let provider = CertificateRecordProvider::new(&lock);
            let mut cursor = provider
                .get_by_certificate_id(&certificates[4].hash)
                .unwrap();
            let cert = cursor
                .next()
                .expect("There should be a certificate in the database with this hash ID.");

            assert_eq!(certificates[4].hash, cert.certificate_id);
        }
    }

    #[test]
    fn delete_certificates_condition_correctly_joins_given_ids() {
        let connection = Connection::open(":memory:").unwrap();
        let provider = DeleteCertificateProvider::new(&connection);
        let condition = provider.get_delete_by_ids_condition(&["a", "b", "c"]);
        let (condition, params) = condition.expand();

        assert_eq!("certificate_id in (?1, ?2, ?3)".to_string(), condition);
        assert_eq!(
            vec![
                Value::String("a".to_string()),
                Value::String("b".to_string()),
                Value::String("c".to_string()),
            ],
            params
        );
    }

    #[tokio::test]
    async fn delete_only_given_certificates() {
        let mut deps = DependenciesBuilder::new(Configuration::new_sample());
        let connection = deps.get_sqlite_connection().await.unwrap();
        let repository = CertificateRepository::new(connection.clone());
        let records = vec![
            CertificateRecord::dummy_genesis("1", Beacon::new(String::new(), 1, 1)),
            CertificateRecord::dummy("2", "1", Beacon::new(String::new(), 1, 2)),
            CertificateRecord::dummy("3", "1", Beacon::new(String::new(), 1, 3)),
        ];
        insert_certificate_records(connection, records.clone()).await;
        let certificates: Vec<Certificate> = records.into_iter().map(|c| c.into()).collect();

        // Delete all records except the first
        repository
            .delete_certificates(
                &certificates
                    .iter()
                    .filter(|r| r.beacon.immutable_file_number > 1)
                    .collect::<Vec<_>>(),
            )
            .await
            .unwrap();

        let expected_remaining_certificate = certificates.first().unwrap().clone();
        let remaining_certificates = repository
            .get_latest_certificates(usize::MAX)
            .await
            .unwrap();

        assert_eq!(vec![expected_remaining_certificate], remaining_certificates)
    }
}

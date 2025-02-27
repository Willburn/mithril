---
sidebar_position: 3
---

# Run a Private Mithril network

:::info

In this guide, you will learn how to run a demonstration of a **Mithril Network** working on top of a private `devnet` **Cardano Network**. This network is completely autonomous and setup to produce Mithril snapshots every 1 minute.

:::

The network will be launched with the following topology:

* `2` **Cardano Nodes** configured as **Stake Pool Operators (SPO)** with a **Mithril Signer** on top
* `1` **Cardano Node** configured as **BFT node** with a **Mithril Aggregator** on top

![Devnet Topology](images/devnet-topology.png)

:::danger

This demonstration is working only on a Linux machine.

:::

:::tip

More information about this private Cardano/Mithril `devnet` is available [here](https://github.com/input-output-hk/mithril/blob/main/mithril-test-lab/mithril-devnet/README.md).

:::

## Pre-requisites

* Install a [correctly configured](https://www.rust-lang.org/learn/get-started) Rust toolchain (latest stable version).

* Install OpenSSL development libraries, for example on Ubuntu/Debian/Mint run `apt install libssl-dev`

* Ensure SQLite3 library is installed on your system and its version is at least `1.35` (released Apr. 2021) on Debian/Ubuntu: `apt install libsqlite3` and `sqlite3 --version`.

## Download source

Download from Github (HTTPS)

```bash
git clone https://github.com/input-output-hk/mithril.git
```

Or (SSH)

```bash
git clone git@github.com:input-output-hk/mithril.git
```

## Change directory

Go to the devnet folder

```bash
cd mithril-test-lab/mithril-devnet
```

## Run a Private Mithril/Cardano network locally (`devnet`)

### Step 1: Launch the devnet

Open a first terminal window.

Run a devnet with 1 BTF and 2 SPO Cardano nodes.

**Option 1**: Remote Docker images

The network will be ready faster with remote Docker images.

```bash
MITHRIL_IMAGE_ID=latest NUM_BFT_NODES=1 NUM_POOL_NODES=2 ./devnet-run.sh
```

Or

**Option 2**: Local Docker images

This takes more time to build local Docker images of the Mithril nodes

```bash
NUM_BFT_NODES=1 NUM_POOL_NODES=2 ./devnet-run.sh
```

:::info

You will see that the devnet is launched with the following steps:

* **Bootstraping the devnet**: generates the artifacts of the devnet depending on the configuration parameters (cryptographic keys, network topology, transactions to setup pool nodes, ...)
* **Start Cardano Network**: run the nodes of the **Cardano Network**, waits for it to be ready and activate the pool nodes
* **Start Mithril Network**: run the nodes of the **Mithril Network** that works on top of the **Cardano Network**

:::

You should see the following information displayed

```bash
=====================================================================
 Bootstrap Mithril/Cardano devnet
=====================================================================

>> Directory: artifacts
>> Cardano BFT nodes: 1
>> Cardano SPO nodes: 2
>> Info: Mithril Aggregator will be attached to the first Cardano BFT node
>> Info: Mithril Signers will be attached to each Cardano SPO node

=====================================================================
 Start Cardano nodes
=====================================================================

>> Start Cardano network
cardano-node: no process found
>> Starting Cardano node 'node-bft1'
>> Starting Cardano node 'node-pool1'
>> Starting Cardano node 'node-pool2'
>> Wait for Cardano network to be ready
>>>> Not ready yet
>>>> Not ready yet
>>>> Not ready yet
>>>> Not ready yet
>>>> Not ready yet
>>>> Not ready yet
>>>> Not ready yet
>>>> Not ready yet
>>>> Not ready yet
>>>> Not ready yet
>>>> Not ready yet
>>>> Not ready yet
>>>> Ready!
>> Activate Cardano pools
Estimated transaction fee: Lovelace 843
Transaction successfully submitted.
Estimated transaction fee: Lovelace 843
Transaction successfully submitted.
>> Wait for Cardano pools to be activated
>>>> Not activated yet
>>>> Not activated yet
>>>> Not activated yet
>>>> Not activated yet
>>>> Not activated yet
>>>> Not activated yet
>>>> Not activated yet
>>>> Not activated yet
>>>> Not activated yet
>>>> Not activated yet
>>>> Activated!
>>>> Found PoolId: pool1v55rfy864kslz86u45w4juahtuqr7cy282rffdnpc9exjlguvys
>>>> Found PoolId: pool1c56jqj5qsala8c24829sxqp0fcrtrrtcmezgrs6w60hl2nwsvav

=====================================================================
 Start Mithril nodes
=====================================================================

>> Start Mithril network
>> Build Mithril node Docker images
>>>> Building Mithril Aggregator node Docker image
>>>> Building Mithril Client node Docker image
>>>> Building Mithril Signer node Docker image
No stopped containers
Creating network "artifacts_cardano_network" with driver "bridge"
Creating network "artifacts_mithril_network" with driver "bridge"
Creating artifacts_mithril-aggregator_1        ... done
Creating artifacts_mithril-signer-node-pool1_1 ... done
Creating artifacts_mithril-signer-node-pool2_1 ... done
Creating artifacts_mithril-aggregator-genesis_run ... done
{"msg":"Started","v":0,"name":"slog-rs","level":20,"time":"2022-09-06T09:24:31.925641527Z","hostname":"e932dff845aa","pid":1,"config":"Configuration { cardano_cli_path: \"/app/bin/cardano-cli\", cardano_node_socket_path: \"/data/ipc/node.sock\", network_magic: Some(42), network: \"devnet\", protocol_parameters: ProtocolParameters { k: 5, m: 100, phi_f: 0.65 }, url_snapshot_manifest: \"\", snapshot_store_type: Local, snapshot_uploader_type: Local, server_url: \"http://0.0.0.0:8080/\", run_interval: 1000, db_directory: \"/data/db\", snapshot_directory: \"/data/mithril/aggregator\", data_stores_directory: \"/data/mithril/aggregator/stores\", genesis_verification_key: \"5b33322c3235332c3138362c3230312c3137372c31312c3131372c3133352c3138372c3136372c3138312c3138382c32322c35392c3230362c3130352c3233312c3135302c3231352c33302c37382c3231322c37362c31362c3235322c3138302c37322c3133342c3133372c3234372c3136312c36385d\" }","run_mode":"dev"}
{"msg":"New LocalSnapshotUploader created","v":0,"name":"slog-rs","level":20Genesis bootstrap for test only
,"time":"2022-09-06T09:24:31.925683285Z","hostname":"e932dff845aa","pid":1,"snapshot_server_url":"http://0.0.0.0:8080/"}
{"msg":"New MultiSignerImpl created","v":0,"name":"slog-rs","level":20,"time":"2022-09-06T09:24:31.925711468Z","hostname":"e932dff845aa","pid":1}
{"msg":"New MithrilCertificateVerifier created","v":0,"name":"slog-rs","level":20,"time":"2022-09-06T09:24:31.925736796Z","hostname":"e932dff845aa","pid":1}
{"msg":"Update current_beacon to Beacon { network: \"devnet\", epoch: Epoch(10), immutable_file_number: 47 }","v":0,"name":"slog-rs","level":20,"time":"2022-09-06T09:24:31.938337155Z","hostname":"e932dff845aa","pid":1}
{"msg":"Get next signers with stake","v":0,"name":"slog-rs","level":20,"time":"2022-09-06T09:24:31.938384324Z","hostname":"e932dff845aa","pid":1}
{"msg":"Get next stake distribution","v":0,"name":"slog-rs","level":20,"time":"2022-09-06T09:24:31.938422585Z","hostname":"e932dff845aa","pid":1}
{"msg":"Get stake distribution with epoch offset","v":0,"name":"slog-rs","level":20,"time":"2022-09-06T09:24:31.938459565Z","hostname":"e932dff845aa","pid":1,"epoch_offset":0}
{"msg":"Get next protocol parameters","v":0,"name":"slog-rs","level":20,"time":"2022-09-06T09:24:31.938500461Z","hostname":"e932dff845aa","pid":1}
{"msg":"Get protocol parameters with epoch offset","v":0,"name":"slog-rs","level":20,"time":"2022-09-06T09:24:31.938535367Z","hostname":"e932dff845aa","pid":1,"epoch_offset":0}
{"msg":"Create clerk","v":0,"name":"slog-rs","level":20,"time":"2022-09-06T09:24:31.93856896Z","hostname":"e932dff845aa","pid":1}
Verify genesis certificate #86a4c56d957636740a75c250fdd9d3b9a9f1539dc93449b1f80fcab49e279d6d @ epoch #10

=====================================================================
 Schedule Cardano Stake Delegation
=====================================================================

>> Begin scheduled delegation
>> 11:24:32: Wait 180s until next delegation round...
>> Run delegation round #1!
>>>> Current Epoch: 12
Estimated transaction fee: Lovelace 436
Transaction successfully submitted.
Estimated transaction fee: Lovelace 436
Transaction successfully submitted.
>> 11:27:32: Wait 180s until next delegation round...
>> Run delegation round #2!
>>>> Current Epoch: 14
Estimated transaction fee: Lovelace 436
Transaction successfully submitted.
Estimated transaction fee: Lovelace 436
Transaction successfully submitted.
>> 11:30:32: Wait 180s until next delegation round...
```

### Step 2: Query the devnet

Open a second terminal window.

Watch the state queried from the devnet

```bash
watch -n 1 ./devnet-query.sh
```

The networks will be queried every `1s` and will display:

* Certificate production informations gathered from the **Mithril Network**
* Utxo, Stake Pools, Stake Distribution from the **Cardano Network**

```bash
=====================================================================
 Query Mithril/Cardano devnet
=====================================================================

=====================================================================
=== Mithril Network
=====================================================================

>> Query pending certificate
{
  "beacon": {
    "network": "devnet",
    "epoch": 2,
    "immutable_file_number": 6
  },
  "protocol": {
    "k": 5,
    "m": 100,
    "phi_f": 0.65
  },
  "signers": []
}

>> Query snapshots
[
  {
    "digest": "224b77ad9cbe7fc81e6808940d391b299c27e77d9978641025f382e2e5ddd2ac",
    "certificate_hash": "5b29543c4af0f369d40e1da53451ebd8a39c4263df1585eb072f54511c1e3333",
    "size": 7986,
    "created_at": "2022-07-05T11:26:55.855498395Z",
    "locations": [
      "http://0.0.0.0:8080/aggregator/snapshot/224b77ad9cbe7fc81e6808940d391b299c27e77d9978641025f382e2e5ddd2ac/download"
    ]
  },
  {
    "digest": "1a39f57c906133421ab7b5c782762b6abff4771b5e9158a977e58db1edc26bd0",
    "certificate_hash": "be758b84a4b495e82af48747356946efb509ccbc4b44a9c985e3cb3099e35c94",
    "size": 6743,
    "created_at": "2022-07-05T11:26:35.658661878Z",
    "locations": [
      "http://0.0.0.0:8080/aggregator/snapshot/1a39f57c906133421ab7b5c782762b6abff4771b5e9158a977e58db1edc26bd0/download"
    ]
  },
  {
    "digest": "fd1a39d28998ba18c96547f62d308c57612ed348be058f615c14db5228a947c1",
    "certificate_hash": "4254a6176afbe17967ad1671e4619e9a3f3412115a63dd0eb0f5e8b64094128a",
    "size": 6199,
    "created_at": "2022-07-05T11:26:20.470029035Z",
    "locations": [
      "http://0.0.0.0:8080/aggregator/snapshot/fd1a39d28998ba18c96547f62d308c57612ed348be058f615c14db5228a947c1/download"
    ]
  }
]

=====================================================================
=== Cardano Network
=====================================================================

>> Query chain tip
{
    "era": "Alonzo",
    "syncProgress": "100.00",
    "hash": "075fc8366d353b45debedfc6faa92148c8fad584d81dbb4ea7b8b4d121489452",
    "epoch": 2,
    "slot": 219,
    "block": 9
}

>> Query whole utxo
                           TxHash                                 TxIx        Amount
--------------------------------------------------------------------------------------
4980fb7c90bc003f6af65778008732cd1b1a8c0873b7d622bfb7442f1312c9b5     0        447999157 lovelace + TxOutDatumNone
4980fb7c90bc003f6af65778008732cd1b1a8c0873b7d622bfb7442f1312c9b5     1        2000000 lovelace + TxOutDatumNone
c31e535531c9eb32bdd8f05e25204186333262674d24c6a770b6b120f020d9a9     0        448999157 lovelace + TxOutDatumNone
c31e535531c9eb32bdd8f05e25204186333262674d24c6a770b6b120f020d9a9     1        1000000 lovelace + TxOutDatumNone
ed265b672873192ea1e9c19092e8f41947c28977438bcff580312de37cfbd46c     0        1002000000 lovelace + TxOutDatumNone

>> Query stake pools
pool1v55rfy864kslz86u45w4juahtuqr7cy282rffdnpc9exjlguvys
pool1c56jqj5qsala8c24829sxqp0fcrtrrtcmezgrs6w60hl2nwsvav

>> Query stake distribution
                           PoolId                                 Stake frac
------------------------------------------------------------------------------
pool1v55rfy864kslz86u45w4juahtuqr7cy282rffdnpc9exjlguvys   1.052e-3
pool1c56jqj5qsala8c24829sxqp0fcrtrrtcmezgrs6w60hl2nwsvav   5.258e-4
```

### Step 3: Observe the devnet

Open a third terminal window.

Watch the logs of each node of the devnet

```bash
watch -n 1 LINES=5 ./devnet-log.sh
```

The nodes will be queried every `1s` and will display as below

```bash
=====================================================================
 Logs Mithril/Cardano devnet
=====================================================================

=====================================================================
--  docker-compose logs --tail=5
=====================================================================
Attaching to artifacts_mithril-signer-node-pool1_1, artifacts_mithril-signer-node-pool2_1, artifacts_mithril-aggregator_1
mithril-aggregator_1         | {"msg":"Epoch computation is not final and needs to be fixed: 4","v":0,"name":"slog-rs","level":40,"time":"2022-07-05T11:29:32.726760492Z","hostname":"ba17593540ac","pid":1}
mithril-aggregator_1         | {"msg":"Get signer pool1c56jqj5qsala8c24829sxqp0fcrtrrtcmezgrs6w60hl2nwsvav","v":0,"name":"slog-rs","level":20,"time":"2022-07-05T11:29:32.72678048Z","hostname":"ba17593540ac","pid":1}
mithril-aggregator_1         | {"msg":"Epoch computation is not final and needs to be fixed: 4","v":0,"name":"slog-rs","level":40,"time":"2022-07-05T11:29:32.72679661Z","hostname":"ba17593540ac","pid":1}
mithril-aggregator_1         | {"msg":"Get signer pool1v55rfy864kslz86u45w4juahtuqr7cy282rffdnpc9exjlguvys","v":0,"name":"slog-rs","level":20,"time":"2022-07-05T11:29:32.734529107Z","hostname":"ba17593540ac","pid":1}
mithril-aggregator_1         | {"msg":"Epoch computation is not final and needs to be fixed: 4","v":0,"name":"slog-rs","level":40,"time":"2022-07-05T11:29:32.734553714Z","hostname":"ba17593540ac","pid":1}
mithril-signer-node-pool1_1  | {"msg":"Signing digest","v":0,"name":"slog-rs","level":30,"time":"2022-07-05T11:29:32.744124074Z","hostname":"4fc53f5ce413","pid":1,"digester_result":"DigesterResult {\n    digest: \"e5ac1579a3fff12bf19ef88b0d9ec9d8a1c53e4d74c38c023b2e33638f454d67\",\n    last_immutable_file_number: 17,\n}"}
mithril-signer-node-pool1_1  | {"msg":"Register signatures","v":0,"name":"slog-rs","level":30,"time":"2022-07-05T11:29:32.744140625Z","hostname":"4fc53f5ce413","pid":1}
mithril-signer-node-pool1_1  | {"msg":"Epoch computation is not final and needs to be fixed: 4","v":0,"name":"slog-rs","level":40,"time":"2022-07-05T11:29:32.744155293Z","hostname":"4fc53f5ce413","pid":1}
mithril-signer-node-pool1_1  | {"msg":"SingleSignaturesComputeFailed(UnregisteredVerificationKey)","v":0,"name":"slog-rs","level":50,"time":"2022-07-05T11:29:32.744336041Z","hostname":"4fc53f5ce413","pid":1}
mithril-signer-node-pool1_1  | {"msg":"Sleeping for 1000","v":0,"name":"slog-rs","level":30,"time":"2022-07-05T11:29:32.744352051Z","hostname":"4fc53f5ce413","pid":1}
mithril-signer-node-pool2_1  | {"msg":"Signing digest","v":0,"name":"slog-rs","level":30,"time":"2022-07-05T11:29:32.73359119Z","hostname":"1c671096ee3f","pid":1,"digester_result":"DigesterResult {\n    digest: \"e5ac1579a3fff12bf19ef88b0d9ec9d8a1c53e4d74c38c023b2e33638f454d67\",\n    last_immutable_file_number: 17,\n}"}
mithril-signer-node-pool2_1  | {"msg":"Register signatures","v":0,"name":"slog-rs","level":30,"time":"2022-07-05T11:29:32.733607821Z","hostname":"1c671096ee3f","pid":1}
mithril-signer-node-pool2_1  | {"msg":"Epoch computation is not final and needs to be fixed: 4","v":0,"name":"slog-rs","level":40,"time":"2022-07-05T11:29:32.733623511Z","hostname":"1c671096ee3f","pid":1}
mithril-signer-node-pool2_1  | {"msg":"SingleSignaturesComputeFailed(UnregisteredVerificationKey)","v":0,"name":"slog-rs","level":50,"time":"2022-07-05T11:29:32.733786246Z","hostname":"1c671096ee3f","pid":1}
mithril-signer-node-pool2_1  | {"msg":"Sleeping for 1000","v":0,"name":"slog-rs","level":30,"time":"2022-07-05T11:29:32.733802416Z","hostname":"1c671096ee3f","pid":1}

=====================================================================
=====================================================================
tail -n 22 ./node-bft1/node.log
=====================================================================
[jp:cardano.node.ChainDB:Info:25] [2022-07-05 11:27:28.01 UTC] Took ledger snapshot DiskSnapshot {dsNumber = 219, dsSuffix = Nothing} at 075fc8366d353b45debedfc6faa92148c8fad584d81dbb4ea7b8b4d121489452 at slot 219
[jp:cardano.node.ChainDB:Notice:21] [2022-07-05 11:27:40.76 UTC] Chain extended, new tip: af93c6964de49d0696bf194c222f6e5a40e5123ef688a20613a33a705b6b736a at slot 253
[jp:cardano.node.ChainDB:Info:25] [2022-07-05 11:27:40.76 UTC] Took ledger snapshot DiskSnapshot {dsNumber = 228, dsSuffix = Nothing} at 720c625a259a23f21926fe7a30dad9b7a4b50958a508c8cfdc96a94625fbf00d at slot 228
[jp:cardano.node.ChainDB:Notice:21] [2022-07-05 11:27:43.75 UTC] Chain extended, new tip: 9f141fe78c0baa433c2554d3a09a9b43c47faa7b740be254893000310e5bad3b at slot 257
[jp:cardano.node.ChainDB:Notice:21] [2022-07-05 11:27:52.76 UTC] Chain extended, new tip: bfc0b2c1c4d06699efcdf6ad7b33c48cea722fb4bb5c5d6761a3768609cf77a4 at slot 269
[jp:cardano.node.ChainDB:Info:25] [2022-07-05 11:27:52.76 UTC] Took ledger snapshot DiskSnapshot {dsNumber = 253, dsSuffix = Nothing} at af93c6964de49d0696bf194c222f6e5a40e5123ef688a20613a33a705b6b736a at slot 253
[jp:cardano.node.ChainDB:Notice:21] [2022-07-05 11:28:03.26 UTC] Chain extended, new tip: c6238e98f186278eeef86d13f3482ebfb9b1d01d2a28da78282bfd241524eccd at slot 283
[jp:cardano.node.ChainDB:Info:25] [2022-07-05 11:28:03.26 UTC] Took ledger snapshot DiskSnapshot {dsNumber = 257, dsSuffix = Nothing} at 9f141fe78c0baa433c2554d3a09a9b43c47faa7b740be254893000310e5bad3b at slot 257
[jp:cardano.node.ChainDB:Notice:21] [2022-07-05 11:28:04.75 UTC] Chain extended, new tip: ac332aea5f043b3fd5ac68a04225932a21935ad7e5c5cfbb7e5b0b00df713bff at slot 285
[jp:cardano.node.ChainDB:Notice:21] [2022-07-05 11:28:16.00 UTC] Chain extended, new tip: e68d08e0c127a5346a74dd06713d8de0b4e37e338a0e03987da356bb70892b99 at slot 300
[jp:cardano.node.ChainDB:Info:25] [2022-07-05 11:28:16.00 UTC] Took ledger snapshot DiskSnapshot {dsNumber = 283, dsSuffix = Nothing} at c6238e98f186278eeef86d13f3482ebfb9b1d01d2a28da78282bfd241524eccd at slot 283
[jp:cardano.node.ChainDB:Notice:21] [2022-07-05 11:28:18.25 UTC] Chain extended, new tip: bc07b985d7f76bacc0a726b2dc5aa76a7254f1e4548a633cdfd62c31e022b3a5 at slot 303
[jp:cardano.node.ChainDB:Notice:21] [2022-07-05 11:28:37.00 UTC] Chain extended, new tip: 7d53c5eba9679c96ba32d79a02cfd953280b3477f1dd8eeb18447638c8a30e20 at slot 328
[jp:cardano.node.ChainDB:Info:25] [2022-07-05 11:28:37.00 UTC] Took ledger snapshot DiskSnapshot {dsNumber = 300, dsSuffix = Nothing} at e68d08e0c127a5346a74dd06713d8de0b4e37e338a0e03987da356bb70892b99 at slot 300
[jp:cardano.node.ChainDB:Notice:21] [2022-07-05 11:28:58.75 UTC] Chain extended, new tip: 3e9734018c585eea160a33accf82f758713f0e7aae1fab4dc40bccd859b8066f at slot 357
[jp:cardano.node.ChainDB:Info:25] [2022-07-05 11:28:58.75 UTC] Took ledger snapshot DiskSnapshot {dsNumber = 303, dsSuffix = Nothing} at bc07b985d7f76bacc0a726b2dc5aa76a7254f1e4548a633cdfd62c31e022b3a5 at slot 303
[jp:cardano.node.ChainDB:Notice:21] [2022-07-05 11:29:05.50 UTC] Chain extended, new tip: 8876850840ae52ca240d517def4b9c8a5db98e2e7db17f8abf87e4f12db13d15 at slot 366
[jp:cardano.node.ChainDB:Info:25] [2022-07-05 11:29:05.50 UTC] Took ledger snapshot DiskSnapshot {dsNumber = 328, dsSuffix = Nothing} at 7d53c5eba9679c96ba32d79a02cfd953280b3477f1dd8eeb18447638c8a30e20 at slot 328
[jp:cardano.node.ChainDB:Notice:21] [2022-07-05 11:29:19.75 UTC] Chain extended, new tip: 5733ec701db5c9dc253dd4b611421de0c2d223e6ee99c8d61010a9fea42d504b at slot 385
[jp:cardano.node.ChainDB:Info:25] [2022-07-05 11:29:19.75 UTC] Took ledger snapshot DiskSnapshot {dsNumber = 357, dsSuffix = Nothing} at 3e9734018c585eea160a33accf82f758713f0e7aae1fab4dc40bccd859b8066f at slot 357
[jp:cardano.node.ChainDB:Notice:21] [2022-07-05 11:29:31.00 UTC] Chain extended, new tip: 331c824ebee92dee7717f7bcc1457ac89b0de33d76073e6edd97a28770fa364b at slot 400
[jp:cardano.node.ChainDB:Info:25] [2022-07-05 11:29:31.00 UTC] Took ledger snapshot DiskSnapshot {dsNumber = 366, dsSuffix = Nothing} at 8876850840ae52ca240d517def4b9c8a5db98e2e7db17f8abf87e4f12db13d15 at slot 366
=====================================================================

=====================================================================
tail -n 22 ./node-pool1/node.log
=====================================================================
[jp:cardano.node.ChainDB:Info:25] [2022-07-05 11:27:28.00 UTC] Took ledger snapshot DiskSnapshot {dsNumber = 219, dsSuffix = Nothing} at 075fc8366d353b45debedfc6faa92148c8fad584d81dbb4ea7b8b4d121489452 at slot 219
[jp:cardano.node.ChainDB:Notice:21] [2022-07-05 11:27:40.76 UTC] Chain extended, new tip: af93c6964de49d0696bf194c222f6e5a40e5123ef688a20613a33a705b6b736a at slot 253
[jp:cardano.node.ChainDB:Info:25] [2022-07-05 11:27:40.76 UTC] Took ledger snapshot DiskSnapshot {dsNumber = 228, dsSuffix = Nothing} at 720c625a259a23f21926fe7a30dad9b7a4b50958a508c8cfdc96a94625fbf00d at slot 228
[jp:cardano.node.ChainDB:Notice:21] [2022-07-05 11:27:43.76 UTC] Chain extended, new tip: 9f141fe78c0baa433c2554d3a09a9b43c47faa7b740be254893000310e5bad3b at slot 257
[jp:cardano.node.ChainDB:Notice:21] [2022-07-05 11:27:52.75 UTC] Chain extended, new tip: bfc0b2c1c4d06699efcdf6ad7b33c48cea722fb4bb5c5d6761a3768609cf77a4 at slot 269
[jp:cardano.node.ChainDB:Info:25] [2022-07-05 11:27:52.75 UTC] Took ledger snapshot DiskSnapshot {dsNumber = 253, dsSuffix = Nothing} at af93c6964de49d0696bf194c222f6e5a40e5123ef688a20613a33a705b6b736a at slot 253
[jp:cardano.node.ChainDB:Notice:21] [2022-07-05 11:28:03.26 UTC] Chain extended, new tip: c6238e98f186278eeef86d13f3482ebfb9b1d01d2a28da78282bfd241524eccd at slot 283
[jp:cardano.node.ChainDB:Info:25] [2022-07-05 11:28:03.26 UTC] Took ledger snapshot DiskSnapshot {dsNumber = 257, dsSuffix = Nothing} at 9f141fe78c0baa433c2554d3a09a9b43c47faa7b740be254893000310e5bad3b at slot 257
[jp:cardano.node.ChainDB:Notice:21] [2022-07-05 11:28:04.75 UTC] Chain extended, new tip: ac332aea5f043b3fd5ac68a04225932a21935ad7e5c5cfbb7e5b0b00df713bff at slot 285
[jp:cardano.node.ChainDB:Notice:21] [2022-07-05 11:28:16.00 UTC] Chain extended, new tip: e68d08e0c127a5346a74dd06713d8de0b4e37e338a0e03987da356bb70892b99 at slot 300
[jp:cardano.node.ChainDB:Info:25] [2022-07-05 11:28:16.01 UTC] Took ledger snapshot DiskSnapshot {dsNumber = 283, dsSuffix = Nothing} at c6238e98f186278eeef86d13f3482ebfb9b1d01d2a28da78282bfd241524eccd at slot 283
[jp:cardano.node.ChainDB:Notice:21] [2022-07-05 11:28:18.25 UTC] Chain extended, new tip: bc07b985d7f76bacc0a726b2dc5aa76a7254f1e4548a633cdfd62c31e022b3a5 at slot 303
[jp:cardano.node.ChainDB:Notice:21] [2022-07-05 11:28:37.00 UTC] Chain extended, new tip: 7d53c5eba9679c96ba32d79a02cfd953280b3477f1dd8eeb18447638c8a30e20 at slot 328
[jp:cardano.node.ChainDB:Info:25] [2022-07-05 11:28:37.00 UTC] Took ledger snapshot DiskSnapshot {dsNumber = 300, dsSuffix = Nothing} at e68d08e0c127a5346a74dd06713d8de0b4e37e338a0e03987da356bb70892b99 at slot 300
[jp:cardano.node.ChainDB:Notice:21] [2022-07-05 11:28:58.76 UTC] Chain extended, new tip: 3e9734018c585eea160a33accf82f758713f0e7aae1fab4dc40bccd859b8066f at slot 357
[jp:cardano.node.ChainDB:Info:25] [2022-07-05 11:28:58.76 UTC] Took ledger snapshot DiskSnapshot {dsNumber = 303, dsSuffix = Nothing} at bc07b985d7f76bacc0a726b2dc5aa76a7254f1e4548a633cdfd62c31e022b3a5 at slot 303
[jp:cardano.node.ChainDB:Notice:21] [2022-07-05 11:29:05.50 UTC] Chain extended, new tip: 8876850840ae52ca240d517def4b9c8a5db98e2e7db17f8abf87e4f12db13d15 at slot 366
[jp:cardano.node.ChainDB:Info:25] [2022-07-05 11:29:05.51 UTC] Took ledger snapshot DiskSnapshot {dsNumber = 328, dsSuffix = Nothing} at 7d53c5eba9679c96ba32d79a02cfd953280b3477f1dd8eeb18447638c8a30e20 at slot 328
[jp:cardano.node.ChainDB:Notice:21] [2022-07-05 11:29:19.75 UTC] Chain extended, new tip: 5733ec701db5c9dc253dd4b611421de0c2d223e6ee99c8d61010a9fea42d504b at slot 385
[jp:cardano.node.ChainDB:Info:25] [2022-07-05 11:29:19.76 UTC] Took ledger snapshot DiskSnapshot {dsNumber = 357, dsSuffix = Nothing} at 3e9734018c585eea160a33accf82f758713f0e7aae1fab4dc40bccd859b8066f at slot 357
[jp:cardano.node.ChainDB:Notice:21] [2022-07-05 11:29:31.00 UTC] Chain extended, new tip: 331c824ebee92dee7717f7bcc1457ac89b0de33d76073e6edd97a28770fa364b at slot 400
[jp:cardano.node.ChainDB:Info:25] [2022-07-05 11:29:31.00 UTC] Took ledger snapshot DiskSnapshot {dsNumber = 366, dsSuffix = Nothing} at 8876850840ae52ca240d517def4b9c8a5db98e2e7db17f8abf87e4f12db13d15 at slot 366
=====================================================================

=====================================================================
tail -n 22 ./node-pool2/node.log
=====================================================================
[jp:cardano.node.ChainDB:Info:25] [2022-07-05 11:27:28.00 UTC] Took ledger snapshot DiskSnapshot {dsNumber = 219, dsSuffix = Nothing} at 075fc8366d353b45debedfc6faa92148c8fad584d81dbb4ea7b8b4d121489452 at slot 219
[jp:cardano.node.ChainDB:Notice:21] [2022-07-05 11:27:40.75 UTC] Chain extended, new tip: af93c6964de49d0696bf194c222f6e5a40e5123ef688a20613a33a705b6b736a at slot 253
[jp:cardano.node.ChainDB:Info:25] [2022-07-05 11:27:40.75 UTC] Took ledger snapshot DiskSnapshot {dsNumber = 228, dsSuffix = Nothing} at 720c625a259a23f21926fe7a30dad9b7a4b50958a508c8cfdc96a94625fbf00d at slot 228
[jp:cardano.node.ChainDB:Notice:21] [2022-07-05 11:27:43.76 UTC] Chain extended, new tip: 9f141fe78c0baa433c2554d3a09a9b43c47faa7b740be254893000310e5bad3b at slot 257
[jp:cardano.node.ChainDB:Notice:21] [2022-07-05 11:27:52.75 UTC] Chain extended, new tip: bfc0b2c1c4d06699efcdf6ad7b33c48cea722fb4bb5c5d6761a3768609cf77a4 at slot 269
[jp:cardano.node.ChainDB:Info:25] [2022-07-05 11:27:52.76 UTC] Took ledger snapshot DiskSnapshot {dsNumber = 253, dsSuffix = Nothing} at af93c6964de49d0696bf194c222f6e5a40e5123ef688a20613a33a705b6b736a at slot 253
[jp:cardano.node.ChainDB:Notice:21] [2022-07-05 11:28:03.25 UTC] Chain extended, new tip: c6238e98f186278eeef86d13f3482ebfb9b1d01d2a28da78282bfd241524eccd at slot 283
[jp:cardano.node.ChainDB:Info:25] [2022-07-05 11:28:03.25 UTC] Took ledger snapshot DiskSnapshot {dsNumber = 257, dsSuffix = Nothing} at 9f141fe78c0baa433c2554d3a09a9b43c47faa7b740be254893000310e5bad3b at slot 257
[jp:cardano.node.ChainDB:Notice:21] [2022-07-05 11:28:04.75 UTC] Chain extended, new tip: ac332aea5f043b3fd5ac68a04225932a21935ad7e5c5cfbb7e5b0b00df713bff at slot 285
[jp:cardano.node.ChainDB:Notice:21] [2022-07-05 11:28:16.00 UTC] Chain extended, new tip: e68d08e0c127a5346a74dd06713d8de0b4e37e338a0e03987da356bb70892b99 at slot 300
[jp:cardano.node.ChainDB:Info:25] [2022-07-05 11:28:16.01 UTC] Took ledger snapshot DiskSnapshot {dsNumber = 283, dsSuffix = Nothing} at c6238e98f186278eeef86d13f3482ebfb9b1d01d2a28da78282bfd241524eccd at slot 283
[jp:cardano.node.ChainDB:Notice:21] [2022-07-05 11:28:18.25 UTC] Chain extended, new tip: bc07b985d7f76bacc0a726b2dc5aa76a7254f1e4548a633cdfd62c31e022b3a5 at slot 303
[jp:cardano.node.ChainDB:Notice:21] [2022-07-05 11:28:37.00 UTC] Chain extended, new tip: 7d53c5eba9679c96ba32d79a02cfd953280b3477f1dd8eeb18447638c8a30e20 at slot 328
[jp:cardano.node.ChainDB:Info:25] [2022-07-05 11:28:37.00 UTC] Took ledger snapshot DiskSnapshot {dsNumber = 300, dsSuffix = Nothing} at e68d08e0c127a5346a74dd06713d8de0b4e37e338a0e03987da356bb70892b99 at slot 300
[jp:cardano.node.ChainDB:Notice:21] [2022-07-05 11:28:58.76 UTC] Chain extended, new tip: 3e9734018c585eea160a33accf82f758713f0e7aae1fab4dc40bccd859b8066f at slot 357
[jp:cardano.node.ChainDB:Info:25] [2022-07-05 11:28:58.76 UTC] Took ledger snapshot DiskSnapshot {dsNumber = 303, dsSuffix = Nothing} at bc07b985d7f76bacc0a726b2dc5aa76a7254f1e4548a633cdfd62c31e022b3a5 at slot 303
[jp:cardano.node.ChainDB:Notice:21] [2022-07-05 11:29:05.50 UTC] Chain extended, new tip: 8876850840ae52ca240d517def4b9c8a5db98e2e7db17f8abf87e4f12db13d15 at slot 366
[jp:cardano.node.ChainDB:Info:25] [2022-07-05 11:29:05.50 UTC] Took ledger snapshot DiskSnapshot {dsNumber = 328, dsSuffix = Nothing} at 7d53c5eba9679c96ba32d79a02cfd953280b3477f1dd8eeb18447638c8a30e20 at slot 328
[jp:cardano.node.ChainDB:Notice:21] [2022-07-05 11:29:19.75 UTC] Chain extended, new tip: 5733ec701db5c9dc253dd4b611421de0c2d223e6ee99c8d61010a9fea42d504b at slot 385
[jp:cardano.node.ChainDB:Info:25] [2022-07-05 11:29:19.76 UTC] Took ledger snapshot DiskSnapshot {dsNumber = 357, dsSuffix = Nothing} at 3e9734018c585eea160a33accf82f758713f0e7aae1fab4dc40bccd859b8066f at slot 357
[jp:cardano.node.ChainDB:Notice:21] [2022-07-05 11:29:31.00 UTC] Chain extended, new tip: 331c824ebee92dee7717f7bcc1457ac89b0de33d76073e6edd97a28770fa364b at slot 400
[jp:cardano.node.ChainDB:Info:25] [2022-07-05 11:29:31.00 UTC] Took ledger snapshot DiskSnapshot {dsNumber = 366, dsSuffix = Nothing} at 8876850840ae52ca240d517def4b9c8a5db98e2e7db17f8abf87e4f12db13d15 at slot 366
=====================================================================

```

## Interact with the Mithril Aggregator by using the Mithril Client

### Step 1: Prepare some useful variables

```bash
# Cardano network
NETWORK=devnet

# Aggregator API endpoint URL
AGGREGATOR_ENDPOINT=http://localhost:8080/aggregator

# Digest of the latest produced snapshot for convenience of the demo
# You can also modify this variable and set it to the value of the digest of a snapshot that you can retrieve at step 2
SNAPSHOT_DIGEST=$(curl -s $AGGREGATOR_ENDPOINT/snapshots | jq -r '.[0].digest')
```

### Step 2: Select A Snapshot

List the available snapshots with which you can bootstrap a Cardano node

```bash
NETWORK=$NETWORK AGGREGATOR_ENDPOINT=$AGGREGATOR_ENDPOINT ./mithril-client list
```

You will see a list of snapshots

```bash
+---------+------------------------------------------------------------------+-------+-----------+--------------------------------+
| Network | Digest                                                           | Size  | Locations | Created                        |
+---------+------------------------------------------------------------------+-------+-----------+--------------------------------+
| devnet  | 85f09b39b0b5a13cec9d8fe7ffb82b5e5f236f02ae896f4e47b77e5cd1f2a917 | 11808 |         1 | 2022-07-04T16:47:00.258482685Z |
+---------+------------------------------------------------------------------+-------+-----------+--------------------------------+
| devnet  | 60d9c6e014d22335b34f55f83da728667f04fc1c63152ccff0bce7d217d08447 | 10793 |         1 | 2022-07-04T16:46:45.069646321Z |
+---------+------------------------------------------------------------------+-------+-----------+--------------------------------+
| devnet  | a3c4bb5f413f1b9648f0a086b3752d25ec62b540b8390917a5a7e78809896d92 |  7991 |         1 | 2022-07-04T16:46:09.817821220Z |
+---------+------------------------------------------------------------------+-------+-----------+--------------------------------+
| devnet  | b952adaa04dbb42206c69589b9951660f40c7262b088b13434b7a446ec90bc36 |  6746 |         1 | 2022-07-04T16:45:49.616260734Z |
+---------+------------------------------------------------------------------+-------+-----------+--------------------------------+
| devnet  | 46425fdcfe89ad5ba41a7822a4395e21b539e80c20e2b10546017b14cdcd4e4b |  6196 |         1 | 2022-07-04T16:45:29.425195132Z |
+---------+------------------------------------------------------------------+-------+-----------+--------------------------------+
```

### Step 3: Show Snapshot Details

Get some more details from a specific snapshot (Optional)

```bash
NETWORK=$NETWORK AGGREGATOR_ENDPOINT=$AGGREGATOR_ENDPOINT ./mithril-client show $SNAPSHOT_DIGEST
```

You will see more information about a snapshot

```bash
+------------+-------------------------------------------------------------------------------------------------------------------+
| Info       | Value                                                                                                             |
+------------+-------------------------------------------------------------------------------------------------------------------+
| Network    | devnet                                                                                                            |
+------------+-------------------------------------------------------------------------------------------------------------------+
| Digest     | 85f09b39b0b5a13cec9d8fe7ffb82b5e5f236f02ae896f4e47b77e5cd1f2a917                                                  |
+------------+-------------------------------------------------------------------------------------------------------------------+
| Size       | 11808                                                                                                             |
+------------+-------------------------------------------------------------------------------------------------------------------+
| Location 1 | http://0.0.0.0:8080/aggregator/snapshot/85f09b39b0b5a13cec9d8fe7ffb82b5e5f236f02ae896f4e47b77e5cd1f2a917/download |
+------------+-------------------------------------------------------------------------------------------------------------------+
| Created    | 2022-07-04T16:47:00.258482685Z                                                                                    |
+------------+-------------------------------------------------------------------------------------------------------------------+
```

### Step 4: Download Selected Snapshot

Download the selected snapshot from the remote location to your remote location

```bash
NETWORK=$NETWORK AGGREGATOR_ENDPOINT=$AGGREGATOR_ENDPOINT ./mithril-client download $SNAPSHOT_DIGEST
```

You will see that the selected snapshot archive has been downloaded locally

```bash
Download success 85f09b39b0b5a13cec9d8fe7ffb82b5e5f236f02ae896f4e47b77e5cd1f2a917 #1
from http://0.0.0.0:8080/aggregator/snapshot/85f09b39b0b5a13cec9d8fe7ffb82b5e5f236f02ae896f4e47b77e5cd1f2a917/download
to /home/mithril/data/devnet /85f09b39b0b5a13cec9d8fe7ffb82b5e5f236f02ae896f4e47b77e5cd1f2a917/snapshot.archive.tar.gz
```

### Step 5: Restore Selected Snapshot

Verify the Certificate of the snapshot and unpack its content in order to feed the Cardano node database

```bash
NETWORK=$NETWORK AGGREGATOR_ENDPOINT=$AGGREGATOR_ENDPOINT ./mithril-client restore $SNAPSHOT_DIGEST
```

You will see that the snapshot archive is unpacked and that the associated certificate is valid

```bash
Unpacking snapshot...
Unpack success 85f09b39b0b5a13cec9d8fe7ffb82b5e5f236f02ae896f4e47b77e5cd1f2a917
to /home/mithril/data/devnet /85f09b39b0b5a13cec9d8fe7ffb82b5e5f236f02ae896f4e47b77e5cd1f2a917/db

Restore a Cardano Node with:

docker run -v cardano-node-ipc:/ipc -v cardano-node-data:/data \
  --mount type=bind,source="./data/devnet/85f09b39b0b5a13cec9d8fe7ffb82b5e5f236f02ae896f4e47b77e5cd1f2a917/db",target=/data/db/ \
  -e NETWORK=devnet \
  inputoutput/cardano-node
```

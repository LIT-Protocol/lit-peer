# lit-recovery
A tool for recovery parties to 

1) retrieve their recovery key shares from the nodes
2) run the restore process

## Configuring Contracts
For local testing modify `consts.rs` to add the `Lit Contract Resolver` address. When main net is deployed this address will be a constant in the code.

## Building

Building for debug
`cargo build`

Building for release
`cargo build --release`

## Testing with multiple parties on the same machine:

You can use the SHARE_DB env var override for the shares db. This makes it possible to test multiple members without docker. Maybe we should make this a proper commandline config option but the env var was a quick and dirty solution. You could run 3 members with the following commands:

```
mkdir -p /tmp/keyringdb
SHARE_DB="sdb1.db3" ./lit-recovery --password=a --file=/tmp/keyringdb/1
SHARE_DB="sdb2.db3" ./lit-recovery --password=a --file=/tmp/keyringdb/2
SHARE_DB="sdb3.db3" ./lit-recovery --password=a --file=/tmp/keyringdb/3
```

## Docker build
The docker image requires a mount volume `data`
`docker volume create data`

For usage in docker you can run `docker build -t lit_rec .`
then `docker run -v data:/tmp/ -e PASSWD='<your db password>' -it lit_rec`

where `-v` can be any directory, if a `db` file exists within the directory the cli tool will use that file or create a new one if not present.
**note** if the db file is present you must provide the same password it was first created with.

## Usage
When running for the first time the tool will generate a new `asymetric key of curve type Secp256k1` and save it to your keyring.
After the first run of the tool it will grab the same key from the keyring. you will be prompted for your OS password to access and persist the key

For downloading shares, you can run the following command:
`cargo run download`

For uploading key shares:
`cargo run upload-pub-keys`

For uploading Verifiable Decryption shares to the mapped node where `Backup-H-2-xyz.cbor` is a Backed up ECDSA root key, `b59...` is the encryption_key for the Recovery DKG, `ecdsa` means for the ECDSA root key:
`cargo run upload-decryption-share -k Secp256k1 -c Backup-H-2-xyz.cbor -e b59..`

For setting the Contract Resolver address:
`cargo run contract-resolver --address=0xd19C16483F4905612aAa717C361d65f2f7fdDf30`

The Contract Resolver address is published in this repo: https://github.com/LIT-Protocol/networks/tree/main

For recovering the nodes, place all `*lit_backup_encrypted_keys*.tar.gz` files in one directory, and run the `recover` commnad:
`cargo run recover --directory=<directory-with-tars> --bls12381g1_encryption_key=<0x0123456789ABCDEF> --secp256k1_encryption_key=<0x0123456789ABCDEF>`

## Configuration
The following is an example of a `config.json` which will be auto generated in `~/.lit-recovery/config.json`.

```json
{
    "resolver_address": "0xf2f8885336aE22f6BbcE9F5Ed6A797728c9Bad60",
    "rpc_url": "https://lit-protocol.calderachain.xyz/http",
    "chain_id": 175177
    "enviorment": 2
}
```

The above configuration data will need setting to the correct context once you are ready to perform recovery. The above `resolver_address` will need to be set to the correct address for the network.
this can be set with 
```sh
lit-recovery contract-resolver --address="<contract resolver address>"
```

## Backing up the secrets and moving them to another computer:

This program keeps two secrets, one will need to be backed up: a mnemonic phrase and the shares database. The mnemonic phrase is output by the tool the first time it is run and it should be written down. When the user switches to a new computer, the tool must be initialized with the following command:

`cargo run mnemonic --phrase="<the-mnemonic-phrase>"`

The shares database can be exported and imported back with the following commands:

`cargo run export --file=<new-shares-db-file-name> [--password=<new-database-password>]`
`cargo run import --file=<old-shares-db-file-name> [--password=<exported-database-password>]`

When a password is not specified during the export process, the exported database is secured using the signing key. Therefore, the mnemonic phrase should be imported before importing the exported shares database.

The mnemonic phrase, the exported shares-database and its password (if specified) should be secured, in order to back up all secrets kept by the recovery tool. 

 

## Interactive Mode notes
### MAC specific

Executing the binary directly will cause a pop up window requesting your login. This is your machine login value and is required to access your key chain. You may either enter this value multiple times in the process, or click "always allow" to enter it only once.

Using the `download` function requires writing to disk - depending on how your file system is secured, it may be required to execute the binary using `sudo`. Doing so will still result in pop-up windows requesting your password.

### General
When changing a wallet using the mnemonic command, the 12 word phrase must be placed in single quotes like this:

```
mnemonic phrase='the quick brown fox jumped over the quick brown fox jumped over'
```

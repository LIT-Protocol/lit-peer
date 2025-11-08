You can run the lit-recovery tool using the commandline.  There are two binaries you can choose from: mac-apple-silicon is built for Mac systems using Apple Silicon, which includes the M1, M2, and M3 chips.  mac-intel-silicon is built for older Mac systems that use the Intel chip, before the M1 was released.

To run the tool, you can double click on the respective "lit-recovery" binary file for your system and it should open a commandline and run it. 

Stage 1:

1. If you have done a backup on this computer previously, then there should be no need to generate a new wallet.  You may be prompted for your user account password when you start the recovery tool, so that it can decrypt this item from your KeyChain.  Skip to step 3.

2. If you've never done a backup on this machine, then the first thing the tool will do is generate a wallet for you.  The tool will print a 12 word mnemonic phrase which is your private key.  Please write this down and save it somewhere safe.  The private key will also be stored in your computer's KeyChain, so that when you run the recovery tool again, it will be available.  You may be prompted for your user account password when you start the recovery tool a second time, so that it can decrypt this item from your KeyChain.  If you get a new computer, you can use the 12 word mnemonic phrase to get the wallet back, so that's why it's important to write it down and save it somewhere.

3. You can type "register" as a command into the tool and it will provide the instructions for what to do with this wallet address (which is, send it to chris@litprotocol.com).  

Once all recovery party members have sent their wallet address to chris@litprotocol.com, then we will set those as the recovery addresses, and run the DKG for the network to create it's root keys.  

We will reach out to you after this process happens, as there are additional steps in Stage 2, below, for you to do.  Do not perform these steps until told to do so.

Stage 2:
(These steps are only to be done after the network DKG gets run, and we will tell you when this happens, and when you should run these next steps)

1. Paste this in and hit enter: "config address=0x5326a59fF2c41bCdA7E64F9afB9C313d0342117B rpc_url=https://yellowstone-rpc.litprotocol.com chain_id=175188 env=2"

2. Type "download" to run the download command.  This will save a recovery share to your machine, and erase it from the node to which you are assigned.  You are now the only holder of this recovery share. 

3. Type "upload-pub-keys" to upload the information about the keys to the smart contracts.
    Look for the words "Upload pub keys txn hash: 0xsomething".  This generally means it worked.  
    If you also see "Error submitting public key info to chain: Contract(Contract call reverted with data: 0xsomething)" that is expected it probably still worked.

4. Type "export file=habanero-and-datil.lit" to export the backup to a file called "habanero-and-datil.lit" in the same folder.  It should say "Exporting shares to file habanero-and-datil.lit".  You should see two lines that say "db_key: ahexstring" which correspond to the old Habanero and new Datil network backups.  Save this file somewhere safe.  This is the actual keyshare that you will need to recover the network.


Thanks for helping keep Lit Protocol users funds safe!
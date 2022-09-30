import { Component, OnInit } from '@angular/core';
import { Account, connect, ConnectConfig, Contract, keyStores, Near, WalletConnection } from 'near-api-js';

import { ContractService } from './services/contract.service';

export let nearConnection: Near = undefined;
export let walletConnection: WalletConnection;
export let contract: Contract;
@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.scss']
})
export class AppComponent implements OnInit {
  title = 'contract-front-end-demo';
  isLoggedIn = false;

  constructor() {}
  async ngOnInit(): Promise<void> {
    const myKeyStore = new keyStores.BrowserLocalStorageKeyStore();
    const connectionConfig: ConnectConfig = {
      networkId: "testnet",
      keyStore: myKeyStore,
      nodeUrl: "https://rpc.testnet.near.org",
      walletUrl: "https://wallet.testnet.near.org",
      helperUrl: "https://helper.testnet.near.org",
      headers: {}
    };
    nearConnection = await connect(connectionConfig);
    // create wallet connection
    walletConnection = new WalletConnection(nearConnection, "nft");
    console.log(`getAccountId: ${walletConnection.getAccountId()}`);
    const account: Account = new Account(nearConnection.connection, walletConnection.getAccountId())
    console.log("Creating contract object");
    console.log(`account(): ${walletConnection.account()}`)
    contract = await new Contract(
      account, // the account object that is connecting
      "faults.testnet",
      {
        // name of contract you're connecting to
        viewMethods: ["nft_tokens_for_owner", "nft_tokens"], // view methods do not change state but usually return a value
        changeMethods: ["nft_mint"], // change methods modify state
        sender: walletConnection.account()
      } as any
    );
    console.log("contract created")

    this.isLoggedIn = walletConnection.isSignedIn();

  }

  async loginLogoutButton() {
    if (this.isLoggedIn) {
      walletConnection.signOut(); 
      location.reload();
    } else {
      await walletConnection.requestSignIn({
        "contractId": "faults.testnet", // contract requesting access
      });  
    }
  }
}

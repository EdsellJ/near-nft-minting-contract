import { Injectable } from '@angular/core';
// near api js
import { Account, connect, ConnectConfig, Contract, keyStores, Near, providers, utils, WalletConnection } from 'near-api-js';
// wallet selector UI
import { setupModal } from '@near-wallet-selector/modal-ui';
// wallet selector options
import { setupWalletSelector, Wallet, WalletSelector } from '@near-wallet-selector/core';
import { setupLedger } from '@near-wallet-selector/ledger';
import { setupMyNearWallet } from '@near-wallet-selector/my-near-wallet';

const THIRTY_TGAS = '30000000000000';
const NO_DEPOSIT = '0';
@Injectable({
  providedIn: 'root'
})
export class ContractService {
  keyStore: keyStores.BrowserLocalStorageKeyStore;
  connection: Near;
  contract: Contract;
  wallet: WalletConnection;

  constructor() {
    this.keyStore = new keyStores.BrowserLocalStorageKeyStore();
  }

  async init(): Promise<void> {
    const connectionConfig: ConnectConfig = {
      networkId: "testnet",
      keyStore: this.keyStore,
      nodeUrl: "https://rpc.testnet.near.org",
      walletUrl: "https://wallet.testnet.near.org",
      helperUrl: "https://helper.testnet.near.org",
      headers: {}
    };
    this.connection = await connect(connectionConfig);
    this.wallet = new WalletConnection(this.connection, "nft-educoin");
    
    const account = new Account(this.connection.connection, this.wallet.getAccountId());
    
    this.contract = new Contract(
      account,
      "faults.testnet",
      {
        // name of contract you're connecting to
        viewMethods: ["nft_tokens_for_owner", "nft_tokens"], // view methods do not change state but usually return a value
        changeMethods: ["nft_mint"], // change methods modify state
        sender: this.wallet.account()
      } as any
    )
  }

  isLoggedIn(): boolean {
    return this.wallet.isSignedIn();
  }

  logout(): void {
    this.wallet.signOut();
  }

  async signIn(): Promise<void> {
    await this.wallet.requestSignIn({
      "contractId": "faults.testnet",
    })
  }

  async mint_nft(token_id: string, receiver_id: string, title: string, description: string, media_link: string): Promise<providers.FinalExecutionOutcome> {
    let res = await this.wallet.account().functionCall({
      "args": {
        token_id: token_id,
        receiver_id: receiver_id,
        metadata: {
          title: title,
          description: description,
          media: media_link
        }
      },
      "contractId": "faults.testnet",
      "methodName": "nft_mint",
      "walletCallbackUrl": "http://localhost:4200/",
      "attachedDeposit": utils.format.parseNearAmount("0.1")
    });
    return res;
  }

  async nft_tokens_for_owner(): Promise<any[]> {
    return await (this.contract as any).nft_tokens_for_owner({account_id: this.wallet.getAccountId()})
  }

  async nft_tokens(): Promise<any[]> {
    return await (this.contract as any).nft_tokens();
  }

  getAccountId(): string {
    return this.wallet.getAccountId()
  }
}

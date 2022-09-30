import { Component, OnInit } from '@angular/core';
import { contract, nearConnection, walletConnection } from '../../../app/app.component';
import { Account, Contract, providers } from 'near-api-js';
import { Router } from '@angular/router';

@Component({
  selector: 'app-home-page',
  templateUrl: './home-page.component.html',
  styleUrls: ['./home-page.component.scss']
})
export class HomePageComponent implements OnInit {
  isLoggedIn: boolean = false;
  imgUrl = "";
  nfts = []
  allNfts = [];
  accountId = "";

  constructor(private router: Router) { }

  ngOnInit(): void {
    this.isLoggedIn = walletConnection.isSignedIn()  
    if (this.isLoggedIn) {
      this.accountId = walletConnection.getAccountId();
    }
  }

  goToMintPage() {
    this.router.navigate(["/mint"])
  }

  async login() {
    console.log("In login");
    await walletConnection.requestSignIn({
      "contractId": "faults.testnet", // contract requesting access
    });  
  }

  logout() {
    console.log("In logout");
    walletConnection.signOut();
    location.reload();
  }

  async getNfts() {
    if (!walletConnection.isSignedIn()) {
      window.alert("Not logged in!")
      return;
    }
    const res = await (contract as any).nft_tokens_for_owner({account_id: walletConnection.getAccountId()})
    if (res.length == 0) {
      window.alert("No NFTS found for this account!")
      return;
    }
    console.log(res);
    this.nfts = res;
  }

  async getAllNfts() {
    const res = await (contract as any).nft_tokens();
    if (res.length == 0) {
      window.alert("No NFTS!");
      return;
    }
    console.log(res);
    this.allNfts = res;
  }
}

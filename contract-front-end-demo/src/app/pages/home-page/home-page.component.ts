import { Component, OnInit } from '@angular/core';
import { Account, Contract, providers } from 'near-api-js';
import { Router } from '@angular/router';
import { ContractService } from '../../../app/services/contract.service';

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

  constructor(private router: Router, private contract: ContractService) { }

  ngOnInit(): void {
    this.isLoggedIn = this.contract.isLoggedIn();  
    if (this.isLoggedIn) {
      this.accountId = this.contract.getAccountId();
    }
  }

  goToMintPage() {
    if (!this.isLoggedIn) {
      window.alert("Not logged in!");
      return;
    }
    this.router.navigate(["/mint"])
  }

  async getNfts() {
    if (!this.contract.isLoggedIn()) {
      window.alert("Not logged in!")
      return;
    }
    const res = await this.contract.nft_tokens_for_owner();
    if (res.length == 0) {
      window.alert("No NFTS found for this account!")
      return;
    }
    console.log(res);
    this.nfts = res;
  }

  async getAllNfts() {
    const res = await this.contract.nft_tokens();
    if (res.length == 0) {
      window.alert("No NFTS!");
      return;
    }
    console.log(res);
    this.allNfts = res;
  }
}

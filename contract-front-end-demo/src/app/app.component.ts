import { Component, OnInit } from '@angular/core';
import { Router } from '@angular/router';
import { Account, connect, ConnectConfig, Contract, keyStores, Near, WalletConnection } from 'near-api-js';

import { ContractService } from './services/contract.service';

// export let nearConnection: Near = undefined;
// export let walletConnection: WalletConnection;
// export let contract: Contract;
@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.scss']
})
export class AppComponent implements OnInit {
  title = 'contract-front-end-demo';
  isLoggedIn = false;

  constructor(private router: Router, private contract: ContractService) {}
  async ngOnInit(): Promise<void> {
    // init contract on app load
    await this.contract.init();
    this.isLoggedIn = this.contract.isLoggedIn();
  }

  async loginLogoutButton() {
    if (this.isLoggedIn) {
      this.contract.logout()
      location.reload();
    } else {
      await this.contract.signIn();
    }
  }

  async goHome() {
    await this.router.navigateByUrl("/")
  }
}

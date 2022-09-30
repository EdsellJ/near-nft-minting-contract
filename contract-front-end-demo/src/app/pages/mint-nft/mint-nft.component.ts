import { Component, OnInit } from '@angular/core';
import { Router } from '@angular/router';
import { utils } from 'near-api-js';
import { contract, walletConnection } from '../../app.component';

@Component({
  selector: 'app-mint-nft',
  templateUrl: './mint-nft.component.html',
  styleUrls: ['./mint-nft.component.scss']
})
export class MintNftComponent implements OnInit {

  constructor(private router: Router) { }

  async ngOnInit(): Promise<void> {
    if (!walletConnection.isSignedIn()) {
      window.alert("Not signed in!");
      await this.router.navigate(["/"]);
      return;
    }
  }

  async onSubmit(event?) {
    console.log("In submit")
    let token_id = (document.getElementById("token_id") as any).value;
    let reciever_id = (document.getElementById("reciever_id") as any).value;
    let title = (document.getElementById("title") as any).value;
    let description = (document.getElementById("description") as any).value;
    let media_link = (document.getElementById("media") as any).value;
    let res = await walletConnection.account().functionCall({
      "args": {
        token_id: token_id,
        receiver_id: reciever_id,
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
    // let res = await (contract as any).nft_mint({
    //   token_id: token_id,
    //   receiver_id: reciever_id,
    //   metadata: {
    //     title: title,
    //     description: description,
    //     media: media_link
    //   }
    // }, undefined, utils.format.parseNearAmount("0.1"));
    console.log(res);

  }

  goBack() {
    this.router.navigate(["/"])
  }

}

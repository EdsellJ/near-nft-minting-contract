import { Component, OnInit } from '@angular/core';
import { Router } from '@angular/router';
import { ContractService } from '../../../app/services/contract.service';
// import { contract, walletConnection } from '../../app.component';

@Component({
  selector: 'app-mint-nft',
  templateUrl: './mint-nft.component.html',
  styleUrls: ['./mint-nft.component.scss']
})
export class MintNftComponent implements OnInit {

  constructor(private router: Router, private contract: ContractService) { }

  async ngOnInit(): Promise<void> {
    if (!this.contract.isLoggedIn()) {
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
    
    const res = await this.contract.mint_nft(token_id, reciever_id, title, description, media_link);
    console.log(res);

  }

  goBack() {
    this.router.navigate(["/"])
  }

}

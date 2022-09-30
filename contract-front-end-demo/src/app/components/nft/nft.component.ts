import { Component, Input, OnInit } from '@angular/core';

@Component({
  selector: 'app-nft',
  templateUrl: './nft.component.html',
  styleUrls: ['./nft.component.scss']
})
export class NftComponent implements OnInit {
  @Input() nft = undefined;

  constructor() { }

  ngOnInit(): void {
  }

}

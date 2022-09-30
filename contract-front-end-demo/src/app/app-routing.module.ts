import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';
import { HomePageComponent } from './pages/home-page/home-page.component';
import { MintNftComponent } from './pages/mint-nft/mint-nft.component';

const routes: Routes = [
  { path: "", component: HomePageComponent },
  { path: "mint", component: MintNftComponent }
];

@NgModule({
  imports: [RouterModule.forRoot(routes)],
  exports: [RouterModule]
})
export class AppRoutingModule { }

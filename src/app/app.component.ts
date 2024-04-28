import { Component, Input } from '@angular/core';
import { RouterOutlet } from '@angular/router';
import { MatIconModule } from '@angular/material/icon';
import { MatToolbarModule } from '@angular/material/toolbar';
import { MatSidenavModule } from '@angular/material/sidenav';
import { MatButtonModule } from '@angular/material/button';

import { MenuComponent } from './components/core/menu/menu.component';

@Component({
  selector: 'app-root',
  standalone: true,
             imports: [
               RouterOutlet,
               MatIconModule,
               MatToolbarModule,
               MatSidenavModule,
               MatButtonModule,
               MenuComponent,
             ],
  templateUrl: './app.component.html',
  styleUrl: './app.component.sass',
})
export class AppComponent {
  private _isMenuOpen: boolean;


  constructor() {
    this._isMenuOpen = false;
  }

  get isMenuOpen(): boolean {
    return this._isMenuOpen;
  }

  @Input('toggle')
  set isMenuOpen(value: boolean) {
    this._isMenuOpen = value;
  }

  chargeStatusMenu(): void {
    this._isMenuOpen = !this._isMenuOpen;
  }
}

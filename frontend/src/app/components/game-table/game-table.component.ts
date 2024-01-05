import { AfterViewInit, ChangeDetectionStrategy, Component, ElementRef, OnInit, ViewChild } from '@angular/core';
import { CommonModule } from '@angular/common';
import { GameService } from 'src/domain/game.service';
import { Card, CardFamily, GameState } from 'src/domain/model/game.model';
import { GameTableService } from './game-table.service';
import { Observable } from 'rxjs';

@Component({
  selector: 'app-game-table',
  standalone: true,
  imports: [CommonModule],
  templateUrl: './game-table.component.html',
  styleUrls: ['./game-table.component.scss'],
  providers:[GameService, GameTableService],
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class GameTableComponent implements OnInit, AfterViewInit {
  
  private _baseUrl: string = "https://github.com/crobertsbmw/deckofcards/blob/master/static/img";

  @ViewChild('render') canvas!: ElementRef<HTMLCanvasElement>;

  constructor(private gameService: GameService, private renderService: GameTableService){

  }
  async ngAfterViewInit(): Promise<void> {
    
    await this.gameService.initGame("hello"); 
    const table = await this.gameService.getGameTable("hello"); 
    this.renderService.initialize(this.canvas.nativeElement, table);
    this.gameService.gameState$.subscribe((state) => {
      console.log(state);
      this.renderService.refresh();
      for(let stack of state.stacks)
        this.renderService.renderStack(stack, true);
      
    })
  }
  async ngOnInit(): Promise<void> {
    
  }

  getFamilyString(card: Card): string {
    switch(card.family){
      case CardFamily.CLUB:
        return 'C';
      case CardFamily.HEART:
        return 'H'
      case CardFamily.DIAMOND:
        return 'D'
      case CardFamily.SPADE:
        return 'S'
    }
  }
  getCardString(card: Card): string {

    const family = this.getFamilyString(card);
    let value = "";
    if(card.value < 10){
      value = card.value.toString(10);
      return `${value}${family}`;
    }
    if(card.value === 10) value = "J";
    if(card.value === 11) value = "Q";
    if(card.value === 12) value = "K";
    if(card.value === 13) value = "A";
    return `${value}${family}`;


  }
  getCardImageUrl(card: Card): string{
    if(!card.visible){
      return `${this._baseUrl}/back.png`;
    }
    else return `${this._baseUrl}/${this.getCardString(card)}.png`;
  }

  async onClick(){
    await this.gameService.playMove();
  }


}

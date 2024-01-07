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
  

  @ViewChild('render') canvas!: ElementRef<HTMLCanvasElement>;

  constructor(private gameService: GameService, private renderService: GameTableService){

  }
  async ngAfterViewInit(): Promise<void> {
    
    await this.gameService.initGame("hello"); 
    const table = await this.gameService.getGameTable("hello"); 
    await this.renderService.initialize(this.canvas.nativeElement, table);

    this.gameService.gameState$.subscribe((state) => {
      this.renderService.refresh();
      for(let stack of state.stacks){
        this.renderService.renderStack(stack, true);
      }
    })
  }
  async ngOnInit(): Promise<void> {
    
  }

  onClick(){
    this.gameService.playMove();
  }


}

import { Injectable } from "@angular/core";
import { Card, GameTable, Stack } from "src/domain/model/game.model";

@Injectable()
export class GameTableService {

    private context: CanvasRenderingContext2D | null = null;
    private gameTable: GameTable | null = null;

    initialize(canvas: HTMLCanvasElement, gameTable: GameTable) {
        this.context = canvas.getContext('2d');
        this.gameTable = gameTable;
    }

    getCardImage(card: Card): HTMLImageElement {

        const image = new Image();
        image.src = './assets/8C.png';
        image.width = 314;
        image.height = 226;
        return image;
    }
    refresh(){
        this.context?.clearRect(0,0,1356,942);
    }
    renderStack(stack: Stack, spread: boolean){
        if(!this.gameTable) throw new Error("Game table not defined");
        if(!this.context) throw new Error("Game table not defined");
        if(stack.cards.length === 0) return;

        const w = this.gameTable.cardWidth;
        const h = this.gameTable.cardHeight;

        let x = (stack.columnPosition) * this.gameTable.cardWidth;
        let y = (stack.rowPosition) * this.gameTable.cardHeight;
        
        if(!spread){
            let image = this.getCardImage(stack.cards[0]);
            this.context.drawImage(image, x, y,w, h);
            return;
        }
        else{
            for(let card of stack.cards){
                let image = this.getCardImage(card);
                this.context?.drawImage(image, x, y,w, h);
                y+=10;
            }
          
        }
    }
}
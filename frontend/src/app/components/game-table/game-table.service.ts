import { Inject, Injectable } from "@angular/core";
import { CARD_PROVIDER, ICardProvider } from "src/domain/card.provider.interface";
import { Card, GameTable, Stack } from "src/domain/model/game.model";

@Injectable()
export class GameTableService {

    private context: CanvasRenderingContext2D | null = null;
    private gameTable: GameTable | null = null;
    private width = 0;
    private height = 0;

    constructor(@Inject(CARD_PROVIDER) private cardProvider: ICardProvider){
    }
 
    async initialize(canvas: HTMLCanvasElement, gameTable: GameTable): Promise<void> {

        this.context = canvas.getContext('2d');
        this.gameTable = gameTable;
        console.log(canvas.width);
        this.width = Math.ceil(canvas.width / gameTable.columnNumber)
        this.height =  Math.ceil(canvas.height / gameTable.rowNumber)
        await this.cardProvider.loadCardImages();
    }   

    refresh(){
        this.context?.clearRect(0,0,1356,942);
    }

    renderStack(stack: Stack, spread: boolean){
        if(!this.gameTable) throw new Error("Game table not defined");
        if(!this.context) throw new Error("Game table not defined");
        if(stack.cards.length === 0) return;

        const w = this.width;
        const h = this.height;

        let x = (stack.columnPosition) * w;
        let y = (stack.rowPosition) * h;

        for(const card of stack.cards){
            const image = this.cardProvider.getCardImage(card);
            this.context?.drawImage(image, x, y,w, h);
            if(!stack.spread) break;
            y+=h*0.16;
        }
    }
}
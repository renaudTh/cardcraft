import { IGameProvider } from "src/domain/game.provider.interface";
import { GameTable, GameState, Stack, Card } from "src/domain/model/game.model";



const GAME_TABLE: GameTable = {
    cardHeight: 314,
    cardWidth: 226,
    columnNumber: 6,
    rowNumber: 3,
}

export class GameProviderService implements IGameProvider {

    private _build: Stack[];
    private _deck: Stack;
    private _state: GameState;

    constructor(){
        this._deck = {
            cards: [],
            rowPosition: 0,
            columnPosition: 0,
        }
        this._build = []
        for(let f = 0; f < 4; f++){ 
            this._build.push({
                cards: [],
                columnPosition: f+1,
                rowPosition: 1
            });
            for(let v = 7; v <=14; v++){
                this._deck.cards.push({
                    family: f,
                    value: v,
                    visible: false
                })
            }
        }
        this._state = {
            isEnded: false,
            isWon: false,
            stacks: [...this._build, this._deck]
        }
    }
    getGameTable(id: string): Promise<GameTable> {
        return Promise.resolve(GAME_TABLE);
    }
    getGameState(id: string): Promise<GameState> {
        return Promise.resolve(this._state);
    }
    playMove(id: string): Promise<GameState> {
        const card = this._deck.cards.pop();
        if(card){
            this._build[card.family].cards.push({
                ...card,
                visible: true,
            })
        }
        else {
            this._state.isEnded = true;
            this._state.isWon = false;
        }
        return Promise.resolve(this._state);
    }
        
}

import { GameFactory } from "cardcraft";
import { IGameProvider } from "src/domain/game.provider.interface";
import { GameTable, GameState, Stack, Card } from "src/domain/model/game.model";



const GAME_TABLE: GameTable = {
    columnNumber: 6,
    rowNumber: 3,
}
function parseCard(c: number): Card{
    return {
        family: c & 0x3,
        value: (c & 0x7C) >> 2,
        visible: (c & 0x80) >> 7 == 1
    }
}
function parseGameState(raw: Uint8Array) : GameState {
    const rows = raw[0];
    const columns = raw[1];
    const nbStacks = raw[2];
    const isWon = raw[3] == 1;
    const isEnded = raw[4] == 1;
    let read = 5;
    const stacks:Stack[] = [];
    for(let s = 0; s < nbStacks; s++){
        const columnPosition = raw[read++];
        const rowPosition = raw[read++];
        const spread = raw[read++] == 1;
        const size = raw[read++];
        const cards:Card[] = []
        for(let c = 0; c <size; c++){
            cards.push(parseCard(raw[read+c]))
        }
        stacks.push({
            cards,
            columnPosition,
            rowPosition,
            spread
        })
        read+=size;
    }
    return {
        isEnded,
        isWon,
        stacks
    }
}
export class GameProviderService implements IGameProvider {

    private _factory: GameFactory;
    constructor(){
        this._factory = GameFactory.new();
    }
    startGame(id: string): Promise<GameState> {
        const state = this._factory.init();
        return Promise.resolve(parseGameState(state));
    }
    getGameTable(id: string): Promise<GameTable> {
        return Promise.resolve(GAME_TABLE);
    }
    getGameState(id: string): Promise<GameState> {
        throw new Error("Method not implemented.");
    }
    playMove(id: string): Promise<GameState> {
        const state = this._factory.play_move();
        return Promise.resolve(parseGameState(state));
    }
}
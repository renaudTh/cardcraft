import { InjectionToken } from "@angular/core";
import { GameState, GameTable } from "./model/game.model";


export const GAME_PROVIDER = new InjectionToken<IGameProvider>('game.provider.interface');

export interface IGameProvider {

    getGameTable(id: string): Promise<GameTable>
    getGameState(id: string): Promise<GameState>
    playMove(id: string): Promise<GameState>
}


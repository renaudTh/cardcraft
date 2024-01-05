import { Observable } from "rxjs";
import { GameState, GameTable } from "./model/game.model";

export interface IGameService {

    getGameTable(id: string): Promise<GameTable>
    playMove(): void
    get gameState$(): Observable<GameState>;

}
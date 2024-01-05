import { BehaviorSubject, Observable } from "rxjs";
import { IGameService } from "./game.service.interface";
import { GameTable, GameState } from "./model/game.model";
import { GAME_PROVIDER, IGameProvider } from "./game.provider.interface";
import { Inject, Injectable } from "@angular/core";

@Injectable()
export class GameService implements IGameService {

    private _gameStateSubject = new BehaviorSubject<GameState>({isEnded: false, isWon: false, stacks: []});
    private _id: string = "";
    constructor(@Inject(GAME_PROVIDER) private provider: IGameProvider){

    }
    async initGame(id: string): Promise<void> {
        this._id = id;
        const t = await this.provider.getGameTable(id);
        const state = await this.provider.getGameState(id);
        this._gameStateSubject.next(state);
    }
    async getGameTable(id: string): Promise<GameTable> {
        return this.provider.getGameTable(id);
    }
    async playMove(): Promise<void>{
        const state = await this.provider.playMove(this._id)
        this._gameStateSubject.next(state);
    }
    get gameState$(): Observable<GameState> {
        return this._gameStateSubject.asObservable();
    }

}
import { Card } from "./model/game.model";

export interface ICardProvider {

    loadCardImages(): Promise<void>
    getCardImage(card: Card): Promise<HTMLImageElement>;

}
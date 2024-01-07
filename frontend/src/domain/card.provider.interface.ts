import { InjectionToken } from "@angular/core";
import { Card } from "./model/game.model";


export const CARD_PROVIDER = new InjectionToken<ICardProvider>('card.provider');
export interface ICardProvider {

    loadCardImages(): Promise<void>
    getCardImage(card: Card): HTMLImageElement;

}
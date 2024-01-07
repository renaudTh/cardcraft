import { Injectable } from "@angular/core";
import { concatMap, from, map } from "rxjs";
import { ICardProvider } from "src/domain/card.provider.interface";
import { Card } from "src/domain/model/game.model";


@Injectable()
export class CardProviderService implements ICardProvider {

    private _cardsImages: HTMLImageElement[] = [];

    private _families = ['C', 'H', 'D', 'S'];
    private _numbers = ["2", "3", "4", "5", "6", "7", "8", "9", "0", "J", "Q", "K", "A"];

    async loadCardImages(): Promise<void> {
        const keys:string[] = [];
        this._families.forEach((f) => {
            this._numbers.forEach((n) => {
                keys.push(`${n}${f}`);
            })
        })
       this._cardsImages = await Promise.all(keys.map((k) => this.loadCard(k, `./assets/cards/${k}.png`)))
       const back = await this.loadCard("back", `./assets/cards/back.png`);
       this._cardsImages.push(back);
    }
    private async loadCard(id: string, src: string) : Promise<HTMLImageElement>{
        return new Promise((resolve, reject) => {
            const img = new Image();
            img.onload = () => resolve(img);
            img.onerror = reject;
            img.width = 314;
            img.height = 226; 
            img.src =  src;
            img.id = id;
        })
    }
    getCardImage(card: Card): HTMLImageElement {
        let ret : HTMLImageElement | undefined;
        if(!card.visible){
          ret = this._cardsImages.find((img) => img.id === `back`);
        } 
        else {
            const family = this._families[card.family];
            const value = this._numbers[card.value - 2];
            ret = this._cardsImages.find((img) => img.id === `${value}${family}`);
        }
        if(ret) return ret;
        else throw new Error(`Card Not Found Card`);
    }
}
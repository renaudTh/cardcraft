export enum CardFamily {
    CLUB,
    HEART,
    DIAMOND,
    SPADE,
}

export interface Card {
    family: CardFamily,
    value: number, 
    visible: boolean,
}

export interface Stack {
    cards: Card[],
    rowPosition: number,
    columnPosition: number,
    spread: boolean,
}

export interface GameState{
    isWon: boolean,
    isEnded: boolean,
    stacks: Stack[]
}
export interface GameTable {
    rowNumber: number,
    columnNumber: number,
}
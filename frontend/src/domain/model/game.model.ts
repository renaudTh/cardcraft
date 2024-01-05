export enum CardFamily {
    CLUB,
    DIAMOND,
    HEART,
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
}

export interface GameState{
    isWon: boolean,
    isEnded: boolean,
    stacks: Stack[]
}
export interface GameTable {
    rowNumber: number,
    columnNumber: number,
    cardWidth: number,
    cardHeight: number,
    state?: GameState,
}
export type Deck = {
    slug: string;
    description: string;
    cards: {
        [key: string]: {
            front: string;
            back: string;
        };
    };
};

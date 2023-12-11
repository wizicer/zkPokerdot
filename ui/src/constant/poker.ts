type Suit = "spades" | "hearts" | "diamonds" | "clubs" | "big" | "little";
type Rank = "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" | "10" | "J" | "Q" | "K" | "A" | "JOKER";
type SortKey = string;

interface PokerCard {
  id: string;
  rank: Rank;
  suit: Suit;
  sortKey: SortKey;
  isSelected: boolean;
}

const NUMS: Rank[] = ["2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K", "A"];
const SUITS: Suit[] = ["spades", "hearts", "diamonds", "clubs"];
let POKERS: PokerCard[] = [];

NUMS.forEach((num) => {
  SUITS.forEach((suit) => {
    POKERS.push({
      id: num + suit,
      rank: num,
      suit: suit,
      sortKey: num,
      isSelected: false,
    });
  });
});

// Add jokers
POKERS = POKERS.concat([
  {
    rank: "JOKER",
    suit: "big",
    sortKey: "big",
    id: "JOKERbig",
    isSelected: false,
  },
  {
    rank: "JOKER",
    suit: "little",
    sortKey: "little",
    id: "JOKERlittle",
    isSelected: false,
  },
]);

export default POKERS;

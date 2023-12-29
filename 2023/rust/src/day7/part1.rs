/*
--- Day 7: Camel Cards ---

Your all-expenses-paid trip turns out to be a one-way, five-minute ride in an airship. (At least it's a cool airship!) It drops you off at the edge of a vast desert and descends back to Island Island.

"Did you bring the parts?"

You turn around to see an Elf completely covered in white clothing, wearing goggles, and riding a large camel.

"Did you bring the parts?" she asks again, louder this time. You aren't sure what parts she's looking for; you're here to figure out why the sand stopped.

"The parts! For the sand, yes! Come with me; I will show you." She beckons you onto the camel.

After riding a bit across the sands of Desert Island, you can see what look like very large rocks covering half of the horizon. The Elf explains that the rocks are all along the part of Desert Island that is directly above Island Island, making it hard to even get there. Normally, they use big machines to move the rocks and filter the sand, but the machines have broken down because Desert Island recently stopped receiving the parts they need to fix the machines.

You've already assumed it'll be your job to figure out why the parts stopped when she asks if you can help. You agree automatically.

Because the journey will take a few days, she offers to teach you the game of Camel Cards. Camel Cards is sort of similar to poker except it's designed to be easier to play while riding a camel.

In Camel Cards, you get a list of hands, and your goal is to order them based on the strength of each hand. A hand consists of five cards labeled one of A, K, Q, J, T, 9, 8, 7, 6, 5, 4, 3, or 2. The relative strength of each card follows this order, where A is the highest and 2 is the lowest.

Every hand is exactly one type. From strongest to weakest, they are:

    Five of a kind, where all five cards have the same label: AAAAA
    Four of a kind, where four cards have the same label and one card has a different label: AA8AA
    Full house, where three cards have the same label, and the remaining two cards share a different label: 23332
    Three of a kind, where three cards have the same label, and the remaining two cards are each different from any other card in the hand: TTT98
    Two pair, where two cards share one label, two other cards share a second label, and the remaining card has a third label: 23432
    One pair, where two cards share one label, and the other three cards have a different label from the pair and each other: A23A4
    High card, where all cards' labels are distinct: 23456

Hands are primarily ordered based on type; for example, every full house is stronger than any three of a kind.

If two hands have the same type, a second ordering rule takes effect. Start by comparing the first card in each hand. If these cards are different, the hand with the stronger first card is considered stronger. If the first card in each hand have the same label, however, then move on to considering the second card in each hand. If they differ, the hand with the higher second card wins; otherwise, continue with the third card in each hand, then the fourth, then the fifth.

So, 33332 and 2AAAA are both four of a kind hands, but 33332 is stronger because its first card is stronger. Similarly, 77888 and 77788 are both a full house, but 77888 is stronger because its third card is stronger (and both hands have the same first and second card).

To play Camel Cards, you are given a list of hands and their corresponding bid (your puzzle input). For example:

32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483

This example shows five hands; each hand is followed by its bid amount. Each hand wins an amount equal to its bid multiplied by its rank, where the weakest hand gets rank 1, the second-weakest hand gets rank 2, and so on up to the strongest hand. Because there are five hands in this example, the strongest hand will have rank 5 and its bid will be multiplied by 5.

So, the first step is to put the hands in order of strength:

    32T3K is the only one pair and the other hands are all a stronger type, so it gets rank 1.
    KK677 and KTJJT are both two pair. Their first cards both have the same label, but the second card of KK677 is stronger (K vs T), so KTJJT gets rank 2 and KK677 gets rank 3.
    T55J5 and QQQJA are both three of a kind. QQQJA has a stronger first card, so it gets rank 5 and T55J5 gets rank 4.

Now, you can determine the total winnings of this set of hands by adding up the result of multiplying each hand's bid with its rank (765 * 1 + 220 * 2 + 28 * 3 + 684 * 4 + 483 * 5). So the total winnings in this example are 6440.

Find the rank of every hand in your set. What are the total winnings?

32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483

32T3K 765 - rank 1
KTJJT 220 - rank 2
KK677 28 - rank 3
T55J5 684 - rank 4
QQQJA 483 - rank 5

*/

use std::{collections::HashMap, cmp::Ordering};

use crate::io_utils::read_lines_fully;

type CardRankingMap = HashMap<char, u8>;

#[derive(Debug, Ord, Eq, PartialEq, PartialOrd)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(Debug)]
struct Hand {
    hand: String,
    hand_set: HashMap<char, u8>,
    hand_type: HandType,
    bid: u16,
}

struct HandEval<'a> {
    pub hand: &'a Hand,
    pub rank: usize,
}

impl Hand {
    fn determine_hand_type(hand_set: &HashMap<char, u8>, the_hand_as_str: &str) -> HandType {
        let distinct_cards_in_hand_set = hand_set.len();

        if distinct_cards_in_hand_set == 1 {
            return HandType::FiveOfAKind;
        }

        if distinct_cards_in_hand_set == 2 {
            for entry in hand_set {
                if entry.1 == &1 || entry.1 == &4 {
                    return HandType::FourOfAKind;
                } else if entry.1 == &2 || entry.1 == &3 {
                    return HandType::FullHouse;
                }
            }
        }

        if distinct_cards_in_hand_set == 3 {
            if let Some(_) = hand_set.iter().find(|item| item.1 == &3) {
                return HandType::ThreeOfAKind;
            } else if let Some(_) = hand_set.iter().find(|item| item.1 == &2) {
                return HandType::TwoPair;
            }
        }

        if distinct_cards_in_hand_set == 4 {
            return HandType::OnePair;
        }

        if distinct_cards_in_hand_set == 5 {
            return HandType::HighCard;
        }

        unreachable!(
            "Cannot determine the hand type for hand: {}",
            the_hand_as_str
        );
    }

    pub fn new(the_hand: String, bid: u16) -> Self {
        let mut hand_set = HashMap::<char, u8>::new();

        for hand_char in the_hand.chars() {
            if let Some(set_char) = hand_set.get_mut(&hand_char) {
                *set_char += 1;
            } else {
                hand_set.insert(hand_char, 1);
            }
        }

        let hand_type = Hand::determine_hand_type(&hand_set, &the_hand);

        return Hand {
            hand: the_hand,
            hand_set,
            hand_type,
            bid,
        };
    }
}

fn parse_hands() -> Vec<Hand> {
    let lines = read_lines_fully("src/day7/input.txt");

    let mut hands = Vec::<Hand>::new();

    for line in lines {
        let split = line.split_whitespace().collect::<Vec<&str>>();

        let the_hand = split.first().unwrap();
        let the_bid = split.last().unwrap().parse::<u16>().unwrap();

        hands.push(Hand::new(the_hand.to_string(), the_bid));
    }

    hands.sort_by(|a, b| b.hand_type.cmp(&a.hand_type));

    return hands;
}

const EXPECTED_ANSWER: u32 = 250370104;

pub fn run() {
    let mut card_ranking_map = CardRankingMap::new();
    card_ranking_map.insert('A', 14);
    card_ranking_map.insert('K', 13);
    card_ranking_map.insert('Q', 12);
    card_ranking_map.insert('J', 11);
    card_ranking_map.insert('T', 10);
    card_ranking_map.insert('9', 9);
    card_ranking_map.insert('8', 8);
    card_ranking_map.insert('7', 7);
    card_ranking_map.insert('6', 6);
    card_ranking_map.insert('5', 5);
    card_ranking_map.insert('4', 4);
    card_ranking_map.insert('3', 3);
    card_ranking_map.insert('2', 2);

    let mut hands = parse_hands();

    hands.sort_by(|a, b| {
        let hand_type_cmp_result = b.hand_type.cmp(&a.hand_type);

        if hand_type_cmp_result != Ordering::Equal {
            return hand_type_cmp_result;
        }

        for zipped_chars in a.hand.chars().zip(b.hand.chars()) {
            if let (Some(char_one_ranking), Some(char_two_ranking)) = (
                card_ranking_map.get(&zipped_chars.0),
                card_ranking_map.get(&zipped_chars.1),
            ) {
                if char_one_ranking > char_two_ranking {
                    return std::cmp::Ordering::Greater;
                }

                if char_one_ranking < char_two_ranking {
                    return std::cmp::Ordering::Less;
                }
            }
        }

        return std::cmp::Ordering::Equal;
    });

    hands.iter().for_each(|hand| println!("{:?}", hand));

    let mut total_winnings: u32 = 0;
    hands.iter().enumerate().for_each(|(ix, hand)| {
        total_winnings += hand.bid as u32 * (ix as u32 + 1);
    });

    assert_eq!(EXPECTED_ANSWER, total_winnings);

    println!("Total winnigs: {}", total_winnings);
}

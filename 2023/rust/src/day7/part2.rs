/*
--- Part Two ---

To make things a little more interesting, the Elf introduces one additional rule. Now, J cards are jokers - wildcards that can act like whatever card would make the hand the strongest type possible.

To balance this, J cards are now the weakest individual cards, weaker even than 2. The other cards stay in the same order: A, K, Q, T, 9, 8, 7, 6, 5, 4, 3, 2, J.

J cards can pretend to be whatever card is best for the purpose of determining hand type; for example, QJJQ2 is now considered four of a kind. However, for the purpose of breaking ties between two hands of the same type, J is always treated as J, not the card it's pretending to be: JKKK2 is weaker than QQQQ2 because J is weaker than Q.

Now, the above example goes very differently:

32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483

    32T3K is still the only one pair; it doesn't contain any jokers, so its strength doesn't increase.
    KK677 is now the only two pair, making it the second-weakest hand.
    T55J5, KTJJT, and QQQJA are now all four of a kind! T55J5 gets rank 3, QQQJA gets rank 4, and KTJJT gets rank 5.

With the new joker rule, the total winnings in this example are 5905.

Using the new joker rule, find the rank of every hand in your set. What are the new total winnings?

*/

use std::{cmp::Ordering, collections::HashMap};

use crate::io_utils::read_lines_fully;

type CardRankingTuple = (char, u8);
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
    have_jokers: bool,
    bid: u16,
}

struct HandEval<'a> {
    pub hand: &'a Hand,
    pub rank: usize,
}

impl Hand {
    fn determine_hand_type(
        hand_set: &HashMap<char, u8>,
        the_hand_as_str: &str,
        have_jokers: bool,
    ) -> HandType {
        let distinct_cards_in_hand_set = hand_set.len();

        if distinct_cards_in_hand_set == 1 {
            return HandType::FiveOfAKind;
        }

        // AAAKK - full house
        // AAAJJ - 5 of a kind
        // JJJAA - 5 of a kind
        // JAAAA - 5 of a kind
        // AAAQQ - full house
        // AAAAQ - 4 of a kind
        if distinct_cards_in_hand_set == 2 {
            if have_jokers {
                return HandType::FiveOfAKind;
            } else {
                if let Some(_) = hand_set.iter().find(|item| item.1 == &4) {
                    return HandType::FourOfAKind;
                } else if let Some(_) = hand_set.iter().find(|item| item.1 == &2) {
                    return HandType::FullHouse;
                }
            }
        }

        // AAJKK - full house ( 1 joker )
        // AAJJK - four of a kind ( 2 jokers)
        // AAAQK - three of a kind
        // AAQQK - two pair
        // T55J5 - four of a kind ( 1 joker )
        if distinct_cards_in_hand_set == 3 {
            if have_jokers {
                let joker_count = hand_set.get(&'J').unwrap();

                if joker_count == &2 {
                    return HandType::FourOfAKind;
                }

                if joker_count == &1 {
                    if hand_set.values().max().unwrap() == &3 {
                        return HandType::FourOfAKind;
                    }
                    return HandType::FullHouse;
                }

                return HandType::FourOfAKind;
            } else {
                if let Some(_) = hand_set.iter().find(|item| item.1 == &3) {
                    return HandType::ThreeOfAKind;
                } else if let Some(_) = hand_set.iter().find(|item| item.1 == &2) {
                    return HandType::TwoPair;
                }
            }
        }

        // 22345 - one pair
        // 22J45 - three of a kind
        // JJ234 - three of a kind
        if distinct_cards_in_hand_set == 4 {
            if have_jokers {
                return HandType::ThreeOfAKind;
            } else {
                return HandType::OnePair;
            }
        }

        // 23456 - high card
        // J3456 - one pair
        if distinct_cards_in_hand_set == 5 {
            if have_jokers {
                return HandType::OnePair;
            } else {
                return HandType::HighCard;
            }
        }

        unreachable!(
            "Cannot determine the hand type for hand: {}",
            the_hand_as_str
        );
    }

    pub fn new(the_hand: String, bid: u16) -> Self {
        let mut hand_set = HashMap::<char, u8>::new();
        let mut have_jokers = false;

        for hand_char in the_hand.chars() {
            if !have_jokers && hand_char == 'J' {
                have_jokers = true;
            }

            if let Some(set_char) = hand_set.get_mut(&hand_char) {
                *set_char += 1;
            } else {
                hand_set.insert(hand_char, 1);
            }
        }

        let hand_type = Hand::determine_hand_type(&hand_set, &the_hand, have_jokers);

        return Hand {
            hand: the_hand,
            hand_set,
            hand_type,
            have_jokers,
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

const EXPECTED_ANSWER: u32 = 251735672;

pub fn run() {
    let mut card_ranking_map = CardRankingMap::new();
    card_ranking_map.insert('A', 14);
    card_ranking_map.insert('K', 13);
    card_ranking_map.insert('Q', 12);
    card_ranking_map.insert('T', 10);
    card_ranking_map.insert('9', 9);
    card_ranking_map.insert('8', 8);
    card_ranking_map.insert('7', 7);
    card_ranking_map.insert('6', 6);
    card_ranking_map.insert('5', 5);
    card_ranking_map.insert('4', 4);
    card_ranking_map.insert('3', 3);
    card_ranking_map.insert('2', 2);
    card_ranking_map.insert('J', 1);

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

    // hands.iter().for_each(|hand| println!("{:?}", hand));

    let mut total_winnings: u32 = 0;
    hands.iter().enumerate().for_each(|(ix, hand)| {
        total_winnings += hand.bid as u32 * (ix as u32 + 1);
    });

    assert_eq!(EXPECTED_ANSWER, total_winnings);

    println!("Total winnigs: {}", total_winnings);
}

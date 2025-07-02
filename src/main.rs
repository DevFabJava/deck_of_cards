use std::{collections::btree_map::Values, vec::Vec};
#[derive(Debug)]


struct Deck {
    cards: Vec<String>,
}


fn main() {
    //Listof 'suits' - 'hearts', 'spades'
    //List of 'values' - 'ace', 'two', 'three'

    let suits = ["Hearts", "Spades", "Diamonds", "Clubs"];
    let values = ["Ace", "Two", "Three","Four", "Five",
    "Six", "Seven", "Eight", "Nine", "Ten", "Jack", "Queen", "King"];

    //Empty vector(mutable)

    let mut  cards = vec![] ;
    

    //Double nested for loop

    for suit in suits  {
        for value in values {
            let card = format!("{} of {}", value, suit);
            cards.push(card);
        }
        
    }



    let deck: Deck = Deck { cards };

    println!("Heres your deck: {:?}", deck);
}

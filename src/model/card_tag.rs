use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub enum CardTag{
    //Action
    CanReplaceAction,
    WillReplaceAction,
    CanGainAction,
    WillGainAction,

    //Cards in hand
    CanReplaceCard,
    WillReplaceCard,
    CanDrawCard,
    NetGainCards,
    NetLoseCards,

    //Discard
    CanDiscardCard,
    WillDiscardCard,
    DiscardFromDeck,

    //Gains Cards
    WillGainCard,
    WillGainCardUnder4,
    WillGainCardToHandUnder4,
    WillGainCardUnder5,
    WillGainCardToHandUnder5,
    GainGold,
    GainSilver,
    GainCopper,
    

    //Trashing
    TrashFromHand,
    TrashMultipleFromHand,
    TrashForBenefit,
    TrashToGain,
    TrashTreasre,
    TrashAction,
    TrashVictory,
    TrashCurse,
    TrashSelf,
    TrashFromDeck,
    
    //Buys
    CanGainBuy,
    WillGainBuy,
    
    //Coin
    CanGainCoin,
    WillGainCoin,
    
    //Miss
    ProtectionFromAttack,
    WeakerForEmptySupply,
    StrongerForEmptySuppy,
    UsesActionInHand,
    
    //Controls Deck Order
    ControlsTopOfDeck,
    AddCardToTopOfDeck,

    //Attack type
    CurseAttack,
    TrashingAttack,
    DiscardingAttack,
    DiscardToTopOfDeckAttack,

    //Card costs
    Costs0,
    Costs1,
    Costs2,
    Costs3,
    Costs4,
    Costs5,
    Costs6,
    Costs7,
    Costs8,

    //Card types
    IsAction,
    IsTreasure,
    IsVictory,
    IsCurse,
    IsAttack,
    IsReaction,
}


#[cfg(test)]
mod test {
    use super::*;
    use rocket::serde::json::serde_json;

    #[test]
    #[allow(clippy::clone_on_copy)]
    fn test_clone() {
        let mut card_tag_one = CardTag::CanDiscardCard;
        let card_tag_two = card_tag_one.clone();
        card_tag_one = CardTag::WillGainCardUnder4;
        assert_ne!(card_tag_one, card_tag_two);
        assert_eq!(card_tag_two, CardTag::CanDiscardCard);
    }
    
    #[test]
    fn test_clone_from() {
        let card_tag_one = CardTag::CanDiscardCard;
        let mut card_tag_two = CardTag::CanGainBuy;
        card_tag_two.clone_from(&card_tag_one);
        assert_eq!(card_tag_two, CardTag::CanDiscardCard);
    }

    #[test]
    fn test_eq_true() {
        let card_tag_one = CardTag::CanDiscardCard;
        let card_tag_two = CardTag::CanDiscardCard;
        assert!(card_tag_one == card_tag_two);
    }

    #[test]
    #[allow(clippy::nonminimal_bool)]
    fn test_eq_false() {
        let card_tag_one = CardTag::CanDiscardCard;
        let card_tag_two = CardTag::TrashingAttack;
        assert!(card_tag_one != card_tag_two);
    }

    #[test]
    #[allow(clippy::nonminimal_bool)]
    fn test_serialize() {
        let card_tag = CardTag::CanDiscardCard;
        let json = serde_json::to_string(&card_tag).unwrap();
        
        assert_eq!(json,  "\"CanDiscardCard\"");
    }

    #[test]
    #[allow(clippy::nonminimal_bool)]
    fn test_deserialize() {
        let card_tag = CardTag::CanDiscardCard;
        let json = "\"CanDiscardCard\"";
        let json_card_tag: CardTag = serde_json::from_str(json).unwrap();

        assert_eq!(json_card_tag, card_tag);
    }

    #[test]
    fn test_fmt() {
        let card_tag = CardTag::CanDiscardCard;
        assert_eq!(
            format!("{card_tag:?}"), 
            "CanDiscardCard");
    }
}

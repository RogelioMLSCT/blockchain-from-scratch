use  blockchain_from_scratch::balances::Pallet;

const ALICE: &str = "Alice";
const BOB: &str = "Bob";



#[test]

fn init_balances() {
    let mut pallet = Pallet::new();
    assert_eq!(pallet.get_balance(&ALICE.to_string()), 0);
    pallet.set_balance(&ALICE.to_string(), 100);
    assert_eq!(pallet.get_balance(&ALICE.to_string()), 100);
    assert_eq!(pallet.get_balance(&BOB.to_string()), 0);
}

#[test]
fn transfer_ok() {
    let mut pallet = Pallet::new();

    pallet.set_balance(&ALICE.to_string(), 100);
    pallet.set_balance(&BOB.to_string(), 50);

    let result = pallet.transfer(&ALICE.to_string(), &BOB.to_string(), 30);

    assert!(result.is_ok());
    assert_eq!(pallet.get_balance(&ALICE.to_string()), 70);
    assert_eq!(pallet.get_balance(&BOB.to_string()), 80);
}

#[test]
fn transfer_insufficient_balance() {
    let mut pallet = Pallet::new();

    pallet.set_balance(&ALICE.to_string(), 10);

    let result = pallet.transfer(&ALICE.to_string(), &BOB.to_string(), 50);

    assert!(result.is_err());
}
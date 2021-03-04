#[derive(Debug)] // 這讓我們可以顯示每個州
enum UsState {
    Alabama,
    Alaska,
    // --省略--
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("25 cent with state:{:?}", state);
            25
        }
    }
}

fn main() {
    let c1 = Coin::Quarter(UsState::Alabama);
    let cents = value_in_cents(c1);
    println!("{}", cents)
}

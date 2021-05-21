// 러스트는 match라 하는 강력한 흐름 제어 연산자를 가지고 있다.
// 이를 통하여 일련의 패턴에 대해 어떤 값을 비교한 뒤 어떤 패턴에 매치되었는지 바탕으로 코드를 수행하도록 해준다.
// 만약 match에서 enum값을 하나라도 빼먹는다면 에러가 발생한다.


enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin { // match를 사용하여 값에 따라 u32를 반환 한다.
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
        _ => (), // 다른 switch, case의 default의 개념이다 
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> { // Option 또한 매칭할 수 있다.
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }

    // if let으로 위와 같은 효과를 줄 수 있다.    
    if let Some(3) = some_u8_value {
        println!("three");
    }
}
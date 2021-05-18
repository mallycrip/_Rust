// if표현식을 이용하여 코드를 조건에 따라 분가할 수 있도록 한다.
// Python과 비슷하게 소괄호를 따로 두지는 않는다.
// 코드의 조건은 반드시 bool 이어야 한다.

// loop를 사용하여 코드를 반복 수행할 수 있다.
// loop를 사용하여 반복하였을 경우 break을 사용하여 루프를 멈출 수 있다.

// while을 이용하여 조건부 반복을 수행할 수 있다.

// for 반복문을 사용하면 컬렉션의 각 요소에 대한 코드를 수행할 수 있다.
// Rust의 for문은 Python의 for문과 유사하다.
// Rust에서 range는 (1..10)과 같은 형태로 사용된다.

fn main() {
    let number = 3;

    if number < 5 {
        println!("True");
    } else if number == 3{
        println!("True");
    } else {
        println!("False");
    }

    let condition = true;
    let number = if condition { 
        5
    } else {
        6
    }; // let에도 if를 사용할 수 있다. if문과 else 모두 타입은 같아야 한다.

    println!("The value of number is : {}", number);


    let mut x = 5;
    loop {
        if x == 0 {
            break
        }
        println!("{}", x);
        x -= 1;
    } // loop 사용

    while x != 5 {
        println!("{}", x);
        x += 1;
    } // while문 사용

    let arr = [1,2,3,4,5];

    for element in arr.iter() {
        println!("value is {}", element);
    } // for문 사용

    for number in (1..10) {
        println!("{}", number)
    } // for문과 range
}
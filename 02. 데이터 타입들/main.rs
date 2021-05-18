// Rust에서 모든 값들은 타입을 가지고 있다. 
// 타이핑을 통하여 Rust에게 데이터를 어떻게 두룰지 알 수 있도록 해야한다.
// 타입은 크게 스칼라와 컴파운드 이 두가지로 나눌 수 있다.

// Rust는 타입이 고정된 언어이다. 이가 의미하는 바는 모든 변수의 타입이 컴파일 시에 반드시 정해져 있어야 한다는 것이다.

fn main() {
    // 스칼라는 하나의 값으로 표현되는 타입이다.
    // Rust에는 정수형, 부동소수점 숫자, boolean그리고 문자 이렇게 네가지의 스칼라 타입을 보유하고 있다.
    // 정수
    println!("signed 8 bit => MAX: {}, MIN: {}", i8::MAX, i8::MIN);
    println!("unsigned 8 bit => MAX: {}, MIN: {}", u8::MAX, u8::MIN);
    println!("signed 16 bit => MAX: {}, MIN: {}", i16::MAX, i16::MIN);
    println!("unsigned 16 bit => MAX: {}, MIN: {}", u16::MAX, u16::MIN);
    println!("signed 32 bit => MAX: {}, MIN: {}", i32::MAX, i32::MIN);
    println!("unsigned 32 bit => MAX: {}, MIN: {}", u32::MAX, u32::MIN);
    println!("signed 64 bit => MAX: {}, MIN: {}", i64::MAX, i64::MIN);
    println!("unsigned 64 bit => MAX: {}, MIN: {}", u64::MAX, u64::MIN);
    println!("arch => MAX: {}, MIN: {}", usize::MAX, usize::MIN);

    // 부동 소수점
    let x = 2.0; // f64

    // addition
    let sum = 5 + 10;
    // subtraction
    let difference = 95.5 - 4.3;
    // multiplication
    let product = 4 * 30;
    // division
    let quotient = 56.7 / 32.2;
    // remainder
    let remainder = 43 % 5;


    // Boolean
    let flag: bool = true;


    // 문자 
    let c = 'z';
    let emoji = '🎈';
    println!("{}", emoji);


    // 튜플
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    let five_hundred = tup.0;


    // 배열
    let some_arr = [1,2,3,4,5];
    // let instance_arr = [1, "some string"]; => Error
    let one = some_arr[0];
}
// Rust에서의 기본 변수는 불변성이다. 
// 이를 통해 Rust가 제공하는 안전성과 손쉬운 동시성이라는 장점을 취할 수 있다.



fn main() {
    let x = 5;
    println!("The value of x is : {}", x);
    // x = 6; => Error 불변성 변수에 재 할당
    // println!("The value of x is : {}", x);

    let mut y = 5;
    println!("The value of y is : {}", y);
    y = 6;
    println!("The value of y is : {}", y);
}
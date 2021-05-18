// Rust에서의 기본 변수는 불변성이다. 
// 이를 통해 Rust가 제공하는 안전성과 손쉬운 동시성이라는 장점을 취할 수 있다.



fn main() {
    let x = 5;
    println!("The value of x is : {}", x);
    // x = 6; => Error 불변성 변수에 재 할당
    // println!("The value of x is : {}", x);

    let mut y = 5; // 가변 변수를 이용할 때에는 mut을 사용하면 된다.
    println!("The value of y is : {}", y);
    y = 6;
    println!("The value of y is : {}", y);

    const Z: u32 = 5; // 상수를 이용할 시, const와 함께 타이핑을 해주어야 한다.
    println!("The value of z is : {}", Z);

    let x = 6; // 만일 이전에 선언한 변수와 같은 이름의 새 변수를 선언할 시 이전 변수를 shadows하게 된다.
    println!("The value of x is : {}", x);

    // let키워드를 사용하면 반복하여 같은 변수명으로 변수를 shadow할 수 있다.    
    // 이러한 방법으로 변수를 사용하는 것은 mut으로 선언하는 것과는 차이를 가지게 된다. 
    // let키워드를 다시 사용하여 효과적으로 새 변수를 선언하고, 값의 유형을 변경할 수 있으면서도 동일 이름을 사용할 수 있다.
    let spaces = "   ";
    let spaces = spaces.len();
    // Shadow를 이용하여 위와 같은 구조가 허영이 된다. 
    // 이러한 방식으로 space_str 또는 space_num과 같이 대체된 이름을 사용하는 대신 간단하게 spaces라는 이름을 사용할 수 있게 해준다.
    // 하지만 mut을 사용하려고 한다면 에러가 발생하게 된다.
    // 다음은 에러가 발생하는 코드이다
    // let mut spaces = "     ";
    // spaces = spaces.len();
}
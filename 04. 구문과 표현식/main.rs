fn main() {
    let y = 6; // 이 구문은 반환값이 없으므로 let x = (let y = 6);과 같은 식을 사용할 수 없다.

    let y = {
        let x = 3;
        x + 1 // 표현식은 종결을 나타내는 세미콜론을 사용하지 않는다. 만일 세미콜론을 표현식 마지막에 추가하면 반환 값이 아니게 된다.
    }; // 이 block은 4를 산출한다. 이 값은 let구문의 일브로 y에 bound된다.


    println!("The value of y is : {}", y);
    println!("The return value of five function : {}", five());
    println!("3 + 5 : {}", sum(3, 5));
}

fn five() -> i32 {
    5
} // 반환 값을 갖는 함수

fn sum(n: i32, m: i32) -> i32 {
    n + m
}

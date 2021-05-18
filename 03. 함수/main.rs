// Rust는 변수, 함수의 이름 형식 규칙으로 snake case를 사용한다. 
// 함수의 위치는 상관 없다. 어디에든 정의만 되어 있으면 된다.
// 함수는 함수 고유한 부분인 특별한 변수 매개변수를 갖는 형식으로 선언될 수 있다.
// 함수가 매개변수를 취할 시, 그들의 전달인자로 제공할 수 있다.


fn main() {
    println!("Hello, World");
    another_function();
    another_function_with_args(5);
}

fn another_function() {
    println!("Another function.");
}

fn another_function_with_args(x: i32) {
    println!("{}", x);
}

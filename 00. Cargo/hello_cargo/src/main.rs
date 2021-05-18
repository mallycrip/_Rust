// cargo new hello_cargo --bin를 이용하여 cargo 프로젝트를 생성할 수 있다. 
// --bin 인자는 라이브러리가 아닌 실행 가능한 어플리케이션으로 만들어 준다.
// toml은 Cargo의 환경설정 포맷이다.

// cargo build와 cargo run을 이용하여 빌드 및 실행을 할 수 있다.
// cargo check를 통하여 실행파일은 생기지 않으나 컴파이 되는지 확인할 수 있다.

// 프로젝트를 배포할 준비가 되었다면 cargo build --release를 사용하여 최적화와 함께 컴파일 할 수 있다.
// 최적화는 코드를 빠르게 하지만 컴파일 속도는 느릴 수 있다.


fn main() {
    println!("Hello, world!");
}

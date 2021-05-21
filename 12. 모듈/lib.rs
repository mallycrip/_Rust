mod network { // mod 키워드 뒤에, 모듈의 이름 network가 쓰여지고 중괄호 안에 코드블록이 온다.
    fn connect() {
    }

    mod client { // 모듈 안에 모듈을 넣는 것도 가능하다. 이럴경우 network::client::connect 함수가 된다.
        fn connect() {
        }
    }
}

mod server; 
// server 모듈을 선언하고 있으나 코드 블록을 세미콜론으로 대체함으로써 server 모듈의 스코프 내에 정리된 코드를
// 다른 위치에서 찾으라고 말할 수 있다.
// 달리 말하자면 mod server;의 의미는 다음과 같다
// mod server {
//     // contents of server.rs
// }

pub mod something { // 다음과 같이 pub를 사용하여 public으로 만들 수 있다.
    pub fn connect() {

    }    
}

// 비공개 규칙이 존재한다
// 만일 어떤 아이템이 공개라면, 이는 부모 모듈의 어디에서건 접근이 가능하다.
// 만일 어떤 아이템이 비공개라면, 같은 파일 내에 있는 부모 모듈 및 이 부모의 자식 모듈에서만 접근이 가능하다.
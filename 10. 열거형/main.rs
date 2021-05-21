// 열거형은 enums라고 불리는 타입중 하나이다.
// 코드를 작성할 때 열거형이 구조체보다 유용하고 적절하게 사용될 수 있다.


fn main() {
    enum ipAddrKind {
        V4,
        V6
    } // 열거형을 이용하여 IP의 버전을 분리하였다.

    struct IpAddr {
        kind: ipAddrKind,
        address: String
    }

    let home = IpAddr {
        kind: ipAddrKind::V4, // 열거형은 이와 같이 사용할 수 있다.
        address: String::from("127.0.0.1"),
    };

    let home_v6 = IpAddr {
        kind: ipAddrKind::V6,
        address: String::from("::1"),
    };

    // 다음과 같이 다른 타입과 양의 데이터를 가질 수도 있다.
    enum IpAddress {
        V4(u8, u8, u8, u8),
        V6(String),
    }
    
    let home = IpAddress::V4(127, 0, 0, 1);
    
    let loopback = IpAddress::V6(String::from("::1"));

    // 열거형에서도 impl을 사용하여 메서드를 정의할 수 있다.
    impl Message {
        fn call(&self) {
            // 메소드 내용은 여기 정의할 수 있습니다.
        }
    }
    
    // let m = Message::Write(String::from("hello"));
    // m.call();

    // Option<T> 열거형을 사용하면 null처리를 편하게 할 수 있다.
    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;

    assert_eq!(some_number.unwrap(), 5); // true
}


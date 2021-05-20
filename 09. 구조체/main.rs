// 구조체는 튜플과 비슷하다. 
// 튜플과 유사하게 구조체의 구성요소는 각자 다른 타입을 지닐 수 있다.
// 구조체는 각각 구성요소에 대해 명명을 할 수 있기에 튜플보다 유연하게 다룰 수 있다.
// 구조체를 정의할 때는 struct 키워드를 먼저 입력하고 명명할 구조체명을 입력하면 된다.


struct User {
    name: String,
    email: String,
    age: u32
} // 구조체 정의


fn main() {
    // 구조체를 사용하기 위해서는 인스턴스를 생성하여야 한다.
    let mut mally = User {
        name: String::from("Minki Son"),
        email: String::from("migsking@naver.com"),
        age: 13
    };

    // 구조체에서 특정한 값을 사용하기 위해서는 . 표기법을 사용한다.
    println!("User name is {}", mally.name);
    mally.name = String::from("Mingi Son");
    println!("User name is {}", mally.name);

    let some_user = build_user(
        String::from("Seongjin Lee"), String::from("a@a.a"), 15);


    // 구조체 갱신법을 이용하여 기존 구조체 인스턴스로 새 구조체 인스턴스를 생성할 수 있다.
    let copy_some_user = User {
        name: some_user.name,
        email: String::from("b@b.b"),
        age: some_user.age
    }; 

    struct Rectangle {
        length: u32,
        width: u32,
    }
    
    // 구조체 안에 함수를 정의하기 위해서는 impl 블록을 사용한다.
    // 시그니처 및 본체 내의 모든 곳에 있는 첫번째 파라미터를 self로 변경 시킨다.
    // self타입이 Rectangle임을 Rust가 추측할 수 있다.
    // &를 사용함으로서 소유권을 가져오는 것을 원하지 않음을 나타낸다
    // 인스턴스가 변하길 원한다면 &mut self를 사용하면 된다.
    impl Rectangle {
        fn area(&self) -> u32 {
            self.length * self.width
        }
    }

    // impl을 이요하여 self를 가지지 않는 함수를 정의할 수 있다.
    // 이를 연관 함수라고 한다. 이는 메서드가 아니라 여전히 함수인데, 이는 함께 동작할 구조체의 인스턴스를 가지지 않고 있지 않아서 그렇다.
    // 다음 함수는 새로운 구조체의 인스턴스를 반환해주는 생성자로 자주 쓰인다.
    impl Rectangle {
        fn square(size: u32) -> Rectangle {
            Rectangle { length: size, width: size }
        }
    }
    // 위와 같은 함수를 사용하는 방법은 다음과 같다.
    let sq = Rectangle::square(3);


    let rect1 = Rectangle { length: 50, width: 30 };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}

fn build_user(name: String, email: String, age: u32) -> User {
    User {
        name,
        email,
        age
    } // 변수명이 필드명과 같은 경우 필드를 간단하게 초기화 할 수 있다.
}
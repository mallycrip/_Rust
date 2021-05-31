// 러스트의 표준 라이브러리에는 컬렉션이라 불리우는 여러개의 데이터 구조들이 존재한다.
// 대부분의 다른 데이터 타입들은 하나의 특정한 값을 나타내나, 컬렉션은 대다수의 값을 담을 수 있다.
// 대표적인 세 가지 컬렉션은 벡터, 스트링, 해쉬맵이다.
// 더 많은 컬렉션은 https://doc.rust-lang.org/std/collections/index.html에서 확인하면 된다.

use std::collections::HashMap;

fn main() {
    let v: Vec<i32> = Vec::new(); // 새 벡터를 생성한다. 이 벡터는 어떠한 값도 없기에 제네릭을 이용하여 구현한다.
    let v = vec![1, 2, 3]; // 값을 저장하고 있는 새로운 백터 생성
    let mut v = Vec::new(); // 값을 추가하기 위해서는 mut 키워드를 사용하여 가변으로 만들어 주저야 한다.

    v.push(5);
    v.push(6);
    v.push(7); // push 메서드를 이용하여 벡터에 값 추가

    let first: &i32 = &v[2]; // 인덱스 문법으로 벡터 내의 아이템에 접근
    let first: Option<&i32> = v.get(2); // get 메서드로 벡터 내의 아이템에 접근, get을 이용하면 None이 반환되므로 오류로 인해 프로그램이 죽지 않는다.

    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];

    // v.push(6); => Error, 새로운 요소를 벡터의 끝에 추가하는 것은 새로 메모리를 할당하여 예전 요소를 새 공간에 복사하는 일을 필요로 하는데
    // 첫번째 요소에 대한 참조자는 할당이 해제된 메모리를 가르키게 된다. 빌림 규칙은 프로그램이 이러한 상황에 빠지지 않도록 해준다.

    for i in &v { 
        println!("{}", i);
    } // 위와 같은 방법으로 for 루프를 이용할 수 있다.

    // 백터 내의 요소에 대한 가변 참조자로 반복하기 위해서는 *이라는 참조 연산자를 사용하여 값을 얻어내면 된다.
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

    let mut s = String::new(); // 비어 있는 새로운 String 생성

    let mut s = "some string".to_string(); // 스트링 리터럴 부터 String 생성
    s.push_str("bar"); // push_str로 스트링 슬라이스 추가
    s.push("l") // 한글자 추가하기
    
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // + 연산자로 새로운 String 값 만들기

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3); // format!으로 포맷팅

    let mut scores = HashMap::new(); // 해쉬맵 생성

    scores.insert(String::from("Blue"), 10); // 요소 넣기
    scores.insert(String::from("Yellow"), 50);

    let teams  = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect(); // zip을 이용하여 리스트로부터 해쉬맵 생성


    let team_name = String::from("Blue");
    let score = scores.get(&team_name); // 값 얻어오기

    for (key, value) in &scores { 
        println!("{}: {}", key, value);
    } // k, v 받아오기

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50); // entry를 이용하여 키 값이 없을 시 추가
}

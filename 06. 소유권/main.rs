// 소유권(Ownership)은 러스트의 가장 유니크한 특성이며, 러스트가 GC없이 메모리 안정성을 보장하게 해준다.

fn main() {
    let s = "hello"; // 이 스트링 리터럴의 값은 변수가 선언됨 시점 부터 스코프가 끝낼때 까지 유효하다.
    // 이 말은 스코프 안에서 s가 등장하면 유효하다는 말과 유효기간은 스코프 밖으로 벗어날 때 까지 유효하다.
    
    let s = String::from("Hello"); // String은 런타임시 발생하는 문자열에 대해 처리하는 자료형이다.
    // 즉 Heap에 값을 저장하게 된다. 
    // 마찬가지로 스코프안에서만 유효하다.
    // s가 유효하지 않는 스코프가 끝날 때, 메모리를 자동으로 해제한다. 이는 다른 gc와는 다른 방법이다.

    let some_str = String::from("some string");
    let copied_str = some_str; // 이 과정에서 move가 발생한다. 발생하는 이유는 이중 메모리 헤제를 막기 위해서 이다.
    // move가 일어나면서 some_str은 사용할 수 없다.

    // println!("{}", some_str); => 에러 발생
    // 이 점이 rust의 메모리 관리가 얕은 복사와는 다름을 알 수 있다.
    // 만일 some_str을 사용하고 싶다면 clone을 사용하면 된다.

    let cloned_str = copied_str.clone();
    println!("{}", cloned_str);

    // 이부분까지는 heap부분을 이용하기에 move가 발생하나 아래와 같이 컴파일 타임에 크기와 타입이 결정 되는 경우 복사가 발생한다.
    let s = "Hello";
    let copied_s = s;
    println!("{} {}", s, copied_s);

    // 이부분은 소유권에 대해 잘 설명한 코드 이다.
    let some_string = String::from("some string");
    takes_ownership(some_string); // some_string이 함수에 이동한 후
                                  // some_string은 메모리 할당이 해제 되었기에
    // takes_ownership(some_string) // 이부분은 오류가 발생한다.

    let some_string = "some string 2"; // 이 문자열은 힙을 사용하지 않기에 문제가 발생하지 않는다.
    makes_copy(some_string);
    makes_copy(some_string);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
} // 여기에서 메모리 할당이 해제 된다.

fn makes_copy(some_string: &str) {
    println!("{}", some_string);
}
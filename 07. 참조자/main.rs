fn main() {
    //값의 소유권을 넘기는 대신 개체에 대한 참조자를 인자로 사용하는 방법이 존재한다.
    let s1 = String::from("Hello");
    let s1_len = calculate_length(&s1); // &는 참조자이며 이 값은 소유권을 넘기지 않고 참조할 수 있도록 해준다.
    println!("{} {}", s1, s1_len);

    // 참조하는 것을 고치는 방법은 가변 참조자를 사용하는 것 이다.
    let mut s2 = String::from("Hello2");
    change(&mut s2); // &mut을 사용하여 가변 참조자를 생성한다.
    println!("{}", s2);

    // 다음은 에러가 나는 부분이다.
    let mut s3 = String::from("Hello3");

    let r1 = &mut s3;
    let r2 = &mut s3;
    
    // r1.push_str(", "); => Error, 러스트가 컴파일 타임에 데이터 레이스를 방지하기 위하여 제한을 둔다.
    // 데이터 레이스는 두 개 이상의 포인터가 동시에 같은 데이터를 접근할 때 하나의 포인터가 데이터를 사용하고 동기화를 하는 어떠한 메커니즘도 없을 때 발생한다.
    // 데이터 레이스는 정의되지 않은 동작을 일으키고 런타임에 이를 추적하고자 할 때 이를 진단하고 고치기 어렵다.
    // 러스트는 데이터 레이스 발생을 컴파일 시에 막아 버린다.

    // 다음은 여러가지의 가변 참조자를 만들 수 있도록 해준다.
    let mut s4 = String::from("Hello4");
    {
        let r3 = &mut s4;
    } // 스코프가 끝났으므로, 새로 가변 참조자를 만드는 것은 문제가 발생하지 않는다.
    let r4 = &mut s4;

    // 가변 참조자와 불변 참조자는 혼용할 수 없다. => 왜인지 컴파일은 된다. 튜토리얼에서는 안된다고 함. 
    let mut s5 = String::from("Hello5");
    let r5 = &s5;
    let r6 = &s5;
    // let r7 = &mut s5; // 문제가 발생
    println!("{}", r5);

    // Rust에서는 댕글링 참조자를 방지한다.

    // let r7 = {
    //     let s6 = String::from("Hello6");
    //     &s6
    // }; // 에러 발생
}


fn calculate_length(s: &String) -> usize { // 함수 시그니처도 &를 사용하여 s의 타입이 참조자임을 나타낸다.
    s.len()
    // s.push_str(", world!") => Error, 참조자를 변경하는 것은 허용되지 않는다.
} // s는 스코프가 끝났음에도 소유권이 없음에 아무런 일도 발생하지 않는다.

fn change(s: &mut String) { // 가변 참조자는 &mut String으로 가변 참조자를 받아야 한다.
    s.push_str(", world!")
}
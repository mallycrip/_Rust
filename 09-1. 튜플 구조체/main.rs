// 구조체명을 통하여 의미를 부여할 수 있으나 필드의 타입만 정의할 수 있고 명명은 할 수 없는 튜플 구조체가 존재한다.
// 튜플 구조체는 일반적인 구조체 정의 방법과 독같이 struct 키워드를 통하여 정의할 수 있고 튜플의 타입정의가 키워드 뒤에서 이루어진다.

fn main() {
    struct Color(i32, i32, i32); // 튜플 구조체 정의

    let black = Color(0 ,0, 0); // 인스턴스 생성
}
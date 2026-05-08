fn main(){
    // let 변수명: 타입 = 값;
    let name : &str = "홍길동"; 
    let age : i32;   // 초깃값 지정없이 생성

    age = 20;

    println!("이름: {}, 나이: {}",name, age);
}
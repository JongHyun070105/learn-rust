use std::env; // std::env 라이브러리를 사용하겠다는 의미

// 기본: 리턴 타입이 없다. | 파라미터로 받는 것이 없다. -> 매우 간단한 형태!!
fn main(){ 
    let args: Vec<String> = env::args().collect(); // env::args().collect() 를 사용해서 입력값을 받는다.

    let src_file = &args[1];
    let tgt_file = &args[2];

    println!("Source File: {}", src_file);
    println!("Target File: {}", tgt_file);

    // println!("hello")
}
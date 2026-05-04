fn main(){
    let mut sum = 0;
    for i in 1..101{ // 1..=100 과 동일함 | C 나 Java 처럼 중괄호 없이 for문 사용 못한다. => 중괄호만 있으면 한줄에 모든 구문을 사용해도 된다.
        sum += i;
    }

    println!("sum = {sum}");
}


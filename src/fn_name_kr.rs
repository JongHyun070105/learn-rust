fn main(){
    println!("1+...+100 = {}", 합구하기(100));
}

fn 합구하기(n:u32) -> u32{
    let mut sum:u32 = 0;
    for i in 1..=n {
        sum += i;
    }

    sum
}
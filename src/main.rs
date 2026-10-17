fn add_numbers(a:i32,b:i32)->i32{
    let result:i32 = a+b;
    if result > 10{
        return result - 10;
    }
    return result;
}
fn main(){
    println!("{}",add_numbers(2,3));
    println!("{}",add_numbers(10,12));
}
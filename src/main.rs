fn main(){
    let s1 = String::from("Hello");
    let s2 = "World"; // here s2 and s3 are declared with default &str as only first string can be mutable 
    let s3 = "Jai Hind";
    let result = s1+" "+s2+" "+s3;
    println!("{result}");

}
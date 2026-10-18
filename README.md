# Learning notes for Rust programming

1. file name starts with .rs , to run a file main.rs ->

```
rustc main.rs
./main
```

2. creating a project using cargo project manager

```
cargo new project_name
```

3. To build a project

```
cargo build
```

where inside the project_name/target/debug/ we can find the executable and execute the same using the command

```
./project_name
```

3. To run directly your project

```
cargo run project_name
```

4. To check for any issues in yout project use -

```
cargo check
```

## Comments in rust -

Single line //
multiline /_ value _/

**Need a semicolon at end of line to end statement**

5. Formatting the code -

```
cd src/
rustfmt filename.rs
```

**main.rs -> main function is entry point of program**

second program -

```
fn main() {
    let x = 4; // implicit type decision
    // x = "hello" will not work since i assigned this to a integer

    println!("x is :{}",x);

}
```

By default all the variables in rust are immutable , to
create a mutable variable use **mut** keyword

```
let mut x = 4;
x = 32; // this allows reassignment
```

Redeclaring existing variable is allowed -

```
fn main() {
    let x = 4;
    println!("x is :{}",x);
    let x = 5;
    println!("x is :{}",x);
}

Output -
 Compiling tutorial1 v0.1.0 (/Users/harshvalecha/harsh_work/rust_practice/tutorial1)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/tutorial1`
x is :4
x is :5
```

incrementing the variable -

```
fn main() {
    let x = 4;
    println!("x is :{}",x);
    let x = x+1;
    println!("x is :{}",x);
}
```

Scope examples -

```
fn main() {
    let x = 4;
    println!("x is :{}",x);
    {
        // scope
        let x = x - 2;
        println!("x is :{}",x);
    }
    let x = x+1;
    println!("x is :{}",x);
}

output -
x is :4
x is :2
x is :5
```

This also works -

```
fn main() {
    let x = 4;
    println!("x is :{}",x);
    let x = "hello";
    println!("x is :{}",x);
}

x is :4
x is :hello
```

Defining Constants -
**Naming convention** - All upper case separated by underscores

```
fn main() {
    const SECONDS_IN_MINUTES:u32 = 60 ; // u32 - unsigned integer 32
    println!("{}",SECONDS_IN_MINUTES);
}
```

No Reassignment allowed -

```
fn main() {
    const SECONDS_IN_MINUTES:u32 = 60 ; // u32 - unsigned integer 32
    const SECONDS_IN_MINUTES:u32 = 100 ; // no reassignment
    println!("{}",SECONDS_IN_MINUTES);
}
```

primitive data types - scalar data type - single value like integer or boolean

```
fn main() {
 let x:i32 = 2 ; // 32 bits integer
 // let x = 2 -- this is bydefault i32 only

 /*
    i8 , i16 ..... i128
 */
 let x = -32; // -ve is allowed for i32
 // unsigned integer cannot have negative number

 /*
 u8 range -> 0 to 2^8-1
 i8 range -> 2^7 to 2^7-1
 */

}
```

f32 - floating 32
f64

**default used for implicit types is f64**

```
fn main() {

    let floating_point:f32 = 23.32;
    println!("number is :{}",floating_point);
}
```

Boolean is a true or false -

```
fn main() {
    let true_or_false:bool = true;
    println!("{}",true_or_false); // while printing always need to print like "{}"
    let true_or_false:bool = false;
    println!("{}",true_or_false);
}
```

char -

```
fn main() {
    let letter: char = 'o';
    println!("{}",letter);
}
```

compound data types -

## tuple - fixed length sequence of elements

normal tuple creation and printing the value

```
fn main() {
    let tup :(i32,bool,char)= (1,true,'s');
    println!("{:?}",tup); // note here {} will not work
}

for each element printed in separate lines use -
println!("{:#?}",tup);
```

index based printing -

```
fn main() {
    let tup :(i32,bool,char)= (1,true,'s'); // 0 index based
    // println!("{:?}",tup);
    // printing by indices
    println!("{}",tup.0);
}
```

Assiging value on a index -

```
fn main() {
    let mut tup:(i32, bool,char) = (1,false,'s');
    tup.0 = 10;
    println!("{}",tup.0);
}
```

reassigning new values -

```
fn main() {
    let mut tup:(i32, bool,char) = (1,false,'s');
    tup = (10,true,'a'); // value is reassigned but u will see a warning
    println!("{:?}",tup);
}

warning displayed -
warning: value assigned to `tup` is never read
 --> src/main.rs:2:36
  |
2 |     let mut tup:(i32, bool,char) = (1,false,'s');
  |                                    ^^^^^^^^^^^^^ this value is reassigned later and never used
3 |     tup = (10,true,'a');
  |     ------------------- `tup` is overwritten here before the previous value is read
  |
  = note: `#[warn(unused_assignments)]` (part of `#[warn(unused)]`) on by default
```

## Arrays

```
fn main() {
    let mut arr:[i32;5]= [1,2,3,4,5]; // all elements need to be same
    // getting the value can be using arr[0] ...
    arr[4] = 32;
    println!("{}",arr[4]);
}
output -
32
```

Arrays cant be leaved uninitialised , or blank array gives
error

Possible examples(not allowed) -

```
let mut arr[i32;5];
let mut arr[i32;5]=[];
```

Multiple values printing -

```
fn main() {
    let x:u8 = 4;
    let y = x;
    println!("{}, {}",x,y);
}
```

## Crates -

preincluded things that automatically imports
into every rust program

like fn , println or standard library etc

### Taking input in rust

```
use std::io;

fn main(){

    let mut input = String::new(); // :: is a path separator operator , creating a new empty string

    io::stdin().read_line(&mut input).expect("failed to readline ");
    println!("This is your value: {}",input);
}
```

## overflowing in rust -

```
fn main(){
    let x:u8 = 256; // 0 - 255
    let y:i8 = 10; // -128 - 127
}

error -
error: literal out of range for `u8`
 --> src/main.rs:4:16
  |
4 |     let x:u8 = 256; // 0 - 255
  |                ^^^
  |
  = note: the literal `256` does not fit into the type `u8` whose range is `0..=255`
  = note: `#[deny(overflowing_literals)]` on by default

```

Addition program -

```
fn main(){
    let x:i8 = 12; // 0 - 255
    let y:i8 = 10; // -128 - 127

    let z = x+y;
    println!("{}",z);

}
```

Floats addition -

```
fn main(){
    let x:f32 = 12.2; // 0 - 255
    let y:f32 = 10.543; // -128 - 127

    let z = x+y;
    println!("{}",z);

}
```

Division -

```
fn main(){
    let x = 32.0;
    let y = 12.0;

    let z = y/x;
    println!("{}",z);
}
```

remainder -

```

fn main(){
    let x = 32.0;
    let y = 12.0;

    let z = x%y;
    println!("{}",z);
}
```

another way

```
fn main(){
    let x = 32.0f32;
    let y = 12.0f32;

    let z = x%y;
    println!("{}",z);
}
```

or underscore as well can be used

```
fn main(){
    let x = 32_i64;
    let y = 12_i64;

    let z = x%y;
    println!("{}",z);
}
```

another way is using as keyword (explicit type conversion)-

```
fn main(){
    let x = 32 as i64;
    let y = 12 as i64;

    let z = x%y;
    println!("{}",z);
}
```

type conversion during operation -

```
fn main(){
    let x = 32 as i64;
    let y = 12 as i32;

    let z = x / (y as i64);
    println!("{}",z);
}
```

one more way of writing numbers with underscore -

```
fn main(){
    let x = 127_000 as i64;
    let y = 10_i32;

    let z = x as i32 / y;
    println!("{}",z);
}
```

Taking integer input and printing -

```
use std::io;

fn main(){
    let mut input  = String::new();
    io::stdin().read_line(&mut input).expect("expected to readline");
    let int_input:i64 = input.trim().parse().unwrap();
    println!("{}",int_input);
}
```

comparision operators -

< , > , <= , >= , != , ==

example -

```

fn main(){
let cond = 2 < 3;
println!("{}",cond);

}

```

operands on left and right hand side should be of same
types -

otherwise we get error like these
error[E0308]: mismatched types
--> src/main.rs:2:21
|
2 | let cond = 2 <= 2.2;
| ^^^ expected integer, found floating-point number

```
fn main(){
    let cond = 2 <= 2.2;
    println!("{}",cond);

}
```

fix -

```
fn main(){
    let cond = (2 as f32) <= 2.2;
    println!("{}",cond);

}
```

## Compound condition

AND - &&
OR - ||
NOT - !

Examples -

```
fn main(){
    let cond = (2 as f32) <= 2.2;
    let cond2 = false && cond;
    println!("{}",cond2);

}

output - false
```

```
fn main(){
    let cond = (2 as f32) <= 2.2;
    let cond2 = false || cond;
    println!("{}",cond2);

}
output - true
```

```
fn main(){

    let cond2 = false || !false;
    println!("{}",cond2);

}
output - true
```

precedance - ! && ||

## Control flow -

If statement -

```
fn main(){
    let food = "cookie";

    if food == "cookie"{
        println!("I Like cookies too!");
    }

}
```

if else -

```
fn main(){
    let food = "cookie";

    if food != "cookie"{
        println!("I Like cookies too!");
    }
    else{
        println!("Oh thats too bad");
    }

}
```

if else else if -

```
fn main(){
    let food = "cookie";

    if food != "cookie"{
        println!("I Like cookies too!");
    }
    else if food =="cookie"{
        println!("OMG you like coookies");
    }
    else{
        println!("Oh thats too bad");
    }

}
```

# functions

```
fn main(){
    println!("Hello world");
    test_one();
}
fn test_one(){ // use snake case in naming of functions
    println!("Test has been called ...");
}

output -
Hello world
Test has been called ...

```

This works fine as well-

```
fn test_one(){ // use snake case in naming of functions
    println!("Test has been called ...");
}
fn main(){
    println!("Hello world");
    test_one();
}
```

## Function parameters -

```
fn add_numbers(a:i32,b:i32){
    println!("The sum is :{}",a+b);
}
fn main(){
    println!("Hello world");
    add_numbers(23,44);
}

op -
Hello world
The sum is :67
```

one more example -

```
fn greet(name: &str){
    println!("Hello {name}");
}
fn main(){
   greet("Kamlesh");
}
op - Hello Kamlesh
```

## Expression vs statement -

statment can be as simple as a simple line like let x = 30; that dont return anything

while expression is something that returns some value
following are expressions -

```
println!("hello");
add_numbers(20,30); // this is also expression

another example -

fn main(){
    let number = {
        let x = 3;
        x + 1
    };
    println!("{}",number);
}
```

## returning values from function -

```
fn main(){
    let result = add_numbers(23,44);
    println!("{}",result);
}
fn add_numbers(x:i32 , y:i32) -> i32{
    x + y // just add a expression at end will return , keep without semicolon
}
```

This one works fine as well -

```
fn main(){
    println!("{}",add_numbers(12,13));
}
fn add_numbers(x:i32 , y:i32) -> i32{
    x + y // just add a expression at end will return keep without semicolon
}
```

return keyword is also present -

```
fn main(){
    println!("{}",add_numbers(12,13));
}
fn add_numbers(x:i32 , y:i32) -> i32{
    return x + y ;
}
```

one more example -

```
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
```

## Creating a string using from

```
fn main(){
    let string = String::from("hello");
    println!("{}",string);
}
```

we store fixed size data in stack and dynamic size
data in heap

# Loops

1. While Loop -

```
fn main(){
    let a = [1,2,3,4,5,6];
    let mut index = 0;
    while index < 5{
        println!("The value at {} is :{}",index,a[index]);
        index +=1;
    }
    println!("Outside the loop");
}
op -
The value at 0 is :1
The value at 1 is :2
The value at 2 is :3
The value at 3 is :4
The value at 4 is :5
Outside the loop
```

2. For loop -

```
fn main(){
    let a = [1,2,3,4,5,6];
    for element in a {
        println!("the value is :{element}");
    }
}
op -
the value is :1
the value is :2
the value is :3
the value is :4
the value is :5
the value is :6
```

Priting the array elements in reverse -

```
fn main(){
    let a = [1,2,3,4,5,6];
    // `rev()` is an iterator method; turn the array into an iterator first.
    for element in a.iter().rev() {
        println!("the value is :{element}");
    }
}
```

3. loop -

```
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}
op - The result is 20
```

For Loop step by -

```
fn main(){
    for i in (1..10).step_by(2){
        println!("{}",i);
    }
}

op -
1
3
5
7
9
```

Match -

```
fn main() {
  let day = 4;

  let result = match day {
    1 => "Monday",
    2 => "Tuesday",
    3 => "Wednesday",
    4 => "Thursday",
    5 => "Friday",
    6 => "Saturday",
    7 => "Sunday",
    _ => "Invalid day.",
  };

  println!("{}", result);
}
op - Thursday
```

## Creating Strings -

Default when string created it is with &str (immutable), while with String (mutable)
This can be done using 2 methods - String::from("somevalue") or "something".to_string()
Note - strings need to be in double quotes

```
fn main(){
    println!("{}",String::from("Kamlapati"));
    let str2 = "Helo world".to_string();
    println!("{}",str2);
}
```

strings are mutable so it can be changed if declared with mut

```
fn main(){
    let mut stru = String::from("Hello");
    stru.push_str(" World!");
    println!("{stru}");
}
op - Hello World!
```

## Concatenating strings -

1. using !format macro -

```
fn main(){
    let s1 = String::from("Hello");
    let s2 = String::from("World");
    let s3 = String::from("Jai Hind");
    let result = format!("{} {} {}",s1,s2,s3);
    println!("{result}");

}
op - Hello World Jai Hind
```

2. directly using + but generally not used as it gets messy

```
fn main(){
    let s1 = String::from("Hello");
    let s2 = "World"; // here s2 and s3 are declared with default &str as only first string can be mutable
    let s3 = "Jai Hind";
    let result = s1+" "+s2+" "+s3;
    println!("{result}");

}
```

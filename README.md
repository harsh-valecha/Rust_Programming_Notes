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

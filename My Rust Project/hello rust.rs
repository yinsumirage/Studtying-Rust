//2023-09-03 9:47 the first time i learn rust
// the reason why i learn it is that i want to learn operating system
// i learn from the websit below. it is very clear.
// https://fasterthanli.me/articles/a-half-hour-to-learn-rust

let x; //declare x
x=42;  // assign 43 to x
let y=42;

let x:i32 //'i32' is signed 32-bit integer
x=42 // there's i8, i16, i32, i64, i128
// also u8, u16, u32, u64, u128 for unsigned

let x:i32 = 42
// the compiler will prevent you from using it before it was declared

// the underscore is a special name-or rather,a "lack of name"
// basically means to throw away something
let _ = 42;

// names start with an underscore 
// compiler won't warn about them being unused
let _x=42;

let x=13;
let x=x+3; // the first x no longer exists

// Rust has tuples
// fixed-length collection of values of different types
let pair = ('a',17);
pair.0; //this is 'a'
pair.1; //this is 17

let pair:(char , i32)=('a',17);

// tuples can be destructured , break them down into their individual fields
let(some_char ,some_int)=('a',17);

// this especially useful when a fuction returns a tuple
let (left,right)=slice.split_at(middle);

// destructuring a tuple, can bu used to throw away part of it:_
let (_,right) = slice.split_at(middle);

// the semi-colon marks the end of a statement
// which meads statements can span mulitiple lines
let x = vec![1,2,3,4,5,6,7,8]
        .iter()
        .map(|x|x+3)
        .fold(0,|x,y|x+y);

// fn declares a function
fn greet(){
    println("Hi there!");
}

// the function that returns a 32-bits signed interger
// the arrow indicates its return type
fn fair_dice_roll()-> i32{
    4
}

// a pair of brackets declares a block, which has its own scope
// this will prints "in". then "out"
fn main(){
    let x="out";
    {
        let x="in";
        println!("{}",x);
    }
    println!("{}",x);
}

// block are also expressions, which mean they evalute to a value
let x = 42;
let x= {42};

// inside a block, there can be multiple statements
let x = {
    let y=1;
    let z=2;
    y+z // this is the *tail* - what the whole block will evalute to
} ;

// that is why omitting the semicolon at the end of a function is the same as returning
fn fair_dice_roll()->i32{
    return 4;
    // is the same as
    // 4
}

// if conditionals are also expressions
fn fair_dice_roll()->i32{
    if feeling_lucky{
        666
    }else{
        4
    }
}

// also an expression match
fn fair_dice_rollz()->i32{
    match feeling_lucky{
        true=>5,
        false->4,
    }
}

// dots are typically used to access fields of a value
let a = (10,20);
a.0; // this is 10
let amos = get_some_struct();
amos.nickname;

//or call a method on a value
let nick = "fasterthanlime"
nick.len(); // this is 14

// the double-colon, is similar but if operates on namespace ::
// function: std cmp min
let least = std::cmp::min(3,8); // this is 3

// use directives can be used to "bring in scope"
use std::cmp::min;
let least = min(7,1);

// within directives, curly brackets have another meaning
// they are globs if we want to import both and
use std::cmp::min;
use std::cmp::max;
// this alse works:
use std::cmp::{min,max};

// a willcard() lets you import every symbol from a namespace *
use std::cmp::*;

// type are namespaces too, and methods can be called as regular functions:
let x = "amos".len();
len x = str::len("amos");

// str is a primitive type, but many non-primtive types are also in scope by default
let v= Vec::new();
let v= std::vec::Vec::new(); // exactly the same code, but with the *full* path to Vec

// this works because Rust inserts this at the beginning of every module
use std::prelude::v1::*;

// stucts are declared with the keyword: struct
struct vec{
    x:f64, // 64-bits floating point, aka "double precision"
    y:f64,
}

// they can be initialized using struct literals
let v1 = Vec2{x:1.0,y:2.0};
let v2 = Vec2(y:2.0,x:4.0);// the order dose not matter, only the names do

//there is a shortcut for initializing the rest of the fields from another struct
let v3 = Vec2{
    x:14.0
    ..v2  //called "struct update syntax", can only happen in last position
};        //and cannot be followed by a comma

// note that the rest of the fields can mean all the fields
let v4 = Vec2{ ..v3};

// structs, like tuples, can be destructured
// just like this is a valid pattern:let
let (left,right) = slice.split_at(middle);

let v = Vec2{ x:3.0, y:6.0};
let Vec2{x,y} = v;
let Vec2{x,..} = y; // throw away the v.y

// let patterns can be used an conditions in if
struct Number{
    odd:bool,
    value:i32,
}
fn mmain(){
    let one = Number{ odd: true, value: 1};
    let two = Number{ odd: false , value:2};
    print_number(one);
    print_number(two);
}
fn print_number(n:Number){
    if let Number{odd : true, value}=n{
        println!{"Odd number:{}",value};
    }else if let Number{odd : false, value}=n{
        println!{"Even number:{}",value};
    }
}

// match arms are also patterns, just like: if let
fn print_number(n: Number){
    match n{
        Number{odd: true,value}=>println!("Odd number:{}",value);
        Number(odd: false,value)=>println!("Even number:{}",value);
    }
}

// match must be exhausive: at least one arm needs to match
// if that is hard, can be used as a “catch-all” pattern: _
// if that arm didn't exist, we would get a compile-time error
fn print_number(n: Number){
    match n.vaule{
        1=>println!("One");
        2=>println!("Two");
        _->println!("{}",n.value);
    }
}

// you can declare methods on your own types:
impl Number{
    fn is_strictly_positive(self)->bool{
        self.value>0
    }
}

fn main(){
    let minus_two = Number {
        odd: false,
        value: -2,
    };
    println!("positive?{}",minus_two.is_strictly_positive());
}

// variable bindings are immutable by default
// which means their interior can't be mutated
fn main(){
    let n= NUmber{
        odd: true,
        value: 17,
    };
    n.odd = false; // error: cannot assign to 'n.odd'
    // as 'n' is not declared to be mutable
    n = Number{
        odd: false,
        value: 22,
    }; // error: cannot assign twice to immutable variable 'n'
}

// mut makes a variable binding mutable
fn main(){
    let mut n = Number{
        odd: true,
        value:17,
    }
    n.value = 19; // all good
}

// traits are somtehing multiple types can have in common:
trait Signed{
    fn is_strictly_positive(self)->bool;
}


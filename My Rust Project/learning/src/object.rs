struct Number{
    odd:bool,
    value:i32,
}

struct Score{
    sco:i32,
    ok:bool,
    whi:i64,
}

// traits are somtehing multiple types can have in common:
trait Signed{
    fn is_strictly_positive(self)->bool;
}

impl Signed for Number{
    fn is_strictly_positive(self)->bool{
        self.value<0
    }
}

impl Signed for i32{
    fn is_strictly_positive(self)->bool{
        self<0
    }
}

impl Signed for Score{
    fn is_strictly_positive(self)->bool{
        self.whi<0
    }
}

impl std::ops::Neg for Number{
    type Output = Number;
    fn neg(self)->Number{
        Number{value:-self.value,odd:self.odd,}
    }
}

fn print_i32(x:i32){
    println!("x={}",x);
}

fn print_number(n: &Number){
    println!("{} number is {}",if n.odd{"odd"}else{"even"},n.value);
}

fn invert(x: &mut Number){
    x.value=-x.value;
}

fn main(){
    let n = Number{odd: false, value: -44};
    println!("{}",n.is_strictly_positive());
    let x:i32 = -44;
    println!("{}",x.is_strictly_positive());
    let z = Score{sco: 5, ok: true, whi: 50};
    println!("{}",z.is_strictly_positive());

    let nn = Number{odd:true,value:987};
    let mm = -nn; // this is only possible because we implemented 'Neg'
    println!("{}",mm.value);

    let a: i32 = 15;
    let b=a; //'a' is copied
    let c=a; //'a' is copied again
    print_i32(a); // 'a' is copied
    // but struct is not, so this doesn't work:Number Copy
    // let m=nn as nn have been used , it is moved ,then error
    // but it works if takes an immutable reference instead: print_number
    print_number(&mm); //'mm' is borrowed for the time of the call
    print_number(&mm); //'mm' is borrowed again
    // it also works if a function takes a mutable reference
    // but our variable binding is alse .mut
    let mut k=Number{odd:true,value:53};
    invert(&mut k);
    print_number(&k);
}
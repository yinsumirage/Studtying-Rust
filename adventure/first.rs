// learning from https://tourofrust.com

const PI: f32 = 3.1415926; // 命名全大写

fn add(x: i32 , y:i32)-> i32{
    return x+y
}

fn swap(x:i32, y:i32)-> (i32,i32){
    return (y,x);
}

fn make_nothing()->(){
    return (); // 空的元组
}



fn main(){
    let a = 13u8;
    let b = 7u32;
    let c = a as u32 + b;
    println!("{}",c);
    let t = true;
    println!("{},{}",t,t as u32); // 使用as转换类型

    println!("I know that the number of pai is {}",PI);

    let nums: [i32;3] = [1,2,3];
    // 数组的数据类型是 [ T ; N ] 其中T是元素类型，N是编译时已知固定长度
    println!("{:?},{}",nums,nums[1]);
    println!("{}",add(42,24));

    let result = (123,321);
    let resu= swap(123,321);
    println!("two number of resu {},{}",resu.0,resu.1);
    println!("two number of result {},{}",result.0,result.1);
    // 元组解构
    let (d,e) = swap(10,20);
    println!("{},{}",d,e);

    let g = make_nothing();
    println!("the value of g: {:?}",g);
}
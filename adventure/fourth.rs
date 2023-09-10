struct BagOfHolding<T>{
    item:T,
}
// 泛型允许我们不完全定义一个struct 或 enum
// 使得编译器能够根据我们代码使用情况，使编译时创建一个完全定义的版本
// ::<T>操作符来显式地操作，被称作turbofish(它是我对好朋友)

// rust 没有 null
// 这种为一个或多个替代值提供 None 替代的模式非常常见
enum Item{
    Inventory(String),
    None,
}

struct Bag {
    item:Item,
}

// Option Rust有一个内置的泛型枚举叫做Option
// 它可以让我们不使用null就可以为空的值
// enum Option<T>{
//     None,
//     Some(T),
// }

struct BB<T>{
    item:Option<T>,
}

// 内置泛型枚举叫Result
// 可以返回一个可能包含错误的值
// 这是编程语言进行错误处理的惯用方法

// enum Result<T,E>{
//     Ok(T),
//     Err(E),
// }

fn do_something_fail(i:i32)->Result<f32,String>{
    if i == 42{
        Ok(13.0)
    } else {
        Err(String::from("this is not the right number"))
    }
}

// 优雅地处理错误
fn do_something_wrong(i:i32)->Result<f32,String>{
    if i==32 {
        Ok(32.0)
    }else {
        Err(String::from("this is not the right thing"))
    }
}

// 主函数不返回值，但可能返回一个错误！
fn main()-> Result<(),String>{
    let i32_bag = BagOfHolding::<i32>{item:42};
    let bool_bag = BagOfHolding::<bool>{item:true};
    let float_bag = BagOfHolding{item:3.14};

    let bag_in_bag = BagOfHolding{ // rust可以推断泛型的类型
        item: BagOfHolding{item: "砰！"},
    };

    println!(
        "{} {} {} {}",
        i32_bag.item,bool_bag.item,float_bag.item,bag_in_bag.item.item
    );

    let i32b=BB::<i32>{item:None};
    if i32b.item.is_none(){
        println!("there's nothing in it");
    }else {
        println!("there's something in it");
    }

    let i32b=BB::<i32>{item:Some(42)};
    if i32b.item.is_some(){
        println!("there's something in it");
    } else {
        println!("there's nothing in it");
    }

    match i32b.item{
        Some(v)=>println!("found {} in bag!",v),
        None=>println!("found nothing"),
    }

    // let result = do_something_fail(12);
    // match result{
    //     Ok(v)=>println!("found {}",v),
    //     Err(e)=>{
    //         return Err(String::from("something went wrong in main"))
    //     }
    // }

    // 很牛的操作符 ？
    // 相当于 match do_something_wrong(){Ok(v)=>v,Err(e)=>return Err(e),}
    let v = do_something_wrong(32)?;
    println!("found {}",v);

    // 快速写些代码
    // Option与Result都有unwrap函数
    // 可以粗暴获取其中的值
    // unwrap会获取内部值，内部为None或Result 会panic！
    let v = do_something_wrong(32).unwrap();

    // my_option.unwrap()
    //等价于
    // match my_option{
    //     Some(v)=>v,
    //     None=>panic!("some erroe message"),
    // }


    // notice we use a unit value inside a Result Ok
    // to represent everything is fine


    // vector vec! 让我可以轻松创建vector
    // vec 有形如 iter()的方法 允许轻松将vector用到for循环里
    let mut i32_vec = Vec::<i32>::new();
    i32_vec.push(1);
    i32_vec.push(2);
    i32_vec.push(3);
    // 聪明的自动检测类型
    let mut float_vec = Vec::new();
    float_vec.push(1.2);
    float_vec.push(2.3);
    float_vec.push(3.4);

    let string_vec = vec![String::from("hello"),String::from("world")];
    for word in string_vec.iter(){
        println!("{}",word);
    }

    Ok(())
}
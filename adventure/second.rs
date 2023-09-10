
fn main(){
    for x in 0..5{ //不包含最后的数字序列
        println!("{}",x);
    }
    for x in -1..=8{ //包含最后
        println!("{}",x);
    }

    let x=9;
    match x {
        0=>{
            println!("find zero");
        }
        1|2=>{
            println!("find one or two");
        }
        3..=9=>{
            println!("find a number between 3 to 9 inclusively");
        }
        matched_num @ 10..=100=>{
            println!("find {} number between 10 to 100",matched_num);
        }
        _=>{
            println!("find something else");
        }
    }

    let mut k=0;
    loop {
        k+=1;
        if k==10{
            // break "find k" 这个在网站里就能跑？这里跑不了
            println!("{}",k);
            break;
        }
    } // loop无限循环

    let v = loop{
        k+=1;
        if k==20{
            println!("find 20");
            break;
        }
    };
    println!("from loop {:?}",v);

    // if match 函数或作用域块中的最后一条语句是不带;的表达式
    // rust将把它作为一个值从块中返回
    let v = {
        let a = 1;
        let b = 2;
        a+b
    };
    println!("{}",v);
    let x=40;
    let v=if x>40{-1} else {1};
    println!("from if :{}",v);
    let food = "hamburger";
    let result = match food{
        "hamburger"=>"is a hamburger",
        _=>"is not a hamburger",
    };
    println!("from match :{}",result);
    
}
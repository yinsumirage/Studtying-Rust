// rust独特的内存管理范例

static PI :f64 = 3.1415;

struct Foo{
    x: i32,
}

struct Bar{
    y:Foo,
}

fn do_something(f:Foo){
    println!("{}",f.x);
}

fn dodo()->Foo{
    Foo{x:22}
}

fn doadd(f:&mut Foo){
    f.x+=1;
}

fn doreturn(a: &Foo)-> &i32{
    return &a.x;
}

fn main(){
    // 实例化这个结构体并将其绑定到具体的变量上
    // 来创建内存资源 foo即为该资源的所有者
    let foo = Foo{x:42};

    // rust将使用资源最后被使用的位置或一个函数域的结束
    // 作为资源被析构和释放的地方 此处析构和释放的概念被称为drop

    println!("1:{}",foo.x);
    // 这里foo将在这里被dropped 因为后面没用过，也是结尾

    //释放是分级进行的
    let bar = Bar{y: Foo{x:42}};
    println!("2:{}",bar.y.x);
    // bar首先被dropped 紧接着是bar.y
    // rust通过自动释放来帮助确保减少内存泄漏 每个内存资源仅会被释放一次

    // 移交所有权
    // 在移动期间，所有者的堆栈值将会被复制到函数调用的参数堆栈中
    let fo = Foo{x:42};
    // fo 被移交至do_something
    do_something(fo);
    // 此后 fo 便无法被使用

    // 函数归还了 ddo成为了所有者 （如果）在函数结尾会被dropped
    let ddo = dodo();

    // 引用允许我们通过&操作符来借用访问权限
    // 用&mut可以借用对一个资源的可变访问权限
    let d=&ddo;
    println!("3:{}",d.x);

    // 被借用的时候没法改原数值 除非被借用的以及被dropped了
    // 避免出现数据争用 data race
    let mut k = Foo{x:10};
    let f = &mut k;
    // 报错：do_something(f);
    // 报错： k.x=5;
    f.x=5;
    println!("4:{}",f.x);
    // 这个引用dropped了
    k.x=7;
    println!("5:{}",k.x);
    do_something(k);

    // 使用&mut引用时 你可以通过*操作符来修改其指向的值
    // 也可以使用*操作符来对拥有的值进行拷贝
    let mut foo = 60;
    let f = &mut foo;
    let bar = *f; // 取得所有者的拷贝
    *f = 13;      // 设置引用所有者的值
    println!("6:{}",bar);
    println!("7:{}",foo);

    // 1.rust只允许同时存在一个可变引用或者多个不可变引用
    // 不允许可变引用和不可变引用同时存在
    // 2.引用永远不会比它的所有者存活更久
    // 第二天规则避免了通过引用的错误访问，(c语言里的悬垂指针)
    let mut foo = Foo{x:44};
    doadd(&mut foo);
    // 因为所有的可变引用都在doadd里面释放了
    // 此时就可再创一个
    doadd(&mut foo);
    println!("8:{}",foo.x);

    // 引用可以在其他引用上
    let mut foo = Foo{x:99};
    let x = &mut foo.x;
    *x = 50;
    // x 在这个dropped完了啊
    let y = doreturn(&foo);
    println!("9:{}",y);
    // y 被 dropped
    // foo 在这里dropped

    // 函数可以通过一些符号来参数化函数签名
    // 以帮助界定那些参数和返回值共享同一声明周期
    // 生命周期注解总是以’开头

    // 多个生命周期
    // md 不想学这个淦 https://tourofrust.com/54_zh-cn.html

    // 静态生命周期
    // 静态变量在编译期间即被创建并存在于整个程序始末的内存资源
    // 但必须明确指定类型 'static永远不会被drop释放
    // 如果静态生命资源包含了引用，那么这些引用的生命周期一定是'static的

    // unsafe{ } 代码块 可以进行一些无法被编译器担保的内存操作
    static mut SECRET: &'static str = "swordfish";
    let msg: &'static str = "hello";
    let p: &'static f64 = &PI;
    println!("{} {}",msg,p);
    unsafe {
        SECRET = "abcd";
        println!("{}",SECRET);
    }

    // 数据类型中的生命周期
    // rust会验证引用所包含的数据结构永远不会比引用指向的所有者存活周期长
    // 不能运行中拥有一个包括指向虚无的引用存在
    // struct Foo<'a>{
    //     i:&'a i32
    // }
}
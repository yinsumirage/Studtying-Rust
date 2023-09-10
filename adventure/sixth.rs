
// 字符串常量 String Literals
// 采用Unicode编码 (utf-8是其中一部分)
// 字符串常量类型为 &'static str
// &意味着该变量为对内存中数据的引用 不是&mut说明不能修改
// 'static意味着一直保存到程序结束
// str意味着变量总是指向一串合法的utf-8字节序列

// utf-8
// ASCII编码256个字符不够用
// utf-8使用1-4个字节来表示1个字符，这使得可表示的字符数大大增加
// 使用可变长的字节来表示字符优点，就是常见的ASCII编码在utf-8中也无需更多字节
// 但可变的缺点是，utf-8文本通过索引匹配字符无法O(1)了
// 因为要遍历才能得到对应Unicode字符的起始位置 O(n)

// 转义字符
// \n-换行符 \r-回车符 \t-水平制表符(TAB) \\-单个反斜杠 \0-空字符 \'-单引号
// https://doc.rust-lang.org/reference/tokens.html

// 使用 \ 可以使得多行字符串不换行

// 原始字符串常量 支持写入原始的文本而无需为特殊符转义
// 以  r"#开头  "#结尾
// 这样就不会导致可读性变差

// 如果需要使用大量文本，可以尝试用宏 include_str!
// 从本地文件中导入文本到程序中
// let hello = include_str!("hello.html");

// 字符串片段 String Slice
// 是对内存中字节序列的引用，而且这段字节序列必须是合法的utf-8字节序列
// &str的常用方法
// len获取字符串常量字节长度
// starts_with / ends_with 用于基础测试
// is_empty长度为0返回true
// find返回Option<usize>其中usize为匹配到的第一个对应文本的索引值

// 为解决Unicode带来的麻烦
// rust提供了将utf-8字节序列转化为类型char的vector的方法
// 每个char都是4字节，查找快

// String是一个结构体
// 持有以堆heap的形式在内存中储存utf-8字节序列
// 可以延长、修改 
// push_str 在结尾加字符串常量
// replace 用于将一段字符串替换
// to_lowercase / to_uppercase 用于大小写转换
// trim 用于去除字符串前后的空格

// 将文本作为函数参数

fn main(){
    let a: &'static str = "你好 🦀";
    println!("{}   {}",a,a.len());

    let b: &'static str = "Ferris 说 \t\"你好\"";
    println!("{}",b);

    println!("啊?\
    zhende?");

    let c:&'static str = r#"
        <div class="advice">
            原始字符串有时候很有用
        </div>
        "#;
    println!("{}",c);

    
    let fisrt_word = &a[0..6];
    let second_word = &a[7..11];
    println!("{} {}",fisrt_word,second_word);
    // 报错:let half_crab = &a[7..9];
    // Rust 不接受无效 unicode字符构成的片段


    let chars="你好 🦀".chars().collect::<Vec<char>>();
    println!("{}",chars.len());
    println!("{}",chars[3] as u32);


    let mut helloworld = String::from("你好");
    helloworld.push_str(" 世界");
    helloworld+="!";
    println!("{}",helloworld);
}
struct SeaCreature{
    species:Species,
    animal_type:String,
    name:String,
    arms:i32,
    legs:i32,
    weapon:String,
}

struct Location(i32,i32);
// 类元组结构体

struct Market;
// 类单元，没有任何字段

enum Species{
    Carb,
    Octopus,
    Fish,
    Clam
}
// 枚举
// 当使用match对一个enum进行模式匹配时，可以将变量名绑定到每个数据
// rust的enum也被称为标签联合(tagged-union)

enum PoisonType{Acidic,Painful,Lethal}
enum Size{Big,Small}
enum Weapon{
    Claw(i32,Size),
    Poison(PoisonType),
    None
}

fn main(){
    // 静态方法，属于某个类型调用::，创建了一个String实例
    let s = String::from("Hello World");
    // 实例方法，属于某个类型的实例，在使用时用.运算符
    println!("{} is {} characters long.",s,s.len());

    let mirage = SeaCreature{
        species:Species::Clam,
        animal_type: String::from("螃蟹"),
        name:String::from("mirage"),
        arms:2,
        legs:2,
        weapon:String::from("hand"),
    };

    let sarah = SeaCreature{
        species:Species::Clam,
        animal_type: String::from("章鱼"),
        name: String::from("sarah"),
        arms:8,
        legs:0,
        weapon:String::from("触手"),
    };

    println!(
        "{}是个{}，它有{}个胳膊，{}条腿，用{}攻击",
        sarah.name,sarah.animal_type,sarah.arms,sarah.legs,sarah.weapon
    );

    let loc = Location(32,42);
    println!("{},{}",loc.0,loc.1);

    let ferris = SeaCreature{
        species:Species ::Carb,
        animal_type:String::from("啊？"),
        name:String::from("ferris"),
        arms:2,
        legs:4,
        weapon:String::from("claw"),
    };
    match ferris.species{
        Species::Carb=>println!("{} is a crab",ferris.name),
        Species::Octopus=>println!("{} is a octopus",ferris.name),
        Species::Fish=>println!("{} is a fish",ferris.name),
        Species::Clam=>println!("{} is a clam",ferris.name),
    }

    // match ferris.species{
    //     Species::Carb=>{
    //         match ferris.weapon{
    //             Weapon::Claw(num_claw,size)=>{
    //                 let size_desciption=match size{
    //                     Size::Big=>"big",
    //                     Size::Small=>"small"
    //                 };
    //                 println!("ferris is a crab with {} {} claw",num_claw,size_desciption)
    //             },
    //             _=>println!("ferris is a crab with some other weapon")
    //         }
    //     },
    //     _=>println!("ferris is some other animal"),
    // }
    // 这个懒得改了，要把上面的weapon全改成 Weapon::Claw(2,size::Small)
}
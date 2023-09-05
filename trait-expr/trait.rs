pub trait Draw{
    fn draw(&self)->String;
}

pub struct Sharp{
    pub components:Vec<Box<dyn Draw>>,//任意实施了Draw类型
}

impl Sharp{
    pub fn run(&self){
        for component in self.components.iter(){
            component.draw();
        }
    }
}

pub struct Circle{
    pub name :String,
}

impl Draw for Circle{
    fn draw(&self)->String{
        println!("{} drawed",self.name);
        self.name.clone()
    }
}

pub struct Square{
    pub name: String,
}

impl Draw for Square{
    fn draw(&self)->String{
        println!("{} drawed",self.name);
        self.name.clone()
    }
}

fn draw_twice<T: Draw>(sh: T){
    sh.draw();sh.draw();
}

fn main(){
    let sharps = Sharp{
        components:vec![
            Box::new(Circle{
                name:"circle".to_string()
            }),
            Box::new(Square{
                name:"Square".to_string()
            }),
        ],
    };
    sharps.run();
    let cir=Circle{name:"Cirlce".to_string()};
    draw_twice(cir);
}
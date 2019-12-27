use std::collections::HashMap;
use std::fmt::Display;
#[derive(Debug)]
struct values<T,U> {
    a:T,
    b:U,
}
impl<T,U>values<T,U> {
    fn myvalue(a:T,b:U)-> Self{
        Self{
            a,
            b,
        }
    }
}

impl<T :Display+PartialOrd>values<T,T>{
    fn cmpDisplay(&self){
        if self.a >= self.b{
            println!("A {} is Greather Than B {} ",self.a,self.b);
        }
        else {
            println!("B {} is Greather Than A {} ",self.b,self.a);
        }
    }

}


fn main() {
    let mut x = values::myvalue(40,21);

    let mut h1 = HashMap::new();
    h1.insert(20,42);

    let mut h2 = HashMap::new();
    h2.insert(50,32);


    let value01 = values::myvalue(h1,h2);
    println!("{:#?}",value01);
    x.cmpDisplay();


}

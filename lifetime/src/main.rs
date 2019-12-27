// fn main() {
//     let mut name;
//     {
//         let full_name = "faizan".to_string();
//         name = &full_name;
//     }
//     println!("{:?}",name)
// }

// fn main(){
//     let mut s1 = "faizan";
//     let mut s2 = String::from("rehan ali");
//     println!("{:#?}",longest(s1,s2.as_str()));
// }

// fn longest<'a>(x:&'a str,y:&'a str) ->&'a str{
//     if x.len() > y.len(){
//         x
//     }
//     else {
//         y
//     }
// }


// 
// fn main(){
    // let s1 = String::from("Pakistan zindabad");
    // {
    // let s2 = String::from("We Loved Pakistan");
    // let result = longest_fun(s1.as_str(),s2.as_str());
    // println!("{:#?}",result);
    // }
    // 
// }
// 
// fn longest_fun<'a>(value1:&'a str,value2:&'a str) ->&'a str{
    // if value1.len() > value2.len(){
        // value1
    // }
    // else {
        // value2
    // }
// }

        //////////////////Life Time With Struct///////////////

 
// #[derive(Debug)]
// struct point01 <'a,'b> {
//     value1:& 'a str,
//     value2:& 'b str
// }        

// #[derive(Debug)]
// struct point02 < 'c , 'd> {
//     value3:& 'c str,
//     value4:& 'd str
// }

// impl <'a,'b> point01 < 'a,'b>{
//     fn new(&self)-> u32{
//         300
        
//     }
//     fn anounce_return_point01(&self,anouncement01:&str)-> &str{
//         println!("{:?}",anouncement01);
//         self.value1
//     }

//     fn anounce_return_point02(&self,anouncement02:&str)-> &str{
//         println!("{:?}",anouncement02);
//         self.value2
//     }
// }

// impl<'c,'d>point02<'c,'d>{
//     fn news(&self)-> u32{
//         821
//     }
//     fn anounce_return_point01(&self,anouncement03:&str)-> &str{
//         println!("{:?}",anouncement03);
//         self.value3
//     }

//     fn anounce_return_point02(&self,anouncement04:&str)-> &str{
//         println!("{:?}",anouncement04);
//         self.value4
//     }
// }


// fn main(){
//     let mut first_para = String::from("Today is the happy day,of my life & me so happy now today!!!hurrah my Januu is Coming.What are you doing now in class room/lab");
//     let mut first_sen = first_para.split('&').next().expect("Failed to read line");
//     let mut second_sen = first_para.split(',').next().expect("Failed to read line");
//     let mut third_sen = first_para.split("!").next().expect("Failed to read Line");
//     let mut forth_sen = first_para.split("/").next().expect("Failed to read Line");

    
//     let a = point01{
//         value1:first_sen,
//         value2:second_sen,
//     };
//     println!("{:#?}",a.new());
//     println!("{:#?}",a.anounce_return_point01("This is The anouncement01"));
//     println!("{:#?}",a.anounce_return_point02("This is The anouncement02"));

//     let b = point02{
//         value3:third_sen,
//         value4:forth_sen,
//     };
//     println!("{:#?}",b.news());

//     println!("{:#?}",b.anounce_return_point01("This is The anouncement03"));
//     println!("{:#?}",b.anounce_return_point02("This is The anouncement04"));
    
// }



////////////////////////////////////////////////////LifeTime With Elision//////////////////////////////////////////////

// fn main(){
//     fn myword(item:&str)-> &str{
//         let mut bytes = item.as_bytes();
//         println!("{:#?}",bytes);
//         for (i,&values) in bytes.iter().enumerate(){
//             if values == b' ' {
//                 return &item[0..i]
//             }
//         }
//         &item[..]
//     }
//     println!("{:?}",myword("Faizan Manzoor"));
// }

    //////////////////////////////Generics Type ,Bound Trait,LifeTime//////////////////////////////////////
    
// use std::fmt::Display;
// fn main(){
//     fn LifeTime<'a,T:Display>(x:&'a str ,y:&'a str ,ann:T) ->&'a str{
//         print!("{}",ann);
//         if x.len() > y.len(){
//             x
//         }
//         else {
//             y
//         }
//     }
//     println!("{}",LifeTime("Faizan ali","Faizan Manzoor", 30));
// }

fn main(){

    println!("{:#?}",add_two_param(&mut 30,&40));
}

fn add_two_param<'a,'b>(x:&'a mut i32,y:&'b i32)-> &'a i32{

    *x = *x + y;
    x
}
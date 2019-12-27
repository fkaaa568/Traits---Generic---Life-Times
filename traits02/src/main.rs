

#[derive(Debug)]
struct Mydata {
    Name:String,
    F_Name:String,
    Age:u32,
    Status:String,    
}

#[derive(Debug)]
struct Adata {
    Name:String,
    
    F_Name:String,
    Age:i32,
    Status:String,    
}

#[derive(Debug)]
struct Phone {
       phone:u64,
}

pub trait information {
    fn myinfo(&self)-> String{
        format!("Read More!!!!!")
    }
}

impl information for Mydata{
    fn myinfo(&self)-> String{
        let mut result_01 = format!("My name is :{}\n My Father Name is :{}\n My age is :{}\n My Status  is :{}\n",self.Name,self.F_Name,
        self.Age,self.Status);
        result_01
    }
}

impl information for Adata{
    fn myinfo(&self)-> String{
        let mut result_02 = format!("My Elder Brother name is :{}\n & Father Name is :{}\n & her age is :{}\n & her Status  is :{}\n",self.Name,self.F_Name,
        self.Age,self.Status);
        result_02
    }
}

impl information for Phone {
    fn myinfo(&self)-> String{
    let result_03 = format!("My phone Number is {}",self.phone);
    result_03
    }
}

//fn notice<T:information>(item:T)-> String{
//    format!("{}",item.myinfo())
//}

//fn notice(item1:impl information,item2:impl information,item3:impl information)-> String{
//    format!("{},{},{}",item1.myinfo(),item2.myinfo(),item3.myinfo())
//}
fn notice<T:information,U:information,V:information>(item1:T,item2:U,item3:V)-> String{
    format!("{},{}",item1.myinfo(),item2.myinfo())
}

fn main() {
    let mut faizan = Mydata{
        Name:"Faizan Manzoor".to_string(),
        F_Name:"Manzoor Ali".to_string(),
        Age:20,
        Status:"Single".to_string(),
};

// let mut faizan1 = Mydata{
//     Name:"Faizan Manzoor".to_string(),
//     F_Name:"Manzoor Ali".to_string(),
//     Age:20,
//     Status:"Single".to_string(),
// };

let mut arsalan = Adata{
    Name:"Arslan Manzoor".to_string(),
    F_Name:"Manzoor Ali".to_string(),
    Age:22,
    Status:"Got Engaged".to_string(),
};

let Number = Phone{
    phone:03121254699,
};



println!("{:#?}",notice(faizan,arsalan,Number));
//println!("{:#?}",notice(arsalan));
//println!("{:#?}",Number.myinfo());

}

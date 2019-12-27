
#[derive(Debug)]
enum License_type {
    HTV,
    LTV,
}
#[derive(Debug)]
struct Driver {
    Driver_Name:String,
    License:u32,
    License_type:String,
}



pub trait Drivers {
    fn Driver_info(&self)-> String;
}

impl Drivers for Driver {
    fn Driver_info(&self)-> String{
        format!("The Driver Name is : {} & And The License No is :{} And license Type of Driver is :{}",self.Driver_Name,self.License,self.License_type)
    }
}



fn main() {
    let car_driver = Driver{
        Driver_Name:"Hunain".to_string(),
        License:442255221,
        License_type:"HTV".to_string()
    };
    println!("Driver Information {:#?}",car_driver.Driver_info())
}

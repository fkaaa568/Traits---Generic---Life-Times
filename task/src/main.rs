
use std::ops::Add;
use std::ops::Div;
use std::cmp::PartialOrd;

fn Average_num<T>(list:&[T])-> T
    where T : Add<Output=T> + Div<Output=T> + Copy +Clone+From<i32>
    
    {
        let mut sum = list[0];
        let count = T::from(list.len()as i32);
        for x in 0..list.len(){
            sum = sum+list[x];
        }
        sum/count
    }
    


fn main() {
        let mut number_list = vec![30,85,56,96,74,50,56];
        let result = Average_num(&number_list);
        print!("The Number list Average is : {:#?}",result );

        let mut float_list = vec![30.2,85.6,56.9,96.7,74.0,50.7,56.1];
        let result_02 = Average_num(&float_list);
        print!("The Number list Average is : {:#?}",result_02 );

    let arrInt = [3,7,8,9,32,52,69,4,55,15,65,25,85,51];
    let count = arrInt.len();
    let mut sum = 0;
    for i in 0..count{

        sum+=arrInt[i];
    }
    println!("The Average is {:?} ",sum/count);

    println!("The Averge is that : {:?} ",myaverage(&arrInt));
}

fn myaverage(arrInt:&[usize ])-> usize{
    let count = arrInt.len();
    let mut sum = 0;
    for i in 0..count{

        sum+=arrInt[i];
    }
    sum/count
}
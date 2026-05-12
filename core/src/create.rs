#[macro_export]
macro_rules! print{
    ( $x:expr ) =>{
        println!("{}",$x);
    }
}

fn main(){
    println!("hi");
    print!(1);
    println!("{}", 1);
}


fn main() {
    println!("Hello, world!");
    greet();
    println!("{}",basic_data_type(3,5));
    let mut s1 = String::from("hello");
    s1.push_str(",World!");
    println!("{}",s1);
  
}
fn greet(){
    let chinese = "你好啊";
    let english = "Hello world";
    let region=[chinese,english];
    for elem in region.iter() {
        println!("{}",elem);
    }
}
fn basic_data_type(x:i32,y:i32)->i32{
    let res = x+y;
    res
}
fn take_ownership(_v: Vec<i32>) {
    println!("Take the ownership");
}

fn main() {
    
 let _u = vec![1,2,3];
 let _v = _u;
 
 println!("Initial Value is {}", _v[0]);
 take_ownership(_v);
 

}

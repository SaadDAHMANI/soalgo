
include!("soa.rs");

fn main() {
    println!("Seagull Optimization Algorithm (SOA)"); 
    
    let result = soa();

    println!("Result = {}", result);

}

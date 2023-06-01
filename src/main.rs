mod stocks;

fn main() {
    println!("{:?}", stocks::Stock::create_random());
    println!("{:?}", stocks::Stock::create(String::from("THE")));
}

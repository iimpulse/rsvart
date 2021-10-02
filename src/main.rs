use granchio::Bound;

fn main() {
    let x = Bound::OPEN;
    let y = x.is_closed();
    println!("{}", y);
}

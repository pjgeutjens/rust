const STARTING_MISSILES: i32 = 8;
const READY_AMOUNT: i32 = 2;

fn main() {
    let (ready, missiles): (i32, i32) = (READY_AMOUNT, STARTING_MISSILES);
    let _unused = 4;
    // READY_AMOUNT = 1;
    println!("Firing {} of my {} missiles...", ready, missiles);
    // missiles = missiles - ready;
    println!("{} missiles left.", missiles - ready);
}

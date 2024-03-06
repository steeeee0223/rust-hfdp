use rust_hfdp::state::v1::GumballMachine;

fn main() {
    let mut gumball_machine = GumballMachine::new(5);

    println!("{}", gumball_machine);
    gumball_machine.insert_quarter();
    gumball_machine.turn_crank();

    println!("{}", gumball_machine);
    gumball_machine.insert_quarter();
    gumball_machine.eject_quarter();
    gumball_machine.turn_crank();

    println!("{}", gumball_machine);
    gumball_machine.insert_quarter();
    gumball_machine.turn_crank();
    gumball_machine.insert_quarter();
    gumball_machine.turn_crank();
    gumball_machine.eject_quarter();

    println!("{}", gumball_machine);
    gumball_machine.insert_quarter();
    gumball_machine.insert_quarter();
    gumball_machine.turn_crank();
    gumball_machine.insert_quarter();
    gumball_machine.turn_crank();
    gumball_machine.insert_quarter();
    gumball_machine.turn_crank();

    println!("{}", gumball_machine);
}

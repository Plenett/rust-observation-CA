mod elementary_ca;

use elementary_ca::{Boundary, CellularAutomaton};
use arrayfire::{Array, Dim4};

fn main() {
    let ca = CellularAutomaton::new(184, Boundary::Fixed(1,0));
    let state = Array::<u8>::new(&[1, 0, 0, 1, 0], Dim4::new(&[5, 1, 1, 1]));

    let state = ca.step(&state);
    let state = ca.step(&state);

    let mut v_state = vec!(u8::default();state.elements());
    state.host(&mut v_state);

    println!("State : {:?}", v_state);
}

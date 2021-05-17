mod elementary_ca;

use elementary_ca::{Boundary, CellularAutomaton};
use arrayfire::{Array, Dim4, sparse, SparseFormat, sparse_to_dense};
use std::time::Instant;

fn main() {
    let ca = CellularAutomaton::new(184, Boundary::Periodic);
    let state = Array::<u8>::new(&[1, 0, 0], Dim4::new(&[3, 1, 1, 1]));

    let state = ca.step(&state);
    let state = ca.step(&state);

    let mut v_state = vec!(u8::default();state.elements());
    state.host(&mut v_state);

    println!("State : {:?}", v_state);

    // Sparse matrix
    let now = Instant::now();
    let sparse = ca.get_transition_matrix(14);
    
    println!("Time elapsed : {}", now.elapsed().as_millis());

    /*let dense = sparse_to_dense(&sparse);
    let mut sparse_as_vec = vec!(f32::default();dense.elements());
    dense.host(&mut sparse_as_vec);

    println!("State : {:?}", sparse_as_vec);*/
}

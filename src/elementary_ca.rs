use arrayfire::{Array, Dim4, convolve1, ConvDomain, ConvMode, shiftr, modulo, sparse, SparseFormat};

pub enum Boundary {
    Periodic,
    Reflexive,
    Fixed(u8,u8),
} 

impl Boundary {
    pub fn get_boundary_value(&self, state: &Array<u8>) -> (u8,u8) {
        let mut v_state = vec!(u8::default();state.elements());
        state.host(&mut v_state);
        
        match self {
            Boundary::Periodic => (v_state[v_state.len()-1], v_state[0]),
            Boundary::Reflexive => (v_state[0], v_state[v_state.len()-1]),
            Boundary::Fixed(l,r) => (*l,*r),
        }
    }
}

pub struct CellularAutomaton {
    rule: u8,
    boundary: Boundary,
}

impl CellularAutomaton {
    pub fn new(rule: u8, boundary: Boundary) -> Self {
        Self {
            rule,
            boundary,
        }
    }

    pub fn step(&self, state: &Array<u8>) -> Array<u8>{
        let boundary = self.boundary.get_boundary_value(state);

        // Append boundaries to state
        let mut v_state = vec!(u8::default();state.elements());
        state.host(&mut v_state);

        v_state.push(boundary.1);
        v_state.insert(0, boundary.0);

        let signal = Array::<u8>::new(&v_state, Dim4::new(&[v_state.len() as u64, 1, 1, 1]));
        let filter = Array::<u8>::new(&[4,2,1], Dim4::new(&[3, 1, 1, 1]));

        // Convolve 1D
        let signal = convolve1(&signal, &filter, ConvMode::DEFAULT, ConvDomain::AUTO);

        // Remove boundaries from state
        let mut v_signal = vec!(u8::default();signal.elements());
        signal.host(&mut v_signal);
        v_signal.pop();
        v_signal.remove(0);
        let signal = Array::<u8>::new(&v_signal, Dim4::new(&[v_signal.len() as u64, 1, 1, 1]));

        // Compute with rule
        let mut rule = Vec::<u8>::new();
        rule.resize(signal.elements(), self.rule);
        let rule = Array::<u8>::new(&rule, Dim4::new(&[rule.len() as u64, 1, 1, 1]));

        let mut two = Vec::<u8>::new();
        two.resize(signal.elements(), 2);
        let two = Array::<u8>::new(&two, Dim4::new(&[two.len() as u64, 1, 1, 1]));

        let signal = shiftr(&rule, &signal, true);
        let signal = modulo(&signal, &two, true);

        return signal;
    }

    pub fn get_transition_matrix(&self, size: usize) -> Array<f32> {
        
        let n = u64::pow(2, size as u32);
        
        // Compute
        let mut v_col = vec!(0; n as usize);
        for s0 in 0..n as u32{
            let bits = u32_to_bits(s0, size);
            let bits = self.step(&bits);
            let s1 = bits_to_u32(&bits);
            v_col[s0 as usize] = s1 as i32;
        }

        let col = Array::<i32>::new(&v_col, Dim4::new(&[n , 1, 1, 1]));

        let v_val = vec!(1.0 as f32; n as usize);
        let val = Array::<f32>::new(&v_val, Dim4::new(&[n, 1, 1, 1]));

        let v_row: Vec<i32> = (0..=n as i32).collect();
        let row = Array::<i32>::new(&v_row, Dim4::new(&[n+1, 1, 1, 1]));        

        sparse(n, n, &val, &row, &col, SparseFormat::CSR)
    }
}

fn u32_to_bits(val: u32, size: usize) -> Array<u8> {
    
    let mut v_bits = vec!(0; size);
    let mut n = val;

    for i in 0..size {
        
        v_bits[i] = (n%2) as u8;
        n = n/2;

        if n == 0 {
            break;
        }
    }

    Array::<u8>::new(&v_bits, Dim4::new(&[size as u64, 1, 1, 1]))
}

fn bits_to_u32(bits: &Array<u8>) -> u32 {
    let mut v_bits = vec!(u8::default();bits.elements());
    bits.host(&mut v_bits);

    v_bits.iter().rev().fold(0, |acc, &b| acc*2 + b as u32)
}

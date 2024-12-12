use crate::gate::{
    and, and_16, dmux_8_way, mux, mux_16, mux_8_way_16, not, not_16, or, or_8_way, xor,
};

pub fn half_adder(a: bool, b: bool) -> (bool, bool) {
    (xor(a, b), and(a, b))
}

pub fn full_adder(a: bool, b: bool, c: bool) -> (bool, bool) {
    let (sum0, carry0) = half_adder(a, b);
    let (sum, carry1) = half_adder(sum0, c);
    (sum, or(carry0, carry1))
}

pub fn add_16(a: [bool; 16], b: [bool; 16]) -> [bool; 16] {
    let mut carry = false;
    let mut out = [false; 16];
    for (idx, (a, b)) in a.into_iter().zip(b.into_iter()).enumerate() {
        let (sum, carry0) = full_adder(a, b, carry);
        out[idx] = sum;
        carry = carry0
    }
    out
}

pub fn inc_16(input: [bool; 16]) -> [bool; 16] {
    let mut carry = true;
    let mut out = [false; 16];

    for (idx, a) in input.into_iter().enumerate() {
        let (sum, carry0) = half_adder(a, carry);
        out[idx] = sum;
        carry = carry0;
    }
    out
}

#[allow(clippy::too_many_arguments)]
pub fn alu(
    x: [bool; 16],
    y: [bool; 16],
    zx: bool,
    nx: bool,
    zy: bool,
    ny: bool,
    f: bool,
    no: bool,
) -> ([bool; 16], bool, bool) {
    let x0 = mux_16(x, [false; 16], zx);
    let y0 = mux_16(y, [false; 16], zy);

    let not_x = not_16(x0);
    let x1 = mux_16(x0, not_x, nx);

    let not_y = not_16(y0);
    let y1 = mux_16(y0, not_y, ny);

    let add_x_y = add_16(x1, y1);
    let and_x_y = and_16(x1, y1);
    let out_0 = mux_16(and_x_y, add_x_y, f);
    let out_no = not_16(out_0);

    let out = mux_16(out_0, out_no, no);
    let mut left_half = [false; 8];
    let mut right_half = [false; 8];

    for (idx, val) in out[0..7].iter().enumerate() {
        left_half[idx] = *val;
    }

    for (idx, val) in out[8..15].iter().enumerate() {
        right_half[idx] = *val;
    }

    let zr_left = or_8_way(left_half);
    let zr_right = or_8_way(right_half);
    let zr = not(or(zr_left, zr_right));

    (out, zr, out[15])
}

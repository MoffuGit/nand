#[cfg(test)]
mod tests;

pub fn nand(a: bool, b: bool) -> bool {
    !(a & b)
}
pub fn not(a: bool) -> bool {
    nand(a, a)
}

pub fn and(a: bool, b: bool) -> bool {
    not(nand(a, b))
}

pub fn or(a: bool, b: bool) -> bool {
    nand(nand(a, a), nand(b, b))
}

pub fn xor(a: bool, b: bool) -> bool {
    or(and(not(a), b), and(a, not(b)))
}

pub fn mux(a: bool, b: bool, sel: bool) -> bool {
    or(and(a, not(sel)), and(b, sel))
}

pub fn dmux(input: bool, sel: bool) -> (bool, bool) {
    (and(input, not(sel)), and(input, sel))
}

pub fn not_16(input: [bool; 16]) -> [bool; 16] {
    input.map(not)
}

pub fn and_16(a: [bool; 16], b: [bool; 16]) -> [bool; 16] {
    let mut out = [false; 16];
    for (idx, (a, b)) in a.iter().zip(b.iter()).enumerate() {
        out[idx] = and(*a, *b)
    }
    out
}

pub fn or_16(a: [bool; 16], b: [bool; 16]) -> [bool; 16] {
    let mut out = [false; 16];
    for (idx, (a, b)) in a.iter().zip(b.iter()).enumerate() {
        out[idx] = or(*a, *b)
    }
    out
}

pub fn mux_16(a: [bool; 16], b: [bool; 16], sel: bool) -> [bool; 16] {
    let mut out = [false; 16];
    for (idx, (a, b)) in a.iter().zip(b.iter()).enumerate() {
        out[idx] = mux(*a, *b, sel)
    }
    out
}

pub fn or_8_way(input: [bool; 8]) -> bool {
    input.iter().fold(false, |acc, a| or(*a, acc))
}

pub fn mux_4_way_16(
    a: [bool; 16],
    b: [bool; 16],
    c: [bool; 16],
    d: [bool; 16],
    sel: [bool; 2],
) -> [bool; 16] {
    mux_16(mux_16(a, b, sel[0]), mux_16(c, d, sel[0]), sel[1])
}

pub fn mux_8_way_16(
    a: ([bool; 16], [bool; 16], [bool; 16], [bool; 16]),
    b: ([bool; 16], [bool; 16], [bool; 16], [bool; 16]),
    sel: [bool; 3],
) -> [bool; 16] {
    mux_16(
        mux_4_way_16(a.0, a.1, a.2, a.3, [sel[0], sel[1]]),
        mux_4_way_16(b.0, b.1, b.2, b.3, [sel[0], sel[1]]),
        sel[2],
    )
}

pub fn dmux_4_way(input: bool, sel: [bool; 2]) -> [bool; 4] {
    let (c0, c1) = dmux(input, sel[1]);
    let (a, b) = dmux(c0, sel[0]);
    let (c, d) = dmux(c1, sel[0]);
    [a, b, c, d]
}

pub fn dmux_8_way(input: bool, sel: [bool; 3]) -> [bool; 8] {
    let (c0, c1) = dmux(input, sel[2]);
    let [a, b, c, d] = dmux_4_way(c0, [sel[0], sel[1]]);
    let [e, f, g, h] = dmux_4_way(c1, [sel[0], sel[1]]);
    [a, b, c, d, e, f, g, h]
}

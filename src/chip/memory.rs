use crate::gate::{dmux, dmux_4_way, dmux_8_way, mux, mux_16, mux_4_way_16, mux_8_way_16, or};

use super::arithmetic::add_16;

#[derive(Copy, Clone)]
pub struct Bit {
    dff: bool,
}

impl Bit {
    pub fn new() -> Self {
        Bit { dff: false }
    }
    pub fn load(&mut self, input: bool, load: bool) -> bool {
        let mout = mux(self.dff, input, load);
        self.dff = mout;
        self.dff
    }
}

#[derive(Copy, Clone)]
pub struct Register {
    bits: [Bit; 16],
}

impl Register {
    pub fn new() -> Self {
        Register {
            bits: [Bit::new(); 16],
        }
    }

    pub fn load(&mut self, input: [bool; 16], load: bool) -> [bool; 16] {
        let mut out = [false; 16];

        for (idx, (bit, input)) in self.bits.iter_mut().zip(input.iter()).enumerate() {
            out[idx] = bit.load(*input, load)
        }

        out
    }
}

#[derive(Copy, Clone)]
pub struct Ram8 {
    registers: [Register; 8],
}

impl Ram8 {
    pub fn new() -> Self {
        Ram8 {
            registers: [Register::new(); 8],
        }
    }

    pub fn load(&mut self, input: [bool; 16], load: bool, address: [bool; 3]) -> [bool; 16] {
        let outs = dmux_8_way(load, address)
            .iter()
            .zip(self.registers.iter_mut())
            .map(|(load, register)| register.load(input, *load))
            .collect::<Vec<_>>();

        mux_8_way_16(
            (outs[0], outs[1], outs[2], outs[3]),
            (outs[4], outs[5], outs[6], outs[7]),
            address,
        )
    }
}

#[derive(Copy, Clone)]
pub struct Ram64 {
    ram8: [Ram8; 8],
}

impl Ram64 {
    pub fn new() -> Self {
        Ram64 {
            ram8: [Ram8::new(); 8],
        }
    }

    pub fn load(&mut self, input: [bool; 16], load: bool, address: [bool; 6]) -> [bool; 16] {
        let mut first_split = [false; 3];
        address[0..3].iter().enumerate().for_each(|(idx, value)| {
            first_split[idx] = *value;
        });
        let mut last_split = [false; 3];
        address[3..].iter().enumerate().for_each(|(idx, value)| {
            last_split[idx] = *value;
        });
        let outs = dmux_8_way(load, last_split)
            .iter()
            .zip(self.ram8.iter_mut())
            .map(|(load, ram8)| ram8.load(input, *load, first_split))
            .collect::<Vec<_>>();

        mux_8_way_16(
            (outs[0], outs[1], outs[2], outs[3]),
            (outs[4], outs[5], outs[6], outs[7]),
            last_split,
        )
    }
}

#[derive(Copy, Clone)]
pub struct Ram512 {
    ram64: [Ram64; 8],
}

impl Ram512 {
    pub fn new() -> Self {
        Ram512 {
            ram64: [Ram64::new(); 8],
        }
    }

    pub fn load(&mut self, input: [bool; 16], load: bool, address: [bool; 9]) -> [bool; 16] {
        let mut ram64_address = [false; 6];
        address[0..6].iter().enumerate().for_each(|(idx, value)| {
            ram64_address[idx] = *value;
        });

        let mut ram64_select = [false; 3];
        address[6..].iter().enumerate().for_each(|(idx, value)| {
            ram64_select[idx] = *value;
        });

        let load_signals = dmux_8_way(load, ram64_select);

        let outputs = self
            .ram64
            .iter_mut()
            .zip(load_signals.iter())
            .map(|(ram64, &load_signal)| ram64.load(input, load_signal, ram64_address))
            .collect::<Vec<_>>();

        mux_8_way_16(
            (outputs[0], outputs[1], outputs[2], outputs[3]),
            (outputs[4], outputs[5], outputs[6], outputs[7]),
            ram64_select,
        )
    }
}

#[derive(Copy, Clone)]
pub struct Ram4K {
    ram512: [Ram512; 8],
}

impl Ram4K {
    pub fn new() -> Self {
        Ram4K {
            ram512: [Ram512::new(); 8],
        }
    }

    pub fn load(&mut self, input: [bool; 16], load: bool, address: [bool; 12]) -> [bool; 16] {
        let mut ram512_address = [false; 9];
        address[0..9].iter().enumerate().for_each(|(idx, value)| {
            ram512_address[idx] = *value;
        });

        let mut ram64_select = [false; 3];
        address[9..].iter().enumerate().for_each(|(idx, value)| {
            ram64_select[idx] = *value;
        });

        let load_signals = dmux_8_way(load, ram64_select);

        let outputs = self
            .ram512
            .iter_mut()
            .zip(load_signals.iter())
            .map(|(ram512, &load_signal)| ram512.load(input, load_signal, ram512_address))
            .collect::<Vec<_>>();

        mux_8_way_16(
            (outputs[0], outputs[1], outputs[2], outputs[3]),
            (outputs[4], outputs[5], outputs[6], outputs[7]),
            ram64_select,
        )
    }
}

#[derive(Copy, Clone)]
pub struct Ram8K {
    ram4K: [Ram4K; 2],
}

impl Ram8K {
    pub fn new() -> Self {
        Ram8K {
            ram4K: [Ram4K::new(); 2],
        }
    }

    pub fn load(&mut self, input: [bool; 16], load: bool, address: [bool; 13]) -> [bool; 16] {
        let mut ram4k_address = [false; 12];
        address[0..12].iter().enumerate().for_each(|(idx, value)| {
            ram4k_address[idx] = *value;
        });

        let ram4k_select = address[12];
        let load_signals = dmux(load, ram4k_select);
        let output = self
            .ram4K
            .iter_mut()
            .zip(load_signals.iter())
            .map(|(ram4k, &load_signal)| ram4k.load(input, load_signal, ram4k_address))
            .collect::<Vec<_>>();

        mux_16(output[0], output[1], ram4k_select)
    }
}

#[derive(Copy, Clone)]
pub struct Ram16K {
    ram4k: [Ram4K; 4],
}

impl Ram16K {
    pub fn new() -> Self {
        Ram16K {
            ram4k: [Ram4K::new(); 4],
        }
    }

    pub fn load(&mut self, input: [bool; 16], load: bool, address: [bool; 14]) -> [bool; 16] {
        let mut ram4k_address = [false; 12];
        address[0..12].iter().enumerate().for_each(|(idx, value)| {
            ram4k_address[idx] = *value;
        });

        let mut ram4k_select = [false; 2];
        address[12..].iter().enumerate().for_each(|(idx, value)| {
            ram4k_select[idx] = *value;
        });

        let load_signals = dmux_4_way(load, ram4k_select);

        let outputs = self
            .ram4k
            .iter_mut()
            .zip(load_signals.iter())
            .map(|(ram4k, &load_signal)| ram4k.load(input, load_signal, ram4k_address))
            .collect::<Vec<_>>();

        mux_4_way_16(outputs[0], outputs[1], outputs[2], outputs[3], ram4k_select)
    }
}

#[derive(Clone, Copy)]
pub struct PC {
    register: Register,
}

impl PC {
    pub fn new() -> Self {
        PC {
            register: Register::new(),
        }
    }

    pub fn load(&mut self, input: [bool; 16], load: bool, inc: bool, reset: bool) -> [bool; 16] {
        let current_out = self.register.load([false; 16], false);
        let added = add_16(
            current_out,
            [
                true, false, false, false, false, false, false, false, false, false, false, false,
                false, false, false, false,
            ],
        );
        let out0 = mux_16(current_out, added, inc);
        let out1 = mux_16(out0, input, load);
        let out2 = mux_16(out1, [false; 16], reset);

        let or0 = or(load, inc);
        let or1 = or(or0, reset);
        self.register.load(out2, or1)
    }
}

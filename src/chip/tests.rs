use crate::chip::arithmetic::*;
use crate::chip::memory::*;

#[test]
fn test_half_adder() {
    struct Test {
        input: (bool, bool),
        expected: (bool, bool),
    }

    let tests = [
        Test {
            input: (false, false),
            expected: (false, false),
        },
        Test {
            input: (false, true),
            expected: (true, false),
        },
        Test {
            input: (true, false),
            expected: (true, false),
        },
        Test {
            input: (true, true),
            expected: (false, true),
        },
    ];

    for Test {
        input: (a, b),
        expected,
    } in tests
    {
        assert_eq!(expected, half_adder(a, b))
    }
}

#[test]
fn test_full_adder() {
    struct Test {
        input: (bool, bool, bool),
        expected: (bool, bool),
    }

    let tests = [
        Test {
            input: (false, false, false),
            expected: (false, false),
        },
        Test {
            input: (true, false, false),
            expected: (true, false),
        },
        Test {
            input: (false, true, false),
            expected: (true, false),
        },
        Test {
            input: (false, false, true),
            expected: (true, false),
        },
        Test {
            input: (true, true, false),
            expected: (false, true),
        },
        Test {
            input: (true, false, true),
            expected: (false, true),
        },
        Test {
            input: (false, true, true),
            expected: (false, true),
        },
        Test {
            input: (true, true, true),
            expected: (true, true),
        },
    ];

    for Test {
        input: (a, b, c),
        expected,
    } in tests
    {
        assert_eq!(expected, full_adder(a, b, c));
    }
}

#[test]
fn test_add_16() {
    struct Test {
        input: ([bool; 16], [bool; 16]),
        expected: [bool; 16],
    }
    let tests = [
        Test {
            input: ([false; 16], [false; 16]),
            expected: [false; 16],
        },
        Test {
            input: ([true; 16], [false; 16]),
            expected: [true; 16],
        },
        Test {
            input: ([true; 16], [true; 16]),
            expected: [
                false, true, true, true, true, true, true, true, true, true, true, true, true,
                true, true, true,
            ],
        },
        Test {
            input: (
                [
                    true, false, true, false, true, false, true, false, true, false, true, false,
                    true, false, true, false,
                ],
                [
                    false, true, false, true, false, true, false, true, false, true, false, true,
                    false, true, false, true,
                ],
            ),
            expected: [true; 16],
        },
    ];

    for Test {
        input: (a, b),
        expected,
    } in tests
    {
        assert_eq!(expected, add_16(a, b));
    }
}

#[test]
fn test_inc_16() {
    struct Test {
        input: [bool; 16],
        expected: [bool; 16],
    }
    let tests = [
        Test {
            input: [false; 16],
            expected: [
                true, false, false, false, false, false, false, false, false, false, false, false,
                false, false, false, false,
            ],
        },
        Test {
            input: [true; 16],
            expected: [false; 16],
        },
        Test {
            input: [
                true, false, true, false, false, false, false, false, false, false, false, false,
                false, false, false, false,
            ],
            expected: [
                false, true, true, false, false, false, false, false, false, false, false, false,
                false, false, false, false,
            ],
        },
    ];

    for Test { input, expected } in tests {
        assert_eq!(expected, inc_16(input));
    }
}

#[test]
fn test_alu() {
    type ALUInput = ([bool; 16], [bool; 16], bool, bool, bool, bool, bool, bool);
    struct Test {
        input: ALUInput,
        expected: ([bool; 16], bool, bool),
    }

    let tests = [
        Test {
            input: (
                [false; 16],
                [true; 16],
                true,
                false,
                true,
                false,
                true,
                false,
            ),
            expected: ([false; 16], true, false),
        },
        Test {
            input: ([false; 16], [true; 16], true, true, true, true, true, true),
            expected: (
                [
                    true, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false, false,
                ],
                false,
                false,
            ),
        },
        Test {
            input: (
                [false; 16],
                [true; 16],
                true,
                true,
                true,
                false,
                true,
                false,
            ),
            expected: (
                [
                    true, true, true, true, true, true, true, true, true, true, true, true, true,
                    true, true, true,
                ],
                false,
                true,
            ),
        },
        Test {
            input: (
                [false; 16],
                [true; 16],
                false,
                false,
                true,
                true,
                false,
                false,
            ),
            expected: (
                [
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false, false,
                ],
                true,
                false,
            ),
        },
        Test {
            input: (
                [false; 16],
                [true; 16],
                true,
                true,
                false,
                false,
                true,
                false,
            ),
            expected: (
                [
                    false, true, true, true, true, true, true, true, true, true, true, true, true,
                    true, true, true,
                ],
                false,
                true,
            ),
        },
        Test {
            input: ([false; 16], [true; 16], false, true, true, true, true, true),
            expected: (
                [
                    true, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false, false,
                ],
                false,
                false,
            ),
        },
    ];

    for Test {
        input: (x, y, zx, nx, zy, ny, f, no),
        expected,
    } in tests
    {
        assert_eq!(expected, alu(x, y, zx, nx, zy, ny, f, no))
    }
}

#[test]
fn test_bit() {
    struct Test {
        input: (bool, bool),
        expected: bool,
    }
    let mut bit = Bit::new();

    let tests = [
        Test {
            input: (false, false),
            expected: false,
        },
        Test {
            input: (true, false),
            expected: false,
        },
        Test {
            input: (true, true),
            expected: true,
        },
        Test {
            input: (false, false),
            expected: true,
        },
        Test {
            input: (false, true),
            expected: false,
        },
    ];

    for Test {
        input: (input, load),
        expected,
    } in tests
    {
        assert_eq!(expected, bit.load(input, load));
    }
}

#[test]
fn test_register() {
    struct Test {
        input: ([bool; 16], bool),
        expected: [bool; 16],
    }
    let mut register = Register::new();

    let tests = [
        Test {
            input: ([true; 16], false),
            expected: [false; 16],
        },
        Test {
            input: ([true; 16], true),
            expected: [true; 16],
        },
        Test {
            input: ([false; 16], false),
            expected: [true; 16],
        },
        Test {
            input: ([false; 16], true),
            expected: [false; 16],
        },
        Test {
            input: (
                [
                    true, false, true, false, true, false, true, false, true, false, true, false,
                    true, false, true, false,
                ],
                false,
            ),
            expected: [false; 16],
        },
        Test {
            input: (
                [
                    true, false, true, false, true, false, true, false, true, false, true, false,
                    true, false, true, false,
                ],
                true,
            ),
            expected: [
                true, false, true, false, true, false, true, false, true, false, true, false, true,
                false, true, false,
            ],
        },
    ];

    for Test {
        input: (input, load),
        expected,
    } in tests
    {
        assert_eq!(expected, register.load(input, load));
    }
}

#[test]
fn test_ram8() {
    struct Test {
        input: ([bool; 16], bool, [bool; 3]),
        expected: [bool; 16],
    }
    let mut ram_8 = Ram8::new();

    let tests = [
        Test {
            input: ([false; 16], false, [false; 3]),
            expected: [false; 16],
        },
        Test {
            input: ([true; 16], false, [false; 3]),
            expected: [false; 16],
        },
        Test {
            input: ([true; 16], false, [false; 3]),
            expected: [false; 16],
        },
        Test {
            input: ([true; 16], true, [false; 3]),
            expected: [true; 16],
        },
        Test {
            input: ([false; 16], false, [false; 3]),
            expected: [true; 16],
        },
        Test {
            input: ([false; 16], false, [true, false, false]),
            expected: [false; 16],
        },
        Test {
            input: ([true; 16], false, [true, false, false]),
            expected: [false; 16],
        },
        Test {
            input: ([true; 16], false, [true, false, false]),
            expected: [false; 16],
        },
        Test {
            input: ([true; 16], true, [true, false, false]),
            expected: [true; 16],
        },
        Test {
            input: ([false; 16], false, [true, false, false]),
            expected: [true; 16],
        },
        Test {
            input: ([false; 16], false, [false, true, false]),
            expected: [false; 16],
        },
        Test {
            input: ([true; 16], false, [false, true, false]),
            expected: [false; 16],
        },
        Test {
            input: ([true; 16], false, [false, true, false]),
            expected: [false; 16],
        },
        Test {
            input: ([true; 16], true, [false, true, false]),
            expected: [true; 16],
        },
        Test {
            input: ([false; 16], false, [false, true, false]),
            expected: [true; 16],
        },
    ];

    for Test {
        input: (input, load, addresse),
        expected,
    } in tests
    {
        assert_eq!(expected, ram_8.load(input, load, addresse))
    }
}

#[test]
fn test_ram64_basic_write_read() {
    let mut ram = Ram64::new();
    let input = [true; 16]; // all ones
    let address = [false; 6]; // address 0

    // Write to address 0
    ram.load(input, true, address);

    // Read from address 0
    let output = ram.load(input, false, address);
    assert_eq!(output, input);
}

#[test]
fn test_ram64_multiple_addresses() {
    let mut ram = Ram64::new();
    let data1 = [true; 16]; // all ones
    let data2 = [false; 16]; // all zeros

    // Write to address 0
    let addr0 = [false; 6];
    ram.load(data1, true, addr0);

    // Write to address 63 (last address)
    let addr63 = [true; 6];
    ram.load(data2, true, addr63);

    // Verify data at address 0
    let read1 = ram.load(data2, false, addr0);
    assert_eq!(read1, data1);

    // Verify data at address 63
    let read2 = ram.load(data1, false, addr63);
    assert_eq!(read2, data2);
}

#[test]
fn test_ram64_no_write_when_load_false() {
    let mut ram = Ram64::new();
    let initial_data = [true; 16];
    let new_data = [false; 16];
    let address = [false; 6];

    // Initial write
    ram.load(initial_data, true, address);

    // Attempt to write with load=false
    ram.load(new_data, false, address);

    // Verify data hasn't changed
    let output = ram.load(new_data, false, address);
    assert_eq!(output, initial_data);
}

#[test]
fn test_ram64_different_ram8_sections() {
    let mut ram = Ram64::new();
    let data = [true; 16];

    // Write to first RAM8 (address 0)
    let addr_ram8_0 = [false, false, false, false, false, false];
    ram.load(data, true, addr_ram8_0);

    // Write to second RAM8 (address 8)
    let addr_ram8_1 = [false, false, false, true, false, false];
    ram.load(data, true, addr_ram8_1);

    // Verify both locations
    assert_eq!(ram.load(data, false, addr_ram8_0), data);
    assert_eq!(ram.load(data, false, addr_ram8_1), data);

    // Check a different address in first RAM8 (address 1)
    let addr1 = [true, false, false, false, false, false];
    let read = ram.load(data, false, addr1);
    assert_ne!(read, data); // Should not contain our written data
}

#[test]
fn test_ram512_basic_write_read() {
    let mut ram = Ram512::new();
    let input = [true; 16];
    let address = [false; 9];

    // Write to address 0
    ram.load(input, true, address);

    // Read from address 0
    let output = ram.load(input, false, address);
    assert_eq!(output, input);
}

#[test]
fn test_ram512_multiple_addresses() {
    let mut ram = Ram512::new();
    let data1 = [true; 16];
    let data2 = [false; 16];

    // Write to first address
    let addr0 = [false; 9];
    ram.load(data1, true, addr0);

    // Write to last address (511)
    let addr511 = [true; 9];
    ram.load(data2, true, addr511);

    // Verify data at both addresses
    assert_eq!(ram.load(data2, false, addr0), data1);
    assert_eq!(ram.load(data1, false, addr511), data2);
}

#[test]
fn test_ram512_different_ram64_sections() {
    let mut ram = Ram512::new();
    let data = [true; 16];

    // Write to first Ram64 (address 0)
    let addr_ram64_0 = [false; 9];
    ram.load(data, true, addr_ram64_0);

    // Write to second Ram64 (address 64)
    let mut addr_ram64_1 = [false; 9];
    addr_ram64_1[6] = true;
    ram.load(data, true, addr_ram64_1);

    // Verify both locations
    assert_eq!(ram.load(data, false, addr_ram64_0), data);
    assert_eq!(ram.load(data, false, addr_ram64_1), data);

    // Check a different address in first Ram64
    let mut addr1 = [false; 9];
    addr1[0] = true;
    let read = ram.load(data, false, addr1);
    assert_ne!(read, data);
}

#[test]
fn test_ram4k_basic_write_read() {
    let mut ram = Ram4K::new();
    let input = [true; 16];
    let address = [false; 12]; // 12 bits needed for 4K addresses

    // Write to address 0
    ram.load(input, true, address);

    // Read from address 0
    let output = ram.load(input, false, address);
    assert_eq!(output, input);
}

#[test]
fn test_ram4k_multiple_addresses() {
    let mut ram = Ram4K::new();
    let data1 = [true; 16];
    let data2 = [false; 16];

    // Write to first address (0)
    let addr0 = [false; 12];
    ram.load(data1, true, addr0);

    // Write to last address (4095)
    let addr4095 = [true; 12];
    ram.load(data2, true, addr4095);

    // Write to middle address (2048)
    let mut addr2048 = [false; 12];
    addr2048[11] = true; // Set highest bit
    let data3 = [
        true, false, true, false, true, false, true, false, true, false, true, false, true, false,
        true, false,
    ]; // alternating pattern
    ram.load(data3, true, addr2048);

    // Verify all three locations
    assert_eq!(ram.load(data2, false, addr0), data1);
    assert_eq!(ram.load(data1, false, addr4095), data2);
    assert_eq!(ram.load(data1, false, addr2048), data3);
}

#[test]
fn test_ram4k_no_write_when_load_false() {
    let mut ram = Ram4K::new();
    let initial_data = [true; 16];
    let new_data = [false; 16];
    let address = [false; 12];

    // Initial write
    ram.load(initial_data, true, address);

    // Attempt to write with load=false
    ram.load(new_data, false, address);

    // Verify data hasn't changed
    let output = ram.load(new_data, false, address);
    assert_eq!(output, initial_data);
}

#[test]
fn test_ram4k_different_ram512_sections() {
    let mut ram = Ram4K::new();
    let data = [true; 16];

    // Write to first Ram512 (address 0)
    let addr_ram512_0 = [false; 12];
    ram.load(data, true, addr_ram512_0);

    // Write to second Ram512 (address 512)
    let mut addr_ram512_1 = [false; 12];
    addr_ram512_1[9] = true;
    ram.load(data, true, addr_ram512_1);

    // Write to last Ram512 (address 3584)
    let mut addr_ram512_7 = [false; 12];
    addr_ram512_7[11] = true;
    addr_ram512_7[10] = true;
    addr_ram512_7[9] = true;
    ram.load(data, true, addr_ram512_7);

    // Verify all locations
    assert_eq!(ram.load(data, false, addr_ram512_0), data);
    assert_eq!(ram.load(data, false, addr_ram512_1), data);
    assert_eq!(ram.load(data, false, addr_ram512_7), data);
}

#[test]
fn test_ram4k_address_boundaries() {
    let mut ram = Ram4K::new();
    let data1 = [true; 16];
    let data2 = [false; 16];

    // Write to last address of first Ram512 (511)
    let mut addr511 = [false; 12];
    (0..9).for_each(|i| {
        addr511[i] = true;
    });
    ram.load(data1, true, addr511);

    // Write to first address of second Ram512 (512)
    let mut addr512 = [false; 12];
    addr512[9] = true;
    ram.load(data2, true, addr512);

    // Verify both locations
    assert_eq!(ram.load(data2, false, addr511), data1);
    assert_eq!(ram.load(data1, false, addr512), data2);
}

#[test]
fn test_ram4k_sequential_access() {
    let mut ram = Ram4K::new();
    let mut address = [false; 12];

    // Write sequential patterns to first 16 addresses
    for i in 0..16 {
        let mut data = [false; 16];
        data[i] = true;

        // Convert i to binary and set address
        (0..4).for_each(|j| {
            address[j] = (i & (1 << j)) != 0;
        });

        ram.load(data, true, address);
    }

    // Verify all 16 addresses
    address = [false; 12];
    for i in 0..16 {
        let mut expected = [false; 16];
        expected[i] = true;

        // Convert i to binary and set address
        (0..4).for_each(|j| {
            address[j] = (i & (1 << j)) != 0;
        });

        assert_eq!(ram.load(expected, false, address), expected);
    }
}

#[test]
fn test_ram16k_basic_write_read() {
    let mut ram = Ram16K::new();
    let input = [true; 16];
    let address = [false; 14]; // 14 bits needed for 16K addresses

    // Write to address 0
    ram.load(input, true, address);

    // Read from address 0
    let output = ram.load(input, false, address);
    assert_eq!(output, input);
}

#[test]
fn test_ram16k_multiple_addresses() {
    let mut ram = Ram16K::new();
    let data1 = [true; 16];
    let data2 = [false; 16];

    // Write to first address (0)
    let addr0 = [false; 14];
    ram.load(data1, true, addr0);

    // Write to last address (16383)
    let addr16383 = [true; 14];
    ram.load(data2, true, addr16383);

    // Write to middle address (8192)
    let mut addr8192 = [false; 14];
    addr8192[13] = true;
    let data3 = [
        true, false, true, false, true, false, true, false, true, false, true, false, true, false,
        true, false,
    ];
    ram.load(data3, true, addr8192);

    // Verify all three locations
    assert_eq!(ram.load(data2, false, addr0), data1);
    assert_eq!(ram.load(data1, false, addr16383), data2);
    assert_eq!(ram.load(data1, false, addr8192), data3);
}

#[test]
fn test_ram16k_no_write_when_load_false() {
    let mut ram = Ram16K::new();
    let initial_data = [true; 16];
    let new_data = [false; 16];
    let address = [false; 14];

    // Initial write
    ram.load(initial_data, true, address);

    // Attempt to write with load=false
    ram.load(new_data, false, address);

    // Verify data hasn't changed
    let output = ram.load(new_data, false, address);
    assert_eq!(output, initial_data);
}

#[test]
fn test_ram16k_different_ram4k_sections() {
    let mut ram = Ram16K::new();
    let data = [true; 16];

    // Write to first Ram4K (address 0)
    let addr_ram4k_0 = [false; 14];
    ram.load(data, true, addr_ram4k_0);

    // Write to second Ram4K (address 4096)
    let mut addr_ram4k_1 = [false; 14];
    addr_ram4k_1[12] = true;
    ram.load(data, true, addr_ram4k_1);

    // Write to last Ram4K (address 12288)
    let mut addr_ram4k_3 = [false; 14];
    addr_ram4k_3[13] = true;
    addr_ram4k_3[12] = true;
    ram.load(data, true, addr_ram4k_3);

    // Verify all locations
    assert_eq!(ram.load(data, false, addr_ram4k_0), data);
    assert_eq!(ram.load(data, false, addr_ram4k_1), data);
    assert_eq!(ram.load(data, false, addr_ram4k_3), data);
}

#[test]
fn test_ram16k_address_boundaries() {
    let mut ram = Ram16K::new();
    let data1 = [true; 16];
    let data2 = [false; 16];

    // Write to last address of first Ram4K (4095)
    let mut addr4095 = [false; 14];
    (0..12).for_each(|i| {
        addr4095[i] = true;
    });
    ram.load(data1, true, addr4095);

    // Write to first address of second Ram4K (4096)
    let mut addr4096 = [false; 14];
    addr4096[12] = true;
    ram.load(data2, true, addr4096);

    // Verify both locations
    assert_eq!(ram.load(data2, false, addr4095), data1);
    assert_eq!(ram.load(data1, false, addr4096), data2);
}

#[test]
fn test_ram16k_large_scale_write() {
    let mut ram = Ram16K::new();
    let mut address = [false; 14];

    // Write to every 1024th address
    for i in 0..16 {
        let mut data = [false; 16];
        data[i % 16] = true;

        // Set address to i * 1024
        let addr_value = i * 1024;
        (0..14).for_each(|j| {
            address[j] = (addr_value & (1 << j)) != 0;
        });

        ram.load(data, true, address);
    }

    // Verify all written addresses
    address = [false; 14];
    for i in 0..16 {
        let mut expected = [false; 16];
        expected[i % 16] = true;

        // Set address to i * 1024
        let addr_value = i * 1024;
        (0..14).for_each(|j| {
            address[j] = (addr_value & (1 << j)) != 0;
        });

        assert_eq!(ram.load(expected, false, address), expected);
    }
}

#[test]
fn test_pc_initial_state() {
    let mut pc = PC::new();
    let output = pc.load([false; 16], false, false, false);
    assert_eq!(output, [false; 16]);
}

#[test]
fn test_pc_increment() {
    let mut pc = PC::new();

    // First increment
    let out1 = pc.load([false; 16], false, true, false);
    assert_eq!(
        out1,
        [
            true, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false
        ]
    );

    // Second increment
    let out2 = pc.load([false; 16], false, true, false);
    assert_eq!(
        out2,
        [
            false, true, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false
        ]
    );

    // Third increment
    let out3 = pc.load([false; 16], false, true, false);
    assert_eq!(
        out3,
        [
            true, true, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false
        ]
    );
}

#[test]
fn test_pc_load() {
    let mut pc = PC::new();
    let input = [
        true, true, true, false, false, false, false, false, false, false, false, false, false,
        false, false, false,
    ];

    // Load value
    let out1 = pc.load(input, true, false, false);
    assert_eq!(out1, input);

    // Next cycle without load should maintain value
    let out2 = pc.load([false; 16], false, false, false);
    assert_eq!(out2, input);
}

#[test]
fn test_pc_reset() {
    let mut pc = PC::new();
    let input = [
        true, true, true, false, false, false, false, false, false, false, false, false, false,
        false, false, false,
    ];

    // Load a value
    pc.load(input, true, false, false);

    // Reset
    let out = pc.load([false; 16], false, false, true);
    assert_eq!(out, [false; 16]);
}

#[test]
fn test_pc_load_priority_over_increment() {
    let mut pc = PC::new();
    let input = [
        true, true, true, false, false, false, false, false, false, false, false, false, false,
        false, false, false,
    ];

    // Try to both load and increment
    let out = pc.load(input, true, true, false);

    // Should load the input value, not increment
    assert_eq!(out, input);
}

#[test]
fn test_pc_reset_priority_over_load_and_increment() {
    let mut pc = PC::new();
    let input = [
        true, true, true, false, false, false, false, false, false, false, false, false, false,
        false, false, false,
    ];

    // Try to load, increment, and reset simultaneously
    let out = pc.load(input, true, true, true);

    // Reset should take priority
    assert_eq!(out, [false; 16]);
}

#[test]
fn test_pc_increment_sequence_then_load() {
    let mut pc = PC::new();

    // Increment three times
    pc.load([false; 16], false, true, false);
    pc.load([false; 16], false, true, false);
    pc.load([false; 16], false, true, false);

    // Load new value
    let input = [
        true, false, true, false, false, false, false, false, false, false, false, false, false,
        false, false, false,
    ];
    let out = pc.load(input, true, false, false);

    // Should have loaded new value
    assert_eq!(out, input);

    // Next increment should start from loaded value
    let next = pc.load([false; 16], false, true, false);
    let expected = [
        false, true, true, false, false, false, false, false, false, false, false, false, false,
        false, false, false,
    ];
    assert_eq!(next, expected);
}

#[test]
fn test_pc_no_change_when_all_control_false() {
    let mut pc = PC::new();

    // Load initial value
    let input = [
        true, false, true, false, false, false, false, false, false, false, false, false, false,
        false, false, false,
    ];
    pc.load(input, true, false, false);

    // Next cycle with all controls false
    let out = pc.load([false; 16], false, false, false);

    // Should maintain previous value
    assert_eq!(out, input);
}

#[test]
fn test_ram8k_write_and_read_first_ram4k() {
    let mut ram = Ram8K::new();
    let input = [
        true, false, true, false, true, false, true, false, true, false, true, false, true, false,
        true, false,
    ];
    let address = [false; 13]; // First location in first RAM4K

    // Write to memory
    ram.load(input, true, address);

    // Read from same address (with load=false)
    let result = ram.load([false; 16], false, address);
    assert_eq!(result, input);
}

#[test]
fn test_ram8k_write_and_read_second_ram4k() {
    let mut ram = Ram8K::new();
    let input = [
        true, true, false, false, true, true, false, false, true, true, false, false, true, true,
        false, false,
    ];
    let mut address = [false; 13];
    address[12] = true; // Set highest bit to select second RAM4K

    // Write to memory
    ram.load(input, true, address);

    // Read from same address
    let result = ram.load([false; 16], false, address);
    assert_eq!(result, input);
}

#[test]
fn test_ram8k_no_write_when_load_is_false() {
    let mut ram = Ram8K::new();
    let input = [true; 16];
    let address = [false; 13];

    // Attempt to write with load=false
    ram.load(input, false, address);

    // Read back should still be zeros
    let result = ram.load([false; 16], false, address);
    assert_eq!(result, [false; 16]);
}

#[test]
fn test_ram8k_different_addresses() {
    let mut ram = Ram8K::new();
    let input1 = [true; 16];
    let input2 = [false; 16];

    // Write to first address
    let mut address1 = [false; 13];
    address1[0] = true;
    ram.load(input1, true, address1);

    // Write to second address
    let mut address2 = [false; 13];
    address2[1] = true;
    ram.load(input2, true, address2);

    // Verify both values
    assert_eq!(ram.load([false; 16], false, address1), input1);
    assert_eq!(ram.load([false; 16], false, address2), input2);
}

#[test]
fn test_ram8k_address_boundaries() {
    let mut ram = Ram8K::new();
    let input = [true; 16];

    // Test last address of first RAM4K
    let mut address_end_first = [true; 13];
    address_end_first[12] = false;
    ram.load(input, true, address_end_first);
    assert_eq!(ram.load([false; 16], false, address_end_first), input);

    // Test first address of second RAM4K
    let mut address_start_second = [false; 13];
    address_start_second[12] = true;
    ram.load(input, true, address_start_second);
    assert_eq!(ram.load([false; 16], false, address_start_second), input);
}

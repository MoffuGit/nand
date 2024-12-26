use super::{
    and, and_16, dmux, dmux_4_way, dmux_8_way, mux, mux_16, mux_4_way_16, mux_8_way_16, not,
    not_16, or, or_16, or_8_way, xor,
};

use super::nand;

#[test]
fn test_nand() {
    struct Test {
        input: (bool, bool),
        expected: bool,
    }
    let tests = vec![
        Test {
            input: (false, false),
            expected: true,
        },
        Test {
            input: (false, true),
            expected: true,
        },
        Test {
            input: (true, false),
            expected: true,
        },
        Test {
            input: (true, true),
            expected: false,
        },
    ];

    for Test {
        input: (a, b),
        expected: output,
    } in tests
    {
        assert_eq!(output, nand(a, b))
    }
}

#[test]
fn test_not() {
    struct Test {
        input: bool,
        expected: bool,
    }

    let tests = vec![
        Test {
            input: false,
            expected: true,
        },
        Test {
            input: true,
            expected: false,
        },
    ];

    for Test {
        input,
        expected: output,
    } in tests
    {
        assert_eq!(not(input), output)
    }
}

#[test]
fn test_and() {
    struct Test {
        input: (bool, bool),
        expected: bool,
    }
    let tests = vec![
        Test {
            input: (false, false),
            expected: false,
        },
        Test {
            input: (false, true),
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
    ];

    for Test {
        input: (a, b),
        expected: output,
    } in tests
    {
        assert_eq!(output, and(a, b))
    }
}

#[test]
fn test_or() {
    struct Test {
        input: (bool, bool),
        expected: bool,
    }
    let tests = vec![
        Test {
            input: (false, false),
            expected: false,
        },
        Test {
            input: (false, true),
            expected: true,
        },
        Test {
            input: (true, false),
            expected: true,
        },
        Test {
            input: (true, true),
            expected: true,
        },
    ];

    for Test {
        input: (a, b),
        expected: output,
    } in tests
    {
        assert_eq!(output, or(a, b))
    }
}

#[test]
fn test_xor() {
    struct Test {
        input: (bool, bool),
        expected: bool,
    }
    let tests = vec![
        Test {
            input: (false, false),
            expected: false,
        },
        Test {
            input: (false, true),
            expected: true,
        },
        Test {
            input: (true, false),
            expected: true,
        },
        Test {
            input: (true, true),
            expected: false,
        },
    ];

    for Test {
        input: (a, b),
        expected: output,
    } in tests
    {
        assert_eq!(output, xor(a, b))
    }
}

#[test]
fn test_mux() {
    struct Test {
        input: (bool, bool, bool),
        expected: bool,
    }
    let tests = vec![
        Test {
            input: (false, false, false),
            expected: false,
        },
        Test {
            input: (false, true, false),
            expected: false,
        },
        Test {
            input: (true, false, false),
            expected: true,
        },
        Test {
            input: (true, true, false),
            expected: true,
        },
        Test {
            input: (false, false, true),
            expected: false,
        },
        Test {
            input: (false, true, true),
            expected: true,
        },
        Test {
            input: (true, false, true),
            expected: false,
        },
        Test {
            input: (true, true, true),
            expected: true,
        },
    ];

    for Test {
        input: (a, b, sel),
        expected: output,
    } in tests
    {
        assert_eq!(output, mux(a, b, sel))
    }
}

#[test]
fn test_dmux() {
    struct Test {
        input: (bool, bool),
        expected: [bool; 2],
    }
    let tests = vec![
        Test {
            input: (false, false),
            expected: [false, false],
        },
        Test {
            input: (false, true),
            expected: [false, false],
        },
        Test {
            input: (true, false),
            expected: [true, false],
        },
        Test {
            input: (true, true),
            expected: [false, true],
        },
    ];

    for Test {
        input: (y, sel),
        expected: output,
    } in tests
    {
        assert_eq!(output, dmux(y, sel))
    }
}

#[test]
fn test_not_16() {
    struct Test {
        input: [bool; 16],
        expected: [bool; 16],
    }

    let tests = vec![
        Test {
            input: [false; 16],
            expected: [true; 16],
        },
        Test {
            input: [true; 16],
            expected: [false; 16],
        },
        Test {
            input: [
                true, false, true, false, true, false, true, false, true, false, true, false, true,
                false, true, false,
            ],
            expected: [
                false, true, false, true, false, true, false, true, false, true, false, true,
                false, true, false, true,
            ],
        },
        Test {
            input: [
                false, false, true, true, false, false, true, true, false, false, true, true,
                false, false, true, true,
            ],
            expected: [
                true, true, false, false, true, true, false, false, true, true, false, false, true,
                true, false, false,
            ],
        },
        Test {
            input: [
                false, false, false, true, false, false, true, false, false, false, true, true,
                false, true, false, false,
            ],
            expected: [
                true, true, true, false, true, true, false, true, true, true, false, false, true,
                false, true, true,
            ],
        },
    ];

    for Test { input, expected } in tests {
        assert_eq!(expected, not_16(input))
    }
}

#[test]
fn test_and_16() {
    struct Test {
        input: ([bool; 16], [bool; 16]),
        expected: [bool; 16],
    }

    let tests = vec![
        Test {
            input: ([false; 16], [false; 16]),
            expected: [false; 16],
        },
        Test {
            input: ([false; 16], [true; 16]),
            expected: [false; 16],
        },
        Test {
            input: ([true; 16], [true; 16]),
            expected: [true; 16],
        },
        Test {
            input: (
                [
                    false, true, false, true, false, true, false, true, false, true, false, true,
                    false, true, false, true,
                ],
                [
                    true, false, true, false, true, false, true, false, true, false, true, false,
                    true, false, true, false,
                ],
            ),
            expected: [false; 16],
        },
        Test {
            input: (
                [
                    false, true, false, false, false, true, false, false, false, false, false,
                    false, false, true, false, false,
                ],
                [
                    false, false, false, false, false, true, false, false, false, false, false,
                    false, false, true, false, false,
                ],
            ),
            expected: [
                false, false, false, false, false, true, false, false, false, false, false, false,
                false, true, false, false,
            ],
        },
    ];

    for Test {
        input: (a, b),
        expected,
    } in tests
    {
        assert_eq!(expected, and_16(a, b));
    }
}

#[test]
fn test_or_16() {
    struct Test {
        input: ([bool; 16], [bool; 16]),
        expected: [bool; 16],
    }

    let tests = vec![
        Test {
            input: ([false; 16], [false; 16]),
            expected: [false; 16],
        },
        Test {
            input: ([false; 16], [true; 16]),
            expected: [true; 16],
        },
        Test {
            input: ([true; 16], [true; 16]),
            expected: [true; 16],
        },
        Test {
            input: (
                [
                    false, true, false, true, false, true, false, true, false, true, false, true,
                    false, true, false, true,
                ],
                [
                    true, false, true, false, true, false, true, false, true, false, true, false,
                    true, false, true, false,
                ],
            ),
            expected: [true; 16],
        },
        Test {
            input: (
                [
                    false, true, false, false, false, true, false, false, false, false, false,
                    false, false, true, false, false,
                ],
                [
                    false, false, false, false, false, true, false, false, false, false, false,
                    true, false, true, false, false,
                ],
            ),
            expected: [
                false, true, false, false, false, true, false, false, false, false, false, true,
                false, true, false, false,
            ],
        },
    ];

    for Test {
        input: (a, b),
        expected,
    } in tests
    {
        assert_eq!(expected, or_16(a, b));
    }
}

#[test]
fn test_mux_16() {
    struct Test {
        input: ([bool; 16], [bool; 16], bool),
        expected: [bool; 16],
    }
    let tests = vec![
        Test {
            input: ([false; 16], [false; 16], false),
            expected: [false; 16],
        },
        Test {
            input: ([false; 16], [false; 16], true),
            expected: [false; 16],
        },
        Test {
            input: (
                [false; 16],
                [
                    false, false, false, true, false, false, true, false, false, false, true, true,
                    false, true, false, false,
                ],
                false,
            ),
            expected: [false; 16],
        },
        Test {
            input: (
                [false; 16],
                [
                    false, false, false, true, false, false, true, false, false, false, true, true,
                    false, true, false, false,
                ],
                true,
            ),
            expected: [
                false, false, false, true, false, false, true, false, false, false, true, true,
                false, true, false, false,
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
                false,
            ),
            expected: [
                true, false, true, false, true, false, true, false, true, false, true, false, true,
                false, true, false,
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
                true,
            ),
            expected: [
                false, true, false, true, false, true, false, true, false, true, false, true,
                false, true, false, true,
            ],
        },
    ];

    for Test {
        input: (a, b, sel),
        expected,
    } in tests
    {
        assert_eq!(expected, mux_16(a, b, sel))
    }
}

#[test]
fn test_or_8_way() {
    struct Test {
        input: [bool; 8],
        expected: bool,
    }

    let tests = vec![
        Test {
            input: [false; 8],
            expected: false,
        },
        Test {
            input: [true; 8],
            expected: true,
        },
        Test {
            input: [false, false, false, true, false, false, false, false],
            expected: true,
        },
        Test {
            input: [false, false, false, false, false, false, false, true],
            expected: true,
        },
    ];

    for Test { input, expected } in tests {
        assert_eq!(expected, or_8_way(input))
    }
}

#[test]
fn test_mux_4_way_16() {
    type Input = ([bool; 16], [bool; 16], [bool; 16], [bool; 16], [bool; 2]);
    struct Test {
        input: Input,
        expected: [bool; 16],
    }

    let tests = vec![
        Test {
            input: (
                [false; 16],
                [false; 16],
                [false; 16],
                [false; 16],
                [false, false],
            ),
            expected: [false; 16],
        },
        Test {
            input: (
                [false; 16],
                [false; 16],
                [false; 16],
                [false; 16],
                [true, false],
            ),
            expected: [false; 16],
        },
        Test {
            input: (
                [false; 16],
                [false; 16],
                [false; 16],
                [false; 16],
                [false, true],
            ),
            expected: [false; 16],
        },
        Test {
            input: (
                [false; 16],
                [false; 16],
                [false; 16],
                [false; 16],
                [true, true],
            ),
            expected: [false; 16],
        },
        Test {
            input: (
                [
                    false, false, false, true, false, false, true, false, false, false, true, true,
                    false, true, false, false,
                ],
                [
                    true, false, false, true, true, false, false, false, false, true, true, true,
                    false, true, true, false,
                ],
                [
                    true, false, true, false, true, false, true, false, true, false, true, false,
                    true, false, true, false,
                ],
                [
                    false, true, false, true, false, true, false, true, false, true, false, true,
                    false, true, false, true,
                ],
                [false, false],
            ),
            expected: [
                false, false, false, true, false, false, true, false, false, false, true, true,
                false, true, false, false,
            ],
        },
        Test {
            input: (
                [
                    false, false, false, true, false, false, true, false, false, false, true, true,
                    false, true, false, false,
                ],
                [
                    true, false, false, true, true, false, false, false, false, true, true, true,
                    false, true, true, false,
                ],
                [
                    true, false, true, false, true, false, true, false, true, false, true, false,
                    true, false, true, false,
                ],
                [
                    false, true, false, true, false, true, false, true, false, true, false, true,
                    false, true, false, true,
                ],
                [true, false],
            ),
            expected: [
                true, false, false, true, true, false, false, false, false, true, true, true,
                false, true, true, false,
            ],
        },
        Test {
            input: (
                [
                    false, false, false, true, false, false, true, false, false, false, true, true,
                    false, true, false, false,
                ],
                [
                    true, false, false, true, true, false, false, false, false, true, true, true,
                    false, true, true, false,
                ],
                [
                    true, false, true, false, true, false, true, false, true, false, true, false,
                    true, false, true, false,
                ],
                [
                    false, true, false, true, false, true, false, true, false, true, false, true,
                    false, true, false, true,
                ],
                [false, true],
            ),
            expected: [
                true, false, true, false, true, false, true, false, true, false, true, false, true,
                false, true, false,
            ],
        },
        Test {
            input: (
                [
                    false, false, false, true, false, false, true, false, false, false, true, true,
                    false, true, false, false,
                ],
                [
                    true, false, false, true, true, false, false, false, false, true, true, true,
                    false, true, true, false,
                ],
                [
                    true, false, true, false, true, false, true, false, true, false, true, false,
                    true, false, true, false,
                ],
                [
                    false, true, false, true, false, true, false, true, false, true, false, true,
                    false, true, false, true,
                ],
                [true, true],
            ),
            expected: [
                false, true, false, true, false, true, false, true, false, true, false, true,
                false, true, false, true,
            ],
        },
    ];

    for Test {
        input: (a, b, c, d, sel),
        expected,
    } in tests
    {
        assert_eq!(expected, mux_4_way_16(a, b, c, d, sel))
    }
}

#[test]
fn test_mux_8_way_16() {
    type Input = (
        ([bool; 16], [bool; 16], [bool; 16], [bool; 16]),
        ([bool; 16], [bool; 16], [bool; 16], [bool; 16]),
        [bool; 3],
    );
    struct Test {
        input: Input,
        expected: [bool; 16],
    }

    let tests = [
        Test {
            input: (
                ([false; 16], [false; 16], [false; 16], [false; 16]),
                ([false; 16], [false; 16], [false; 16], [false; 16]),
                [false; 3],
            ),
            expected: [false; 16],
        },
        Test {
            input: (
                ([false; 16], [false; 16], [false; 16], [false; 16]),
                ([false; 16], [false; 16], [false; 16], [false; 16]),
                [true, false, false],
            ),
            expected: [false; 16],
        },
        Test {
            input: (
                ([false; 16], [false; 16], [false; 16], [false; 16]),
                ([false; 16], [false; 16], [false; 16], [false; 16]),
                [false, true, false],
            ),
            expected: [false; 16],
        },
        Test {
            input: (
                (
                    [
                        false, false, false, true, false, false, true, false, false, false, true,
                        true, false, true, false, false,
                    ],
                    [
                        true, false, false, true, true, false, false, false, false, true, true,
                        true, false, true, true, false,
                    ],
                    [
                        true, false, true, false, true, false, true, false, true, false, true,
                        false, true, false, true, false,
                    ],
                    [
                        false, true, false, true, false, true, false, true, false, true, false,
                        true, false, true, false, true,
                    ],
                ),
                (
                    [
                        false, true, false, true, false, true, false, true, false, true, false,
                        true, false, true, false, true,
                    ],
                    [
                        true, false, true, false, true, false, true, false, true, false, true,
                        false, true, false, true, false,
                    ],
                    [
                        true, false, false, true, true, false, false, false, false, true, true,
                        true, false, true, true, false,
                    ],
                    [
                        false, false, false, true, false, false, true, false, false, false, true,
                        true, false, true, false, false,
                    ],
                ),
                [false, false, false],
            ),
            expected: [
                false, false, false, true, false, false, true, false, false, false, true, true,
                false, true, false, false,
            ],
        },
        Test {
            input: (
                (
                    [
                        false, false, false, true, false, false, true, false, false, false, true,
                        true, false, true, false, false,
                    ],
                    [
                        true, false, false, true, true, false, false, false, false, true, true,
                        true, false, true, true, false,
                    ],
                    [
                        true, false, true, false, true, false, true, false, true, false, true,
                        false, true, false, true, false,
                    ],
                    [
                        false, true, false, true, false, true, false, true, false, true, false,
                        true, false, true, false, true,
                    ],
                ),
                (
                    [
                        false, true, false, true, false, true, false, true, false, true, false,
                        true, false, true, false, true,
                    ],
                    [
                        true, false, true, false, true, false, true, false, true, false, true,
                        false, true, false, true, false,
                    ],
                    [
                        true, false, false, true, true, false, false, false, false, true, true,
                        true, false, true, true, false,
                    ],
                    [
                        false, false, false, true, false, false, true, false, false, false, true,
                        true, false, true, false, false,
                    ],
                ),
                [true, false, false],
            ),
            expected: [
                true, false, false, true, true, false, false, false, false, true, true, true,
                false, true, true, false,
            ],
        },
        Test {
            input: (
                (
                    [
                        false, false, false, true, false, false, true, false, false, false, true,
                        true, false, true, false, false,
                    ],
                    [
                        true, false, false, true, true, false, false, false, false, true, true,
                        true, false, true, true, false,
                    ],
                    [
                        true, false, true, false, true, false, true, false, true, false, true,
                        false, true, false, true, false,
                    ],
                    [
                        false, true, false, true, false, true, false, true, false, true, false,
                        true, false, true, false, true,
                    ],
                ),
                (
                    [
                        false, true, false, true, false, true, false, true, false, true, false,
                        true, false, true, false, true,
                    ],
                    [
                        true, false, true, false, true, false, true, false, true, false, true,
                        false, true, false, true, false,
                    ],
                    [
                        true, false, false, true, true, false, false, false, false, true, true,
                        true, false, true, true, false,
                    ],
                    [
                        false, false, false, true, false, false, true, false, false, false, true,
                        true, false, true, false, false,
                    ],
                ),
                [false, true, false],
            ),
            expected: [
                true, false, true, false, true, false, true, false, true, false, true, false, true,
                false, true, false,
            ],
        },
        Test {
            input: (
                (
                    [
                        false, false, false, true, false, false, true, false, false, false, true,
                        true, false, true, false, false,
                    ],
                    [
                        true, false, false, true, true, false, false, false, false, true, true,
                        true, false, true, true, false,
                    ],
                    [
                        true, false, true, false, true, false, true, false, true, false, true,
                        false, true, false, true, false,
                    ],
                    [
                        false, true, false, true, false, true, false, true, false, true, false,
                        true, false, true, false, true,
                    ],
                ),
                (
                    [
                        false, true, false, true, false, true, false, true, false, true, false,
                        true, true, true, false, true,
                    ],
                    [
                        true, false, true, false, true, false, true, false, true, false, true,
                        false, true, false, true, false,
                    ],
                    [
                        true, false, false, true, true, false, false, false, false, true, true,
                        true, false, true, true, false,
                    ],
                    [
                        false, false, false, true, false, false, true, false, false, false, true,
                        true, false, true, false, false,
                    ],
                ),
                [false, false, true],
            ),
            expected: [
                false, true, false, true, false, true, false, true, false, true, false, true, true,
                true, false, true,
            ],
        },
        Test {
            input: (
                (
                    [
                        false, false, false, true, false, false, true, false, false, false, true,
                        true, false, true, false, false,
                    ],
                    [
                        true, false, false, true, true, false, false, false, false, true, true,
                        true, false, true, true, false,
                    ],
                    [
                        true, false, true, false, true, false, true, false, true, false, true,
                        false, true, false, true, false,
                    ],
                    [
                        false, true, false, true, false, true, false, true, false, true, false,
                        true, false, true, false, true,
                    ],
                ),
                (
                    [
                        false, true, false, true, false, true, false, true, false, true, false,
                        true, true, true, false, true,
                    ],
                    [
                        true, true, true, false, true, false, true, false, true, false, true,
                        false, true, false, true, false,
                    ],
                    [
                        true, false, false, true, true, false, false, false, false, true, true,
                        true, false, true, true, false,
                    ],
                    [
                        false, false, false, true, false, false, true, false, false, false, true,
                        true, false, true, false, false,
                    ],
                ),
                [true, false, true],
            ),
            expected: [
                true, true, true, false, true, false, true, false, true, false, true, false, true,
                false, true, false,
            ],
        },
    ];

    for Test {
        input: (a, b, sel),
        expected,
    } in tests
    {
        assert_eq!(expected, mux_8_way_16(a, b, sel))
    }
}

#[test]
fn test_dmux_4_way() {
    struct Test {
        input: (bool, [bool; 2]),
        expected: [bool; 4],
    }

    let tests = [
        Test {
            input: (false, [false; 2]),
            expected: [false; 4],
        },
        Test {
            input: (false, [true, false]),
            expected: [false; 4],
        },
        Test {
            input: (false, [false, true]),
            expected: [false; 4],
        },
        Test {
            input: (false, [true, true]),
            expected: [false; 4],
        },
        Test {
            input: (true, [false; 2]),
            expected: [true, false, false, false],
        },
        Test {
            input: (true, [true, false]),
            expected: [false, true, false, false],
        },
        Test {
            input: (true, [false, true]),
            expected: [false, false, true, false],
        },
        Test {
            input: (true, [true, true]),
            expected: [false, false, false, true],
        },
    ];
    for Test {
        input: (input, sel),
        expected,
    } in tests
    {
        assert_eq!(expected, dmux_4_way(input, sel))
    }
}

#[test]
fn test_dmux_8_way() {
    struct Test {
        input: (bool, [bool; 3]),
        expected: [bool; 8],
    }

    let tests = [
        Test {
            input: (false, [false; 3]),
            expected: [false; 8],
        },
        Test {
            input: (false, [false, true, false]),
            expected: [false; 8],
        },
        Test {
            input: (false, [true, true, false]),
            expected: [false; 8],
        },
        Test {
            input: (false, [true, true, true]),
            expected: [false; 8],
        },
        Test {
            input: (true, [false; 3]),
            expected: [true, false, false, false, false, false, false, false],
        },
        Test {
            input: (true, [true, false, false]),
            expected: [false, true, false, false, false, false, false, false],
        },
        Test {
            input: (true, [false, true, false]),
            expected: [false, false, true, false, false, false, false, false],
        },
        Test {
            input: (true, [true, true, false]),
            expected: [false, false, false, true, false, false, false, false],
        },
        Test {
            input: (true, [false, false, true]),
            expected: [false, false, false, false, true, false, false, false],
        },
        Test {
            input: (true, [true, false, true]),
            expected: [false, false, false, false, false, true, false, false],
        },
        Test {
            input: (true, [false, true, true]),
            expected: [false, false, false, false, false, false, true, false],
        },
        Test {
            input: (true, [true, true, true]),
            expected: [false, false, false, false, false, false, false, true],
        },
    ];
    for Test {
        input: (input, sel),
        expected,
    } in tests
    {
        assert_eq!(expected, dmux_8_way(input, sel))
    }
}

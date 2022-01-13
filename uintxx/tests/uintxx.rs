use uintxx::{Element, U128, U256, U32};

#[test]
fn test_wrapping_div() {
    let case_list = [[
        U256 {
            lo: U128(0xd3e04adfb2db76e8ce58bba4207434a4),
            hi: U128(0x15de88272aefffffffffffffffffffff),
        },
        U256 {
            lo: U128(0x686f332000000000000000000dd2966b),
            hi: U128(0x00000bea6a6af75538be984c83ce8648),
        },
        U256 {
            lo: U128(0x0000000000000000000000000001d5d8),
            hi: U128(0x00000000000000000000000000000000),
        },
    ]];
    for case in &case_list {
        let lhs = case[0];
        let rhs = case[1];
        let e = case[2];
        let r = lhs.wrapping_div(rhs);
        assert_eq!(r, e);
    }
}

#[test]
fn test_wrapping_div_s() {
    let case_list = [
        [
            U256 {
                lo: U128(0x00000000000000000000000000000001),
                hi: U128(0x00000000000000000000000000000000),
            },
            U256 {
                lo: U128(0x00000000000000000000000000000000),
                hi: U128(0x00000000000000000000000000000000),
            },
            U256 {
                lo: U128(0xffffffffffffffffffffffffffffffff),
                hi: U128(0xffffffffffffffffffffffffffffffff),
            },
        ],
        [
            U256 {
                lo: U128(0x00000000000000000000000000000000),
                hi: U128(0x80000000000000000000000000000000),
            },
            U256 {
                lo: U128(0xffffffffffffffffffffffffffffffff),
                hi: U128(0xffffffffffffffffffffffffffffffff),
            },
            U256 {
                lo: U128(0x00000000000000000000000000000000),
                hi: U128(0x80000000000000000000000000000000),
            },
        ],
        [
            U256 {
                lo: U128(0x2c1fb5204d24891731a7445bdf8bcb5c),
                hi: U128(0xea2177d8d51000000000000000000000),
            },
            U256 {
                lo: U128(0x686f332000000000000000000dd2966b),
                hi: U128(0x00000bea6a6af75538be984c83ce8648),
            },
            U256 {
                lo: U128(0xfffffffffffffffffffffffffffe2a28),
                hi: U128(0xffffffffffffffffffffffffffffffff),
            },
        ],
    ];
    for case in &case_list {
        let lhs = case[0];
        let rhs = case[1];
        let e = case[2];
        let r = lhs.wrapping_div_s(rhs);
        assert_eq!(r, e);
    }
}

#[test]
fn test_wrapping_rem() {
    let case_list = [[
        U256 {
            lo: U128(0x00000000000000000000000000000007),
            hi: U128(0x00000000000000000000000000000000),
        },
        U256 {
            lo: U128(0x00000000000000000000000000000002),
            hi: U128(0x00000000000000000000000000000000),
        },
        U256 {
            lo: U128(0x00000000000000000000000000000001),
            hi: U128(0x00000000000000000000000000000000),
        },
    ]];
    for case in &case_list {
        let lhs = case[0];
        let rhs = case[1];
        let e = case[2];
        let r = lhs.wrapping_rem(rhs);
        assert_eq!(r, e);
    }
}

#[test]
fn test_wrapping_rem_s() {
    let case_list = [
        [
            U256 {
                lo: U128(0x00000000000000000000000000000001),
                hi: U128(0x00000000000000000000000000000000),
            },
            U256 {
                lo: U128(0x00000000000000000000000000000000),
                hi: U128(0x00000000000000000000000000000000),
            },
            U256 {
                lo: U128(0x00000000000000000000000000000001),
                hi: U128(0x00000000000000000000000000000000),
            },
        ],
        [
            U256 {
                lo: U128(0x00000000000000000000000000000000),
                hi: U128(0x80000000000000000000000000000000),
            },
            U256 {
                lo: U128(0xffffffffffffffffffffffffffffffff),
                hi: U128(0xffffffffffffffffffffffffffffffff),
            },
            U256 {
                lo: U128(0x00000000000000000000000000000000),
                hi: U128(0x00000000000000000000000000000000),
            },
        ],
        [
            U256 {
                lo: U128(0xfffffffffffffffffffffffffffffff9),
                hi: U128(0xffffffffffffffffffffffffffffffff),
            },
            U256 {
                lo: U128(0x00000000000000000000000000000003),
                hi: U128(0x00000000000000000000000000000000),
            },
            U256 {
                lo: U128(0xffffffffffffffffffffffffffffffff),
                hi: U128(0xffffffffffffffffffffffffffffffff),
            },
        ],
    ];
    for case in &case_list {
        let lhs = case[0];
        let rhs = case[1];
        let e = case[2];
        let r = lhs.wrapping_rem_s(rhs);
        assert_eq!(r, e);
    }
}

#[test]
fn test_average_add() {
    let case_list = [
        [
            U256 {
                lo: U128(0xffffffffffffffffffffffffffffffff),
                hi: U128(0xffffffffffffffffffffffffffffffff),
            },
            U256 {
                lo: U128(0xffffffffffffffffffffffffffffffff),
                hi: U128(0xffffffffffffffffffffffffffffffff),
            },
            U256 {
                lo: U128(0xffffffffffffffffffffffffffffffff),
                hi: U128(0xffffffffffffffffffffffffffffffff),
            },
        ],
        [
            U256 {
                lo: U128(0x00000000000000000000000000000004),
                hi: U128(0x00000000000000000000000000000000),
            },
            U256 {
                lo: U128(0x00000000000000000000000000000006),
                hi: U128(0x00000000000000000000000000000000),
            },
            U256 {
                lo: U128(0x00000000000000000000000000000005),
                hi: U128(0x00000000000000000000000000000000),
            },
        ],
    ];
    for case in &case_list {
        let lhs = case[0];
        let rhs = case[1];
        let e = case[2];
        let r = lhs.average_add(rhs);
        assert_eq!(r, e);
    }
}

#[test]
fn test_widening_mul_s() {
    let case_list = [
        [U32(0xffffffff), U32(0xffffffff), U32(0x00000001), U32(0x00000000)],
        [U32(0x00000002), U32(0xffffffff), U32(0xfffffffe), U32(0xffffffff)],
        [U32(0x00000002), U32(0x00000002), U32(0x00000004), U32(0x00000000)],
    ];
    for case in &case_list {
        let lhs = case[0];
        let rhs = case[1];
        let elo = case[2];
        let ehi = case[3];
        let (rlo, rhi) = lhs.widening_mul_s(rhs);
        assert_eq!(rlo, elo);
        assert_eq!(rhi, ehi);
    }
}

#[test]
fn test_bug_fix_0() {
    let a = U256 {
        lo: U128(0x00000000000022330000000000001122),
        hi: U128(0x00000000000044550000000000003344),
    };
    let b = U256 {
        lo: U128(0x00000000000023450000000000001234),
        hi: U128(0x00000000000056780000000000004567),
    };
    let c = U256 {
        lo: U128(0x00000000000000bb00000000000000aa),
        hi: U128(0x00000000000000cc00000000000000dd),
    };
    let e = U256 {
        lo: U128(0x000000000047f182000000000017771c),
        hi: U128(0x000000000117122b0000000000a16174),
    };
    let r = (a + b) * c;
    assert_eq!(r, e);
}

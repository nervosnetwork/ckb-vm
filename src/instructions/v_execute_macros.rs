pub use eint::{Eint, E1024, E128, E16, E2048, E256, E32, E512, E64, E8};

pub fn is_aligned(val: u64, pos: u64) -> bool {
    if pos != 0 {
        (val & (pos - 1)) == 0
    } else {
        true
    }
}

macro_rules! require {
    ($val:expr, $msg:expr) => {
        if !$val {
            return Err(Error::RVVTrap($msg));
        }
    };
}

macro_rules! require_vill {
    ($machine:expr) => {
        require!(!$machine.vill(), String::from("vill"));
    };
}

macro_rules! require_align {
    ($val:expr, $pos:expr) => {
        require!(
            is_aligned(val, pos),
            format!("require align: val={} pos={}", val, pos)
        );
    };
}

macro_rules! ld {
    ($inst:expr, $machine:expr, $vl:expr, $stride:expr, $size:expr, $mask:expr) => {
        require_vill!($machine);

        let i = VXtype($inst);
        let vd = i.vd();
        let addr = $machine.registers()[i.rs1()].to_u64();
        let stride = if $stride != 0 {
            $machine.registers()[i.vs2()].to_u64()
        } else {
            $size
        };
        for j in 0..$vl {
            if $mask != 0 && i.vm() == 0 && !$machine.get_bit(0, j as usize) {
                continue;
            }
            let data = $machine
                .memory_mut()
                .load_bytes(stride.wrapping_mul(j).wrapping_add(addr), $size)?;
            $machine
                .element_mut(vd, $size << 3, j as usize)
                .copy_from_slice(&data);
        }
    };
}

macro_rules! ld_index {
    ($inst:expr, $machine:expr, $size:expr) => {
        require_vill!($machine);

        let i = VXtype($inst);
        let sew = $machine.vsew();
        let vd = i.vd();
        let addr = $machine.registers()[i.rs1()].to_u64();
        for j in 0..$machine.vl() as usize {
            if i.vm() == 0 && !$machine.get_bit(0, j as usize) {
                continue;
            }
            match sew {
                8 => {
                    let offset = match $size {
                        8 => E8::get($machine.element_ref(i.vs2(), $size, j)).u64(),
                        16 => E16::get($machine.element_ref(i.vs2(), $size, j)).u64(),
                        32 => E32::get($machine.element_ref(i.vs2(), $size, j)).u64(),
                        64 => E64::get($machine.element_ref(i.vs2(), $size, j)).u64(),
                        _ => unreachable!(),
                    };
                    let data = $machine
                        .memory_mut()
                        .load_bytes(addr.wrapping_add(offset), sew >> 3)?;
                    $machine
                        .element_mut(vd, sew, j as usize)
                        .copy_from_slice(&data);
                }
                16 => {
                    let offset = match $size {
                        8 => E8::get($machine.element_ref(i.vs2(), $size, j)).u64(),
                        16 => E16::get($machine.element_ref(i.vs2(), $size, j)).u64(),
                        32 => E32::get($machine.element_ref(i.vs2(), $size, j)).u64(),
                        64 => E64::get($machine.element_ref(i.vs2(), $size, j)).u64(),
                        _ => unreachable!(),
                    };
                    let data = $machine
                        .memory_mut()
                        .load_bytes(addr.wrapping_add(offset), sew >> 3)?;
                    $machine
                        .element_mut(vd, sew, j as usize)
                        .copy_from_slice(&data);
                }
                32 => {
                    let offset = match $size {
                        8 => E8::get($machine.element_ref(i.vs2(), $size, j)).u64(),
                        16 => E16::get($machine.element_ref(i.vs2(), $size, j)).u64(),
                        32 => E32::get($machine.element_ref(i.vs2(), $size, j)).u64(),
                        64 => E64::get($machine.element_ref(i.vs2(), $size, j)).u64(),
                        _ => unreachable!(),
                    };
                    let data = $machine
                        .memory_mut()
                        .load_bytes(addr.wrapping_add(offset), sew >> 3)?;
                    $machine
                        .element_mut(vd, sew, j as usize)
                        .copy_from_slice(&data);
                }
                64 => {
                    let offset = match $size {
                        8 => E8::get($machine.element_ref(i.vs2(), $size, j)).u64(),
                        16 => E16::get($machine.element_ref(i.vs2(), $size, j)).u64(),
                        32 => E32::get($machine.element_ref(i.vs2(), $size, j)).u64(),
                        64 => E64::get($machine.element_ref(i.vs2(), $size, j)).u64(),
                        _ => unreachable!(),
                    };
                    let data = $machine
                        .memory_mut()
                        .load_bytes(addr.wrapping_add(offset), sew >> 3)?;
                    $machine
                        .element_mut(vd, sew, j as usize)
                        .copy_from_slice(&data);
                }
                128 => {
                    let offset = match $size {
                        8 => E8::get($machine.element_ref(i.vs2(), $size, j)).u64(),
                        16 => E16::get($machine.element_ref(i.vs2(), $size, j)).u64(),
                        32 => E32::get($machine.element_ref(i.vs2(), $size, j)).u64(),
                        64 => E64::get($machine.element_ref(i.vs2(), $size, j)).u64(),
                        _ => unreachable!(),
                    };
                    let data = $machine
                        .memory_mut()
                        .load_bytes(addr.wrapping_add(offset), sew >> 3)?;
                    $machine
                        .element_mut(vd, sew, j as usize)
                        .copy_from_slice(&data);
                }
                256 => {
                    let offset = match $size {
                        8 => E8::get($machine.element_ref(i.vs2(), $size, j)).u64(),
                        16 => E16::get($machine.element_ref(i.vs2(), $size, j)).u64(),
                        32 => E32::get($machine.element_ref(i.vs2(), $size, j)).u64(),
                        64 => E64::get($machine.element_ref(i.vs2(), $size, j)).u64(),
                        _ => unreachable!(),
                    };
                    let data = $machine
                        .memory_mut()
                        .load_bytes(addr.wrapping_add(offset), sew >> 3)?;
                    $machine
                        .element_mut(vd, sew, j as usize)
                        .copy_from_slice(&data);
                }
                512 => {
                    let offset = match $size {
                        8 => E8::get($machine.element_ref(i.vs2(), $size, j)).u64(),
                        16 => E16::get($machine.element_ref(i.vs2(), $size, j)).u64(),
                        32 => E32::get($machine.element_ref(i.vs2(), $size, j)).u64(),
                        64 => E64::get($machine.element_ref(i.vs2(), $size, j)).u64(),
                        _ => unreachable!(),
                    };
                    let data = $machine
                        .memory_mut()
                        .load_bytes(addr.wrapping_add(offset), sew >> 3)?;
                    $machine
                        .element_mut(vd, sew, j as usize)
                        .copy_from_slice(&data);
                }
                1024 => {
                    let offset = match $size {
                        8 => E8::get($machine.element_ref(i.vs2(), $size, j)).u64(),
                        16 => E16::get($machine.element_ref(i.vs2(), $size, j)).u64(),
                        32 => E32::get($machine.element_ref(i.vs2(), $size, j)).u64(),
                        64 => E64::get($machine.element_ref(i.vs2(), $size, j)).u64(),
                        _ => unreachable!(),
                    };
                    let data = $machine
                        .memory_mut()
                        .load_bytes(addr.wrapping_add(offset), sew >> 3)?;
                    $machine
                        .element_mut(vd, sew, j as usize)
                        .copy_from_slice(&data);
                }
                _ => unreachable!(),
            }
        }
    };
}

macro_rules! ld_whole {
    ($inst:expr, $machine:expr, $size:expr) => {
        require_vill!($machine);

        let i = VXtype($inst);
        let addr = $machine.registers()[i.rs1()].to_u64();
        let data = $machine.memory_mut().load_bytes(addr, $size)?;
        $machine
            .element_mut(i.vd(), $size << 3, 0)
            .copy_from_slice(&data);
    };
}

macro_rules! sd {
    ($inst:expr, $machine:expr, $vl:expr, $stride:expr, $size:expr, $mask:expr) => {
        require_vill!($machine);

        let i = VXtype($inst);
        let vd = i.vd();
        let addr = $machine.registers()[i.rs1()].to_u64();
        let stride = if $stride != 0 {
            $machine.registers()[i.vs2()].to_u64()
        } else {
            $size
        };
        for j in 0..$vl {
            if $mask != 0 && i.vm() == 0 && !$machine.get_bit(0, j as usize) {
                continue;
            }
            let data = $machine.element_ref(vd, $size << 3, j as usize).to_vec();
            $machine
                .memory_mut()
                .store_bytes(stride.wrapping_mul(j).wrapping_add(addr), &data)?;
        }
    };
}

macro_rules! sd_index {
    ($inst:expr, $machine:expr, $size:expr) => {
        require_vill!($machine);

        let i = VXtype($inst);
        let sew = $machine.vsew();
        let vd = i.vd();
        let addr = $machine.registers()[i.rs1()].to_u64();
        for j in 0..$machine.vl() as usize {
            if i.vm() == 0 && !$machine.get_bit(0, j as usize) {
                continue;
            }
            match sew {
                8 => {
                    let offset = match $size {
                        8 => E8::get($machine.element_ref(i.vs2(), $size, j)).u64(),
                        16 => E16::get($machine.element_ref(i.vs2(), $size, j)).u64(),
                        32 => E32::get($machine.element_ref(i.vs2(), $size, j)).u64(),
                        64 => E64::get($machine.element_ref(i.vs2(), $size, j)).u64(),
                        _ => unreachable!(),
                    };
                    let data = $machine.element_ref(vd, sew, j as usize).to_vec();
                    $machine
                        .memory_mut()
                        .store_bytes(addr.wrapping_add(offset), &data)?;
                }
                16 => {
                    let offset = match $size {
                        8 => E8::get($machine.element_ref(i.vs2(), $size, j)).u64(),
                        16 => E16::get($machine.element_ref(i.vs2(), $size, j)).u64(),
                        32 => E32::get($machine.element_ref(i.vs2(), $size, j)).u64(),
                        64 => E64::get($machine.element_ref(i.vs2(), $size, j)).u64(),
                        _ => unreachable!(),
                    };
                    let data = $machine.element_ref(vd, sew, j as usize).to_vec();
                    $machine
                        .memory_mut()
                        .store_bytes(addr.wrapping_add(offset), &data)?;
                }
                32 => {
                    let offset = match $size {
                        8 => E8::get($machine.element_ref(i.vs2(), $size, j)).u64(),
                        16 => E16::get($machine.element_ref(i.vs2(), $size, j)).u64(),
                        32 => E32::get($machine.element_ref(i.vs2(), $size, j)).u64(),
                        64 => E64::get($machine.element_ref(i.vs2(), $size, j)).u64(),
                        _ => unreachable!(),
                    };
                    let data = $machine.element_ref(vd, sew, j as usize).to_vec();
                    $machine
                        .memory_mut()
                        .store_bytes(addr.wrapping_add(offset), &data)?;
                }
                64 => {
                    let offset = match $size {
                        8 => E8::get($machine.element_ref(i.vs2(), $size, j)).u64(),
                        16 => E16::get($machine.element_ref(i.vs2(), $size, j)).u64(),
                        32 => E32::get($machine.element_ref(i.vs2(), $size, j)).u64(),
                        64 => E64::get($machine.element_ref(i.vs2(), $size, j)).u64(),
                        _ => unreachable!(),
                    };
                    let data = $machine.element_ref(vd, sew, j as usize).to_vec();
                    $machine
                        .memory_mut()
                        .store_bytes(addr.wrapping_add(offset), &data)?;
                }
                128 => {
                    let offset = match $size {
                        8 => E8::get($machine.element_ref(i.vs2(), $size, j)).u64(),
                        16 => E16::get($machine.element_ref(i.vs2(), $size, j)).u64(),
                        32 => E32::get($machine.element_ref(i.vs2(), $size, j)).u64(),
                        64 => E64::get($machine.element_ref(i.vs2(), $size, j)).u64(),
                        _ => unreachable!(),
                    };
                    let data = $machine.element_ref(vd, sew, j as usize).to_vec();
                    $machine
                        .memory_mut()
                        .store_bytes(addr.wrapping_add(offset), &data)?;
                }
                256 => {
                    let offset = match $size {
                        8 => E8::get($machine.element_ref(i.vs2(), $size, j)).u64(),
                        16 => E16::get($machine.element_ref(i.vs2(), $size, j)).u64(),
                        32 => E32::get($machine.element_ref(i.vs2(), $size, j)).u64(),
                        64 => E64::get($machine.element_ref(i.vs2(), $size, j)).u64(),
                        _ => unreachable!(),
                    };
                    let data = $machine.element_ref(vd, sew, j as usize).to_vec();
                    $machine
                        .memory_mut()
                        .store_bytes(addr.wrapping_add(offset), &data)?;
                }
                512 => {
                    let offset = match $size {
                        8 => E8::get($machine.element_ref(i.vs2(), $size, j)).u64(),
                        16 => E16::get($machine.element_ref(i.vs2(), $size, j)).u64(),
                        32 => E32::get($machine.element_ref(i.vs2(), $size, j)).u64(),
                        64 => E64::get($machine.element_ref(i.vs2(), $size, j)).u64(),
                        _ => unreachable!(),
                    };
                    let data = $machine.element_ref(vd, sew, j as usize).to_vec();
                    $machine
                        .memory_mut()
                        .store_bytes(addr.wrapping_add(offset), &data)?;
                }
                1024 => {
                    let offset = match $size {
                        8 => E8::get($machine.element_ref(i.vs2(), $size, j)).u64(),
                        16 => E16::get($machine.element_ref(i.vs2(), $size, j)).u64(),
                        32 => E32::get($machine.element_ref(i.vs2(), $size, j)).u64(),
                        64 => E64::get($machine.element_ref(i.vs2(), $size, j)).u64(),
                        _ => unreachable!(),
                    };
                    let data = $machine.element_ref(vd, sew, j as usize).to_vec();
                    $machine
                        .memory_mut()
                        .store_bytes(addr.wrapping_add(offset), &data)?;
                }
                _ => unreachable!(),
            }
        }
    };
}

macro_rules! sd_whole {
    ($inst:expr, $machine:expr, $size:expr) => {
        require_vill!($machine);

        let i = VXtype($inst);
        let addr = $machine.registers()[i.rs1()].to_u64();
        let data = $machine.element_ref(i.vd(), $size << 3, 0).to_vec();
        $machine.memory_mut().store_bytes(addr, &data)?;
    };
}

macro_rules! v_vv_loop {
    ($inst:expr, $machine:expr, $body:expr) => {
        require_vill!($machine);

        let i = VVtype($inst);
        let sew = $machine.vsew();
        for j in 0..$machine.vl() as usize {
            if i.vm() == 0 && !$machine.get_bit(0, j) {
                continue;
            }
            match sew {
                8 => {
                    let b = E8::get($machine.element_ref(i.vs2(), sew, j));
                    let a = E8::get($machine.element_ref(i.vs1(), sew, j));
                    let r = $body(b, a);
                    r.put($machine.element_mut(i.vd(), sew, j));
                }
                16 => {
                    let b = E16::get($machine.element_ref(i.vs2(), sew, j));
                    let a = E16::get($machine.element_ref(i.vs1(), sew, j));
                    let r = $body(b, a);
                    r.put($machine.element_mut(i.vd(), sew, j));
                }
                32 => {
                    let b = E32::get($machine.element_ref(i.vs2(), sew, j));
                    let a = E32::get($machine.element_ref(i.vs1(), sew, j));
                    let r = $body(b, a);
                    r.put($machine.element_mut(i.vd(), sew, j));
                }
                64 => {
                    let b = E64::get($machine.element_ref(i.vs2(), sew, j));
                    let a = E64::get($machine.element_ref(i.vs1(), sew, j));
                    let r = $body(b, a);
                    r.put($machine.element_mut(i.vd(), sew, j));
                }
                128 => {
                    let b = E128::get($machine.element_ref(i.vs2(), sew, j));
                    let a = E128::get($machine.element_ref(i.vs1(), sew, j));
                    let r = $body(b, a);
                    r.put($machine.element_mut(i.vd(), sew, j));
                }
                256 => {
                    let b = E256::get($machine.element_ref(i.vs2(), sew, j));
                    let a = E256::get($machine.element_ref(i.vs1(), sew, j));
                    let r = $body(b, a);
                    r.put($machine.element_mut(i.vd(), sew, j));
                }
                512 => {
                    let b = E512::get($machine.element_ref(i.vs2(), sew, j));
                    let a = E512::get($machine.element_ref(i.vs1(), sew, j));
                    let r = $body(b, a);
                    r.put($machine.element_mut(i.vd(), sew, j));
                }
                1024 => {
                    let b = E1024::get($machine.element_ref(i.vs2(), sew, j));
                    let a = E1024::get($machine.element_ref(i.vs1(), sew, j));
                    let r = $body(b, a);
                    r.put($machine.element_mut(i.vd(), sew, j));
                }
                _ => unreachable!(),
            }
        }
    };
}

macro_rules! v_vv_loop_s {
    ($inst:expr, $machine:expr, $body:expr) => {
        v_vv_loop!($inst, $machine, $body);
    };
}

macro_rules! v_vv_loop_u {
    ($inst:expr, $machine:expr, $body:expr) => {
        v_vv_loop!($inst, $machine, $body);
    };
}

macro_rules! v_vx_loop {
    ($inst:expr, $machine:expr, $body:expr, $sign:expr) => {
        require_vill!($machine);

        let i = VXtype($inst);
        let sew = $machine.vsew();
        for j in 0..$machine.vl() as usize {
            if i.vm() == 0 && !$machine.get_bit(0, j) {
                continue;
            }
            match sew {
                8 => {
                    let b = E8::get($machine.element_ref(i.vs2(), sew, j));
                    let a = if $sign != 0 {
                        E8::from($machine.registers()[i.rs1()].to_i64())
                    } else {
                        E8::from($machine.registers()[i.rs1()].to_u64())
                    };
                    let r = $body(b, a);
                    r.put($machine.element_mut(i.vd(), sew, j));
                }
                16 => {
                    let b = E16::get($machine.element_ref(i.vs2(), sew, j));
                    let a = if $sign != 0 {
                        E16::from($machine.registers()[i.rs1()].to_i64())
                    } else {
                        E16::from($machine.registers()[i.rs1()].to_u64())
                    };
                    let r = $body(b, a);
                    r.put($machine.element_mut(i.vd(), sew, j));
                }
                32 => {
                    let b = E32::get($machine.element_ref(i.vs2(), sew, j));
                    let a = if $sign != 0 {
                        E32::from($machine.registers()[i.rs1()].to_i64())
                    } else {
                        E32::from($machine.registers()[i.rs1()].to_u64())
                    };
                    let r = $body(b, a);
                    r.put($machine.element_mut(i.vd(), sew, j));
                }
                64 => {
                    let b = E64::get($machine.element_ref(i.vs2(), sew, j));
                    let a = if $sign != 0 {
                        E64::from($machine.registers()[i.rs1()].to_i64())
                    } else {
                        E64::from($machine.registers()[i.rs1()].to_u64())
                    };
                    let r = $body(b, a);
                    r.put($machine.element_mut(i.vd(), sew, j));
                }
                128 => {
                    let b = E128::get($machine.element_ref(i.vs2(), sew, j));
                    let a = if $sign != 0 {
                        E128::from($machine.registers()[i.rs1()].to_i64())
                    } else {
                        E128::from($machine.registers()[i.rs1()].to_u64())
                    };
                    let r = $body(b, a);
                    r.put($machine.element_mut(i.vd(), sew, j));
                }
                256 => {
                    let b = E256::get($machine.element_ref(i.vs2(), sew, j));
                    let a = if $sign != 0 {
                        E256::from($machine.registers()[i.rs1()].to_i64())
                    } else {
                        E256::from($machine.registers()[i.rs1()].to_u64())
                    };
                    let r = $body(b, a);
                    r.put($machine.element_mut(i.vd(), sew, j));
                }
                512 => {
                    let b = E512::get($machine.element_ref(i.vs2(), sew, j));
                    let a = if $sign != 0 {
                        E512::from($machine.registers()[i.rs1()].to_i64())
                    } else {
                        E512::from($machine.registers()[i.rs1()].to_u64())
                    };
                    let r = $body(b, a);
                    r.put($machine.element_mut(i.vd(), sew, j));
                }
                1024 => {
                    let b = E1024::get($machine.element_ref(i.vs2(), sew, j));
                    let a = if $sign != 0 {
                        E1024::from($machine.registers()[i.rs1()].to_i64())
                    } else {
                        E1024::from($machine.registers()[i.rs1()].to_u64())
                    };
                    let r = $body(b, a);
                    r.put($machine.element_mut(i.vd(), sew, j));
                }
                _ => unreachable!(),
            }
        }
    };
}

macro_rules! v_vx_loop_s {
    ($inst:expr, $machine:expr, $body:expr) => {
        v_vx_loop!($inst, $machine, $body, 1);
    };
}

macro_rules! v_vx_loop_u {
    ($inst:expr, $machine:expr, $body:expr) => {
        v_vx_loop!($inst, $machine, $body, 0);
    };
}

macro_rules! v_vi_loop {
    ($inst:expr, $machine:expr, $body:expr, $sign:expr) => {
        require_vill!($machine);

        let i = VItype($inst);
        let sew = $machine.vsew();
        for j in 0..$machine.vl() as usize {
            if i.vm() == 0 && !$machine.get_bit(0, j) {
                continue;
            }
            match sew {
                8 => {
                    let b = E8::get($machine.element_ref(i.vs2(), sew, j));
                    let a = if $sign != 0 {
                        E8::from(i.immediate_s())
                    } else {
                        E8::from(i.immediate_u())
                    };
                    let r = $body(b, a);
                    r.put($machine.element_mut(i.vd(), sew, j));
                }
                16 => {
                    let b = E16::get($machine.element_ref(i.vs2(), sew, j));
                    let a = if $sign != 0 {
                        E16::from(i.immediate_s())
                    } else {
                        E16::from(i.immediate_u())
                    };
                    let r = $body(b, a);
                    r.put($machine.element_mut(i.vd(), sew, j));
                }
                32 => {
                    let b = E32::get($machine.element_ref(i.vs2(), sew, j));
                    let a = if $sign != 0 {
                        E32::from(i.immediate_s())
                    } else {
                        E32::from(i.immediate_u())
                    };
                    let r = $body(b, a);
                    r.put($machine.element_mut(i.vd(), sew, j));
                }
                64 => {
                    let b = E64::get($machine.element_ref(i.vs2(), sew, j));
                    let a = if $sign != 0 {
                        E64::from(i.immediate_s())
                    } else {
                        E64::from(i.immediate_u())
                    };
                    let r = $body(b, a);
                    r.put($machine.element_mut(i.vd(), sew, j));
                }
                128 => {
                    let b = E128::get($machine.element_ref(i.vs2(), sew, j));
                    let a = if $sign != 0 {
                        E128::from(i.immediate_s())
                    } else {
                        E128::from(i.immediate_u())
                    };
                    let r = $body(b, a);
                    r.put($machine.element_mut(i.vd(), sew, j));
                }
                256 => {
                    let b = E256::get($machine.element_ref(i.vs2(), sew, j));
                    let a = if $sign != 0 {
                        E256::from(i.immediate_s())
                    } else {
                        E256::from(i.immediate_u())
                    };
                    let r = $body(b, a);
                    r.put($machine.element_mut(i.vd(), sew, j));
                }
                512 => {
                    let b = E512::get($machine.element_ref(i.vs2(), sew, j));
                    let a = if $sign != 0 {
                        E512::from(i.immediate_s())
                    } else {
                        E512::from(i.immediate_u())
                    };
                    let r = $body(b, a);
                    r.put($machine.element_mut(i.vd(), sew, j));
                }
                1024 => {
                    let b = E1024::get($machine.element_ref(i.vs2(), sew, j));
                    let a = if $sign != 0 {
                        E1024::from(i.immediate_s())
                    } else {
                        E1024::from(i.immediate_u())
                    };
                    let r = $body(b, a);
                    r.put($machine.element_mut(i.vd(), sew, j));
                }
                _ => unreachable!(),
            }
        }
    };
}

macro_rules! v_vi_loop_s {
    ($inst:expr, $machine:expr, $body:expr) => {
        v_vi_loop!($inst, $machine, $body, 1);
    };
}

macro_rules! v_vi_loop_u {
    ($inst:expr, $machine:expr, $body:expr) => {
        v_vi_loop!($inst, $machine, $body, 0);
    };
}

macro_rules! m_vv_loop {
    ($inst:expr, $machine:expr, $cond:expr) => {
        require_vill!($machine);

        let i = VVtype($inst);
        let sew = $machine.vsew();
        for j in 0..$machine.vl() as usize {
            if i.vm() == 0 && !$machine.get_bit(0, j) {
                continue;
            }
            match sew {
                8 => {
                    let b = E8::get($machine.element_ref(i.vs2(), sew, j));
                    let a = E8::get($machine.element_ref(i.vs1(), sew, j));
                    if $cond(b, a) {
                        $machine.set_bit(i.vd(), j);
                    } else {
                        $machine.clr_bit(i.vd(), j);
                    };
                }
                16 => {
                    let b = E16::get($machine.element_ref(i.vs2(), sew, j));
                    let a = E16::get($machine.element_ref(i.vs1(), sew, j));
                    if $cond(b, a) {
                        $machine.set_bit(i.vd(), j);
                    } else {
                        $machine.clr_bit(i.vd(), j);
                    };
                }
                32 => {
                    let b = E32::get($machine.element_ref(i.vs2(), sew, j));
                    let a = E32::get($machine.element_ref(i.vs1(), sew, j));
                    if $cond(b, a) {
                        $machine.set_bit(i.vd(), j);
                    } else {
                        $machine.clr_bit(i.vd(), j);
                    };
                }
                64 => {
                    let b = E64::get($machine.element_ref(i.vs2(), sew, j));
                    let a = E64::get($machine.element_ref(i.vs1(), sew, j));
                    if $cond(b, a) {
                        $machine.set_bit(i.vd(), j);
                    } else {
                        $machine.clr_bit(i.vd(), j);
                    };
                }
                128 => {
                    let b = E128::get($machine.element_ref(i.vs2(), sew, j));
                    let a = E128::get($machine.element_ref(i.vs1(), sew, j));
                    if $cond(b, a) {
                        $machine.set_bit(i.vd(), j);
                    } else {
                        $machine.clr_bit(i.vd(), j);
                    };
                }
                256 => {
                    let b = E256::get($machine.element_ref(i.vs2(), sew, j));
                    let a = E256::get($machine.element_ref(i.vs1(), sew, j));
                    if $cond(b, a) {
                        $machine.set_bit(i.vd(), j);
                    } else {
                        $machine.clr_bit(i.vd(), j);
                    };
                }
                512 => {
                    let b = E512::get($machine.element_ref(i.vs2(), sew, j));
                    let a = E512::get($machine.element_ref(i.vs1(), sew, j));
                    if $cond(b, a) {
                        $machine.set_bit(i.vd(), j);
                    } else {
                        $machine.clr_bit(i.vd(), j);
                    };
                }
                1024 => {
                    let b = E1024::get($machine.element_ref(i.vs2(), sew, j));
                    let a = E1024::get($machine.element_ref(i.vs1(), sew, j));
                    if $cond(b, a) {
                        $machine.set_bit(i.vd(), j);
                    } else {
                        $machine.clr_bit(i.vd(), j);
                    };
                }
                _ => unreachable!(),
            }
        }
    };
}

macro_rules! m_vv_loop_s {
    ($inst:expr, $machine:expr, $body:expr) => {
        m_vv_loop!($inst, $machine, $body);
    };
}

macro_rules! m_vv_loop_u {
    ($inst:expr, $machine:expr, $body:expr) => {
        m_vv_loop!($inst, $machine, $body);
    };
}

macro_rules! m_vx_loop {
    ($inst:expr, $machine:expr, $cond:expr, $sign:expr) => {
        require_vill!($machine);

        let i = VXtype($inst);
        let sew = $machine.vsew();
        for j in 0..$machine.vl() as usize {
            if i.vm() == 0 && !$machine.get_bit(0, j) {
                continue;
            }
            match sew {
                8 => {
                    let b = E8::get($machine.element_ref(i.vs2(), sew, j));
                    let a = if $sign != 0 {
                        E8::from($machine.registers()[i.rs1()].to_i64())
                    } else {
                        E8::from($machine.registers()[i.rs1()].to_u64())
                    };
                    if $cond(b, a) {
                        $machine.set_bit(i.vd(), j);
                    } else {
                        $machine.clr_bit(i.vd(), j);
                    };
                }
                16 => {
                    let b = E16::get($machine.element_ref(i.vs2(), sew, j));
                    let a = if $sign != 0 {
                        E16::from($machine.registers()[i.rs1()].to_i64())
                    } else {
                        E16::from($machine.registers()[i.rs1()].to_u64())
                    };
                    if $cond(b, a) {
                        $machine.set_bit(i.vd(), j);
                    } else {
                        $machine.clr_bit(i.vd(), j);
                    };
                }
                32 => {
                    let b = E32::get($machine.element_ref(i.vs2(), sew, j));
                    let a = if $sign != 0 {
                        E32::from($machine.registers()[i.rs1()].to_i64())
                    } else {
                        E32::from($machine.registers()[i.rs1()].to_u64())
                    };
                    if $cond(b, a) {
                        $machine.set_bit(i.vd(), j);
                    } else {
                        $machine.clr_bit(i.vd(), j);
                    };
                }
                64 => {
                    let b = E64::get($machine.element_ref(i.vs2(), sew, j));
                    let a = if $sign != 0 {
                        E64::from($machine.registers()[i.rs1()].to_i64())
                    } else {
                        E64::from($machine.registers()[i.rs1()].to_u64())
                    };
                    if $cond(b, a) {
                        $machine.set_bit(i.vd(), j);
                    } else {
                        $machine.clr_bit(i.vd(), j);
                    };
                }
                128 => {
                    let b = E128::get($machine.element_ref(i.vs2(), sew, j));
                    let a = if $sign != 0 {
                        E128::from($machine.registers()[i.rs1()].to_i64())
                    } else {
                        E128::from($machine.registers()[i.rs1()].to_u64())
                    };
                    if $cond(b, a) {
                        $machine.set_bit(i.vd(), j);
                    } else {
                        $machine.clr_bit(i.vd(), j);
                    };
                }
                256 => {
                    let b = E256::get($machine.element_ref(i.vs2(), sew, j));
                    let a = if $sign != 0 {
                        E256::from($machine.registers()[i.rs1()].to_i64())
                    } else {
                        E256::from($machine.registers()[i.rs1()].to_u64())
                    };
                    if $cond(b, a) {
                        $machine.set_bit(i.vd(), j);
                    } else {
                        $machine.clr_bit(i.vd(), j);
                    };
                }
                512 => {
                    let b = E512::get($machine.element_ref(i.vs2(), sew, j));
                    let a = if $sign != 0 {
                        E512::from($machine.registers()[i.rs1()].to_i64())
                    } else {
                        E512::from($machine.registers()[i.rs1()].to_u64())
                    };
                    if $cond(b, a) {
                        $machine.set_bit(i.vd(), j);
                    } else {
                        $machine.clr_bit(i.vd(), j);
                    };
                }
                1024 => {
                    let b = E1024::get($machine.element_ref(i.vs2(), sew, j));
                    let a = if $sign != 0 {
                        E1024::from($machine.registers()[i.rs1()].to_i64())
                    } else {
                        E1024::from($machine.registers()[i.rs1()].to_u64())
                    };
                    if $cond(b, a) {
                        $machine.set_bit(i.vd(), j);
                    } else {
                        $machine.clr_bit(i.vd(), j);
                    };
                }
                _ => unreachable!(),
            }
        }
    };
}

macro_rules! m_vx_loop_s {
    ($inst:expr, $machine:expr, $cond:expr) => {
        m_vx_loop!($inst, $machine, $cond, 1);
    };
}

macro_rules! m_vx_loop_u {
    ($inst:expr, $machine:expr, $cond:expr) => {
        m_vx_loop!($inst, $machine, $cond, 0);
    };
}

macro_rules! m_vi_loop {
    ($inst:expr, $machine:expr, $cond:expr, $sign:expr) => {
        require_vill!($machine);

        let i = VItype($inst);
        let sew = $machine.vsew();
        for j in 0..$machine.vl() as usize {
            if i.vm() == 0 && !$machine.get_bit(0, j) {
                continue;
            }
            match sew {
                8 => {
                    let b = E8::get($machine.element_ref(i.vs2(), sew, j));
                    let a = if $sign != 0 {
                        E8::from(i.immediate_s())
                    } else {
                        E8::from(i.immediate_u())
                    };
                    if $cond(b, a) {
                        $machine.set_bit(i.vd(), j);
                    } else {
                        $machine.clr_bit(i.vd(), j);
                    };
                }
                16 => {
                    let b = E16::get($machine.element_ref(i.vs2(), sew, j));
                    let a = if $sign != 0 {
                        E16::from(i.immediate_s())
                    } else {
                        E16::from(i.immediate_u())
                    };
                    if $cond(b, a) {
                        $machine.set_bit(i.vd(), j);
                    } else {
                        $machine.clr_bit(i.vd(), j);
                    };
                }
                32 => {
                    let b = E32::get($machine.element_ref(i.vs2(), sew, j));
                    let a = if $sign != 0 {
                        E32::from(i.immediate_s())
                    } else {
                        E32::from(i.immediate_u())
                    };
                    if $cond(b, a) {
                        $machine.set_bit(i.vd(), j);
                    } else {
                        $machine.clr_bit(i.vd(), j);
                    };
                }
                64 => {
                    let b = E64::get($machine.element_ref(i.vs2(), sew, j));
                    let a = if $sign != 0 {
                        E64::from(i.immediate_s())
                    } else {
                        E64::from(i.immediate_u())
                    };
                    if $cond(b, a) {
                        $machine.set_bit(i.vd(), j);
                    } else {
                        $machine.clr_bit(i.vd(), j);
                    };
                }
                128 => {
                    let b = E128::get($machine.element_ref(i.vs2(), sew, j));
                    let a = if $sign != 0 {
                        E128::from(i.immediate_s())
                    } else {
                        E128::from(i.immediate_u())
                    };
                    if $cond(b, a) {
                        $machine.set_bit(i.vd(), j);
                    } else {
                        $machine.clr_bit(i.vd(), j);
                    };
                }
                256 => {
                    let b = E256::get($machine.element_ref(i.vs2(), sew, j));
                    let a = if $sign != 0 {
                        E256::from(i.immediate_s())
                    } else {
                        E256::from(i.immediate_u())
                    };
                    if $cond(b, a) {
                        $machine.set_bit(i.vd(), j);
                    } else {
                        $machine.clr_bit(i.vd(), j);
                    };
                }
                512 => {
                    let b = E512::get($machine.element_ref(i.vs2(), sew, j));
                    let a = if $sign != 0 {
                        E512::from(i.immediate_s())
                    } else {
                        E512::from(i.immediate_u())
                    };
                    if $cond(b, a) {
                        $machine.set_bit(i.vd(), j);
                    } else {
                        $machine.clr_bit(i.vd(), j);
                    };
                }
                1024 => {
                    let b = E1024::get($machine.element_ref(i.vs2(), sew, j));
                    let a = if $sign != 0 {
                        E1024::from(i.immediate_s())
                    } else {
                        E1024::from(i.immediate_u())
                    };
                    if $cond(b, a) {
                        $machine.set_bit(i.vd(), j);
                    } else {
                        $machine.clr_bit(i.vd(), j);
                    };
                }
                _ => unreachable!(),
            }
        }
    };
}

macro_rules! m_vi_loop_s {
    ($inst:expr, $machine:expr, $cond:expr) => {
        m_vi_loop!($inst, $machine, $cond, 1);
    };
}

macro_rules! m_vi_loop_u {
    ($inst:expr, $machine:expr, $cond:expr) => {
        m_vi_loop!($inst, $machine, $cond, 0);
    };
}

macro_rules! m_mm_loop {
    ($inst:expr, $machine:expr, $body:expr) => {
        require_vill!($machine);

        let i = VVtype($inst);
        for j in 0..$machine.vl() as usize {
            let b = $machine.get_bit(i.vs2(), j);
            let a = $machine.get_bit(i.vs1(), j);
            if $body(b, a) {
                $machine.set_bit(i.vd(), j);
            } else {
                $machine.clr_bit(i.vd(), j);
            }
        }
    };
}

macro_rules! w_vv_loop {
    ($inst:expr, $machine:expr, $body:expr) => {
        require_vill!($machine);

        let i = VVtype($inst);
        let sew = $machine.vsew();
        for j in 0..$machine.vl() as usize {
            if i.vm() == 0 && !$machine.get_bit(0, j) {
                continue;
            }
            match sew {
                8 => {
                    let b = E8::get($machine.element_ref(i.vs2(), sew, j));
                    let a = E8::get($machine.element_ref(i.vs1(), sew, j));
                    let (lo, hi) = $body(b, a);
                    lo.put($machine.element_mut(i.vd(), sew, j * 2));
                    hi.put($machine.element_mut(i.vd(), sew, j * 2 + 1));
                }
                16 => {
                    let b = E16::get($machine.element_ref(i.vs2(), sew, j));
                    let a = E16::get($machine.element_ref(i.vs1(), sew, j));
                    let (lo, hi) = $body(b, a);
                    lo.put($machine.element_mut(i.vd(), sew, j * 2));
                    hi.put($machine.element_mut(i.vd(), sew, j * 2 + 1));
                }
                32 => {
                    let b = E32::get($machine.element_ref(i.vs2(), sew, j));
                    let a = E32::get($machine.element_ref(i.vs1(), sew, j));
                    let (lo, hi) = $body(b, a);
                    lo.put($machine.element_mut(i.vd(), sew, j * 2));
                    hi.put($machine.element_mut(i.vd(), sew, j * 2 + 1));
                }
                64 => {
                    let b = E64::get($machine.element_ref(i.vs2(), sew, j));
                    let a = E64::get($machine.element_ref(i.vs1(), sew, j));
                    let (lo, hi) = $body(b, a);
                    lo.put($machine.element_mut(i.vd(), sew, j * 2));
                    hi.put($machine.element_mut(i.vd(), sew, j * 2 + 1));
                }
                128 => {
                    let b = E128::get($machine.element_ref(i.vs2(), sew, j));
                    let a = E128::get($machine.element_ref(i.vs1(), sew, j));
                    let (lo, hi) = $body(b, a);
                    lo.put($machine.element_mut(i.vd(), sew, j * 2));
                    hi.put($machine.element_mut(i.vd(), sew, j * 2 + 1));
                }
                256 => {
                    let b = E256::get($machine.element_ref(i.vs2(), sew, j));
                    let a = E256::get($machine.element_ref(i.vs1(), sew, j));
                    let (lo, hi) = $body(b, a);
                    lo.put($machine.element_mut(i.vd(), sew, j * 2));
                    hi.put($machine.element_mut(i.vd(), sew, j * 2 + 1));
                }
                512 => {
                    let b = E512::get($machine.element_ref(i.vs2(), sew, j));
                    let a = E512::get($machine.element_ref(i.vs1(), sew, j));
                    let (lo, hi) = $body(b, a);
                    lo.put($machine.element_mut(i.vd(), sew, j * 2));
                    hi.put($machine.element_mut(i.vd(), sew, j * 2 + 1));
                }
                1024 => {
                    let b = E1024::get($machine.element_ref(i.vs2(), sew, j));
                    let a = E1024::get($machine.element_ref(i.vs1(), sew, j));
                    let (lo, hi) = $body(b, a);
                    lo.put($machine.element_mut(i.vd(), sew, j * 2));
                    hi.put($machine.element_mut(i.vd(), sew, j * 2 + 1));
                }
                _ => unreachable!(),
            }
        }
    };
}

macro_rules! w_vv_loop_s {
    ($inst:expr, $machine:expr, $cond:expr) => {
        w_vv_loop!($inst, $machine, $cond);
    };
}

macro_rules! w_vv_loop_u {
    ($inst:expr, $machine:expr, $cond:expr) => {
        w_vv_loop!($inst, $machine, $cond);
    };
}

macro_rules! w_vx_loop {
    ($inst:expr, $machine:expr, $body:expr, $sign:expr) => {
        require_vill!($machine);

        let i = VXtype($inst);
        let sew = $machine.vsew();
        for j in 0..$machine.vl() as usize {
            if i.vm() == 0 && !$machine.get_bit(0, j) {
                continue;
            }
            match sew {
                8 => {
                    let b = E8::get($machine.element_ref(i.vs2(), sew, j));
                    let a = if $sign != 0 {
                        E8::from($machine.registers()[i.rs1()].to_i64())
                    } else {
                        E8::from($machine.registers()[i.rs1()].to_u64())
                    };
                    let (lo, hi) = $body(b, a);
                    lo.put($machine.element_mut(i.vd(), sew, j * 2));
                    hi.put($machine.element_mut(i.vd(), sew, j * 2 + 1));
                }
                16 => {
                    let b = E16::get($machine.element_ref(i.vs2(), sew, j));
                    let a = if $sign != 0 {
                        E16::from($machine.registers()[i.rs1()].to_i64())
                    } else {
                        E16::from($machine.registers()[i.rs1()].to_u64())
                    };
                    let (lo, hi) = $body(b, a);
                    lo.put($machine.element_mut(i.vd(), sew, j * 2));
                    hi.put($machine.element_mut(i.vd(), sew, j * 2 + 1));
                }
                32 => {
                    let b = E32::get($machine.element_ref(i.vs2(), sew, j));
                    let a = if $sign != 0 {
                        E32::from($machine.registers()[i.rs1()].to_i64())
                    } else {
                        E32::from($machine.registers()[i.rs1()].to_u64())
                    };
                    let (lo, hi) = $body(b, a);
                    lo.put($machine.element_mut(i.vd(), sew, j * 2));
                    hi.put($machine.element_mut(i.vd(), sew, j * 2 + 1));
                }
                64 => {
                    let b = E64::get($machine.element_ref(i.vs2(), sew, j));
                    let a = if $sign != 0 {
                        E64::from($machine.registers()[i.rs1()].to_i64())
                    } else {
                        E64::from($machine.registers()[i.rs1()].to_u64())
                    };
                    let (lo, hi) = $body(b, a);
                    lo.put($machine.element_mut(i.vd(), sew, j * 2));
                    hi.put($machine.element_mut(i.vd(), sew, j * 2 + 1));
                }
                128 => {
                    let b = E128::get($machine.element_ref(i.vs2(), sew, j));
                    let a = if $sign != 0 {
                        E128::from($machine.registers()[i.rs1()].to_i64())
                    } else {
                        E128::from($machine.registers()[i.rs1()].to_u64())
                    };
                    let (lo, hi) = $body(b, a);
                    lo.put($machine.element_mut(i.vd(), sew, j * 2));
                    hi.put($machine.element_mut(i.vd(), sew, j * 2 + 1));
                }
                256 => {
                    let b = E256::get($machine.element_ref(i.vs2(), sew, j));
                    let a = if $sign != 0 {
                        E256::from($machine.registers()[i.rs1()].to_i64())
                    } else {
                        E256::from($machine.registers()[i.rs1()].to_u64())
                    };
                    let (lo, hi) = $body(b, a);
                    lo.put($machine.element_mut(i.vd(), sew, j * 2));
                    hi.put($machine.element_mut(i.vd(), sew, j * 2 + 1));
                }
                512 => {
                    let b = E512::get($machine.element_ref(i.vs2(), sew, j));
                    let a = if $sign != 0 {
                        E512::from($machine.registers()[i.rs1()].to_i64())
                    } else {
                        E512::from($machine.registers()[i.rs1()].to_u64())
                    };
                    let (lo, hi) = $body(b, a);
                    lo.put($machine.element_mut(i.vd(), sew, j * 2));
                    hi.put($machine.element_mut(i.vd(), sew, j * 2 + 1));
                }
                1024 => {
                    let b = E1024::get($machine.element_ref(i.vs2(), sew, j));
                    let a = if $sign != 0 {
                        E1024::from($machine.registers()[i.rs1()].to_i64())
                    } else {
                        E1024::from($machine.registers()[i.rs1()].to_u64())
                    };
                    let (lo, hi) = $body(b, a);
                    lo.put($machine.element_mut(i.vd(), sew, j * 2));
                    hi.put($machine.element_mut(i.vd(), sew, j * 2 + 1));
                }
                _ => unreachable!(),
            }
        }
    };
}

macro_rules! w_vx_loop_s {
    ($inst:expr, $machine:expr, $body:expr) => {
        w_vx_loop!($inst, $machine, $body, 1);
    };
}

macro_rules! w_vx_loop_u {
    ($inst:expr, $machine:expr, $body:expr) => {
        w_vx_loop!($inst, $machine, $body, 0);
    };
}

macro_rules! w_wv_loop {
    ($inst:expr, $machine:expr, $body:expr, $sign:expr) => {
        require_vill!($machine);

        let i = VVtype($inst);
        let sew = $machine.vsew();
        for j in 0..$machine.vl() as usize {
            if i.vm() == 0 && !$machine.get_bit(0, j) {
                continue;
            }
            match sew {
                8 => {
                    let b = E16::get($machine.element_ref(i.vs2(), sew * 2, j));
                    let a = if $sign != 0 {
                        E16::from(E8::get($machine.element_ref(i.vs1(), sew, j))).lo_sext()
                    } else {
                        E16::from(E8::get($machine.element_ref(i.vs1(), sew, j)))
                    };
                    let r = $body(b, a);
                    r.put($machine.element_mut(i.vd(), sew * 2, j));
                }
                16 => {
                    let b = E32::get($machine.element_ref(i.vs2(), sew * 2, j));
                    let a = if $sign != 0 {
                        E32::from(E16::get($machine.element_ref(i.vs1(), sew, j))).lo_sext()
                    } else {
                        E32::from(E16::get($machine.element_ref(i.vs1(), sew, j)))
                    };
                    let r = $body(b, a);
                    r.put($machine.element_mut(i.vd(), sew * 2, j));
                }
                32 => {
                    let b = E64::get($machine.element_ref(i.vs2(), sew * 2, j));
                    let a = if $sign != 0 {
                        E64::from(E32::get($machine.element_ref(i.vs1(), sew, j))).lo_sext()
                    } else {
                        E64::from(E32::get($machine.element_ref(i.vs1(), sew, j)))
                    };
                    let r = $body(b, a);
                    r.put($machine.element_mut(i.vd(), sew * 2, j));
                }
                64 => {
                    let b = E128::get($machine.element_ref(i.vs2(), sew * 2, j));
                    let a = if $sign != 0 {
                        E128::from(E64::get($machine.element_ref(i.vs1(), sew, j))).lo_sext()
                    } else {
                        E128::from(E64::get($machine.element_ref(i.vs1(), sew, j)))
                    };
                    let r = $body(b, a);
                    r.put($machine.element_mut(i.vd(), sew * 2, j));
                }
                128 => {
                    let b = E256::get($machine.element_ref(i.vs2(), sew * 2, j));
                    let a = if $sign != 0 {
                        E256::from(E128::get($machine.element_ref(i.vs1(), sew, j))).lo_sext()
                    } else {
                        E256::from(E128::get($machine.element_ref(i.vs1(), sew, j)))
                    };
                    let r = $body(b, a);
                    r.put($machine.element_mut(i.vd(), sew * 2, j));
                }
                256 => {
                    let b = E512::get($machine.element_ref(i.vs2(), sew * 2, j));
                    let a = if $sign != 0 {
                        E512::from(E256::get($machine.element_ref(i.vs1(), sew, j))).lo_sext()
                    } else {
                        E512::from(E256::get($machine.element_ref(i.vs1(), sew, j)))
                    };
                    let r = $body(b, a);
                    r.put($machine.element_mut(i.vd(), sew * 2, j));
                }
                512 => {
                    let b = E1024::get($machine.element_ref(i.vs2(), sew * 2, j));
                    let a = if $sign != 0 {
                        E1024::from(E512::get($machine.element_ref(i.vs1(), sew, j))).lo_sext()
                    } else {
                        E1024::from(E512::get($machine.element_ref(i.vs1(), sew, j)))
                    };
                    let r = $body(b, a);
                    r.put($machine.element_mut(i.vd(), sew * 2, j));
                }
                1024 => {
                    let b = E2048::get($machine.element_ref(i.vs2(), sew * 2, j));
                    let a = if $sign != 0 {
                        E2048::from(E1024::get($machine.element_ref(i.vs1(), sew, j))).lo_sext()
                    } else {
                        E2048::from(E1024::get($machine.element_ref(i.vs1(), sew, j)))
                    };
                    let r = $body(b, a);
                    r.put($machine.element_mut(i.vd(), sew * 2, j));
                }
                _ => unreachable!(),
            }
        }
    };
}

macro_rules! w_wv_loop_s {
    ($inst:expr, $machine:expr, $body:expr) => {
        w_wv_loop!($inst, $machine, $body, 1);
    };
}

macro_rules! w_wv_loop_u {
    ($inst:expr, $machine:expr, $body:expr) => {
        w_wv_loop!($inst, $machine, $body, 0);
    };
}

macro_rules! w_wx_loop {
    ($inst:expr, $machine:expr, $body:expr, $sign:expr) => {
        require_vill!($machine);

        let i = VXtype($inst);
        let sew = $machine.vsew();
        for j in 0..$machine.vl() as usize {
            if i.vm() == 0 && !$machine.get_bit(0, j) {
                continue;
            }
            match sew {
                8 => {
                    let b = E16::get($machine.element_ref(i.vs2(), sew * 2, j));
                    let a = if $sign != 0 {
                        E16::from(E8::from($machine.registers()[i.rs1()].to_i64())).lo_sext()
                    } else {
                        E16::from(E8::from($machine.registers()[i.rs1()].to_u64()))
                    };
                    let r = $body(b, a);
                    r.put($machine.element_mut(i.vd(), sew * 2, j));
                }
                16 => {
                    let b = E32::get($machine.element_ref(i.vs2(), sew * 2, j));
                    let a = if $sign != 0 {
                        E32::from(E16::from($machine.registers()[i.rs1()].to_i64())).lo_sext()
                    } else {
                        E32::from(E16::from($machine.registers()[i.rs1()].to_u64()))
                    };
                    let r = $body(b, a);
                    r.put($machine.element_mut(i.vd(), sew * 2, j));
                }
                32 => {
                    let b = E64::get($machine.element_ref(i.vs2(), sew * 2, j));
                    let a = if $sign != 0 {
                        E64::from(E32::from($machine.registers()[i.rs1()].to_i64())).lo_sext()
                    } else {
                        E64::from(E32::from($machine.registers()[i.rs1()].to_u64()))
                    };
                    let r = $body(b, a);
                    r.put($machine.element_mut(i.vd(), sew * 2, j));
                }
                64 => {
                    let b = E128::get($machine.element_ref(i.vs2(), sew * 2, j));
                    let a = if $sign != 0 {
                        E128::from(E64::from($machine.registers()[i.rs1()].to_i64())).lo_sext()
                    } else {
                        E128::from(E64::from($machine.registers()[i.rs1()].to_u64()))
                    };
                    let r = $body(b, a);
                    r.put($machine.element_mut(i.vd(), sew * 2, j));
                }
                128 => {
                    let b = E256::get($machine.element_ref(i.vs2(), sew * 2, j));
                    let a = if $sign != 0 {
                        E256::from(E128::from($machine.registers()[i.rs1()].to_i64())).lo_sext()
                    } else {
                        E256::from(E128::from($machine.registers()[i.rs1()].to_u64()))
                    };
                    let r = $body(b, a);
                    r.put($machine.element_mut(i.vd(), sew * 2, j));
                }
                256 => {
                    let b = E512::get($machine.element_ref(i.vs2(), sew * 2, j));
                    let a = if $sign != 0 {
                        E512::from(E256::from($machine.registers()[i.rs1()].to_i64())).lo_sext()
                    } else {
                        E512::from(E256::from($machine.registers()[i.rs1()].to_u64()))
                    };
                    let r = $body(b, a);
                    r.put($machine.element_mut(i.vd(), sew * 2, j));
                }
                512 => {
                    let b = E1024::get($machine.element_ref(i.vs2(), sew * 2, j));
                    let a = if $sign != 0 {
                        E1024::from(E512::from($machine.registers()[i.rs1()].to_i64())).lo_sext()
                    } else {
                        E1024::from(E512::from($machine.registers()[i.rs1()].to_u64()))
                    };
                    let r = $body(b, a);
                    r.put($machine.element_mut(i.vd(), sew * 2, j));
                }
                1024 => {
                    let b = E2048::get($machine.element_ref(i.vs2(), sew * 2, j));
                    let a = if $sign != 0 {
                        E2048::from(E1024::from($machine.registers()[i.rs1()].to_i64())).lo_sext()
                    } else {
                        E2048::from(E1024::from($machine.registers()[i.rs1()].to_u64()))
                    };
                    let r = $body(b, a);
                    r.put($machine.element_mut(i.vd(), sew * 2, j));
                }
                _ => unreachable!(),
            }
        }
    };
}

macro_rules! w_wx_loop_s {
    ($inst:expr, $machine:expr, $body:expr) => {
        w_wx_loop!($inst, $machine, $body, 1);
    };
}

macro_rules! w_wx_loop_u {
    ($inst:expr, $machine:expr, $body:expr) => {
        w_wx_loop!($inst, $machine, $body, 0);
    };
}

macro_rules! v_wv_loop {
    ($inst:expr, $machine:expr, $body:expr, $size:expr) => {
        require_vill!($machine);

        let i = VVtype($inst);
        let sew = $machine.vsew();
        for j in 0..$machine.vl() as usize {
            if i.vm() == 0 && !$machine.get_bit(0, j) {
                continue;
            }
            match sew {
                8 => {
                    let b = E16::get($machine.element_ref(i.vs2(), sew * 2, j));
                    let a = if $size != 0 {
                        E16::from(E8::get($machine.element_ref(i.vs1(), sew, j))).lo_sext()
                    } else {
                        E16::from(E8::get($machine.element_ref(i.vs1(), sew, j)))
                    };
                    let r = $body(b, a);
                    r.put_lo($machine.element_mut(i.vd(), sew, j));
                }
                16 => {
                    let b = E32::get($machine.element_ref(i.vs2(), sew * 2, j));
                    let a = if $size != 0 {
                        E32::from(E16::get($machine.element_ref(i.vs1(), sew, j))).lo_sext()
                    } else {
                        E32::from(E16::get($machine.element_ref(i.vs1(), sew, j)))
                    };
                    let r = $body(b, a);
                    r.put_lo($machine.element_mut(i.vd(), sew, j));
                }
                32 => {
                    let b = E64::get($machine.element_ref(i.vs2(), sew * 2, j));
                    let a = if $size != 0 {
                        E64::from(E32::get($machine.element_ref(i.vs1(), sew, j))).lo_sext()
                    } else {
                        E64::from(E32::get($machine.element_ref(i.vs1(), sew, j)))
                    };
                    let r = $body(b, a);
                    r.put_lo($machine.element_mut(i.vd(), sew, j));
                }
                64 => {
                    let b = E128::get($machine.element_ref(i.vs2(), sew * 2, j));
                    let a = if $size != 0 {
                        E128::from(E64::get($machine.element_ref(i.vs1(), sew, j))).lo_sext()
                    } else {
                        E128::from(E64::get($machine.element_ref(i.vs1(), sew, j)))
                    };
                    let r = $body(b, a);
                    r.put_lo($machine.element_mut(i.vd(), sew, j));
                }
                128 => {
                    let b = E256::get($machine.element_ref(i.vs2(), sew * 2, j));
                    let a = if $size != 0 {
                        E256::from(E128::get($machine.element_ref(i.vs1(), sew, j))).lo_sext()
                    } else {
                        E256::from(E128::get($machine.element_ref(i.vs1(), sew, j)))
                    };
                    let r = $body(b, a);
                    r.put_lo($machine.element_mut(i.vd(), sew, j));
                }
                256 => {
                    let b = E512::get($machine.element_ref(i.vs2(), sew * 2, j));
                    let a = if $size != 0 {
                        E512::from(E256::get($machine.element_ref(i.vs1(), sew, j))).lo_sext()
                    } else {
                        E512::from(E256::get($machine.element_ref(i.vs1(), sew, j)))
                    };
                    let r = $body(b, a);
                    r.put_lo($machine.element_mut(i.vd(), sew, j));
                }
                512 => {
                    let b = E1024::get($machine.element_ref(i.vs2(), sew * 2, j));
                    let a = if $size != 0 {
                        E1024::from(E512::get($machine.element_ref(i.vs1(), sew, j))).lo_sext()
                    } else {
                        E1024::from(E512::get($machine.element_ref(i.vs1(), sew, j)))
                    };
                    let r = $body(b, a);
                    r.put_lo($machine.element_mut(i.vd(), sew, j));
                }
                1024 => {
                    let b = E2048::get($machine.element_ref(i.vs2(), sew * 2, j));
                    let a = if $size != 0 {
                        E2048::from(E1024::get($machine.element_ref(i.vs1(), sew, j))).lo_sext()
                    } else {
                        E2048::from(E1024::get($machine.element_ref(i.vs1(), sew, j)))
                    };
                    let r = $body(b, a);
                    r.put_lo($machine.element_mut(i.vd(), sew, j));
                }
                _ => unreachable!(),
            }
        }
    };
}

macro_rules! v_wv_loop_u {
    ($inst:expr, $machine:expr, $body:expr) => {
        v_wv_loop!($inst, $machine, $body, 0);
    };
}

macro_rules! v_wx_loop {
    ($inst:expr, $machine:expr, $body:expr, $sign:expr) => {
        require_vill!($machine);

        let i = VXtype($inst);
        let sew = $machine.vsew();
        for j in 0..$machine.vl() as usize {
            if i.vm() == 0 && !$machine.get_bit(0, j) {
                continue;
            }
            match sew {
                8 => {
                    let b = E16::get($machine.element_ref(i.vs2(), sew * 2, j));
                    let a = if $sign != 0 {
                        E16::from(E8::from($machine.registers()[i.rs1()].to_i64())).lo_sext()
                    } else {
                        E16::from(E8::from($machine.registers()[i.rs1()].to_u64()))
                    };
                    let r = $body(b, a);
                    r.put_lo($machine.element_mut(i.vd(), sew, j));
                }
                16 => {
                    let b = E32::get($machine.element_ref(i.vs2(), sew * 2, j));
                    let a = if $sign != 0 {
                        E32::from(E16::from($machine.registers()[i.rs1()].to_i64())).lo_sext()
                    } else {
                        E32::from(E16::from($machine.registers()[i.rs1()].to_u64()))
                    };
                    let r = $body(b, a);
                    r.put_lo($machine.element_mut(i.vd(), sew, j));
                }
                32 => {
                    let b = E64::get($machine.element_ref(i.vs2(), sew * 2, j));
                    let a = if $sign != 0 {
                        E64::from(E32::from($machine.registers()[i.rs1()].to_i64())).lo_sext()
                    } else {
                        E64::from(E32::from($machine.registers()[i.rs1()].to_u64()))
                    };
                    let r = $body(b, a);
                    r.put_lo($machine.element_mut(i.vd(), sew, j));
                }
                64 => {
                    let b = E128::get($machine.element_ref(i.vs2(), sew * 2, j));
                    let a = if $sign != 0 {
                        E128::from(E64::from($machine.registers()[i.rs1()].to_i64())).lo_sext()
                    } else {
                        E128::from(E64::from($machine.registers()[i.rs1()].to_u64()))
                    };
                    let r = $body(b, a);
                    r.put_lo($machine.element_mut(i.vd(), sew, j));
                }
                128 => {
                    let b = E256::get($machine.element_ref(i.vs2(), sew * 2, j));
                    let a = if $sign != 0 {
                        E256::from(E128::from($machine.registers()[i.rs1()].to_i64())).lo_sext()
                    } else {
                        E256::from(E128::from($machine.registers()[i.rs1()].to_u64()))
                    };
                    let r = $body(b, a);
                    r.put_lo($machine.element_mut(i.vd(), sew, j));
                }
                256 => {
                    let b = E512::get($machine.element_ref(i.vs2(), sew * 2, j));
                    let a = if $sign != 0 {
                        E512::from(E256::from($machine.registers()[i.rs1()].to_i64())).lo_sext()
                    } else {
                        E512::from(E256::from($machine.registers()[i.rs1()].to_u64()))
                    };
                    let r = $body(b, a);
                    r.put_lo($machine.element_mut(i.vd(), sew, j));
                }
                512 => {
                    let b = E1024::get($machine.element_ref(i.vs2(), sew * 2, j));
                    let a = if $sign != 0 {
                        E1024::from(E512::from($machine.registers()[i.rs1()].to_i64())).lo_sext()
                    } else {
                        E1024::from(E512::from($machine.registers()[i.rs1()].to_u64()))
                    };
                    let r = $body(b, a);
                    r.put_lo($machine.element_mut(i.vd(), sew, j));
                }
                1024 => {
                    let b = E2048::get($machine.element_ref(i.vs2(), sew * 2, j));
                    let a = if $sign != 0 {
                        E2048::from(E1024::from($machine.registers()[i.rs1()].to_i64())).lo_sext()
                    } else {
                        E2048::from(E1024::from($machine.registers()[i.rs1()].to_u64()))
                    };
                    let r = $body(b, a);
                    r.put_lo($machine.element_mut(i.vd(), sew, j));
                }
                _ => unreachable!(),
            }
        }
    };
}

macro_rules! v_wx_loop_u {
    ($inst:expr, $machine:expr, $body:expr) => {
        v_wx_loop!($inst, $machine, $body, 0);
    };
}

macro_rules! v_wi_loop {
    ($inst:expr, $machine:expr, $body:expr, $sign:expr) => {
        require_vill!($machine);

        let i = VItype($inst);
        let sew = $machine.vsew();
        for j in 0..$machine.vl() as usize {
            if i.vm() == 0 && !$machine.get_bit(0, j) {
                continue;
            }
            match sew {
                8 => {
                    let b = E16::get($machine.element_ref(i.vs2(), sew * 2, j));
                    let a = if $sign != 0 {
                        E16::from(E8::from(i.immediate_s())).lo_sext()
                    } else {
                        E16::from(E8::from(i.immediate_u()))
                    };
                    let r = $body(b, a);
                    r.put_lo($machine.element_mut(i.vd(), sew, j));
                }
                16 => {
                    let b = E32::get($machine.element_ref(i.vs2(), sew * 2, j));
                    let a = if $sign != 0 {
                        E32::from(E16::from(i.immediate_s())).lo_sext()
                    } else {
                        E32::from(E16::from(i.immediate_u()))
                    };
                    let r = $body(b, a);
                    r.put_lo($machine.element_mut(i.vd(), sew, j));
                }
                32 => {
                    let b = E64::get($machine.element_ref(i.vs2(), sew * 2, j));
                    let a = if $sign != 0 {
                        E64::from(E32::from(i.immediate_s())).lo_sext()
                    } else {
                        E64::from(E32::from(i.immediate_u()))
                    };
                    let r = $body(b, a);
                    r.put_lo($machine.element_mut(i.vd(), sew, j));
                }
                64 => {
                    let b = E128::get($machine.element_ref(i.vs2(), sew * 2, j));
                    let a = if $sign != 0 {
                        E128::from(E64::from(i.immediate_s())).lo_sext()
                    } else {
                        E128::from(E64::from(i.immediate_u()))
                    };
                    let r = $body(b, a);
                    r.put_lo($machine.element_mut(i.vd(), sew, j));
                }
                128 => {
                    let b = E256::get($machine.element_ref(i.vs2(), sew * 2, j));
                    let a = if $sign != 0 {
                        E256::from(E128::from(i.immediate_s())).lo_sext()
                    } else {
                        E256::from(E128::from(i.immediate_u()))
                    };
                    let r = $body(b, a);
                    r.put_lo($machine.element_mut(i.vd(), sew, j));
                }
                256 => {
                    let b = E512::get($machine.element_ref(i.vs2(), sew * 2, j));
                    let a = if $sign != 0 {
                        E512::from(E256::from(i.immediate_s())).lo_sext()
                    } else {
                        E512::from(E256::from(i.immediate_u()))
                    };
                    let r = $body(b, a);
                    r.put_lo($machine.element_mut(i.vd(), sew, j));
                }
                512 => {
                    let b = E1024::get($machine.element_ref(i.vs2(), sew * 2, j));
                    let a = if $sign != 0 {
                        E1024::from(E512::from(i.immediate_s())).lo_sext()
                    } else {
                        E1024::from(E512::from(i.immediate_u()))
                    };
                    let r = $body(b, a);
                    r.put_lo($machine.element_mut(i.vd(), sew, j));
                }
                1024 => {
                    let b = E2048::get($machine.element_ref(i.vs2(), sew * 2, j));
                    let a = if $sign != 0 {
                        E2048::from(E1024::from(i.immediate_s())).lo_sext()
                    } else {
                        E2048::from(E1024::from(i.immediate_u()))
                    };
                    let r = $body(b, a);
                    r.put_lo($machine.element_mut(i.vd(), sew, j));
                }
                _ => unreachable!(),
            }
        }
    };
}

macro_rules! v_wi_loop_u {
    ($inst:expr, $machine:expr, $body:expr) => {
        v_wi_loop!($inst, $machine, $body, 0);
    };
}

macro_rules! v_vvm_loop {
    ($inst:expr, $machine:expr, $body:expr) => {
        require_vill!($machine);

        let i = VVtype($inst);
        let sew = $machine.vsew();
        for j in 0..$machine.vl() as usize {
            let mbit = $machine.get_bit(0, j);
            match sew {
                8 => {
                    let b = E8::get($machine.element_ref(i.vs2(), sew, j));
                    let a = E8::get($machine.element_ref(i.vs1(), sew, j));
                    let r = $body(b, a, mbit);
                    r.put($machine.element_mut(i.vd(), sew, j));
                }
                16 => {
                    let b = E16::get($machine.element_ref(i.vs2(), sew, j));
                    let a = E16::get($machine.element_ref(i.vs1(), sew, j));
                    let r = $body(b, a, mbit);
                    r.put($machine.element_mut(i.vd(), sew, j));
                }
                32 => {
                    let b = E32::get($machine.element_ref(i.vs2(), sew, j));
                    let a = E32::get($machine.element_ref(i.vs1(), sew, j));
                    let r = $body(b, a, mbit);
                    r.put($machine.element_mut(i.vd(), sew, j));
                }
                64 => {
                    let b = E64::get($machine.element_ref(i.vs2(), sew, j));
                    let a = E64::get($machine.element_ref(i.vs1(), sew, j));
                    let r = $body(b, a, mbit);
                    r.put($machine.element_mut(i.vd(), sew, j));
                }
                128 => {
                    let b = E128::get($machine.element_ref(i.vs2(), sew, j));
                    let a = E128::get($machine.element_ref(i.vs1(), sew, j));
                    let r = $body(b, a, mbit);
                    r.put($machine.element_mut(i.vd(), sew, j));
                }
                256 => {
                    let b = E256::get($machine.element_ref(i.vs2(), sew, j));
                    let a = E256::get($machine.element_ref(i.vs1(), sew, j));
                    let r = $body(b, a, mbit);
                    r.put($machine.element_mut(i.vd(), sew, j));
                }
                512 => {
                    let b = E512::get($machine.element_ref(i.vs2(), sew, j));
                    let a = E512::get($machine.element_ref(i.vs1(), sew, j));
                    let r = $body(b, a, mbit);
                    r.put($machine.element_mut(i.vd(), sew, j));
                }
                1024 => {
                    let b = E1024::get($machine.element_ref(i.vs2(), sew, j));
                    let a = E1024::get($machine.element_ref(i.vs1(), sew, j));
                    let r = $body(b, a, mbit);
                    r.put($machine.element_mut(i.vd(), sew, j));
                }
                _ => unreachable!(),
            }
        }
    };
}

macro_rules! v_vvm_loop_s {
    ($inst:expr, $machine:expr, $body:expr) => {
        v_vvm_loop!($inst, $machine, $body);
    };
}

macro_rules! v_vxm_loop {
    ($inst:expr, $machine:expr, $body:expr, $sign:expr) => {
        require_vill!($machine);

        let i = VXtype($inst);
        let sew = $machine.vsew();
        for j in 0..$machine.vl() as usize {
            let mbit = $machine.get_bit(0, j);
            match sew {
                8 => {
                    let b = E8::get($machine.element_ref(i.vs2(), sew, j));
                    let a = if $sign != 0 {
                        E8::from($machine.registers()[i.rs1()].to_i64())
                    } else {
                        E8::from($machine.registers()[i.rs1()].to_u64())
                    };
                    let r = $body(b, a, mbit);
                    r.put($machine.element_mut(i.vd(), sew, j));
                }
                16 => {
                    let b = E16::get($machine.element_ref(i.vs2(), sew, j));
                    let a = if $sign != 0 {
                        E16::from($machine.registers()[i.rs1()].to_i64())
                    } else {
                        E16::from($machine.registers()[i.rs1()].to_u64())
                    };
                    let r = $body(b, a, mbit);
                    r.put($machine.element_mut(i.vd(), sew, j));
                }
                32 => {
                    let b = E32::get($machine.element_ref(i.vs2(), sew, j));
                    let a = if $sign != 0 {
                        E32::from($machine.registers()[i.rs1()].to_i64())
                    } else {
                        E32::from($machine.registers()[i.rs1()].to_u64())
                    };
                    let r = $body(b, a, mbit);
                    r.put($machine.element_mut(i.vd(), sew, j));
                }
                64 => {
                    let b = E64::get($machine.element_ref(i.vs2(), sew, j));
                    let a = if $sign != 0 {
                        E64::from($machine.registers()[i.rs1()].to_i64())
                    } else {
                        E64::from($machine.registers()[i.rs1()].to_u64())
                    };
                    let r = $body(b, a, mbit);
                    r.put($machine.element_mut(i.vd(), sew, j));
                }
                128 => {
                    let b = E128::get($machine.element_ref(i.vs2(), sew, j));
                    let a = if $sign != 0 {
                        E128::from($machine.registers()[i.rs1()].to_i64())
                    } else {
                        E128::from($machine.registers()[i.rs1()].to_u64())
                    };
                    let r = $body(b, a, mbit);
                    r.put($machine.element_mut(i.vd(), sew, j));
                }
                256 => {
                    let b = E256::get($machine.element_ref(i.vs2(), sew, j));
                    let a = if $sign != 0 {
                        E256::from($machine.registers()[i.rs1()].to_i64())
                    } else {
                        E256::from($machine.registers()[i.rs1()].to_u64())
                    };
                    let r = $body(b, a, mbit);
                    r.put($machine.element_mut(i.vd(), sew, j));
                }
                512 => {
                    let b = E512::get($machine.element_ref(i.vs2(), sew, j));
                    let a = if $sign != 0 {
                        E512::from($machine.registers()[i.rs1()].to_i64())
                    } else {
                        E512::from($machine.registers()[i.rs1()].to_u64())
                    };
                    let r = $body(b, a, mbit);
                    r.put($machine.element_mut(i.vd(), sew, j));
                }
                1024 => {
                    let b = E1024::get($machine.element_ref(i.vs2(), sew, j));
                    let a = if $sign != 0 {
                        E1024::from($machine.registers()[i.rs1()].to_i64())
                    } else {
                        E1024::from($machine.registers()[i.rs1()].to_u64())
                    };
                    let r = $body(b, a, mbit);
                    r.put($machine.element_mut(i.vd(), sew, j));
                }
                _ => unreachable!(),
            }
        }
    };
}

macro_rules! v_vxm_loop_s {
    ($inst:expr, $machine:expr, $body:expr) => {
        v_vxm_loop!($inst, $machine, $body, 1);
    };
}

macro_rules! v_vim_loop {
    ($inst:expr, $machine:expr, $body:expr, $sign:expr) => {
        require_vill!($machine);

        let i = VItype($inst);
        let sew = $machine.vsew();
        for j in 0..$machine.vl() as usize {
            let mbit = $machine.get_bit(0, j);
            match sew {
                8 => {
                    let b = E8::get($machine.element_ref(i.vs2(), sew, j));
                    let a = if $sign != 0 {
                        E8::from(i.immediate_s())
                    } else {
                        E8::from(i.immediate_u())
                    };
                    let r = $body(b, a, mbit);
                    r.put($machine.element_mut(i.vd(), sew, j));
                }
                16 => {
                    let b = E16::get($machine.element_ref(i.vs2(), sew, j));
                    let a = if $sign != 0 {
                        E16::from(i.immediate_s())
                    } else {
                        E16::from(i.immediate_u())
                    };
                    let r = $body(b, a, mbit);
                    r.put($machine.element_mut(i.vd(), sew, j));
                }
                32 => {
                    let b = E32::get($machine.element_ref(i.vs2(), sew, j));
                    let a = if $sign != 0 {
                        E32::from(i.immediate_s())
                    } else {
                        E32::from(i.immediate_u())
                    };
                    let r = $body(b, a, mbit);
                    r.put($machine.element_mut(i.vd(), sew, j));
                }
                64 => {
                    let b = E64::get($machine.element_ref(i.vs2(), sew, j));
                    let a = if $sign != 0 {
                        E64::from(i.immediate_s())
                    } else {
                        E64::from(i.immediate_u())
                    };
                    let r = $body(b, a, mbit);
                    r.put($machine.element_mut(i.vd(), sew, j));
                }
                128 => {
                    let b = E128::get($machine.element_ref(i.vs2(), sew, j));
                    let a = if $sign != 0 {
                        E128::from(i.immediate_s())
                    } else {
                        E128::from(i.immediate_u())
                    };
                    let r = $body(b, a, mbit);
                    r.put($machine.element_mut(i.vd(), sew, j));
                }
                256 => {
                    let b = E256::get($machine.element_ref(i.vs2(), sew, j));
                    let a = if $sign != 0 {
                        E256::from(i.immediate_s())
                    } else {
                        E256::from(i.immediate_u())
                    };
                    let r = $body(b, a, mbit);
                    r.put($machine.element_mut(i.vd(), sew, j));
                }
                512 => {
                    let b = E512::get($machine.element_ref(i.vs2(), sew, j));
                    let a = if $sign != 0 {
                        E512::from(i.immediate_s())
                    } else {
                        E512::from(i.immediate_u())
                    };
                    let r = $body(b, a, mbit);
                    r.put($machine.element_mut(i.vd(), sew, j));
                }
                1024 => {
                    let b = E1024::get($machine.element_ref(i.vs2(), sew, j));
                    let a = if $sign != 0 {
                        E1024::from(i.immediate_s())
                    } else {
                        E1024::from(i.immediate_u())
                    };
                    let r = $body(b, a, mbit);
                    r.put($machine.element_mut(i.vd(), sew, j));
                }
                _ => unreachable!(),
            }
        }
    };
}

macro_rules! v_vim_loop_s {
    ($inst:expr, $machine:expr, $body:expr) => {
        v_vim_loop!($inst, $machine, $body, 1);
    };
}

macro_rules! m_vvm_loop {
    ($inst:expr, $machine:expr, $cond:expr) => {
        require_vill!($machine);

        let i = VVtype($inst);
        let sew = $machine.vsew();
        for j in 0..$machine.vl() as usize {
            let mbit = $machine.get_bit(0, j);
            match sew {
                8 => {
                    let b = E8::get($machine.element_ref(i.vs2(), sew, j));
                    let a = E8::get($machine.element_ref(i.vs1(), sew, j));
                    if $cond(b, a, mbit) {
                        $machine.set_bit(i.vd(), j);
                    } else {
                        $machine.clr_bit(i.vd(), j);
                    };
                }
                16 => {
                    let b = E16::get($machine.element_ref(i.vs2(), sew, j));
                    let a = E16::get($machine.element_ref(i.vs1(), sew, j));
                    if $cond(b, a, mbit) {
                        $machine.set_bit(i.vd(), j);
                    } else {
                        $machine.clr_bit(i.vd(), j);
                    };
                }
                32 => {
                    let b = E32::get($machine.element_ref(i.vs2(), sew, j));
                    let a = E32::get($machine.element_ref(i.vs1(), sew, j));
                    if $cond(b, a, mbit) {
                        $machine.set_bit(i.vd(), j);
                    } else {
                        $machine.clr_bit(i.vd(), j);
                    };
                }
                64 => {
                    let b = E64::get($machine.element_ref(i.vs2(), sew, j));
                    let a = E64::get($machine.element_ref(i.vs1(), sew, j));
                    if $cond(b, a, mbit) {
                        $machine.set_bit(i.vd(), j);
                    } else {
                        $machine.clr_bit(i.vd(), j);
                    };
                }
                128 => {
                    let b = E128::get($machine.element_ref(i.vs2(), sew, j));
                    let a = E128::get($machine.element_ref(i.vs1(), sew, j));
                    if $cond(b, a, mbit) {
                        $machine.set_bit(i.vd(), j);
                    } else {
                        $machine.clr_bit(i.vd(), j);
                    };
                }
                256 => {
                    let b = E256::get($machine.element_ref(i.vs2(), sew, j));
                    let a = E256::get($machine.element_ref(i.vs1(), sew, j));
                    if $cond(b, a, mbit) {
                        $machine.set_bit(i.vd(), j);
                    } else {
                        $machine.clr_bit(i.vd(), j);
                    };
                }
                512 => {
                    let b = E512::get($machine.element_ref(i.vs2(), sew, j));
                    let a = E512::get($machine.element_ref(i.vs1(), sew, j));
                    if $cond(b, a, mbit) {
                        $machine.set_bit(i.vd(), j);
                    } else {
                        $machine.clr_bit(i.vd(), j);
                    };
                }
                1024 => {
                    let b = E1024::get($machine.element_ref(i.vs2(), sew, j));
                    let a = E1024::get($machine.element_ref(i.vs1(), sew, j));
                    if $cond(b, a, mbit) {
                        $machine.set_bit(i.vd(), j);
                    } else {
                        $machine.clr_bit(i.vd(), j);
                    };
                }
                _ => unreachable!(),
            }
        }
    };
}

macro_rules! m_vvm_loop_s {
    ($inst:expr, $machine:expr, $body:expr) => {
        m_vvm_loop!($inst, $machine, $body);
    };
}

macro_rules! m_vxm_loop {
    ($inst:expr, $machine:expr, $cond:expr, $sign:expr) => {
        require_vill!($machine);

        let i = VXtype($inst);
        let sew = $machine.vsew();
        for j in 0..$machine.vl() as usize {
            let mbit = $machine.get_bit(0, j);
            match sew {
                8 => {
                    let b = E8::get($machine.element_ref(i.vs2(), sew, j));
                    let a = if $sign != 0 {
                        E8::from($machine.registers()[i.rs1()].to_i64())
                    } else {
                        E8::from($machine.registers()[i.rs1()].to_u64())
                    };
                    if $cond(b, a, mbit) {
                        $machine.set_bit(i.vd(), j);
                    } else {
                        $machine.clr_bit(i.vd(), j);
                    };
                }
                16 => {
                    let b = E16::get($machine.element_ref(i.vs2(), sew, j));
                    let a = if $sign != 0 {
                        E16::from($machine.registers()[i.rs1()].to_i64())
                    } else {
                        E16::from($machine.registers()[i.rs1()].to_u64())
                    };
                    if $cond(b, a, mbit) {
                        $machine.set_bit(i.vd(), j);
                    } else {
                        $machine.clr_bit(i.vd(), j);
                    };
                }
                32 => {
                    let b = E32::get($machine.element_ref(i.vs2(), sew, j));
                    let a = if $sign != 0 {
                        E32::from($machine.registers()[i.rs1()].to_i64())
                    } else {
                        E32::from($machine.registers()[i.rs1()].to_u64())
                    };
                    if $cond(b, a, mbit) {
                        $machine.set_bit(i.vd(), j);
                    } else {
                        $machine.clr_bit(i.vd(), j);
                    };
                }
                64 => {
                    let b = E64::get($machine.element_ref(i.vs2(), sew, j));
                    let a = if $sign != 0 {
                        E64::from($machine.registers()[i.rs1()].to_i64())
                    } else {
                        E64::from($machine.registers()[i.rs1()].to_u64())
                    };
                    if $cond(b, a, mbit) {
                        $machine.set_bit(i.vd(), j);
                    } else {
                        $machine.clr_bit(i.vd(), j);
                    };
                }
                128 => {
                    let b = E128::get($machine.element_ref(i.vs2(), sew, j));
                    let a = if $sign != 0 {
                        E128::from($machine.registers()[i.rs1()].to_i64())
                    } else {
                        E128::from($machine.registers()[i.rs1()].to_u64())
                    };
                    if $cond(b, a, mbit) {
                        $machine.set_bit(i.vd(), j);
                    } else {
                        $machine.clr_bit(i.vd(), j);
                    };
                }
                256 => {
                    let b = E256::get($machine.element_ref(i.vs2(), sew, j));
                    let a = if $sign != 0 {
                        E256::from($machine.registers()[i.rs1()].to_i64())
                    } else {
                        E256::from($machine.registers()[i.rs1()].to_u64())
                    };
                    if $cond(b, a, mbit) {
                        $machine.set_bit(i.vd(), j);
                    } else {
                        $machine.clr_bit(i.vd(), j);
                    };
                }
                512 => {
                    let b = E512::get($machine.element_ref(i.vs2(), sew, j));
                    let a = if $sign != 0 {
                        E512::from($machine.registers()[i.rs1()].to_i64())
                    } else {
                        E512::from($machine.registers()[i.rs1()].to_u64())
                    };
                    if $cond(b, a, mbit) {
                        $machine.set_bit(i.vd(), j);
                    } else {
                        $machine.clr_bit(i.vd(), j);
                    };
                }
                1024 => {
                    let b = E1024::get($machine.element_ref(i.vs2(), sew, j));
                    let a = if $sign != 0 {
                        E1024::from($machine.registers()[i.rs1()].to_i64())
                    } else {
                        E1024::from($machine.registers()[i.rs1()].to_u64())
                    };
                    if $cond(b, a, mbit) {
                        $machine.set_bit(i.vd(), j);
                    } else {
                        $machine.clr_bit(i.vd(), j);
                    };
                }
                _ => unreachable!(),
            }
        }
    };
}

macro_rules! m_vxm_loop_s {
    ($inst:expr, $machine:expr, $cond:expr) => {
        m_vxm_loop!($inst, $machine, $cond, 1);
    };
}

macro_rules! m_vim_loop {
    ($inst:expr, $machine:expr, $cond:expr, $sign:expr) => {
        require_vill!($machine);

        let i = VItype($inst);
        let sew = $machine.vsew();
        for j in 0..$machine.vl() as usize {
            let mbit = $machine.get_bit(0, j);
            match sew {
                8 => {
                    let b = E8::get($machine.element_ref(i.vs2(), sew, j));
                    let a = if $sign != 0 {
                        E8::from(i.immediate_s())
                    } else {
                        E8::from(i.immediate_u())
                    };
                    if $cond(b, a, mbit) {
                        $machine.set_bit(i.vd(), j);
                    } else {
                        $machine.clr_bit(i.vd(), j);
                    };
                }
                16 => {
                    let b = E16::get($machine.element_ref(i.vs2(), sew, j));
                    let a = if $sign != 0 {
                        E16::from(i.immediate_s())
                    } else {
                        E16::from(i.immediate_u())
                    };
                    if $cond(b, a, mbit) {
                        $machine.set_bit(i.vd(), j);
                    } else {
                        $machine.clr_bit(i.vd(), j);
                    };
                }
                32 => {
                    let b = E32::get($machine.element_ref(i.vs2(), sew, j));
                    let a = if $sign != 0 {
                        E32::from(i.immediate_s())
                    } else {
                        E32::from(i.immediate_u())
                    };
                    if $cond(b, a, mbit) {
                        $machine.set_bit(i.vd(), j);
                    } else {
                        $machine.clr_bit(i.vd(), j);
                    };
                }
                64 => {
                    let b = E64::get($machine.element_ref(i.vs2(), sew, j));
                    let a = if $sign != 0 {
                        E64::from(i.immediate_s())
                    } else {
                        E64::from(i.immediate_u())
                    };
                    if $cond(b, a, mbit) {
                        $machine.set_bit(i.vd(), j);
                    } else {
                        $machine.clr_bit(i.vd(), j);
                    };
                }
                128 => {
                    let b = E128::get($machine.element_ref(i.vs2(), sew, j));
                    let a = if $sign != 0 {
                        E128::from(i.immediate_s())
                    } else {
                        E128::from(i.immediate_u())
                    };
                    if $cond(b, a, mbit) {
                        $machine.set_bit(i.vd(), j);
                    } else {
                        $machine.clr_bit(i.vd(), j);
                    };
                }
                256 => {
                    let b = E256::get($machine.element_ref(i.vs2(), sew, j));
                    let a = if $sign != 0 {
                        E256::from(i.immediate_s())
                    } else {
                        E256::from(i.immediate_u())
                    };
                    if $cond(b, a, mbit) {
                        $machine.set_bit(i.vd(), j);
                    } else {
                        $machine.clr_bit(i.vd(), j);
                    };
                }
                512 => {
                    let b = E512::get($machine.element_ref(i.vs2(), sew, j));
                    let a = if $sign != 0 {
                        E512::from(i.immediate_s())
                    } else {
                        E512::from(i.immediate_u())
                    };
                    if $cond(b, a, mbit) {
                        $machine.set_bit(i.vd(), j);
                    } else {
                        $machine.clr_bit(i.vd(), j);
                    };
                }
                1024 => {
                    let b = E1024::get($machine.element_ref(i.vs2(), sew, j));
                    let a = if $sign != 0 {
                        E1024::from(i.immediate_s())
                    } else {
                        E1024::from(i.immediate_u())
                    };
                    if $cond(b, a, mbit) {
                        $machine.set_bit(i.vd(), j);
                    } else {
                        $machine.clr_bit(i.vd(), j);
                    };
                }
                _ => unreachable!(),
            }
        }
    };
}

macro_rules! m_vim_loop_s {
    ($inst:expr, $machine:expr, $cond:expr) => {
        m_vim_loop!($inst, $machine, $cond, 1);
    };
}

macro_rules! v_vv_loop_destructive {
    ($inst:expr, $machine:expr, $body:expr) => {
        require_vill!($machine);

        let i = VVtype($inst);
        let sew = $machine.vsew();
        for j in 0..$machine.vl() as usize {
            if i.vm() == 0 && !$machine.get_bit(0, j) {
                continue;
            }
            match sew {
                8 => {
                    let b = E8::get($machine.element_ref(i.vs2(), sew, j));
                    let a = E8::get($machine.element_ref(i.vs1(), sew, j));
                    let c = E8::get($machine.element_ref(i.vd(), sew, j));
                    let r = $body(b, a, c);
                    r.put($machine.element_mut(i.vd(), sew, j));
                }
                16 => {
                    let b = E16::get($machine.element_ref(i.vs2(), sew, j));
                    let a = E16::get($machine.element_ref(i.vs1(), sew, j));
                    let c = E16::get($machine.element_ref(i.vd(), sew, j));
                    let r = $body(b, a, c);
                    r.put($machine.element_mut(i.vd(), sew, j));
                }
                32 => {
                    let b = E32::get($machine.element_ref(i.vs2(), sew, j));
                    let a = E32::get($machine.element_ref(i.vs1(), sew, j));
                    let c = E32::get($machine.element_ref(i.vd(), sew, j));
                    let r = $body(b, a, c);
                    r.put($machine.element_mut(i.vd(), sew, j));
                }
                64 => {
                    let b = E64::get($machine.element_ref(i.vs2(), sew, j));
                    let a = E64::get($machine.element_ref(i.vs1(), sew, j));
                    let c = E64::get($machine.element_ref(i.vd(), sew, j));
                    let r = $body(b, a, c);
                    r.put($machine.element_mut(i.vd(), sew, j));
                }
                128 => {
                    let b = E128::get($machine.element_ref(i.vs2(), sew, j));
                    let a = E128::get($machine.element_ref(i.vs1(), sew, j));
                    let c = E128::get($machine.element_ref(i.vd(), sew, j));
                    let r = $body(b, a, c);
                    r.put($machine.element_mut(i.vd(), sew, j));
                }
                256 => {
                    let b = E256::get($machine.element_ref(i.vs2(), sew, j));
                    let a = E256::get($machine.element_ref(i.vs1(), sew, j));
                    let c = E256::get($machine.element_ref(i.vd(), sew, j));
                    let r = $body(b, a, c);
                    r.put($machine.element_mut(i.vd(), sew, j));
                }
                512 => {
                    let b = E512::get($machine.element_ref(i.vs2(), sew, j));
                    let a = E512::get($machine.element_ref(i.vs1(), sew, j));
                    let c = E512::get($machine.element_ref(i.vd(), sew, j));
                    let r = $body(b, a, c);
                    r.put($machine.element_mut(i.vd(), sew, j));
                }
                1024 => {
                    let b = E1024::get($machine.element_ref(i.vs2(), sew, j));
                    let a = E1024::get($machine.element_ref(i.vs1(), sew, j));
                    let c = E1024::get($machine.element_ref(i.vd(), sew, j));
                    let r = $body(b, a, c);
                    r.put($machine.element_mut(i.vd(), sew, j));
                }
                _ => unreachable!(),
            }
        }
    };
}

macro_rules! v_vv_loop_destructive_s {
    ($inst:expr, $machine:expr, $body:expr) => {
        v_vv_loop_destructive!($inst, $machine, $body);
    };
}

macro_rules! v_vx_loop_destructive {
    ($inst:expr, $machine:expr, $body:expr, $sign:expr) => {
        require_vill!($machine);

        let i = VXtype($inst);
        let sew = $machine.vsew();
        for j in 0..$machine.vl() as usize {
            if i.vm() == 0 && !$machine.get_bit(0, j) {
                continue;
            }
            match sew {
                8 => {
                    let b = E8::get($machine.element_ref(i.vs2(), sew, j));
                    let a = if $sign != 0 {
                        E8::from($machine.registers()[i.rs1()].to_i64())
                    } else {
                        E8::from($machine.registers()[i.rs1()].to_u64())
                    };
                    let c = E8::get($machine.element_ref(i.vd(), sew, j));
                    let r = $body(b, a, c);
                    r.put($machine.element_mut(i.vd(), sew, j));
                }
                16 => {
                    let b = E16::get($machine.element_ref(i.vs2(), sew, j));
                    let a = if $sign != 0 {
                        E16::from($machine.registers()[i.rs1()].to_i64())
                    } else {
                        E16::from($machine.registers()[i.rs1()].to_u64())
                    };
                    let c = E16::get($machine.element_ref(i.vd(), sew, j));
                    let r = $body(b, a, c);
                    r.put($machine.element_mut(i.vd(), sew, j));
                }
                32 => {
                    let b = E32::get($machine.element_ref(i.vs2(), sew, j));
                    let a = if $sign != 0 {
                        E32::from($machine.registers()[i.rs1()].to_i64())
                    } else {
                        E32::from($machine.registers()[i.rs1()].to_u64())
                    };
                    let c = E32::get($machine.element_ref(i.vd(), sew, j));
                    let r = $body(b, a, c);
                    r.put($machine.element_mut(i.vd(), sew, j));
                }
                64 => {
                    let b = E64::get($machine.element_ref(i.vs2(), sew, j));
                    let a = if $sign != 0 {
                        E64::from($machine.registers()[i.rs1()].to_i64())
                    } else {
                        E64::from($machine.registers()[i.rs1()].to_u64())
                    };
                    let c = E64::get($machine.element_ref(i.vd(), sew, j));
                    let r = $body(b, a, c);
                    r.put($machine.element_mut(i.vd(), sew, j));
                }
                128 => {
                    let b = E128::get($machine.element_ref(i.vs2(), sew, j));
                    let a = if $sign != 0 {
                        E128::from($machine.registers()[i.rs1()].to_i64())
                    } else {
                        E128::from($machine.registers()[i.rs1()].to_u64())
                    };
                    let c = E128::get($machine.element_ref(i.vd(), sew, j));
                    let r = $body(b, a, c);
                    r.put($machine.element_mut(i.vd(), sew, j));
                }
                256 => {
                    let b = E256::get($machine.element_ref(i.vs2(), sew, j));
                    let a = if $sign != 0 {
                        E256::from($machine.registers()[i.rs1()].to_i64())
                    } else {
                        E256::from($machine.registers()[i.rs1()].to_u64())
                    };
                    let c = E256::get($machine.element_ref(i.vd(), sew, j));
                    let r = $body(b, a, c);
                    r.put($machine.element_mut(i.vd(), sew, j));
                }
                512 => {
                    let b = E512::get($machine.element_ref(i.vs2(), sew, j));
                    let a = if $sign != 0 {
                        E512::from($machine.registers()[i.rs1()].to_i64())
                    } else {
                        E512::from($machine.registers()[i.rs1()].to_u64())
                    };
                    let c = E512::get($machine.element_ref(i.vd(), sew, j));
                    let r = $body(b, a, c);
                    r.put($machine.element_mut(i.vd(), sew, j));
                }
                1024 => {
                    let b = E1024::get($machine.element_ref(i.vs2(), sew, j));
                    let a = if $sign != 0 {
                        E1024::from($machine.registers()[i.rs1()].to_i64())
                    } else {
                        E1024::from($machine.registers()[i.rs1()].to_u64())
                    };
                    let c = E1024::get($machine.element_ref(i.vd(), sew, j));
                    let r = $body(b, a, c);
                    r.put($machine.element_mut(i.vd(), sew, j));
                }
                _ => unreachable!(),
            }
        }
    };
}

macro_rules! v_vx_loop_destructive_s {
    ($inst:expr, $machine:expr, $body:expr) => {
        v_vx_loop_destructive!($inst, $machine, $body, 1);
    };
}

// macro_rules! v_vx_loop_destructive_u {
//     ($inst:expr, $machine:expr, $body:expr) => {
//         v_vx_loop_destructive!($inst, $machine, $body, 0);
//     };
// }

macro_rules! w_vv_loop_destructive {
    ($inst:expr, $machine:expr, $body:expr) => {
        require_vill!($machine);

        let i = VVtype($inst);
        let sew = $machine.vsew();
        for j in 0..$machine.vl() as usize {
            if i.vm() == 0 && !$machine.get_bit(0, j) {
                continue;
            }
            match sew {
                8 => {
                    let b = E8::get($machine.element_ref(i.vs2(), sew, j));
                    let a = E8::get($machine.element_ref(i.vs1(), sew, j));
                    let c = E8::get($machine.element_ref(i.vd(), sew, j * 2));
                    let d = E8::get($machine.element_ref(i.vd(), sew, j * 2 + 1));
                    let r = $body(b, a, c, d);
                    r.0.put($machine.element_mut(i.vd(), sew, j * 2));
                    r.1.put($machine.element_mut(i.vd(), sew, j * 2 + 1));
                }
                16 => {
                    let b = E16::get($machine.element_ref(i.vs2(), sew, j));
                    let a = E16::get($machine.element_ref(i.vs1(), sew, j));
                    let c = E16::get($machine.element_ref(i.vd(), sew, j * 2));
                    let d = E16::get($machine.element_ref(i.vd(), sew, j * 2 + 1));
                    let r = $body(b, a, c, d);
                    r.0.put($machine.element_mut(i.vd(), sew, j * 2));
                    r.1.put($machine.element_mut(i.vd(), sew, j * 2 + 1));
                }
                32 => {
                    let b = E32::get($machine.element_ref(i.vs2(), sew, j));
                    let a = E32::get($machine.element_ref(i.vs1(), sew, j));
                    let c = E32::get($machine.element_ref(i.vd(), sew, j * 2));
                    let d = E32::get($machine.element_ref(i.vd(), sew, j * 2 + 1));
                    let r = $body(b, a, c, d);
                    r.0.put($machine.element_mut(i.vd(), sew, j * 2));
                    r.1.put($machine.element_mut(i.vd(), sew, j * 2 + 1));
                }
                64 => {
                    let b = E64::get($machine.element_ref(i.vs2(), sew, j));
                    let a = E64::get($machine.element_ref(i.vs1(), sew, j));
                    let c = E64::get($machine.element_ref(i.vd(), sew, j * 2));
                    let d = E64::get($machine.element_ref(i.vd(), sew, j * 2 + 1));
                    let r = $body(b, a, c, d);
                    r.0.put($machine.element_mut(i.vd(), sew, j * 2));
                    r.1.put($machine.element_mut(i.vd(), sew, j * 2 + 1));
                }
                128 => {
                    let b = E128::get($machine.element_ref(i.vs2(), sew, j));
                    let a = E128::get($machine.element_ref(i.vs1(), sew, j));
                    let c = E128::get($machine.element_ref(i.vd(), sew, j * 2));
                    let d = E128::get($machine.element_ref(i.vd(), sew, j * 2 + 1));
                    let r = $body(b, a, c, d);
                    r.0.put($machine.element_mut(i.vd(), sew, j * 2));
                    r.1.put($machine.element_mut(i.vd(), sew, j * 2 + 1));
                }
                256 => {
                    let b = E256::get($machine.element_ref(i.vs2(), sew, j));
                    let a = E256::get($machine.element_ref(i.vs1(), sew, j));
                    let c = E256::get($machine.element_ref(i.vd(), sew, j * 2));
                    let d = E256::get($machine.element_ref(i.vd(), sew, j * 2 + 1));
                    let r = $body(b, a, c, d);
                    r.0.put($machine.element_mut(i.vd(), sew, j * 2));
                    r.1.put($machine.element_mut(i.vd(), sew, j * 2 + 1));
                }
                512 => {
                    let b = E512::get($machine.element_ref(i.vs2(), sew, j));
                    let a = E512::get($machine.element_ref(i.vs1(), sew, j));
                    let c = E512::get($machine.element_ref(i.vd(), sew, j * 2));
                    let d = E512::get($machine.element_ref(i.vd(), sew, j * 2 + 1));
                    let r = $body(b, a, c, d);
                    r.0.put($machine.element_mut(i.vd(), sew, j * 2));
                    r.1.put($machine.element_mut(i.vd(), sew, j * 2 + 1));
                }
                1024 => {
                    let b = E1024::get($machine.element_ref(i.vs2(), sew, j));
                    let a = E1024::get($machine.element_ref(i.vs1(), sew, j));
                    let c = E1024::get($machine.element_ref(i.vd(), sew, j * 2));
                    let d = E1024::get($machine.element_ref(i.vd(), sew, j * 2 + 1));
                    let r = $body(b, a, c, d);
                    r.0.put($machine.element_mut(i.vd(), sew, j * 2));
                    r.1.put($machine.element_mut(i.vd(), sew, j * 2 + 1));
                }
                _ => unreachable!(),
            }
        }
    };
}

macro_rules! w_vv_loop_destructive_s {
    ($inst:expr, $machine:expr, $body:expr) => {
        w_vv_loop_destructive!($inst, $machine, $body);
    };
}

macro_rules! w_vx_loop_destructive {
    ($inst:expr, $machine:expr, $body:expr, $sign:expr) => {
        require_vill!($machine);

        let i = VXtype($inst);
        let sew = $machine.vsew();
        for j in 0..$machine.vl() as usize {
            if i.vm() == 0 && !$machine.get_bit(0, j) {
                continue;
            }
            match sew {
                8 => {
                    let b = E8::get($machine.element_ref(i.vs2(), sew, j));
                    let a = if $sign != 0 {
                        E8::from($machine.registers()[i.rs1()].to_i64())
                    } else {
                        E8::from($machine.registers()[i.rs1()].to_u64())
                    };
                    let c = E8::get($machine.element_ref(i.vd(), sew, j * 2));
                    let d = E8::get($machine.element_ref(i.vd(), sew, j * 2 + 1));
                    let r = $body(b, a, c, d);
                    r.0.put($machine.element_mut(i.vd(), sew, j * 2));
                    r.1.put($machine.element_mut(i.vd(), sew, j * 2 + 1));
                }
                16 => {
                    let b = E16::get($machine.element_ref(i.vs2(), sew, j));
                    let a = if $sign != 0 {
                        E16::from($machine.registers()[i.rs1()].to_i64())
                    } else {
                        E16::from($machine.registers()[i.rs1()].to_u64())
                    };
                    let c = E16::get($machine.element_ref(i.vd(), sew, j * 2));
                    let d = E16::get($machine.element_ref(i.vd(), sew, j * 2 + 1));
                    let r = $body(b, a, c, d);
                    r.0.put($machine.element_mut(i.vd(), sew, j * 2));
                    r.1.put($machine.element_mut(i.vd(), sew, j * 2 + 1));
                }
                32 => {
                    let b = E32::get($machine.element_ref(i.vs2(), sew, j));
                    let a = if $sign != 0 {
                        E32::from($machine.registers()[i.rs1()].to_i64())
                    } else {
                        E32::from($machine.registers()[i.rs1()].to_u64())
                    };
                    let c = E32::get($machine.element_ref(i.vd(), sew, j * 2));
                    let d = E32::get($machine.element_ref(i.vd(), sew, j * 2 + 1));
                    let r = $body(b, a, c, d);
                    r.0.put($machine.element_mut(i.vd(), sew, j * 2));
                    r.1.put($machine.element_mut(i.vd(), sew, j * 2 + 1));
                }
                64 => {
                    let b = E64::get($machine.element_ref(i.vs2(), sew, j));
                    let a = if $sign != 0 {
                        E64::from($machine.registers()[i.rs1()].to_i64())
                    } else {
                        E64::from($machine.registers()[i.rs1()].to_u64())
                    };
                    let c = E64::get($machine.element_ref(i.vd(), sew, j * 2));
                    let d = E64::get($machine.element_ref(i.vd(), sew, j * 2 + 1));
                    let r = $body(b, a, c, d);
                    r.0.put($machine.element_mut(i.vd(), sew, j * 2));
                    r.1.put($machine.element_mut(i.vd(), sew, j * 2 + 1));
                }
                128 => {
                    let b = E128::get($machine.element_ref(i.vs2(), sew, j));
                    let a = if $sign != 0 {
                        E128::from($machine.registers()[i.rs1()].to_i64())
                    } else {
                        E128::from($machine.registers()[i.rs1()].to_u64())
                    };
                    let c = E128::get($machine.element_ref(i.vd(), sew, j * 2));
                    let d = E128::get($machine.element_ref(i.vd(), sew, j * 2 + 1));
                    let r = $body(b, a, c, d);
                    r.0.put($machine.element_mut(i.vd(), sew, j * 2));
                    r.1.put($machine.element_mut(i.vd(), sew, j * 2 + 1));
                }
                256 => {
                    let b = E256::get($machine.element_ref(i.vs2(), sew, j));
                    let a = if $sign != 0 {
                        E256::from($machine.registers()[i.rs1()].to_i64())
                    } else {
                        E256::from($machine.registers()[i.rs1()].to_u64())
                    };
                    let c = E256::get($machine.element_ref(i.vd(), sew, j * 2));
                    let d = E256::get($machine.element_ref(i.vd(), sew, j * 2 + 1));
                    let r = $body(b, a, c, d);
                    r.0.put($machine.element_mut(i.vd(), sew, j * 2));
                    r.1.put($machine.element_mut(i.vd(), sew, j * 2 + 1));
                }
                512 => {
                    let b = E512::get($machine.element_ref(i.vs2(), sew, j));
                    let a = if $sign != 0 {
                        E512::from($machine.registers()[i.rs1()].to_i64())
                    } else {
                        E512::from($machine.registers()[i.rs1()].to_u64())
                    };
                    let c = E512::get($machine.element_ref(i.vd(), sew, j * 2));
                    let d = E512::get($machine.element_ref(i.vd(), sew, j * 2 + 1));
                    let r = $body(b, a, c, d);
                    r.0.put($machine.element_mut(i.vd(), sew, j * 2));
                    r.1.put($machine.element_mut(i.vd(), sew, j * 2 + 1));
                }
                1024 => {
                    let b = E1024::get($machine.element_ref(i.vs2(), sew, j));
                    let a = if $sign != 0 {
                        E1024::from($machine.registers()[i.rs1()].to_i64())
                    } else {
                        E1024::from($machine.registers()[i.rs1()].to_u64())
                    };
                    let c = E1024::get($machine.element_ref(i.vd(), sew, j * 2));
                    let d = E1024::get($machine.element_ref(i.vd(), sew, j * 2 + 1));
                    let r = $body(b, a, c, d);
                    r.0.put($machine.element_mut(i.vd(), sew, j * 2));
                    r.1.put($machine.element_mut(i.vd(), sew, j * 2 + 1));
                }
                _ => unreachable!(),
            }
        }
    };
}

macro_rules! w_vx_loop_destructive_s {
    ($inst:expr, $machine:expr, $body:expr) => {
        w_vx_loop_destructive!($inst, $machine, $body, 1);
    };
}

macro_rules! v_vs_loop {
    ($inst:expr, $machine:expr, $body:expr) => {
        require_vill!($machine);

        let i = VVtype($inst);
        let sew = $machine.vsew();
        let vs1 = $machine.element_ref(i.vs1(), sew, 0).to_vec();
        $machine.element_mut(i.vd(), sew, 0).copy_from_slice(&vs1);
        for j in 0..$machine.vl() as usize {
            if i.vm() == 0 && !$machine.get_bit(0, j) {
                continue;
            }
            match sew {
                8 => {
                    let b = E8::get($machine.element_ref(i.vs2(), sew, j));
                    let a = E8::get($machine.element_ref(i.vd(), sew, 0));
                    let r = $body(b, a);
                    r.put($machine.element_mut(i.vd(), sew, 0));
                }
                16 => {
                    let b = E16::get($machine.element_ref(i.vs2(), sew, j));
                    let a = E16::get($machine.element_ref(i.vd(), sew, 0));
                    let r = $body(b, a);
                    r.put($machine.element_mut(i.vd(), sew, 0));
                }
                32 => {
                    let b = E32::get($machine.element_ref(i.vs2(), sew, j));
                    let a = E32::get($machine.element_ref(i.vd(), sew, 0));
                    let r = $body(b, a);
                    r.put($machine.element_mut(i.vd(), sew, 0));
                }
                64 => {
                    let b = E64::get($machine.element_ref(i.vs2(), sew, j));
                    let a = E64::get($machine.element_ref(i.vd(), sew, 0));
                    let r = $body(b, a);
                    r.put($machine.element_mut(i.vd(), sew, 0));
                }
                128 => {
                    let b = E128::get($machine.element_ref(i.vs2(), sew, j));
                    let a = E128::get($machine.element_ref(i.vd(), sew, 0));
                    let r = $body(b, a);
                    r.put($machine.element_mut(i.vd(), sew, 0));
                }
                256 => {
                    let b = E256::get($machine.element_ref(i.vs2(), sew, j));
                    let a = E256::get($machine.element_ref(i.vd(), sew, 0));
                    let r = $body(b, a);
                    r.put($machine.element_mut(i.vd(), sew, 0));
                }
                512 => {
                    let b = E512::get($machine.element_ref(i.vs2(), sew, j));
                    let a = E512::get($machine.element_ref(i.vd(), sew, 0));
                    let r = $body(b, a);
                    r.put($machine.element_mut(i.vd(), sew, 0));
                }
                1024 => {
                    let b = E1024::get($machine.element_ref(i.vs2(), sew, j));
                    let a = E1024::get($machine.element_ref(i.vd(), sew, 0));
                    let r = $body(b, a);
                    r.put($machine.element_mut(i.vd(), sew, 0));
                }
                _ => unreachable!(),
            }
        }
    };
}

macro_rules! v_vs_loop_s {
    ($inst:expr, $machine:expr, $body:expr) => {
        v_vs_loop!($inst, $machine, $body);
    };
}

macro_rules! w_vs_loop {
    ($inst:expr, $machine:expr, $body:expr, $sign:expr) => {
        require_vill!($machine);

        let i = VVtype($inst);
        let sew = $machine.vsew();
        let vs1 = $machine.element_ref(i.vs1(), sew * 2, 0).to_vec();
        $machine
            .element_mut(i.vd(), sew * 2, 0)
            .copy_from_slice(&vs1);
        for j in 0..$machine.vl() as usize {
            if i.vm() == 0 && !$machine.get_bit(0, j) {
                continue;
            }
            match sew {
                8 => {
                    let b = E16::get($machine.element_ref(i.vd(), sew * 2, 0));
                    let a = E8::get($machine.element_ref(i.vs2(), sew, j));
                    let a = if $sign != 0 && a.is_negative() {
                        E16::from(a).lo_sext()
                    } else {
                        E16::from(a)
                    };
                    let r = $body(b, a);
                    r.put($machine.element_mut(i.vd(), sew * 2, 0));
                }
                16 => {
                    let b = E32::get($machine.element_ref(i.vd(), sew * 2, 0));
                    let a = E16::get($machine.element_ref(i.vs2(), sew, j));
                    let a = if $sign != 0 && a.is_negative() {
                        E32::from(a).lo_sext()
                    } else {
                        E32::from(a)
                    };
                    let r = $body(b, a);
                    r.put($machine.element_mut(i.vd(), sew * 2, 0));
                }
                32 => {
                    let b = E64::get($machine.element_ref(i.vd(), sew * 2, 0));
                    let a = E32::get($machine.element_ref(i.vs2(), sew, j));
                    let a = if $sign != 0 && a.is_negative() {
                        E64::from(a).lo_sext()
                    } else {
                        E64::from(a)
                    };
                    let r = $body(b, a);
                    r.put($machine.element_mut(i.vd(), sew * 2, 0));
                }
                64 => {
                    let b = E128::get($machine.element_ref(i.vd(), sew * 2, 0));
                    let a = E64::get($machine.element_ref(i.vs2(), sew, j));
                    let a = if $sign != 0 && a.is_negative() {
                        E128::from(a).lo_sext()
                    } else {
                        E128::from(a)
                    };
                    let r = $body(b, a);
                    r.put($machine.element_mut(i.vd(), sew * 2, 0));
                }
                128 => {
                    let b = E256::get($machine.element_ref(i.vd(), sew * 2, 0));
                    let a = E128::get($machine.element_ref(i.vs2(), sew, j));
                    let a = if $sign != 0 && a.is_negative() {
                        E256::from(a).lo_sext()
                    } else {
                        E256::from(a)
                    };
                    let r = $body(b, a);
                    r.put($machine.element_mut(i.vd(), sew * 2, 0));
                }
                256 => {
                    let b = E512::get($machine.element_ref(i.vd(), sew * 2, 0));
                    let a = E256::get($machine.element_ref(i.vs2(), sew, j));
                    let a = if $sign != 0 && a.is_negative() {
                        E512::from(a).lo_sext()
                    } else {
                        E512::from(a)
                    };
                    let r = $body(b, a);
                    r.put($machine.element_mut(i.vd(), sew * 2, 0));
                }
                512 => {
                    let b = E1024::get($machine.element_ref(i.vd(), sew * 2, 0));
                    let a = E512::get($machine.element_ref(i.vs2(), sew, j));
                    let a = if $sign != 0 && a.is_negative() {
                        E1024::from(a).lo_sext()
                    } else {
                        E1024::from(a)
                    };
                    let r = $body(b, a);
                    r.put($machine.element_mut(i.vd(), sew * 2, 0));
                }
                1024 => {
                    let b = E2048::get($machine.element_ref(i.vd(), sew * 2, 0));
                    let a = E1024::get($machine.element_ref(i.vs2(), sew, j));
                    let a = if $sign != 0 && a.is_negative() {
                        E2048::from(a).lo_sext()
                    } else {
                        E2048::from(a)
                    };
                    let r = $body(b, a);
                    r.put($machine.element_mut(i.vd(), sew * 2, 0));
                }
                _ => unreachable!(),
            }
        }
    };
}

macro_rules! w_vs_loop_s {
    ($inst:expr, $machine:expr, $body:expr) => {
        w_vs_loop!($inst, $machine, $body, 1);
    };
}

macro_rules! w_vs_loop_u {
    ($inst:expr, $machine:expr, $body:expr) => {
        w_vs_loop!($inst, $machine, $body, 0);
    };
}

macro_rules! v_vv_loop_ext_s {
    ($inst:expr, $machine:expr, $size:expr) => {
        require_vill!($machine);

        let i = VVtype($inst);
        let sew = $machine.vsew();
        for j in 0..$machine.vl() as usize {
            if i.vm() == 0 && !$machine.get_bit(0, j) {
                continue;
            }
            let mut b = $machine.element_ref(i.vs2(), sew / $size, j).to_vec();
            if b.last().unwrap() > &127 {
                b.resize(sew as usize >> 3, 0xff);
            } else {
                b.resize(sew as usize >> 3, 0x00);
            }
            $machine.element_mut(i.vd(), sew, j).copy_from_slice(&b);
        }
    };
}

macro_rules! v_vv_loop_ext_u {
    ($inst:expr, $machine:expr, $size:expr) => {
        require_vill!($machine);

        let i = VVtype($inst);
        let sew = $machine.vsew();
        for j in 0..$machine.vl() as usize {
            if i.vm() == 0 && !$machine.get_bit(0, j) {
                continue;
            }
            let mut b = $machine.element_ref(i.vs2(), sew / $size, j).to_vec();
            b.resize(sew as usize >> 3, 0x00);
            $machine.element_mut(i.vd(), sew, j).copy_from_slice(&b);
        }
    };
}

macro_rules! x_m_loop {
    ($inst:expr, $machine:expr, $body:expr) => {
        require_vill!($machine);

        let i = VVtype($inst);
        let vs2 = E2048::get($machine.element_ref(i.vs2(), VLEN as u64, 0));
        let m = if i.vm() == 0 {
            E2048::get($machine.element_ref(0, VLEN as u64, 0))
        } else {
            E2048::MAX_U
        };
        let vl = $machine.vl();
        let r = $body(vs2, m, vl);
        update_register($machine, i.vd(), Mac::REG::from_u64(r));
    };
}

macro_rules! m_m_loop {
    ($inst:expr, $machine:expr, $body:expr) => {
        require_vill!($machine);

        let i = VVtype($inst);
        let vs2 = E2048::get($machine.element_ref(i.vs2(), VLEN as u64, 0));
        let vd = E2048::get($machine.element_ref(i.vd(), VLEN as u64, 0));
        let m = if i.vm() == 0 {
            E2048::get($machine.element_ref(0, VLEN as u64, 0))
        } else {
            E2048::MAX_U
        };
        let vl = $machine.vl();
        let r = $body(vs2, vd, m, vl);
        r.put($machine.element_mut(i.vd(), VLEN as u64, 0));
    };
}

macro_rules! iota_loop {
    ($inst:expr, $machine:expr) => {
        require_vill!($machine);

        let i = VVtype($inst);
        let sew = $machine.vsew();
        let mut iota: u32 = 0;
        for j in 0..$machine.vl() as usize {
            if i.vm() == 0 && !$machine.get_bit(0, j) {
                continue;
            }
            match sew {
                8 => E8::from(iota as u8).put($machine.element_mut(i.vd(), sew, j)),
                16 => E16::from(iota as u16).put($machine.element_mut(i.vd(), sew, j)),
                32 => E32::from(iota as u16).put($machine.element_mut(i.vd(), sew, j)),
                64 => E64::from(iota as u16).put($machine.element_mut(i.vd(), sew, j)),
                128 => E128::from(iota as u16).put($machine.element_mut(i.vd(), sew, j)),
                256 => E256::from(iota as u16).put($machine.element_mut(i.vd(), sew, j)),
                512 => E512::from(iota as u16).put($machine.element_mut(i.vd(), sew, j)),
                1024 => E1024::from(iota as u16).put($machine.element_mut(i.vd(), sew, j)),
                _ => unreachable!(),
            }
            if $machine.get_bit(i.vs2(), j) {
                iota += 1;
            }
        }
    };
}

macro_rules! id_loop {
    ($inst:expr, $machine:expr) => {
        require_vill!($machine);

        let i = VVtype($inst);
        let sew = $machine.vsew();
        for j in 0..$machine.vl() as usize {
            if i.vm() == 0 && !$machine.get_bit(0, j) {
                continue;
            }
            match sew {
                8 => E8::from(j as u8).put($machine.element_mut(i.vd(), sew, j)),
                16 => E16::from(j as u16).put($machine.element_mut(i.vd(), sew, j)),
                32 => E32::from(j as u16).put($machine.element_mut(i.vd(), sew, j)),
                64 => E64::from(j as u16).put($machine.element_mut(i.vd(), sew, j)),
                128 => E128::from(j as u16).put($machine.element_mut(i.vd(), sew, j)),
                256 => E256::from(j as u16).put($machine.element_mut(i.vd(), sew, j)),
                512 => E512::from(j as u16).put($machine.element_mut(i.vd(), sew, j)),
                1024 => E1024::from(j as u16).put($machine.element_mut(i.vd(), sew, j)),
                _ => unreachable!(),
            }
        }
    };
}

macro_rules! vmv_r {
    ($inst:expr, $machine:expr, $size:expr) => {
        let i = VItype($inst);
        let data = $machine
            .element_ref(i.vs2(), (VLEN as u64) * $size, 0)
            .to_vec();
        $machine
            .element_mut(i.vd(), (VLEN as u64) * $size, 0)
            .copy_from_slice(&data);
    };
}

pub(crate) use require;
pub(crate) use require_align;
pub(crate) use require_vill;

pub(crate) use id_loop;
pub(crate) use iota_loop;
pub(crate) use ld;
pub(crate) use ld_index;
pub(crate) use ld_whole;
pub(crate) use m_m_loop;
pub(crate) use m_mm_loop;
pub(crate) use m_vi_loop;
pub(crate) use m_vi_loop_s;
pub(crate) use m_vi_loop_u;
pub(crate) use m_vim_loop;
pub(crate) use m_vim_loop_s;
pub(crate) use m_vv_loop;
pub(crate) use m_vv_loop_s;
pub(crate) use m_vv_loop_u;
pub(crate) use m_vvm_loop;
pub(crate) use m_vvm_loop_s;
pub(crate) use m_vx_loop;
pub(crate) use m_vx_loop_s;
pub(crate) use m_vx_loop_u;
pub(crate) use m_vxm_loop;
pub(crate) use m_vxm_loop_s;
pub(crate) use sd;
pub(crate) use sd_index;
pub(crate) use sd_whole;
pub(crate) use v_vi_loop;
pub(crate) use v_vi_loop_s;
pub(crate) use v_vi_loop_u;
pub(crate) use v_vim_loop;
pub(crate) use v_vim_loop_s;
pub(crate) use v_vs_loop;
pub(crate) use v_vs_loop_s;
pub(crate) use v_vv_loop;
pub(crate) use v_vv_loop_destructive;
pub(crate) use v_vv_loop_destructive_s;
pub(crate) use v_vv_loop_ext_s;
pub(crate) use v_vv_loop_ext_u;
pub(crate) use v_vv_loop_s;
pub(crate) use v_vv_loop_u;
pub(crate) use v_vvm_loop;
pub(crate) use v_vvm_loop_s;
pub(crate) use v_vx_loop;
pub(crate) use v_vx_loop_destructive;
pub(crate) use v_vx_loop_destructive_s;
pub(crate) use v_vx_loop_s;
pub(crate) use v_vx_loop_u;
pub(crate) use v_vxm_loop;
pub(crate) use v_vxm_loop_s;
pub(crate) use v_wi_loop;
pub(crate) use v_wi_loop_u;
pub(crate) use v_wv_loop;
pub(crate) use v_wv_loop_u;
pub(crate) use v_wx_loop;
pub(crate) use v_wx_loop_u;
pub(crate) use vmv_r;
pub(crate) use w_vs_loop;
pub(crate) use w_vs_loop_s;
pub(crate) use w_vs_loop_u;
pub(crate) use w_vv_loop;
pub(crate) use w_vv_loop_destructive;
pub(crate) use w_vv_loop_destructive_s;
pub(crate) use w_vv_loop_s;
pub(crate) use w_vv_loop_u;
pub(crate) use w_vx_loop;
pub(crate) use w_vx_loop_destructive;
pub(crate) use w_vx_loop_destructive_s;
pub(crate) use w_vx_loop_s;
pub(crate) use w_vx_loop_u;
pub(crate) use w_wv_loop;
pub(crate) use w_wv_loop_s;
pub(crate) use w_wv_loop_u;
pub(crate) use w_wx_loop;
pub(crate) use w_wx_loop_s;
pub(crate) use w_wx_loop_u;
pub(crate) use x_m_loop;

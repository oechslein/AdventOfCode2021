
    use fxhash::FxHashSet;

    fn alu_add(arg1: i64, arg2: i64) -> i64 {
        arg1 + arg2
    }
    
    fn alu_mul(arg1: i64, arg2: i64) -> i64 {
        arg1 * arg2
    }
    
    fn alu_mod(arg1: i64, arg2: i64) -> i64 {
        arg1 % arg2
    }
    
    //
    fn alu_div(arg1: i64, arg2: i64) -> i64 {
        arg1 / arg2
    }
    
    fn alu_eql(arg1: i64, arg2: i64) -> i64 {
        if arg1 == arg2 {
            1
        } else {
            0
        }
    }

    fn calc_step_01(input: i64, mut x: i64, mut y: i64, mut z: i64, mut w: i64) -> (i64,i64,i64,i64) {
    w = input;
    x = 0; // x = alu_mul(x, 0);
    x = alu_add(x, z);
    x = alu_mod(x, 26);
    // z = z; // z = alu_div(z, 1);
    x = alu_add(x, 10);
    x = alu_eql(x, w);
    x = alu_eql(x, 0);
    y = 0; // y = alu_mul(y, 0);
    y = alu_add(y, 25);
    y = alu_mul(y, x);
    y = alu_add(y, 1);
    z = alu_mul(z, y);
    y = 0; // y = alu_mul(y, 0);
    y = alu_add(y, w);
    y = alu_add(y, 2);
    y = alu_mul(y, x);
    z = alu_add(z, y);
    (x,y,z,w)
}
fn calc_step_02(input: i64, mut x: i64, mut y: i64, mut z: i64, mut w: i64) -> (i64,i64,i64,i64) {
    w = input;
    x = 0; // x = alu_mul(x, 0);
    x = alu_add(x, z);
    x = alu_mod(x, 26);
    // z = z; // z = alu_div(z, 1);
    x = alu_add(x, 14);
    x = alu_eql(x, w);
    x = alu_eql(x, 0);
    y = 0; // y = alu_mul(y, 0);
    y = alu_add(y, 25);
    y = alu_mul(y, x);
    y = alu_add(y, 1);
    z = alu_mul(z, y);
    y = 0; // y = alu_mul(y, 0);
    y = alu_add(y, w);
    y = alu_add(y, 13);
    y = alu_mul(y, x);
    z = alu_add(z, y);
    (x,y,z,w)
}
fn calc_step_03(input: i64, mut x: i64, mut y: i64, mut z: i64, mut w: i64) -> (i64,i64,i64,i64) {
    w = input;
    x = 0; // x = alu_mul(x, 0);
    x = alu_add(x, z);
    x = alu_mod(x, 26);
    // z = z; // z = alu_div(z, 1);
    x = alu_add(x, 14);
    x = alu_eql(x, w);
    x = alu_eql(x, 0);
    y = 0; // y = alu_mul(y, 0);
    y = alu_add(y, 25);
    y = alu_mul(y, x);
    y = alu_add(y, 1);
    z = alu_mul(z, y);
    y = 0; // y = alu_mul(y, 0);
    y = alu_add(y, w);
    y = alu_add(y, 13);
    y = alu_mul(y, x);
    z = alu_add(z, y);
    (x,y,z,w)
}
fn calc_step_04(input: i64, mut x: i64, mut y: i64, mut z: i64, mut w: i64) -> (i64,i64,i64,i64) {
    w = input;
    x = 0; // x = alu_mul(x, 0);
    x = alu_add(x, z);
    x = alu_mod(x, 26);
    z = alu_div(z, 26);
    x = alu_add(x, -13);
    x = alu_eql(x, w);
    x = alu_eql(x, 0);
    y = 0; // y = alu_mul(y, 0);
    y = alu_add(y, 25);
    y = alu_mul(y, x);
    y = alu_add(y, 1);
    z = alu_mul(z, y);
    y = 0; // y = alu_mul(y, 0);
    y = alu_add(y, w);
    y = alu_add(y, 9);
    y = alu_mul(y, x);
    z = alu_add(z, y);
    (x,y,z,w)
}
fn calc_step_05(input: i64, mut x: i64, mut y: i64, mut z: i64, mut w: i64) -> (i64,i64,i64,i64) {
    w = input;
    x = 0; // x = alu_mul(x, 0);
    x = alu_add(x, z);
    x = alu_mod(x, 26);
    // z = z; // z = alu_div(z, 1);
    x = alu_add(x, 10);
    x = alu_eql(x, w);
    x = alu_eql(x, 0);
    y = 0; // y = alu_mul(y, 0);
    y = alu_add(y, 25);
    y = alu_mul(y, x);
    y = alu_add(y, 1);
    z = alu_mul(z, y);
    y = 0; // y = alu_mul(y, 0);
    y = alu_add(y, w);
    y = alu_add(y, 15);
    y = alu_mul(y, x);
    z = alu_add(z, y);
    (x,y,z,w)
}
fn calc_step_06(input: i64, mut x: i64, mut y: i64, mut z: i64, mut w: i64) -> (i64,i64,i64,i64) {
    w = input;
    x = 0; // x = alu_mul(x, 0);
    x = alu_add(x, z);
    x = alu_mod(x, 26);
    z = alu_div(z, 26);
    x = alu_add(x, -13);
    x = alu_eql(x, w);
    x = alu_eql(x, 0);
    y = 0; // y = alu_mul(y, 0);
    y = alu_add(y, 25);
    y = alu_mul(y, x);
    y = alu_add(y, 1);
    z = alu_mul(z, y);
    y = 0; // y = alu_mul(y, 0);
    y = alu_add(y, w);
    y = alu_add(y, 3);
    y = alu_mul(y, x);
    z = alu_add(z, y);
    (x,y,z,w)
}
fn calc_step_07(input: i64, mut x: i64, mut y: i64, mut z: i64, mut w: i64) -> (i64,i64,i64,i64) {
    w = input;
    x = 0; // x = alu_mul(x, 0);
    x = alu_add(x, z);
    x = alu_mod(x, 26);
    z = alu_div(z, 26);
    x = alu_add(x, -7);
    x = alu_eql(x, w);
    x = alu_eql(x, 0);
    y = 0; // y = alu_mul(y, 0);
    y = alu_add(y, 25);
    y = alu_mul(y, x);
    y = alu_add(y, 1);
    z = alu_mul(z, y);
    y = 0; // y = alu_mul(y, 0);
    y = alu_add(y, w);
    y = alu_add(y, 6);
    y = alu_mul(y, x);
    z = alu_add(z, y);
    (x,y,z,w)
}
fn calc_step_08(input: i64, mut x: i64, mut y: i64, mut z: i64, mut w: i64) -> (i64,i64,i64,i64) {
    w = input;
    x = 0; // x = alu_mul(x, 0);
    x = alu_add(x, z);
    x = alu_mod(x, 26);
    // z = z; // z = alu_div(z, 1);
    x = alu_add(x, 11);
    x = alu_eql(x, w);
    x = alu_eql(x, 0);
    y = 0; // y = alu_mul(y, 0);
    y = alu_add(y, 25);
    y = alu_mul(y, x);
    y = alu_add(y, 1);
    z = alu_mul(z, y);
    y = 0; // y = alu_mul(y, 0);
    y = alu_add(y, w);
    y = alu_add(y, 5);
    y = alu_mul(y, x);
    z = alu_add(z, y);
    (x,y,z,w)
}
fn calc_step_09(input: i64, mut x: i64, mut y: i64, mut z: i64, mut w: i64) -> (i64,i64,i64,i64) {
    w = input;
    x = 0; // x = alu_mul(x, 0);
    x = alu_add(x, z);
    x = alu_mod(x, 26);
    // z = z; // z = alu_div(z, 1);
    x = alu_add(x, 10);
    x = alu_eql(x, w);
    x = alu_eql(x, 0);
    y = 0; // y = alu_mul(y, 0);
    y = alu_add(y, 25);
    y = alu_mul(y, x);
    y = alu_add(y, 1);
    z = alu_mul(z, y);
    y = 0; // y = alu_mul(y, 0);
    y = alu_add(y, w);
    y = alu_add(y, 16);
    y = alu_mul(y, x);
    z = alu_add(z, y);
    (x,y,z,w)
}
fn calc_step_10(input: i64, mut x: i64, mut y: i64, mut z: i64, mut w: i64) -> (i64,i64,i64,i64) {
    w = input;
    x = 0; // x = alu_mul(x, 0);
    x = alu_add(x, z);
    x = alu_mod(x, 26);
    // z = z; // z = alu_div(z, 1);
    x = alu_add(x, 13);
    x = alu_eql(x, w);
    x = alu_eql(x, 0);
    y = 0; // y = alu_mul(y, 0);
    y = alu_add(y, 25);
    y = alu_mul(y, x);
    y = alu_add(y, 1);
    z = alu_mul(z, y);
    y = 0; // y = alu_mul(y, 0);
    y = alu_add(y, w);
    y = alu_add(y, 1);
    y = alu_mul(y, x);
    z = alu_add(z, y);
    (x,y,z,w)
}
fn calc_step_11(input: i64, mut x: i64, mut y: i64, mut z: i64, mut w: i64) -> (i64,i64,i64,i64) {
    w = input;
    x = 0; // x = alu_mul(x, 0);
    x = alu_add(x, z);
    x = alu_mod(x, 26);
    z = alu_div(z, 26);
    x = alu_add(x, -4);
    x = alu_eql(x, w);
    x = alu_eql(x, 0);
    y = 0; // y = alu_mul(y, 0);
    y = alu_add(y, 25);
    y = alu_mul(y, x);
    y = alu_add(y, 1);
    z = alu_mul(z, y);
    y = 0; // y = alu_mul(y, 0);
    y = alu_add(y, w);
    y = alu_add(y, 6);
    y = alu_mul(y, x);
    z = alu_add(z, y);
    (x,y,z,w)
}
fn calc_step_12(input: i64, mut x: i64, mut y: i64, mut z: i64, mut w: i64) -> (i64,i64,i64,i64) {
    w = input;
    x = 0; // x = alu_mul(x, 0);
    x = alu_add(x, z);
    x = alu_mod(x, 26);
    z = alu_div(z, 26);
    x = alu_add(x, -9);
    x = alu_eql(x, w);
    x = alu_eql(x, 0);
    y = 0; // y = alu_mul(y, 0);
    y = alu_add(y, 25);
    y = alu_mul(y, x);
    y = alu_add(y, 1);
    z = alu_mul(z, y);
    y = 0; // y = alu_mul(y, 0);
    y = alu_add(y, w);
    y = alu_add(y, 3);
    y = alu_mul(y, x);
    z = alu_add(z, y);
    (x,y,z,w)
}
fn calc_step_13(input: i64, mut x: i64, mut y: i64, mut z: i64, mut w: i64) -> (i64,i64,i64,i64) {
    w = input;
    x = 0; // x = alu_mul(x, 0);
    x = alu_add(x, z);
    x = alu_mod(x, 26);
    z = alu_div(z, 26);
    x = alu_add(x, -13);
    x = alu_eql(x, w);
    x = alu_eql(x, 0);
    y = 0; // y = alu_mul(y, 0);
    y = alu_add(y, 25);
    y = alu_mul(y, x);
    y = alu_add(y, 1);
    z = alu_mul(z, y);
    y = 0; // y = alu_mul(y, 0);
    y = alu_add(y, w);
    y = alu_add(y, 7);
    y = alu_mul(y, x);
    z = alu_add(z, y);
    (x,y,z,w)
}
fn calc_step_14(input: i64, mut x: i64, mut y: i64, mut z: i64, mut w: i64) -> (i64,i64,i64,i64) {
    w = input;
    x = 0; // x = alu_mul(x, 0);
    x = alu_add(x, z);
    x = alu_mod(x, 26);
    z = alu_div(z, 26);
    x = alu_add(x, -9);
    x = alu_eql(x, w);
    x = alu_eql(x, 0);
    y = 0; // y = alu_mul(y, 0);
    y = alu_add(y, 25);
    y = alu_mul(y, x);
    y = alu_add(y, 1);
    z = alu_mul(z, y);
    y = 0; // y = alu_mul(y, 0);
    y = alu_add(y, w);
    y = alu_add(y, 9);
    y = alu_mul(y, x);
    z = alu_add(z, y);
    (x,y,z,w)
}

pub fn calc_part1() -> Option<i64> {
    let mut known_not_working_01: FxHashSet<(i64, i64, i64, i64)> = FxHashSet::default();
    let mut known_not_working_02: FxHashSet<(i64, i64, i64, i64)> = FxHashSet::default();
    let mut known_not_working_03: FxHashSet<(i64, i64, i64, i64)> = FxHashSet::default();
    let mut known_not_working_04: FxHashSet<(i64, i64, i64, i64)> = FxHashSet::default();
    let mut known_not_working_05: FxHashSet<(i64, i64, i64, i64)> = FxHashSet::default();
    let mut known_not_working_06: FxHashSet<(i64, i64, i64, i64)> = FxHashSet::default();
    let mut known_not_working_07: FxHashSet<(i64, i64, i64, i64)> = FxHashSet::default();
    let mut known_not_working_08: FxHashSet<(i64, i64, i64, i64)> = FxHashSet::default();
    let mut known_not_working_09: FxHashSet<(i64, i64, i64, i64)> = FxHashSet::default();
    let mut known_not_working_10: FxHashSet<(i64, i64, i64, i64)> = FxHashSet::default();
    let mut known_not_working_11: FxHashSet<(i64, i64, i64, i64)> = FxHashSet::default();
    let mut known_not_working_12: FxHashSet<(i64, i64, i64, i64)> = FxHashSet::default();
    let mut known_not_working_13: FxHashSet<(i64, i64, i64, i64)> = FxHashSet::default();
    let (x, y, z, w) = (0, 0, 0, 0);    for input_01 in (1..=9).rev() {
        let (x, y, z, w) = calc_step_01(input_01, x, y, z, w);
        if known_not_working_01.contains(&(x,y,z,w)) { continue; }
        for input_02 in (1..=9).rev() {
            let (x, y, z, w) = calc_step_02(input_02, x, y, z, w);
            if known_not_working_02.contains(&(x,y,z,w)) { continue; }
            for input_03 in (1..=9).rev() {
                let (x, y, z, w) = calc_step_03(input_03, x, y, z, w);
                if known_not_working_03.contains(&(x,y,z,w)) { continue; }
                for input_04 in (1..=9).rev() {
                    let (x, y, z, w) = calc_step_04(input_04, x, y, z, w);
                    if known_not_working_04.contains(&(x,y,z,w)) { continue; }
                    for input_05 in (1..=9).rev() {
                        let (x, y, z, w) = calc_step_05(input_05, x, y, z, w);
                        if known_not_working_05.contains(&(x,y,z,w)) { continue; }
                        for input_06 in (1..=9).rev() {
                            let (x, y, z, w) = calc_step_06(input_06, x, y, z, w);
                            if known_not_working_06.contains(&(x,y,z,w)) { continue; }
                            for input_07 in (1..=9).rev() {
                                let (x, y, z, w) = calc_step_07(input_07, x, y, z, w);
                                if known_not_working_07.contains(&(x,y,z,w)) { continue; }
                                for input_08 in (1..=9).rev() {
                                    let (x, y, z, w) = calc_step_08(input_08, x, y, z, w);
                                    if known_not_working_08.contains(&(x,y,z,w)) { continue; }
                                    for input_09 in (1..=9).rev() {
                                        let (x, y, z, w) = calc_step_09(input_09, x, y, z, w);
                                        if known_not_working_09.contains(&(x,y,z,w)) { continue; }
                                        for input_10 in (1..=9).rev() {
                                            let (x, y, z, w) = calc_step_10(input_10, x, y, z, w);
                                            if known_not_working_10.contains(&(x,y,z,w)) { continue; }
                                            for input_11 in (1..=9).rev() {
                                                let (x, y, z, w) = calc_step_11(input_11, x, y, z, w);
                                                if known_not_working_11.contains(&(x,y,z,w)) { continue; }
                                                for input_12 in (1..=9).rev() {
                                                    let (x, y, z, w) = calc_step_12(input_12, x, y, z, w);
                                                    if known_not_working_12.contains(&(x,y,z,w)) { continue; }
                                                    for input_13 in (1..=9).rev() {
                                                        let (x, y, z, w) = calc_step_13(input_13, x, y, z, w);
                                                        if known_not_working_13.contains(&(x,y,z,w)) { continue; }
                                                        for input_14 in (1..=9).rev() {
                                                            let (x, y, z, w) = calc_step_14(input_14, x, y, z, w);
                                                            if z == 0 { return Some(input_01 * 10_i64.pow(13)+input_02 * 10_i64.pow(12)+input_03 * 10_i64.pow(11)+input_04 * 10_i64.pow(10)+input_05 * 10_i64.pow(9)+input_06 * 10_i64.pow(8)+input_07 * 10_i64.pow(7)+input_08 * 10_i64.pow(6)+input_09 * 10_i64.pow(5)+input_10 * 10_i64.pow(4)+input_11 * 10_i64.pow(3)+input_12 * 10_i64.pow(2)+input_13 * 10_i64.pow(1)+input_14 * 10_i64.pow(0)); }
                                                        }
                                                        known_not_working_13.insert((x, y, z, w));
                                                    }
                                                    known_not_working_12.insert((x, y, z, w));
                                                }
                                                known_not_working_11.insert((x, y, z, w));
                                            }
                                            known_not_working_10.insert((x, y, z, w));
                                        }
                                        known_not_working_09.insert((x, y, z, w));
                                    }
                                    known_not_working_08.insert((x, y, z, w));
                                }
                                known_not_working_07.insert((x, y, z, w));
                            }
                            known_not_working_06.insert((x, y, z, w));
                        }
                        known_not_working_05.insert((x, y, z, w));
                    }
                    known_not_working_04.insert((x, y, z, w));
                }
                known_not_working_03.insert((x, y, z, w));
            }
            known_not_working_02.insert((x, y, z, w));
        }
        known_not_working_01.insert((x, y, z, w));
    }
    None
}

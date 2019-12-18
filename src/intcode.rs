use crate::input::{load_input_commas, split_commas};
use std::io::{stdout, Write};

fn input(input_ptr: usize, values: &[i32]) -> i32 {
    values[input_ptr]
}

fn output<W: Write>(val: i32, mut out: &mut W) {
    println!("out: {}", val);
    out.write(&val.to_ne_bytes());
}

fn get_value(pos: i32, immediate_mode: bool, data: &Vec<i32>) -> i32{
//    println!("Getting value {} in mode {}", pos, immediate_mode);
    if immediate_mode {
        pos
    } else {
        data[pos as usize]
    }
}

fn parse_modes(param: i32) -> (bool, bool, bool) {
    let op = param % 100;
    let param = param - op;
    let m1 = param % 1_000;
    let param = param - m1;
    let m2 = param % 10_000;
    let param = param - m2;
//    println!("Parsed as {} {} {} {}", m1, m2, param, op);
    (m1 == 100, m2 == 1_000, param == 10_000)
}

fn execute<W: Write>(opt: Vec<i32>, mut out: &mut W, input_values: &[i32]) -> Vec<i32> {
    let mut iptr: usize = 0;
    let mut inputptr: usize = 0;
    let mut data = opt.clone();
    loop {
        if iptr+3 > data.len() {
            break;
        }
        let op = data[iptr] % 100;
        let (m1, m2, m3) = parse_modes(data[iptr]);
//        println!("{}, {}, {}, {}", op, data[iptr+1], data[iptr+2], data[iptr+3]);
        let v1 = *data.get(iptr+1).unwrap_or(&0);
        let v2 = *data.get(iptr+2).unwrap_or(&0);
        let v3 = *data.get(iptr+3).unwrap_or(&0);
//        println!("Op {}", op);
        match op {
            1 => { // ADD
//                println!("data[{}] = get({}, {}) + get({}, {})", v3, v1, m1, v2, m2);
                data[v3 as usize] = get_value(v1, m1, &data) + get_value(v2, m2, &data);
                iptr += 4;
            },
            2 => { // MUL
                data[v3 as usize] = get_value(v1, m1, &data) * get_value(v2, m2, &data);
                iptr += 4;
            },
            3 => { // INPUT
                data[v1 as usize] = input(inputptr, &input_values);
                iptr += 2;
            },
            4 => { // OUTPUT
                let val = get_value(v1, m1, &data);
                output(get_value(v1, m1, &data), out);
                iptr += 2;

            },
            5 => { // JUMP-IF-TRUE
                let val = get_value(v1, m1, &data);
                if val != 0 {
                    let target = get_value(v2, m2, &data);
                    iptr = target as usize;
                }
            },
            6 => { // JUMP-IF-FALSE
                let val = get_value(v1, m1, &data);
                if val == 0 {
                    let target = get_value(v2, m2, &data);
                    iptr = target as usize;
                }
            },
            7 => { // LESS THAN
                let lhs = get_value(v1, m1, &data);
                let rhs = get_value(v1, m1, &data);
                let target = get_value(v3, m3, &data);
                data[target as usize] = if lhs < rhs {1} else {0};
                iptr += 4
            },
            8 => { // EQUALS
                let lhs = get_value(v1, m1, &data);
                let rhs = get_value(v1, m1, &data);
                let target = get_value(v3, m3, &data);
                data[target as usize] = if lhs == rhs {1} else {0};
                iptr += 4
            },
            99 => break,
            _ => unimplemented!("No opcode {}", op)
        }
    }
    data
}

pub fn day2() {
    let mut ops = load_input_commas();
    let mut writer: Vec<u8> = Vec::new();
    for i in 0..99 {
        for j in 0..99 {
            let mut ops = ops.clone();
            ops[1] = i;
            ops[2] = j;
            let output = execute(ops, &mut writer,&[1]); // ops dropped after here
            let result = output[0];
            if result == 19_690_720 {
                println!("Day 2, part 2: {}", 100 * i + j);
            }
        }
    }
    ops[1] = 12;
    ops[2] = 2;
    let result = execute(ops,&mut writer,&[1]);
    println!("Day 2, part 1: {:?}", &result[0]);
}

pub fn day5_1() {
    let mut writer: Vec<u8> = Vec::new();
    let program_s = "3,225,1,225,6,6,1100,1,238,225,104,0,1101,69,55,225,1001,144,76,224,101,-139,224,224,4,224,1002,223,8,223,1001,224,3,224,1,223,224,223,1102,60,49,225,1102,51,78,225,1101,82,33,224,1001,224,-115,224,4,224,1002,223,8,223,1001,224,3,224,1,224,223,223,1102,69,5,225,2,39,13,224,1001,224,-4140,224,4,224,102,8,223,223,101,2,224,224,1,224,223,223,101,42,44,224,101,-120,224,224,4,224,102,8,223,223,101,3,224,224,1,223,224,223,1102,68,49,224,101,-3332,224,224,4,224,1002,223,8,223,1001,224,4,224,1,224,223,223,1101,50,27,225,1102,5,63,225,1002,139,75,224,1001,224,-3750,224,4,224,1002,223,8,223,1001,224,3,224,1,223,224,223,102,79,213,224,1001,224,-2844,224,4,224,102,8,223,223,1001,224,4,224,1,223,224,223,1,217,69,224,1001,224,-95,224,4,224,102,8,223,223,1001,224,5,224,1,223,224,223,1102,36,37,225,1101,26,16,225,4,223,99,0,0,0,677,0,0,0,0,0,0,0,0,0,0,0,1105,0,99999,1105,227,247,1105,1,99999,1005,227,99999,1005,0,256,1105,1,99999,1106,227,99999,1106,0,265,1105,1,99999,1006,0,99999,1006,227,274,1105,1,99999,1105,1,280,1105,1,99999,1,225,225,225,1101,294,0,0,105,1,0,1105,1,99999,1106,0,300,1105,1,99999,1,225,225,225,1101,314,0,0,106,0,0,1105,1,99999,1107,677,677,224,102,2,223,223,1006,224,329,1001,223,1,223,1108,677,677,224,1002,223,2,223,1006,224,344,1001,223,1,223,107,226,226,224,1002,223,2,223,1006,224,359,101,1,223,223,1008,226,226,224,102,2,223,223,1005,224,374,1001,223,1,223,1107,226,677,224,1002,223,2,223,1006,224,389,1001,223,1,223,1008,677,226,224,1002,223,2,223,1005,224,404,1001,223,1,223,7,677,226,224,102,2,223,223,1005,224,419,1001,223,1,223,1008,677,677,224,1002,223,2,223,1006,224,434,1001,223,1,223,108,226,226,224,102,2,223,223,1006,224,449,1001,223,1,223,108,677,677,224,102,2,223,223,1006,224,464,1001,223,1,223,107,226,677,224,1002,223,2,223,1005,224,479,101,1,223,223,1108,226,677,224,1002,223,2,223,1006,224,494,1001,223,1,223,107,677,677,224,1002,223,2,223,1006,224,509,101,1,223,223,7,677,677,224,102,2,223,223,1006,224,524,1001,223,1,223,1007,226,677,224,1002,223,2,223,1005,224,539,1001,223,1,223,8,226,677,224,1002,223,2,223,1005,224,554,101,1,223,223,8,677,677,224,102,2,223,223,1005,224,569,101,1,223,223,7,226,677,224,102,2,223,223,1006,224,584,1001,223,1,223,1007,226,226,224,102,2,223,223,1006,224,599,1001,223,1,223,1107,677,226,224,1002,223,2,223,1006,224,614,1001,223,1,223,1108,677,226,224,1002,223,2,223,1005,224,629,1001,223,1,223,1007,677,677,224,102,2,223,223,1006,224,644,1001,223,1,223,108,226,677,224,102,2,223,223,1005,224,659,101,1,223,223,8,677,226,224,1002,223,2,223,1006,224,674,1001,223,1,223,4,223,99,226";
    let program = split_commas(program_s.into());
    let data_out = execute(program, &mut writer, &[1]);
    println!("{:?}", writer);
}

pub fn day5_2() {
    let mut writer: Vec<u8> = Vec::new();
    let program_s = "3,225,1,225,6,6,1100,1,238,225,104,0,1101,69,55,225,1001,144,76,224,101,-139,224,224,4,224,1002,223,8,223,1001,224,3,224,1,223,224,223,1102,60,49,225,1102,51,78,225,1101,82,33,224,1001,224,-115,224,4,224,1002,223,8,223,1001,224,3,224,1,224,223,223,1102,69,5,225,2,39,13,224,1001,224,-4140,224,4,224,102,8,223,223,101,2,224,224,1,224,223,223,101,42,44,224,101,-120,224,224,4,224,102,8,223,223,101,3,224,224,1,223,224,223,1102,68,49,224,101,-3332,224,224,4,224,1002,223,8,223,1001,224,4,224,1,224,223,223,1101,50,27,225,1102,5,63,225,1002,139,75,224,1001,224,-3750,224,4,224,1002,223,8,223,1001,224,3,224,1,223,224,223,102,79,213,224,1001,224,-2844,224,4,224,102,8,223,223,1001,224,4,224,1,223,224,223,1,217,69,224,1001,224,-95,224,4,224,102,8,223,223,1001,224,5,224,1,223,224,223,1102,36,37,225,1101,26,16,225,4,223,99,0,0,0,677,0,0,0,0,0,0,0,0,0,0,0,1105,0,99999,1105,227,247,1105,1,99999,1005,227,99999,1005,0,256,1105,1,99999,1106,227,99999,1106,0,265,1105,1,99999,1006,0,99999,1006,227,274,1105,1,99999,1105,1,280,1105,1,99999,1,225,225,225,1101,294,0,0,105,1,0,1105,1,99999,1106,0,300,1105,1,99999,1,225,225,225,1101,314,0,0,106,0,0,1105,1,99999,1107,677,677,224,102,2,223,223,1006,224,329,1001,223,1,223,1108,677,677,224,1002,223,2,223,1006,224,344,1001,223,1,223,107,226,226,224,1002,223,2,223,1006,224,359,101,1,223,223,1008,226,226,224,102,2,223,223,1005,224,374,1001,223,1,223,1107,226,677,224,1002,223,2,223,1006,224,389,1001,223,1,223,1008,677,226,224,1002,223,2,223,1005,224,404,1001,223,1,223,7,677,226,224,102,2,223,223,1005,224,419,1001,223,1,223,1008,677,677,224,1002,223,2,223,1006,224,434,1001,223,1,223,108,226,226,224,102,2,223,223,1006,224,449,1001,223,1,223,108,677,677,224,102,2,223,223,1006,224,464,1001,223,1,223,107,226,677,224,1002,223,2,223,1005,224,479,101,1,223,223,1108,226,677,224,1002,223,2,223,1006,224,494,1001,223,1,223,107,677,677,224,1002,223,2,223,1006,224,509,101,1,223,223,7,677,677,224,102,2,223,223,1006,224,524,1001,223,1,223,1007,226,677,224,1002,223,2,223,1005,224,539,1001,223,1,223,8,226,677,224,1002,223,2,223,1005,224,554,101,1,223,223,8,677,677,224,102,2,223,223,1005,224,569,101,1,223,223,7,226,677,224,102,2,223,223,1006,224,584,1001,223,1,223,1007,226,226,224,102,2,223,223,1006,224,599,1001,223,1,223,1107,677,226,224,1002,223,2,223,1006,224,614,1001,223,1,223,1108,677,226,224,1002,223,2,223,1005,224,629,1001,223,1,223,1007,677,677,224,102,2,223,223,1006,224,644,1001,223,1,223,108,226,677,224,102,2,223,223,1005,224,659,101,1,223,223,8,677,226,224,1002,223,2,223,1006,224,674,1001,223,1,223,4,223,99,226";
    let program = split_commas(program_s.into());
    let data_out = execute(program, &mut writer, &[5]);
    println!("{:?}", writer);
}

#[cfg(test)]
mod day5 {
    use super::*;
    use crate::input::split_commas;

    #[test]
    fn test_input_output() {
        let mut writer: Vec<u8> = Vec::new();
        let program = split_commas("3,0,4,0,99".into());
        assert_eq!(execute(program.clone(), &mut writer,&[1]), split_commas("1,0,4,0,99".into()));
        // test output?
        assert_eq!(&writer[0..4], &1i32.to_ne_bytes());
        writer.truncate(0);
        assert_eq!(execute(program, &mut writer,&[5]), split_commas("5,0,4,0,99".into()));
        // test output?
        assert_eq!(&writer[0..4], &5i32.to_ne_bytes());
    }

    #[test]
    fn test_output() {
        let mut writer: Vec<u8> = Vec::new();
        output(5, &mut writer);
        assert_eq!(writer[0], 5u8);
        assert_eq!(writer[1], 0u8);
    }

    #[test]
    fn test_parse_modes() {
        assert_eq!(parse_modes(1002), (false,true,false));
        assert_eq!(parse_modes(11102), (true,true,true));
        assert_eq!(parse_modes(2), (false,false,false));
        assert_eq!(parse_modes(11002), (false,true,true));
        assert_eq!(parse_modes(1101), (true,true,false));

    }

    #[test]
    fn test_parameter_modes_mul() {
        let mut writer: Vec<u8> = Vec::new();
        let program = split_commas("1002,4,3,4,33".into());
        assert_eq!(execute(program,&mut writer,&[1]), split_commas("1002,4,3,4,99".into()));
    }

    #[test]
    fn test_negative_numbers() {
        let mut writer: Vec<u8> = Vec::new();
        let program = split_commas("1101,100,-1,4,0".into());
        assert_eq!(execute(program,&mut writer,&[1]), split_commas("1101,100,-1,4,99".into()));


    }
}

#[cfg(test)]
mod day2 {
    use super::*;
    use crate::input::split_commas;
    #[test]
    fn test1() {
        let mut writer: Vec<u8> = Vec::new();
        let program = split_commas("1,0,0,0,99".into());
        assert_eq!(execute(program,&mut writer,&[1]), split_commas("2,0,0,0,99".into()));
    }
    #[test]
    fn test2() {
        let mut writer: Vec<u8> = Vec::new();
        let program = split_commas("2,3,0,3,99".into());
        assert_eq!(execute(program,&mut writer,&[1]), split_commas("2,3,0,6,99".into()));

    }
    #[test]
    fn test3() {
        let mut writer: Vec<u8> = Vec::new();
        let program = split_commas("2,4,4,5,99,0".into());
        assert_eq!(execute(program,&mut writer,&[1]), split_commas("2,4,4,5,99,9801".into()));
    }
    #[test]
    fn test4() {
        let mut writer: Vec<u8> = Vec::new();
        let program = split_commas("1,1,1,4,99,5,6,0,99".into());
        assert_eq!(execute(program,&mut writer,&[1]), split_commas("30,1,1,4,2,5,6,0,99".into()));
    }
}

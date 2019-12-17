use crate::input::load_input_commas;
use std::io::{stdout, Write};

fn input() -> i32 {
    0
}

fn output<W: Write>(val: i32, mut out: &mut W) {
    out.write(&val.to_ne_bytes());
}

fn handle_parameter_modes(op: i32, v1: i32, v2: i32, v3: i32, data: &mut Vec<i32>) {

}

fn execute<W: Write>(opt: Vec<i32>, mut out: &mut W) -> Vec<i32> {
    let mut iptr: usize = 0;
    let mut data = opt.clone();
    loop {
        if iptr+3 > data.len() {
            break;
        }
        let op = data[iptr];
//        println!("{}, {}, {}, {}", op, data[iptr+1], data[iptr+2], data[iptr+3]);
        let v1 = data[iptr+1];
        let v2 = data[iptr+2];
        let v3 = data[iptr+3];
        match op {
            1 => data[v3 as usize] = data[v1 as usize] + data[v2 as usize],
            2 => data[v3 as usize] = data[v1 as usize] * data[v2 as usize],
            3 => data[v1 as usize] = input(),
            4 => output(data[v1 as usize], out),
            99 => break,
            _ => {
                if 1000 <= op && op <= 9999 {
                    handle_parameter_modes(op, v1, v2, v3, &mut data);
                } else {
                    unimplemented!()
                }
            }
        }
//        println!("{:?}", &data);
        iptr += 4;
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
            let output = execute(ops, &mut writer); // ops dropped after here
            let result = output[0];
            if result == 19_690_720 {
                println!("Day 2, part 2: {}", 100 * i + j);
            }

        }
    }
    ops[1] = 12;
    ops[2] = 2;
    let result = execute(ops,&mut writer);
    println!("Day 2, part 1: {:?}", &result[0]);
}

pub fn day5() {

}

#[cfg(test)]
mod day5 {
    use super::*;
    use crate::input::split_commas;

    #[test]
    fn test_eg1() {
        let mut writer: Vec<u8> = Vec::new();
        let input = split_commas("3,0,4,0,99".into());
        assert_eq!(execute(input, &mut writer), split_commas("3,0,4,0,99".into()));
        // how to test output?
    }

    #[test]
    fn test_output() {
        let mut writer: Vec<u8> = Vec::new();
        output(5, &mut writer);
        assert_eq!(writer[0], 5u8);
        assert_eq!(writer[1], 0u8);

    }

    #[test]
    fn test_parameter_modes_mul() {
        let mut writer: Vec<u8> = Vec::new();
        let input = split_commas("1002,4,3,4,33".into());
        assert_eq!(execute(input,&mut writer), split_commas("1002,4,3,4,99".into()));
    }

    #[test]
    fn test_negative_numbers() {
        let mut writer: Vec<u8> = Vec::new();
        let input = split_commas("1101,100,-1,4,0".into());
        assert_eq!(execute(input,&mut writer), split_commas("1101,100,-1,4,99".into()));


    }
}

#[cfg(test)]
mod day2 {
    use super::*;
    use crate::input::split_commas;
    #[test]
    fn test1() {
        let mut writer: Vec<u8> = Vec::new();
        let input = split_commas("1,0,0,0,99".into());
        assert_eq!(execute(input,&mut writer), split_commas("2,0,0,0,99".into()));
    }
    #[test]
    fn test2() {
        let mut writer: Vec<u8> = Vec::new();
        let input = split_commas("2,3,0,3,99".into());
        assert_eq!(execute(input,&mut writer), split_commas("2,3,0,6,99".into()));

    }
    #[test]
    fn test3() {
        let mut writer: Vec<u8> = Vec::new();
        let input = split_commas("2,4,4,5,99,0".into());
        assert_eq!(execute(input,&mut writer), split_commas("2,4,4,5,99,9801".into()));
    }
    #[test]
    fn test4() {
        let mut writer: Vec<u8> = Vec::new();
        let input = split_commas("1,1,1,4,99,5,6,0,99".into());
        assert_eq!(execute(input,&mut writer), split_commas("30,1,1,4,2,5,6,0,99".into()));
    }
}

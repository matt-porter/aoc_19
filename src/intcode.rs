use crate::input::load_input_commas;

fn execute(opt: Vec<i32>) -> Vec<i32> {
    let mut iptr: usize = 0;
    let mut data = opt.clone();
    loop {
        if iptr+3 > data.len() {
            break;
        }
        let op = data[iptr];
//        println!("{}, {}, {}, {}", op, data[iptr+1], data[iptr+2], data[iptr+3]);
        let v1 = data[iptr+1] as usize;
        let v2 = data[iptr+2] as usize;
        let v3 = data[iptr+3] as usize;
        match op {
            1 => data[v3] = data[v1] + data[v2],
            2 => data[v3] = data[v1] * data[v2],
            99 => break,
            _ => unimplemented!()
        }
//        println!("{:?}", &data);
        iptr += 4;
    }
    data
}

pub fn day2() {
    let mut ops = load_input_commas();
    for i in 0..99 {
        for j in 0..99 {
            let mut ops = ops.clone();
            ops[1] = i;
            ops[2] = j;
            let output = execute(ops); // ops dropped after here
            let result = output[0];
            if result == 19_690_720 {
                println!("Day 2, part 2: {}", 100 * i + j);
            }

        }
    }
    ops[1] = 12;
    ops[2] = 2;
    let result = execute(ops);
    println!("Day 2, part 1: {:?}", &result[0]);
}

#[cfg(test)]
mod day2 {
    use super::*;
    use crate::input::split_commas;
    #[test]
    fn test1() {
        let input = split_commas("1,0,0,0,99".into());
        assert_eq!(execute(input), split_commas("2,0,0,0,99".into()));
    }
    #[test]
    fn test2() {
        let input = split_commas("2,3,0,3,99".into());
        assert_eq!(execute(input), split_commas("2,3,0,6,99".into()));

    }
    #[test]
    fn test3() {
        let input = split_commas("2,4,4,5,99,0".into());
        assert_eq!(execute(input), split_commas("2,4,4,5,99,9801".into()));
    }
    #[test]
    fn test4() {
        let input = split_commas("1,1,1,4,99,5,6,0,99".into());
        assert_eq!(execute(input), split_commas("30,1,1,4,2,5,6,0,99".into()));
    }
}

pub fn process() -> Vec<u32> {
    let (mut a, mut b, mut c) = (56256477, 0, 0);
    let program: Vec<u32> = vec![2,4,1,1,7,5,1,5,0,3,4,3,5,5,3,0];

    let mut pointer = 0;
    let mut output: Vec<u32> = vec![];

    while pointer < program.len() {
        let opcode = program[pointer];
        let operand = program[pointer + 1];
        let combo = if operand <= 3 { operand } else if operand == 4 { a } else if operand == 5 { b } else { c };

        match opcode {
            0 => { a >>= combo; }
            1 => { b ^= operand; }
            2 => { b = combo % 8; }
            3 => {
                if a!=0 {
                    pointer = operand as usize;
                    continue;
                }
            }
            4 => { b ^= c; }
            5 => { output.push(combo % 8); }
            6 => { b = a >> combo; }
            7 => { c = a >> combo; }
            _ => panic!("Invalid opcode: {}", opcode),
        }
        pointer += 2;
    }
    println!("{:?}", output);
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        assert_eq!(vec![4,1,5,3,1,5,3,5,7], process());
    }
}
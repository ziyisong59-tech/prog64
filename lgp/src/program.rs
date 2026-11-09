pub struct Program {
    pub genome: u64,
}

impl Program {
    pub fn execute(&self,state:[f32;4])->f32{
        let mut registers = [0.0_f32;4];

        for i in 0..5 {
            let instruction = (self.genome>>(i * 11))&0x7FF;

            let operation = (instruction>>8) & 0b111;
            let destination = ((instruction >> 6) & 0b11) as usize;
            let operand1 = (instruction >> 3) & 0b111;
            let operand2 = instruction & 0b111;
            
            let a = get_operand(operand1, &state, &registers);
            let b = get_operand(operand2, &state, &registers);
            

        }



        registers[0]
    }
}

fn get_operand(operand:u64,state:&[f32;4],registers:&[f32;4])->f32{
    let index=(operand & 0b11) as usize;
    if operand & 0b100==0{
        state [index]
    }else{
        registers[index]
    }
}

    #[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_operand() {
        let state = [10.0, 20.0, 30.0, 40.0];
        let registers = [1.0, 2.0, 3.0, 4.0];

        assert_eq!(get_operand(0b001, &state, &registers), 20.0);
        assert_eq!(get_operand(0b110, &state, &registers), 3.0);
    }
}
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
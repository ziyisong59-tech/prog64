fn main() {
    // Operation: MAX = 110
    let operation: u64 = 0b110;

    // Destination: R[0] = 00
    let destination: u64 = 0b00;

    // Operand 1: I[1] = 001
    let operand1: u64 = 0b001;

    // Operand 2: R[2] = 110
    let operand2: u64 = 0b110;

    // Encode instruction
    let instruction: u64 =
        (operation << 8)
        | (destination << 6)
        | (operand1 << 3)
        | operand2;

    println!("Instruction: {:011b}", instruction);
// Decode instruction
let decoded_operation = (instruction >> 8) & 0b111;
let decoded_destination = (instruction >> 6) & 0b11;
let decoded_operand1 = (instruction >> 3) & 0b111;
let decoded_operand2 = instruction & 0b111;

println!("Operation: {:03b}", decoded_operation);
println!("Destination: {:02b}", decoded_destination);
println!("Operand 1: {:03b}", decoded_operand1);
println!("Operand 2: {:03b}", decoded_operand2);

}

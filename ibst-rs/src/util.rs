
///// Disassemble some buffer of x86 code
//pub fn disas(buf: &[u8]) {
//    use yaxpeax_x86::amd64::{Arch as x86_64};
//    use yaxpeax_arch::{Arch, AddressDisplay, Decoder, Reader, ReaderBuilder};
//
//    fn decode_stream<'data, A: yaxpeax_arch::Arch, 
//       U: ReaderBuilder<A::Address, A::Word>>(data: U) 
//    where A::Instruction: std::fmt::Display
//    {
//        let mut reader = ReaderBuilder::read_from(data);
//        let mut address: A::Address = reader.total_offset();
//
//        let decoder = A::Decoder::default();
//        let mut decode_res = decoder.decode(&mut reader);
//        loop {
//            match decode_res {
//                Ok(ref inst) => {
//                    println!("{:04x?}: {}", address, inst);
//                    decode_res = decoder.decode(&mut reader);
//                    address = reader.total_offset();
//                }
//                Err(e) => { break; }
//            }
//        }
//    }
//
//    decode_stream::<x86_64, _>(buf);
//}



pub fn disas(buf: &[u8]) {
    use iced_x86::{
        Decoder, DecoderOptions, Formatter, Instruction, IntelFormatter
    };
    let mut decoder = Decoder::with_ip(64, buf, 0, DecoderOptions::NONE);
    let mut formatter = IntelFormatter::new();

    // Change some options, there are many more
    formatter.options_mut().set_digit_separator("_");
    formatter.options_mut().set_first_operand_char_index(10);

    let mut output = String::new();
    let mut instruction = Instruction::default();

    while decoder.can_decode() {
        decoder.decode_out(&mut instruction);
        output.clear();
        formatter.format(&instruction, &mut output);

        print!("{:04X} ", instruction.ip());
        let start_index = (instruction.ip() - 0) as usize;
        let instr_bytes = &buf[start_index..start_index + instruction.len()];
        for b in instr_bytes.iter() {
            print!("{:02X}", b);
        }
        if instr_bytes.len() < 10 {
            for _ in 0..10 - instr_bytes.len() {
                print!("  ");
            }
        }
        println!(" {}", output);
    }
}

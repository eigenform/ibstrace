
// No goddamn idea how yaxpeax interfaces work, but they sure do work.
use yaxpeax_x86::amd64::{Arch as x86_64};
use yaxpeax_arch::{Arch, AddressDisplay, Decoder, Reader, ReaderBuilder};

fn decode_stream<'data, A: yaxpeax_arch::Arch, 
   U: ReaderBuilder<A::Address, A::Word>>(data: U) 
where A::Instruction: std::fmt::Display
{
    let mut reader = ReaderBuilder::read_from(data);
    let mut address: A::Address = reader.total_offset();

    let decoder = A::Decoder::default();
    let mut decode_res = decoder.decode(&mut reader);
    loop {
        match decode_res {
            Ok(ref inst) => {
                println!("{}", inst);
                decode_res = decoder.decode(&mut reader);
                address = reader.total_offset();
            }
            Err(e) => { break; }
        }
    }
}

/// Disassemble some buffer of x86 code
pub fn disas(buf: &[u8]) {
    decode_stream::<x86_64, _>(buf);
}



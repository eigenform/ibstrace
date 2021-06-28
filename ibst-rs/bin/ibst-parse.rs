use std::env;
use std::fs::File;
use std::io::Read;

fn main() -> Result<(), &'static str> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("usage: {} <input file>", args[0]);
        return Err("Invalid arguments");
    }

    let mut input = File::open(&args[1])
        .expect("Couldn't open input file");
    let input_len = std::fs::metadata(&args[1])
        .unwrap().len() as usize;
    let mut buf = vec![0; input_len];
    input.read(&mut buf).unwrap();

    let data = unsafe { 
        std::slice::from_raw_parts(
            buf.as_ptr() as *mut ibst::Sample,
            buf.len() / std::mem::size_of::<ibst::Sample>()
        )
    };

    for (idx, sample) in data.iter().enumerate() {
        if sample.data.rip_invalid() {
            continue;
        }

        println!("Sample {:08} @ {:016x}", idx, sample.rip);
        if sample.data.op_microcode() {
            println!("  Microcoded operation");
        }

        let brn_fuse = sample.data.op_brn_fuse();
        let brn_ret = sample.data.op_brn_ret();
        let brn_misp = sample.data.op_brn_misp();
        if brn_fuse || brn_ret || brn_misp {
            println!("  Branch micro-op")
        }

        let st_op = sample.data3.st_op();
        let ld_op = sample.data3.ld_op();
        if st_op || ld_op {
            println!("  Load/store micro-op");

            if sample.data3.dc_lin_addr_valid() {
                println!("\tLinear address:   {:016}", sample.linad);
            }
            if sample.data3.dc_phy_addr_valid() {
                println!("\tPhysical address: {:016}", sample.phyad);
            }
            println!("\tWidth: {} bits", sample.data3.op_mem_width() as usize);
            if st_op { println!("\tType: Store"); }
            if ld_op { println!("\tType: Load"); }
        }

        let tag2ret = sample.data.tag_to_ret_ctr();
        let comp2ret = sample.data.comp_to_ret_ctr();
        let inflight = tag2ret.checked_sub(comp2ret).unwrap();

        println!("  Tag to retire:        {:05} cycles", tag2ret);
        println!("  Completion to retire: {:05} cycles", comp2ret);

        println!("");
    }

    Ok(())
}




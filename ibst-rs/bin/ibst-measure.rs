
use std::env;
use std::fs::File;
use std::io::{ Read, Write };

fn main() -> Result<(), &'static str> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("usage: {} <input file> <output file>", args[0]);
        return Err("Invalid arguments");
    }

    let mut input = File::open(&args[1])
        .expect("Couldn't open input file");

    let mut output = File::create(&args[2])
        .expect("Couldn't create output file");

    let input_len = std::fs::metadata(&args[1])
        .unwrap().len() as usize;

    let mut user_code = vec![0; input_len];
    input.read(&mut user_code).unwrap();

    let msg = ibst::ioctl::UserBuf::new(user_code.as_ptr(), user_code.len());

    let data = unsafe { 
        let fd = match ibst::ibstrace_open() {
            Ok(fd) => fd,
            Err(e) => return Err(e),
        };

        match ibst::ibstrace_write(fd, &msg as *const ibst::ioctl::UserBuf) {
            Ok(res) => if res < 0 { 
                return Err("Failed write to character device");
            },
            Err(e) => panic!("{}", e),
        }

        match ibst::ibstrace_measure(fd) { 
            Ok(res) => if res < 0 { 
                return Err("Failed to execute/sample code"); 
            },
            Err(e) => panic!("{}", e),
        }

        let data = match ibst::ibstrace_read(fd) {
            Ok(buf) => buf,
            Err(e) => panic!("{}", e),
        };
        ibst::ibstrace_close(fd);
        data
    };

    match output.write(&data) {
        Ok(res) => if res != data.len() {
            return Err("Failed to write sample data to output file");
        },
        Err(e) => panic!("{}, e"),
    }

    Ok(())
}

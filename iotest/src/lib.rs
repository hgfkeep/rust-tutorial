use std::io::{stdin, stdout, Result, Write};

pub fn command_read() -> Result<()> {
    let mut input: String = String::new();
    let stdout = stdout();
    let mut handle = stdout.lock();
    if let Ok(bytes) = stdin().read_line(&mut input) {
        println!("read {} bytes", bytes);
        handle.write_all(input.as_bytes())?;
    }else{
        handle.write_all(b"no input bytes")?;
    }

    Ok(())
}

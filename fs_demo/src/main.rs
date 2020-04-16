use std::fs;
use std::io;
use std::path;

const TMP_DIR: &str = "/tmp/a";

fn list_tmp_dir() -> io::Result<()> {
    let x: Vec<path::PathBuf> = fs::read_dir(TMP_DIR)?
        .flat_map(|entry| -> io::Result<_> { Ok(entry?.path()) })
        .inspect(|path| println!("path: {:?}", path))
        .collect();
    println!("-----------");
    Ok(())
}

fn create_dir_first() -> io::Result<()> {
    fs::remove_dir_all(TMP_DIR)?;
    fs::create_dir(TMP_DIR)?;
    fs::File::create(String::from(TMP_DIR) + "/test.doc")?;
    list_tmp_dir()?;
    Ok(())
}

fn create_dir_exists_with_create_dir() -> io::Result<()> {
    create_dir_first()?;

    println!("create already exists dir!");
    //will err , AlreadyExists error
    fs::create_dir(TMP_DIR)?;
    list_tmp_dir()?;

    Ok(())
}

fn create_dir_exists_with_create_dir_all() -> io::Result<()> {
    create_dir_first()?;

    println!("create already exists dir!");
    //will err , AlreadyExists error
    fs::create_dir_all(TMP_DIR)?;
    list_tmp_dir()?;

    Ok(())
}



fn main()-> io::Result<()> {
    create_dir_exists_with_create_dir_all()?;
    Ok(())
}

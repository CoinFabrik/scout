fn main() {
    let a = result_fn().expect("description goes here");
}

fn result_fn() -> Result<u8, std::io::Error> {
    let a = 1;
    Ok(a)
}

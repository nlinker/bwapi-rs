use library::AimBox;

fn main() {
    println!("{}", std::mem::size_of::<AimBox>());
    library::ffi_main::cpp_main();
}

fn main() {
    vcpkg::Config::new()
        .emit_includes(true)
        .find_package("unicorn").unwrap();
}
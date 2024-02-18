#[cfg(test)]
mod tests {
    #[test]
    fn main_test() {
        let generator = opgen::generator::new().generate("opgen/encoding/lr35902.yml", "foo.rs");
        let x = 0;
        let r = 202.0;
        print!("hi");
    }
}
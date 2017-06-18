pub mod hero;

#[cfg(test)]
mod tests {
    use hero::*;

    #[test]
    fn it_works() {
        let hero = HeroBuilder::new()
            .name("Conan".to_string())
            .atk(10)
            .def(8)
            .speed(4)
            .build();
        println!("{:?}", hero);
        assert_eq!(hero.name, "Conan".to_string());
        assert_eq!(hero.atk, 10);
        assert_eq!(hero.def, 8);
        assert_eq!(hero.speed, 4);
    }
}

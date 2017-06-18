#[derive(Debug)]
pub struct Hero {
    pub name: String,
    pub atk: u32,
    pub def: u32,
    pub speed: u32,
}

pub struct HeroBuilder {
    name: String,
    atk: u32,
    def: u32,
    speed: u32,
}

impl HeroBuilder {
    pub fn new() -> HeroBuilder {
        HeroBuilder {
            name: "".to_string(),
            atk: 1,
            def: 1,
            speed: 1,
        }
    }

    pub fn name(&mut self, value: String) -> &mut HeroBuilder {
        self.name = value;
        self
    }

    pub fn atk(&mut self, value: u32) -> &mut HeroBuilder {
        self.atk = value;
        self
    }

    pub fn def(&mut self, value: u32) -> &mut HeroBuilder {
        self.def = value;
        self
    }

    pub fn speed(&mut self, value: u32) -> &mut HeroBuilder {
        self.speed = value;
        self
    }

    pub fn build(&self) -> Hero {
        Hero {
            name: self.name.clone(),
            atk: self.atk,
            def: self.def,
            speed: self.speed,
        }
    }
}

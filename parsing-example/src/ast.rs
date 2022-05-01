pub enum Kind {
    Program,
    End
}

pub struct Program {
    end: End,
}

pub struct End;

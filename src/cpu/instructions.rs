pub enum Instruction {
    ADD(Target),
}

pub enum Target {
    A, B, C, D, E, F, H, L,
}
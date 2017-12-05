use time::Tm;
use specs::{Component, VecStorage};

#[derive(Debug)]
pub struct Utterance {
    // utterer: &Entity,
    pub text: String,
    pub dead_at: Tm,
}

impl Component for Utterance {
    type Storage = VecStorage<Self>;
}

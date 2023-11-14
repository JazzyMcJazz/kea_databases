pub trait Thingify {
    fn thingify(&mut self);
}

pub trait Terafy {
    fn terafy(&self, things: &mut Vec<impl Thingify>) {
        for thing in things {
            thing.thingify();
        }
    }
}

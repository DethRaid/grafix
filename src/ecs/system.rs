pub trait System {
    fn tick(&mut self, delta_time: f32);
}
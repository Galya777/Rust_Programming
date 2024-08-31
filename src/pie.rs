pub struct InfiniteSeries<S: Clone, F: Fn(f64, S) -> (f64, S)> {
    pub value: f64,
    state: S,
    calculator: F,
}

impl<S: Clone, F: Fn(f64, S) -> (f64, S)> InfiniteSeries<S, F> {
    pub fn new(value: f64, state: S, calculator: F) -> Self {
        InfiniteSeries { value, state, calculator }
    }
}

impl<S: Clone, F: Fn(f64, S) -> (f64, S)> Iterator for InfiniteSeries<S, F> {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        let (new_value, new_state) = (self.calculator)(self.value, self.state.clone());
        self.state = new_state;
        self.value = new_value;

        Some(self.value)
    }
}

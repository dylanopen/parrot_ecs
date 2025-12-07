pub struct Scheduler {
    systems: Vec<fn()>,
}

impl Scheduler {
    pub fn add_system(&mut self, system: fn()) {
        self.systems.push(system);
    }

    pub fn start(self) {
        for system in self.systems {
            system();
        }
    }
}

#[expect(clippy::derivable_impls, reason = "non-default features will later be added")]
impl Default for Scheduler {
    fn default() -> Self {
        Scheduler {
            systems: Vec::new()
        }
    }
}

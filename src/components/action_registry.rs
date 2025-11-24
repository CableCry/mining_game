use bevy::{
    prelude::*,
};

use std::collections::HashMap;


#[derive(Component)]
pub struct ActionRegistry {
    pub actions: HashMap<String, fn()>,
}

impl ActionRegistry {

    pub fn add_action(&mut self, name: impl Into<String> , func: fn()){
        self.actions.insert(name.into(), func); 
    }


    pub fn run(&self, name: &str) {
        if let Some(f) = self.actions.get(name) {
            (f)();
        }
    }
}

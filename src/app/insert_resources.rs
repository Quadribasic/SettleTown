use bevy::{app::App, prelude::Resource};

pub trait MultiResourceInsertion {
    fn insert_resources<R: Resource>(&mut self, resources: Vec<R>) -> &mut Self;
}

impl MultiResourceInsertion for App {
    fn insert_resources<R: Resource>(&mut self, resources: Vec<R>) -> &mut Self {
        let mut next_self = self;
        for i in resources {
            next_self = next_self.insert_resource(i);
        }

        next_self
    }
}

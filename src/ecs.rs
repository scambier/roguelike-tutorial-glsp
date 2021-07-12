use std::collections::HashMap;

use glsp::prelude::*;

/// A simple, naïve (and absolutely unoptimized) ECS world
pub struct World {
    entity_counter: i32,
    entities: HashMap<i32, HashMap<String, Root<Obj>>>,
    resources: HashMap<Sym, Val>,
}

impl World {
    pub fn bind_world() -> GResult<()> {
        glsp::bind_rfn("World2", &World::new)?;
        glsp::RClassBuilder::<World>::new()
            .met("add-entity", &World::add_entity)
            .met("get-entities", &World::get_entities)
            .met("get-cmp", &World::get_components)
            .met("query", &World::query)
            .met("save", &World::save)
            .met("fetch", &World::fetch)
            .build();

        Ok(())
    }

    pub fn new() -> Self {
        World {
            entity_counter: 0,
            entities: HashMap::new(),
            resources: HashMap::new(),
        }
    }

    pub fn add_entity(&mut self, components: Vec<Root<Obj>>) -> i32 {
        self.entity_counter += 1;
        self.entities.insert(self.entity_counter, HashMap::new());
        self.add_components(self.entity_counter, components);
        self.entity_counter
    }

    pub fn add_components(&mut self, entity: i32, components: Vec<Root<Obj>>) {
        let registered = self.entities.get_mut(&entity).unwrap();
        for c in components {
            registered.insert(c.class().to_string(), c);
        }
    }

    pub fn get_entities(&self, types: Vec<Root<Class>>) -> Vec<i32> {
        // let copy = types.to_vec();
        self.entities
            .keys()
            .filter(|&e| {
                types
                    .iter()
                    .all(|t| self.entities.get(e).unwrap().contains_key(&t.to_string()))
            })
            .cloned()
            .collect::<Vec<_>>()
    }

    pub fn get_component(&self, entity: i32, class: Root<Class>) -> Root<Obj> {
        self.entities
            .get(&entity)
            .unwrap()
            .get(&class.to_string())
            .unwrap()
            .to_owned()
    }

    pub fn get_components(&self, entity: i32, types: Vec<Root<Class>>) -> Vec<Root<Obj>> {
        types
            .iter()
            .map(|t| self.get_component(entity, t.to_owned()))
            .collect()
    }

    pub fn query(&self, types: Vec<Root<Class>>) -> Vec<(i32, Vec<Root<Obj>>)> {
        let entities = self.get_entities(types.clone());
        let mut data = vec![];
        for e in entities {
            data.push((e, self.get_components(e, types.clone())))
        }
        data
    }

    pub fn save(&mut self, key: Sym, val: Val) {
        self.resources.insert(key, val);
    }

    pub fn fetch(&self, key: Sym) -> Val {
        self.resources.get(&key).unwrap().to_owned()
    }
}

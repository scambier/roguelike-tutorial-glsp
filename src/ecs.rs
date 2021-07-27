use bracket_lib::prelude::console;
use glsp::prelude::*;
use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

use crate::gamelog::GameLog;

pub type Entity = i32;

#[inline(always)]
fn types_to_string(types: Vec<Root<Class>>) -> Vec<String> {
    types.iter().map(|t| t.to_string()).collect()
}

/// A simple, naïve (and absolutely unoptimized) ECS world
pub struct World {
    entity_counter: Entity,
    entities: HashMap<Entity, HashMap<String, Root<Obj>>>,
    resources: HashMap<Sym, Val>,
}

impl World {
    pub fn bind_world() -> GResult<()> {
        glsp::bind_rfn("World", &World::new)?;
        glsp::RClassBuilder::<World>::new()
            .met("add-entity", &World::add_entity)
            .met("get-entities", &World::get_entities_glsp)
            .met("get-cmp", &World::get_components_glsp)
            .met("add-cmp", &World::add_components)
            .met("del-cmp", &World::remove_component)
            .met("clear-cmp", &World::clear_component)
            .met("query", &World::query)
            .met("save", &World::save)
            .met("fetch", &World::fetch)
            .met("delete", &World::delete_entity)
            .build();
        Ok(())
    }

    fn new() -> Self {
        World {
            entity_counter: 0,
            entities: HashMap::new(),
            resources: HashMap::new(),
        }
    }

    fn add_entity(&mut self, components: Vec<Root<Obj>>) -> Entity {
        self.entity_counter += 1;
        self.entities.insert(self.entity_counter, HashMap::new());
        self.add_components(self.entity_counter, components);
        self.entity_counter
    }

    fn add_components(&mut self, entity: Entity, components: Vec<Root<Obj>>) {
        let registered = self.entities.get_mut(&entity);
        match registered {
            Some(registered) => {
                for c in components {
                    registered.insert(c.class().to_string(), c);
                }
            }
            None => {
                let message = format!(
                    "Could not add components - entity {:} does not exist",
                    entity
                );
                console::log(&message);
                GameLog::rglobal_add(&message);
            }
        }
    }

    fn get_entities_glsp(&self, types: Vec<Root<Class>>) -> Vec<Entity> {
        self.get_entities(&types_to_string(types))
    }

    fn get_entities(&self, types: &Vec<String>) -> Vec<Entity> {
        // let copy = types.to_vec();
        self.entities
            .keys()
            .filter(|&e| {
                types
                    .iter()
                    .all(|t| self.entities.get(e).unwrap().contains_key(t))
            })
            .copied()
            .collect()
    }

    fn get_component_glsp(&self, entity: Entity, class: Root<Class>) -> Option<Root<Obj>> {
        let class = class.to_string();
        self.get_component(entity, &class)
    }

    fn get_component(&self, entity: Entity, class: &String) -> Option<Root<Obj>> {
        match self.entities.get(&entity) {
            Some(entity) => match entity.get(&class.to_string()) {
                Some(cmp) => Some(cmp.to_owned()),
                None => None,
            },
            None => {
                println!("Entity {} does not exist anymore", entity);
                None
            }
        }
    }

    fn get_components_glsp(
        &self,
        entity: Entity,
        types: Vec<Root<Class>>,
    ) -> Vec<Option<Root<Obj>>> {
        let types = types_to_string(types);
        self.get_components(entity, &types)
    }

    fn get_components(&self, entity: Entity, types: &Vec<String>) -> Vec<Option<Root<Obj>>> {
        types
            .iter()
            .map(|t| self.get_component(entity, &t))
            .collect()
    }

    fn query(&self, types: Vec<Root<Class>>) -> Vec<(Entity, Vec<Root<Obj>>)> {
        let types = types_to_string(types);
        let entities = self.get_entities(&types);
        // entities
        //     .iter()
        //     .map(|e| {
        //         (
        //             *e,
        //             self.get_components(*e, &types)
        //                 .iter()
        //                 .map(|cmp| cmp.to_owned().unwrap())
        //                 .collect::<Vec<_>>(),
        //         )
        //     })
        // .collect()

        let mut data = vec![];
        for e in entities {
            data.push((
                e,
                self.get_components(e, &types)
                    .iter()
                    .map(|cmp| cmp.to_owned().unwrap())
                    .collect(),
            ))
        }
        data
    }

    fn save(&mut self, key: Sym, val: Val) {
        self.resources.insert(key, val);
    }

    fn fetch(&self, key: Sym) -> Val {
        self.resources.get(&key).unwrap().to_owned()
    }

    fn remove_component(&mut self, entity: Entity, cmp_type: &Root<Class>) {
        let k = cmp_type.to_string();
        match self.entities.get_mut(&entity) {
            Some(components) => {
                components.remove(&k);
            }
            None => todo!(),
        }
    }

    /// Removes all components
    fn clear_component(&mut self, cmp_type: Root<Class>) {
        let k = &cmp_type.to_string();
        let to_clean = self
            .entities
            .iter()
            .filter(|(_, cmps)| cmps.contains_key(k))
            .map(|(e, _)| *e)
            .collect::<Vec<Entity>>();
        for e in to_clean {
            self.remove_component(e, &cmp_type);
        }
    }

    fn delete_entity(&mut self, entity: Entity) {
        self.entities.remove(&entity);
    }
}

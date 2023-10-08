use crate::utils::Position;
use std::collections::HashMap;

use gloo_console::log;

use yew::UseStateHandle;
use yew::hook;
use yew::Callback;
use yew::use_state;

use yew_hooks::UseMapHandle;
use yew_hooks::use_map;

#[hook]
pub fn use_handle_registry() -> (UseMapHandle<String, Position>, Callback<(String, Option<Position>), ()>) {
    let handle_registry: UseMapHandle<String, Position> = use_map(HashMap::new());

    let update_handle_registry = {
        let handle_registry = handle_registry.clone();
        Callback::from(move |(new_registry_key, new_registry_value)| {
            match new_registry_value {
                Some(new_registry_value) => {
                    handle_registry.insert(new_registry_key, new_registry_value);
                },
                None => {
                    handle_registry.remove(&new_registry_key);
                }
            };
        })
    };

    (handle_registry, update_handle_registry)
}

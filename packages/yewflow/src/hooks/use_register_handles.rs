use crate::utils::AttributeExtractHelper;
use gloo_console::warn;
use web_sys::{MutationRecord, MutationObserver, MutationObserverInit, HtmlElement, HtmlCollection};
use yew::{NodeRef, hook, use_effect_with};
use yew_hooks::UseMapHandle;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use utils::Position;

use crate::{utils, panel::Viewport};

#[hook]
pub fn use_register_handles(node_ref: NodeRef, handle_registry: UseMapHandle<String, Position>, viewport: Viewport) {

    fn search_and_register(element: HtmlElement, handle_registry: UseMapHandle<String, Position>) {
        if let Ok(children) = element.children().dyn_into::<HtmlCollection>() {
            let array = js_sys::Array::from(&children);
            array.for_each(&mut |child: wasm_bindgen::JsValue, _: u32, _: js_sys::Array| {
                let _ = child.dyn_into::<HtmlElement>().map(|child| {
                    let handle_registry = handle_registry.clone();
                    let class_names = child.get_class_names();

                    if class_names.contains(&"handle".to_string()) {
                        let id = child.id();
                        let rect = child.get_bounding_client_rect();
                        let x = rect.x();
                        let y = rect.y();
                        let center_offset_x = rect.width() / 2.0;
                        let center_offset_y = rect.height() / 2.0;
                        handle_registry.insert(id.clone(), (x + center_offset_x, y + center_offset_y));
                    }

                    search_and_register(child, handle_registry);
                });
            })
        }
    }

    {
        let viewport = viewport;
        let node_ref = node_ref.clone();
        let handle_registry = handle_registry.clone();
        use_effect_with((viewport, node_ref), move |(_viewport, node_ref)| {
            let element = node_ref.cast::<HtmlElement>();
            if let Some(element) = element.clone() {
                search_and_register(element.clone(), handle_registry.clone());
            }
        });
    }

    {
        let node_ref = node_ref.clone();
        use_effect_with((), move |_| {

            let element = node_ref.cast::<HtmlElement>();

            if let Some(element) = element.clone() {
                search_and_register(element.clone(), handle_registry.clone())
            }

            let cb: Closure<dyn std::ops::Fn(Vec<MutationRecord>, MutationObserver) -> ()> = Closure::new(move |_mutation_list: Vec<MutationRecord>, _mutation_observer: MutationObserver| {
                if let Some(element) = element.clone() {
                    search_and_register(element.clone(), handle_registry.clone());
                }
            });
    
            let observer = MutationObserver::new(cb.as_ref().unchecked_ref());
    
            let observing = observer.clone().and_then(|observer| {
                node_ref.get().ok_or_else(|| "could not get the node".into()).and_then(|node| {
                    observer.observe_with_options(&node, MutationObserverInit::new().subtree(true).child_list(true).attributes(true))
                })
            });
    
            match observing {
                Ok(()) => {},
                Err(e) => warn!(e)
            };

            {
                let observer = observer.clone();
                || {
                    cb.forget();
                    if let Ok(observer) = observer { observer.disconnect() }
                }
            }
        })
    }
}
// use std::{ops::{DerefMut, Deref}, rc::Rc, cell::RefCell, collections::HashMap, borrow::{BorrowMut, Borrow}};

// use yew::{hook, use_state, UseStateHandle};

// use crate::utils::nonce;

// pub struct UseMapInner<K, V> {
//     nonce: u32,
//     map: Rc<RefCell<&HashMap<K, V>>>
// }

// pub struct UseMapHandle<K, V> {
//     handle: UseStateHandle<UseMapInner<K, V>>
// }

// impl<K, V> Deref for UseMapHandle<K, V> {
//     type Target = HashMap<K, V>;

//     fn deref(&self) -> &Self::Target {
//         // let handle = &self.handle;
//         // let map = handle.map.clone();
//         // let ref_cell = map.deref();
//         // let to_owned = ref_cell.to_owned();
//         // &to_owned.borrow()
//     }
// }

// impl<K, V> PartialEq for UseMapHandle<K, V> {
//     fn eq(&self, other: &Self) -> bool {
//         self.handle.nonce == other.handle.nonce
//     }
// }

// impl<K, V> Borrow<UseMapHandleRef<K, V>> for UseMapHandle<K, V> {
//     fn borrow(&self) -> &UseMapHandleRef<K, V> {
//         &UseMapHandleRef { 
//             handle: self.handle
//         }
//     }
// }

// impl<K, V> BorrowMut<UseMapHandleRef<K, V>> for UseMapHandle<K, V> {
//     fn borrow_mut(&mut self) -> &mut UseMapHandleRef<K, V> {
//         &mut UseMapHandleRef {
//             handle: self.handle
//         }
//     }
// }

// pub struct UseMapHandleRef<K, V> {
//     handle: UseStateHandle<UseMapInner<K, V>>,

// }

// impl<K, V> Drop for UseMapHandleRef<K, V> {
//     fn drop(&mut self) {
//         self.handle.set(UseMapInner { nonce: nonce(), map: self.handle.map.clone() });
//     }
// }

// impl<K, V> Deref for UseMapHandleRef<K, V> {
//     type Target = HashMap<K, V>;

//     fn deref(&self) -> &Self::Target {
//         let handle = self.handle;
//         let map = *handle.map;
//         &map.borrow()
//     }
// }

// impl<K, V> DerefMut for UseMapHandleRef<K, V> {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         let handle = self.handle;
//         let map = *handle.map;
//         &mut map.borrow_mut()
//     }
// }

// #[hook]
// pub fn use_map<K, V>() -> UseMapHandle<K, V> {
//     let state: UseStateHandle<UseMapInner<K, V>> = use_state(|| UseMapInner {
//         nonce: 0,
//         map: Rc::new(RefCell::new(HashMap::new()))
//     });

//     UseMapHandle { handle: state }
// }
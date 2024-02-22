use web_sys::HtmlElement;
use yew::NodeRef;
use web_sys::DomRect;

use crate::utils::Position;

#[derive(PartialEq, Clone, Copy)]
pub struct Viewport {
    /*The absolute x coordinate of the current viewport (in pixels) */
    pub x: f64,
    /*The absolute y coordinate of the current viewport (in pixels) */
    pub y: f64,
    /*The zoom level (default is 1.0) */
    pub z: f64,
    /*The previous value of x. Needed for panning */
    old_x: f64,
    /*The previous value of y. Needed for panning */
    old_y: f64
}

impl Viewport {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Viewport {
            x,
            y,
            z,
            old_x: x,
            old_y: y,
        }
    }

    pub fn pan_start(&self, (x, y): Position) -> Self {
        Viewport {
            x: self.x,
            y: self.y,
            z: self.z,
            old_x: x,
            old_y: y,
        }
    }

    pub fn pan(&self, (x, y): Position) -> Self {
        Viewport {
            x: self.x + (x - self.old_x),
            y: self.y + (y - self.old_y),
            z: self.z,
            old_x: x,
            old_y: y
        }
    }

    pub fn zoom(&self, container_rect: DomRect, (_x, _y, z): (f64, f64, f64), (mouse_x, mouse_y): (f64, f64)) -> Self {
        let new_z = self.z + z;
        /*Hardcoded limits on zooming in and out. Strange things happen if you go into the negatives */
        if !(0.5..=2.0).contains(&new_z) {
            *self
        } else {
            Viewport {
                x: self.x + ((container_rect.width() * (z / 2.0)) / self.z) + (((mouse_x - self.x) * self.z - (mouse_x - self.x) * new_z) / self.z),
                y: self.y + (((mouse_y - self.y) * self.z - (mouse_y - self.y) * new_z) / self.z),
                z: new_z,
                old_x: self.old_x,
                old_y: self.old_y
            }
        }

    }

    pub fn center(&self, panel_ref: NodeRef, (nodes_width, nodes_height): (f64, f64)) -> Self {
        panel_ref.cast::<HtmlElement>().map(|panel_ref| {
            let panel_width = panel_ref.client_width() as f64;
            let panel_height = panel_ref.client_height() as f64;
            
            Viewport {
                x: (panel_width / 2.0) - (nodes_width / 2.0),
                y: (panel_height / 2.0) - (nodes_height / 2.0),
                z: 1.0,
                old_x: self.x,
                old_y: self.y,
            }
        }).unwrap_or(*self)
    }
}
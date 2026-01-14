use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use wayland_client::{protocol::wl_registry, Connection, Dispatch, Proxy, QueueHandle, event_created_child};
use cosmic_protocols::toplevel_info::v1::client::{
    zcosmic_toplevel_info_v1::{self, ZcosmicToplevelInfoV1},
    zcosmic_toplevel_handle_v1::{self, ZcosmicToplevelHandleV1},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct WindowInfo {
    app_id: String,
    title: String,
}

#[derive(Debug, Serialize)]
enum OutputMessage {
    Window(WindowInfo),
    NoWindow,
    Error(String),
}

struct AppState {
    toplevel_info: Option<ZcosmicToplevelInfoV1>,
    toplevels: HashMap<ZcosmicToplevelHandleV1, ToplevelData>,
    current_window: Option<WindowInfo>,
}

struct ToplevelData {
    title: String,
    app_id: String,
    activated: bool,
}

impl Dispatch<wl_registry::WlRegistry, ()> for AppState {
    fn event(
        state: &mut Self,
        registry: &wl_registry::WlRegistry,
        event: wl_registry::Event,
        _: &(),
        _: &Connection,
        qh: &QueueHandle<AppState>,
    ) {
        if let wl_registry::Event::Global {
            name,
            interface,
            version,
        } = event
        {
            if interface == ZcosmicToplevelInfoV1::interface().name {
                // Bind to version 1 to get automatic toplevel events
                let info = registry.bind::<ZcosmicToplevelInfoV1, _, _>(
                    name,
                    1,
                    qh,
                    (),
                );
                state.toplevel_info = Some(info);
            }
        }
    }
}

impl Dispatch<ZcosmicToplevelInfoV1, ()> for AppState {
    fn event(
        state: &mut Self,
        _info: &ZcosmicToplevelInfoV1,
        event: zcosmic_toplevel_info_v1::Event,
        _: &(),
        _: &Connection,
        _qh: &QueueHandle<AppState>,
    ) {
        if let zcosmic_toplevel_info_v1::Event::Toplevel { toplevel } = event {
            state.toplevels.insert(
                toplevel.clone(),
                ToplevelData {
                    title: String::new(),
                    app_id: String::new(),
                    activated: false,
                },
            );
        }
    }

    event_created_child!(AppState, ZcosmicToplevelInfoV1, [
        0 => (ZcosmicToplevelHandleV1, ()),
    ]);
}

impl Dispatch<ZcosmicToplevelHandleV1, ()> for AppState {
    fn event(
        state: &mut Self,
        handle: &ZcosmicToplevelHandleV1,
        event: zcosmic_toplevel_handle_v1::Event,
        _: &(),
        _: &Connection,
        _qh: &QueueHandle<AppState>,
    ) {
        match event {
            zcosmic_toplevel_handle_v1::Event::Title { title } => {
                if let Some(toplevel) = state.toplevels.get_mut(handle) {
                    toplevel.title = title;
                    state.update_current_window();
                }
            }
            zcosmic_toplevel_handle_v1::Event::AppId { app_id } => {
                if let Some(toplevel) = state.toplevels.get_mut(handle) {
                    toplevel.app_id = app_id;
                    state.update_current_window();
                }
            }
            zcosmic_toplevel_handle_v1::Event::State { state: window_state } => {
                if let Some(toplevel) = state.toplevels.get_mut(handle) {
                    // Check if activated state is set (state 2 = activated)
                    // The state array contains u8 values where 2 indicates activated
                    toplevel.activated = window_state.contains(&2);
                    state.update_current_window();
                }
            }
            zcosmic_toplevel_handle_v1::Event::Closed => {
                state.toplevels.remove(handle);
                state.update_current_window();
            }
            _ => {}
        }
    }
}

impl AppState {
    fn update_current_window(&mut self) {
        // Find the activated window, or fall back to first available
        self.current_window = self
            .toplevels
            .values()
            .find(|t| t.activated)
            .or_else(|| self.toplevels.values().next())
            .map(|t| WindowInfo {
                app_id: t.app_id.clone(),
                title: t.title.clone(),
            });
    }

    fn send_current_window(&self) {
        let msg = match &self.current_window {
            Some(window) => OutputMessage::Window(window.clone()),
            None => OutputMessage::NoWindow,
        };
        println!("{}", serde_json::to_string(&msg).unwrap());
    }
}

fn main() {
    let conn = match Connection::connect_to_env() {
        Ok(conn) => conn,
        Err(e) => {
            eprintln!(
                "{}",
                serde_json::to_string(&OutputMessage::Error(format!(
                    "Failed to connect to Wayland display: {}",
                    e
                )))
                .unwrap()
            );
            std::process::exit(1);
        }
    };

    let display = conn.display();
    let mut event_queue = conn.new_event_queue::<AppState>();
    let qh = event_queue.handle();

    let mut state = AppState {
        toplevel_info: None,
        toplevels: HashMap::new(),
        current_window: None,
    };

    let registry = display.get_registry(&qh, ());
    let _registry = registry;
    event_queue.roundtrip(&mut state).unwrap();

    if state.toplevel_info.is_none() {
        eprintln!(
            "{}",
            serde_json::to_string(&OutputMessage::Error(
                "zcosmic-toplevel-info-v1 protocol not available".to_string()
            ))
            .unwrap()
        );
        std::process::exit(1);
    }

    // Do a blocking roundtrip to receive initial toplevel events
    event_queue.roundtrip(&mut state).unwrap();
    
    state.send_current_window();

    loop {
        // Use blocking_dispatch to properly wait for events
        event_queue.blocking_dispatch(&mut state).unwrap();
        state.send_current_window();
    }
}

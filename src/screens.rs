use crate::config::DaVinciConfig;

mod debug;
pub mod editor;
mod clipboard;

pub(crate) struct ScreenManager {
    active_screen: Option<ActiveScreen>
}
pub(crate) struct EditorScreen {
    screen: editor::Editor,
}
pub(crate) struct DebugScreen {
    screen: debug::DebugScreen,
}
pub(crate) struct ClipboardScreen {
    screen: clipboard::ClipboardScreen,
}
impl ScreenManager {
    pub(crate) fn new() -> Self {
        Self {
            active_screen: None
        }
    }
    
    pub(crate) fn set_active_screen(&mut self, screen: ActiveScreen) {
        self.active_screen = Some(screen);
    }
    
    pub(crate) fn active_screen(&self) -> Option<&ActiveScreen> {
        self.active_screen.as_ref()
    }
    
    pub(crate) fn active_screen_mut(&mut self) -> Option<&mut ActiveScreen> {
        self.active_screen.as_mut()
    }

    pub(crate) fn show_editor_screen(&mut self, config: DaVinciConfig) {
        let editor = editor::Editor::new(config);
        self.set_active_screen(ActiveScreen::Editor(EditorScreen { screen: editor }));
    }
    
    pub(crate) fn run_active(&mut self) {
        match self.active_screen_mut() {
            Some(ActiveScreen::Editor(editor)) => editor.run(),
            Some(ActiveScreen::Debug(debug)) => debug.run(),
            Some(ActiveScreen::Clipboard(clipboard)) => clipboard.run(),
            None => {
                eprintln!("No active screen to run.");
            }
        }
    }
    
}

impl EditorScreen {
    pub(crate) fn run(&mut self) {
        while self.screen.run().expect("Could not run DaVinci Editor") {}
    }
}

impl DebugScreen {
    pub(crate) fn run(&mut self) {
        //while self.screen.run().expect("Could not run DaVinci Debugger") {}
    }
}

impl ClipboardScreen {
    pub(crate) fn run(&mut self) {
        // Implement clipboard screen logic here
    }}

pub enum ActiveScreen {
    Editor(EditorScreen),
    Debug(DebugScreen),
    Clipboard(ClipboardScreen),
}

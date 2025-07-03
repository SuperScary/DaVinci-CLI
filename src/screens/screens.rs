use crate::config::NinjaConfig;
use crate::screens::{clipboard, debug, editor};

pub struct ScreenManager {
    active_screen: Option<ActiveScreen>
}
pub struct EditorScreen {
    screen: editor::Editor,
}
pub struct DebugScreen {
    screen: debug::DebugScreen,
}
pub struct ClipboardScreen {
    screen: clipboard::ClipboardScreen,
}
impl ScreenManager {
    pub fn new() -> Self {
        Self {
            active_screen: None
        }
    }
    
    pub fn set_active_screen(&mut self, screen: ActiveScreen) {
        self.active_screen = Some(screen);
    }
    
    pub fn active_screen(&self) -> Option<&ActiveScreen> {
        self.active_screen.as_ref()
    }
    
    pub fn active_screen_mut(&mut self) -> Option<&mut ActiveScreen> {
        self.active_screen.as_mut()
    }

    pub fn show_editor_screen(&mut self, config: NinjaConfig) {
        let editor = editor::Editor::new(config);
        self.set_active_screen(ActiveScreen::Editor(EditorScreen { screen: editor }));
    }
    
    pub fn run_active(&mut self) {
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
    pub fn run(&mut self) {
        while self.screen.run().expect("Could not run Ninja Editor") {}
    }
}

impl DebugScreen {
    pub fn run(&mut self) {
        //while self.screen.run().expect("Could not run Ninja Debugger") {}
    }
}

impl ClipboardScreen {
    pub fn run(&mut self) {
        // Implement clipboard screen logic here
    }}

pub enum ActiveScreen {
    Editor(EditorScreen),
    Debug(DebugScreen),
    Clipboard(ClipboardScreen),
}

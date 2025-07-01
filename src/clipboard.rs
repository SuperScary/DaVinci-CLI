use cli_clipboard::{ClipboardContext, ClipboardProvider};

pub(crate) struct Clipboard {
    stack: Vec<String>,
    ctx: ClipboardContext
}

impl Clipboard {
    pub(crate) fn new() -> Self {
        Clipboard { stack: Vec::new(), ctx: ClipboardContext::new().unwrap() }
    }
    
    /// Initialize the clipboard with the current clipboard contents if they exist.
    pub(crate) fn init(mut self) -> Self {
        if let Ok(stack) = self.ctx.get_contents() { 
            self.stack.push(stack);
        }
        self
    }

    /// Copy text to the clipboard stack at the top
    pub(crate) fn add(&mut self, text: String) {
        if !text.is_empty() {
            self.stack.push(text.clone());
            self.ctx.set_contents(text.clone()).unwrap();
        }
    }

    /// Paste and remove the last copied text
    pub(crate) fn paste(&mut self) -> Option<String> {
        if let Some(text) = self.stack.pop() {
            Some(text)
        } else {
            None
        }
    }

    /// Paste without removing the last copied text
    pub(crate) fn paste_peek(&self) -> Option<String> {
        if let Some(text) = self.stack.last() {
            Some(text.clone())
        } else {
            None
        }
    }

    pub(crate) fn peek(&self) -> Option<&String> {
        self.stack.last()
    }

    pub(crate) fn size(&self) -> usize {
        self.stack.len()
    }
    
    pub(crate) fn clear(&mut self) {
        self.stack.clear();
    }
    
    pub(crate) fn get(&self, index: usize) -> Option<&String> {
        self.stack.get(index)
    }
    
    pub(crate) fn get_top(&self) -> Option<&String> {
        self.stack.last()
    }
    
    pub(crate) fn remove(&mut self, index: usize) -> Option<String> {
        if index < self.stack.len() {
            Some(self.stack.remove(index))
        } else {
            None
        }
    }
    
    pub(crate) fn get_contents(&self) -> Vec<String> {
        self.stack.clone()
    }
    
    pub(crate) fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }
}
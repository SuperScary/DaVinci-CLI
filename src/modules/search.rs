use crate::modules::highlighting::HighlightType;

pub(crate) enum SearchDirection {
    Forward,
    Backward,
}

pub(crate) struct SearchIndex {
    pub(crate) x_index: usize,
    pub(crate) y_index: usize,
    pub(crate) x_direction: Option<SearchDirection>,
    pub(crate) y_direction: Option<SearchDirection>,
    pub(crate) previous_highlight: Option<(usize, Vec<HighlightType>)>,
}

impl SearchIndex {
    pub(crate) fn new() -> Self {
        Self {
            x_index: 0,
            y_index: 0,
            x_direction: None,
            y_direction: None,
            previous_highlight: None,
        }
    }

    pub(crate) fn reset(&mut self) {
        self.y_index = 0;
        self.x_index = 0;
        self.y_direction = None;
        self.x_direction = None;
        self.previous_highlight = None
    }
}
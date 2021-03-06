use super::*;

pub struct Row {
    pub group: Group,
}

impl Row {
    pub fn new(cells: Vec<Cell>) -> Self {
        Self {
            group: Group::new(cells),
        }
    }

    pub fn wrap(widgets: Vec<Cell>) -> Box<Self> {
        Box::new(Self::new(widgets))
    }

    pub fn focus_next(&mut self) {
        self.group.focus_next();
    }

    pub fn focus_prev(&mut self) {
        self.group.focus_prev();
    }
}

impl Widget for Row {
    fn render(&mut self, context: &mut Rcui, rect: &Rect, active: bool) {
        let n = self.group.cells.len();
        let cell_size = self.group.cell_size(rect.w);
        let mut x = rect.x;
        for i in 0..n {
            let widget_size = self.group.cells[i].size(cell_size);
            self.group.cells[i].get_widget_mut().render(
                context,
                &Rect {
                    x,
                    y: rect.y,
                    w: widget_size,
                    h: rect.h,
                },
                active && i == self.group.focus,
            );
            x += widget_size;
        }
    }

    fn handle_event(&mut self, context: &mut Rcui, event: &Event) {
        self.group.handle_event(context, event);
    }
}

mod input;
mod symbol;

use crate::input::KanaInputArea;
use qt_widgets::{QApplication, QWidget, qt_core::qs, QVBoxLayout};
use symbol::KanaSymbol;


static WINDOW_TITLE: &str = "Kana Trainer";

fn main() {
    QApplication::init(|_app| unsafe {
        let widget = QWidget::new_0a();
        widget.set_window_title(&qs(WINDOW_TITLE));
        let layout = QVBoxLayout::new_0a();        
        widget.set_layout(&layout);

        let kana_input = KanaInputArea::new(&layout);

        widget.show();

        kana_input.set_timer(30);
        QApplication::exec()
    })
}

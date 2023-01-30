mod input;

use crate::input::KanaInputArea;
use qt_widgets::{QApplication, QWidget, qt_core::qs, QVBoxLayout};



static WINDOW_TITLE: &str = "Kana Trainer";

fn main() {
    QApplication::init(|_app| unsafe {
        let widget = QWidget::new_0a();
        widget.set_window_title(&qs(WINDOW_TITLE));
        let layout = QVBoxLayout::new_0a();        
        widget.set_layout(&layout);

        let _kana_input = KanaInputArea::new(&layout);
        
        widget.show();
        QApplication::exec()
    })
}

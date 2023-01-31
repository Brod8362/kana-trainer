mod input;
mod symbol;

use crate::input::KanaInputArea;
use qt_widgets::{QApplication, QWidget, qt_core::qs, QVBoxLayout};


static WINDOW_TITLE: &str = "Kana Trainer";

fn main() {
    QApplication::init(|_app| unsafe {
        let widget = QWidget::new_0a();
        widget.set_window_title(&qs(WINDOW_TITLE));
        let layout = QVBoxLayout::new_0a();        
        widget.set_layout(&layout);

        let hira_family = symbol::parse_symbols_from_file("hiragana.json").expect("failed to parse hiragana");
        let kata_family = symbol::parse_symbols_from_file("katakana.json").expect("failed to parse katakana");
        let hangul_family = symbol::parse_symbols_from_file("hangul.json").expect("failed to parse hangul");

        let _kana_input = KanaInputArea::new(&layout, &vec![hangul_family, hira_family, kata_family]);


        widget.show();
        QApplication::exec()
    })
}

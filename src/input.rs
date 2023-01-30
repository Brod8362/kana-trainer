use std::{rc::Rc, cell::RefCell};

use cpp_core::{StaticUpcast, Ptr, Ref};
use qt_widgets::{QWidget, QLineEdit, QHBoxLayout, QVBoxLayout, QProgressBar, QLabel, qt_gui::QFont};
use qt_core::{QObject, QBox, QString, slot, SlotOfQString, SlotNoArgs, QTimer, qs, AlignmentFlag, QFlags};

use crate::symbol::{KanaSymbol, self};

static TIMER_BAR_RESOLUTION: i32 = 100; //10 "ticks" per second

pub struct KanaInputArea {
    widget: QBox<QWidget>,
    symbol_label: QBox<QLabel>,
    timer: QBox<QTimer>,
    timer_bar: QBox<QProgressBar>,
    line_edit: QBox<QLineEdit>,
    current_symbol: RefCell<Option<KanaSymbol>>
}

impl StaticUpcast<QObject> for KanaInputArea {
    unsafe fn static_upcast(ptr: Ptr<Self>) -> Ptr<QObject> {
        ptr.widget.as_ptr().static_upcast()
    }
}

impl KanaInputArea {
    pub fn new(parent_layout: &QVBoxLayout) -> Rc<KanaInputArea> {
        unsafe {
            let widget = QWidget::new_0a();
            let layout = QVBoxLayout::new_1a(&widget);
            widget.set_layout(&layout);

            // Input widget for typing the answer
            let line_edit = QLineEdit::new();

            //Label for showing the symbol you need to type
            let font = QFont::new();
            font.set_family(&qs("Arial"));
            font.set_bold(true);
            font.set_point_size(32);
            let symbol_label = QLabel::new();
            symbol_label.set_font(&font);
            symbol_label.set_text(&qs(""));
            symbol_label.set_alignment(QFlags::from(AlignmentFlag::AlignCenter));
            
            let timer_box = QHBoxLayout::new_1a(&widget);
            let clock_label = QLabel::new();
            clock_label.set_text(&qs("⏱ Timer"));

            // Used to display the bar. Generally updated automatically via the timer
            let timer_bar = QProgressBar::new_1a(&widget);

            timer_box.add_widget(&clock_label);
            timer_box.add_widget(&timer_bar);

            //Internal timer used for updating the bar and other events
            let timer = QTimer::new_1a(&widget);
            
            layout.add_widget(&symbol_label);
            layout.add_layout_1a(&timer_box);
            layout.add_widget(&line_edit);

            parent_layout.add_widget(&widget);
            
            let this = Rc::new(Self {
                widget,
                symbol_label,
                timer,
                timer_bar,
                line_edit,
                current_symbol: RefCell::new(None)
            });

            this.init();
            this
        }
    }

    pub unsafe fn set_timer(self: &Rc<Self>, duration_sec: i32) {
        self.timer_bar.set_value(duration_sec*TIMER_BAR_RESOLUTION);
        self.timer_bar.set_maximum(duration_sec*TIMER_BAR_RESOLUTION);
        self.timer.start_1a(1000/TIMER_BAR_RESOLUTION);
    }

    #[slot(SlotNoArgs)]
    pub unsafe fn on_time_update(self: &Rc<Self>) {
        let value = self.timer_bar.value();
        if value <= 0 {
            //TODO emit failure signal
            self.timer.stop();
            self.timer_bar.set_format(&qs("Time's Up!"));
        } else {
            self.timer_bar.set_value(value-1);
            self.timer_bar.set_format(&qs(format!("{}s", value/TIMER_BAR_RESOLUTION)));
        }
    }

    unsafe fn init(self: &Rc<Self>) {
        self.timer.timeout().connect(&self.slot_on_time_update());
        self.line_edit
            .text_edited()
            .connect(&self.slot_on_text_edited());
    }

    #[slot(SlotOfQString)]
    unsafe fn on_text_edited(self: &Rc<Self>, text_content: Ref<QString>) {
        let refer = self.current_symbol.borrow();
        match &(*refer) {
            Some(symbol) => {
                for trans in symbol.get_translations() {
                    if &text_content.to_std_string() == trans {
                        //TODO emit success signal
                        self.set_timer(5);
                        self.line_edit.clear();
                    }
                }
            },  
            None => {}
        }
    }

    //Symbol managaement

    pub unsafe fn set_symbol(self: &Rc<Self>, symbol: &KanaSymbol) {
        let mut refer = self.current_symbol.borrow_mut();
        *refer = Some(symbol.clone());
        self.symbol_label.set_text(&qs(symbol.get_display()));
    }

    pub unsafe fn clear_symbol(self: &Rc<Self>) {
        let mut refer = self.current_symbol.borrow_mut();
        *refer = None;
        self.symbol_label.clear();
    }

}
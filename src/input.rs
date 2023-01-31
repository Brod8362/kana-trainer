use std::{rc::Rc, cell::Cell, borrow::Borrow};

use cpp_core::{StaticUpcast, Ptr, Ref};
use qt_widgets::{QWidget, QLineEdit, QHBoxLayout, QVBoxLayout, QProgressBar, QLabel, qt_gui::QFont};
use qt_core::{QObject, QBox, QString, slot, SlotOfQString, SlotNoArgs, QTimer, qs, AlignmentFlag, QFlags, SignalOfBool};

use crate::symbol::{KanaSymbol, SymbolFamily};

static TIMER_BAR_RESOLUTION: i32 = 100; //10 "ticks" per second

static STYLE_SHEET_SUCCESS: &str = "{color: #00FF00}";
static STYLE_SHEET_FAILURE: &str = "{color: #FF0000}";

pub struct KanaInputArea {
    widget: QBox<QWidget>,
    previous_symbol_label: QBox<QLabel>,
    symbol_label: QBox<QLabel>,
    timer: QBox<QTimer>,
    timer_bar: QBox<QProgressBar>,
    line_edit: QBox<QLineEdit>,
    current_symbol: Cell<Option<KanaSymbol>>,
    complete_signal: QBox<SignalOfBool>,
    families: Vec<SymbolFamily>,
}

impl StaticUpcast<QObject> for KanaInputArea {
    unsafe fn static_upcast(ptr: Ptr<Self>) -> Ptr<QObject> {
        ptr.widget.as_ptr().static_upcast()
    }
}

impl KanaInputArea {
    pub fn new(parent_layout: &QVBoxLayout, families: &Vec<SymbolFamily>) -> Rc<KanaInputArea> {
        unsafe {
            let widget = QWidget::new_0a();
            let layout = QVBoxLayout::new_1a(&widget);
            widget.set_layout(&layout);

            // Input widget for typing the answer
            let line_edit = QLineEdit::new();

            // Label for showing the previous (failed) entry
            let secondary_font = QFont::new();
            secondary_font.set_family(&qs("Arial"));
            secondary_font.set_bold(true);
            secondary_font.set_point_size(32);
            let previous_symbol_label = QLabel::new();
            previous_symbol_label.set_font(&secondary_font);
            previous_symbol_label.set_alignment(QFlags::from(AlignmentFlag::AlignCenter));

            //Label for showing the symbol you need to type
            let primary_font = QFont::new();
            primary_font.set_family(&qs("Arial"));
            primary_font.set_bold(true);
            primary_font.set_point_size(64);
            let symbol_label = QLabel::new();
            symbol_label.set_font(&primary_font);
            symbol_label.set_text(&qs(""));
            symbol_label.set_alignment(QFlags::from(AlignmentFlag::AlignCenter));

            let complete_signal = SignalOfBool::new();
            
            let timer_box = QHBoxLayout::new_1a(&widget);
            let clock_label = QLabel::new();
            clock_label.set_text(&qs("⏱ Timer"));

            // Used to display the bar. Generally updated automatically via the timer
            let timer_bar = QProgressBar::new_1a(&widget);

            timer_box.add_widget(&clock_label);
            timer_box.add_widget(&timer_bar);

            //Internal timer used for updating the bar and other events
            let timer = QTimer::new_1a(&widget);
            
            layout.add_widget(&previous_symbol_label);
            layout.add_widget(&symbol_label);
            layout.add_layout_1a(&timer_box);
            layout.add_widget(&line_edit);

            parent_layout.add_widget(&widget);
            
            let this = Rc::new(Self {
                widget,
                previous_symbol_label,
                symbol_label,
                timer,
                timer_bar,
                line_edit,
                current_symbol: Cell::new(None),
                complete_signal,
                families: families.clone()
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
    unsafe fn on_time_update(self: &Rc<Self>) {
        let value = self.timer_bar.value();        
        if value <= 0 {
            
            match symbol {
                Some(symbol) => self.on_failure(&symbol),
                None => {}
            }
            
        } else {
            self.timer_bar.set_value(value-1);
            self.timer_bar.set_format(&qs(format!("{}s", value/TIMER_BAR_RESOLUTION)));
        }
    }

    #[slot(SlotNoArgs)]
    unsafe fn on_enter_pressed(self: &Rc<Self>) {
        //TODO: immediately mark current one as failed
        self.set_random_symbol()
    }

    unsafe fn init(self: &Rc<Self>) {
        self.timer.timeout().connect(&self.slot_on_time_update());
        self.line_edit
            .text_edited()
            .connect(&self.slot_on_text_edited());
        self.line_edit
            .return_pressed()
            .connect(&self.slot_on_enter_pressed());
    }

    #[slot(SlotOfQString)]
    unsafe fn on_text_edited(self: &Rc<Self>, text_content: Ref<QString>) {
        if self.timer_bar.value() <= 0 {
            return;
        }
        
        match *self.current_symbol.as_ptr() {
            Some(symbol) => {
                for trans in symbol.get_translations() {
                    if &text_content.to_std_string() == trans {
                        self.on_success(&symbol);
                        break;
                    }
                }
            },  
            None => {}
        }
    }

    pub unsafe fn on_success(self: &Rc<Self>, character: &KanaSymbol) {
        println!("{} entered successfully", character.get_display());
        self.update_previous_symbol(character, true);
        self.complete_signal.emit(true);
        self.set_random_symbol();
    }

    pub unsafe fn on_failure(self: &Rc<Self>, character: &KanaSymbol) {
        self.timer.stop();
        self.timer_bar.set_format(&qs("Time's Up!"));
        self.update_previous_symbol(character, false);
        self.complete_signal.emit(false);
    }

    unsafe fn set_random_symbol(self: &Rc<Self>) {
        let family = &self.families[0];
        self.set_symbol(family.random_symbol());
    }

    //Symbol managaement

    pub unsafe fn set_symbol(self: &Rc<Self>, symbol: &KanaSymbol) {
        self.current_symbol.set(Some(symbol.clone()));
        self.symbol_label.set_text(&qs(symbol.get_display()));
        self.set_timer(10);
        self.line_edit.clear();
    }

    pub unsafe fn clear_symbol(self: &Rc<Self>) {
        self.current_symbol.set(None);
        self.symbol_label.clear();
    }

    pub fn on_complete<'a>(self: &'a Rc<Self>) -> &'a QBox<SignalOfBool> {
        &self.complete_signal
    }

    unsafe fn update_previous_symbol(self: &Rc<Self>, symbol: &KanaSymbol, success: bool) {
        self.previous_symbol_label.set_text(&qs(format!("{} - {}", symbol.get_display(), symbol.get_translations().join(","))));
        if success {
            self.previous_symbol_label.set_style_sheet(&qs(STYLE_SHEET_SUCCESS));
        } else {
            self.previous_symbol_label.set_style_sheet(&qs(STYLE_SHEET_FAILURE));
        }
    }

}
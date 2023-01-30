use std::rc::Rc;

use cpp_core::{StaticUpcast, Ptr};
use qt_widgets::{QWidget, QLineEdit, QVBoxLayout};
use qt_core::{SlotNoArgs, QObject, QBox, slot, SlotOfQString, QString};

pub struct KanaInputArea {
    widget: QBox<QWidget>,
    line_edit: QBox<QLineEdit>
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

            let line_edit = QLineEdit::new();

            layout.add_widget(&line_edit);

            parent_layout.add_widget(&widget);
            
            let this = Rc::new(Self {
                widget,
                line_edit
            });

            this.init();
            this
        }
    }

    unsafe fn init(self: &Rc<Self>) {
        self.line_edit
            .text_edited()
            .connect(&SlotOfQString::new(&self.widget, move |str| {
                println!("{}", str.to_std_string());
            }));
    }

    

}
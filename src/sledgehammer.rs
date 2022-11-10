use core::panic;

use js_sys::Math;
use sledgehammer::channel::MaybeId;
use sledgehammer::element::Element;
use sledgehammer::*;
use web_sys::{console, Node};

const ADJECTIVES_LEN: usize = 25;
const ADJECTIVES: &[&str; ADJECTIVES_LEN] = &[
    "pretty",
    "large",
    "big",
    "small",
    "tall",
    "short",
    "long",
    "handsome",
    "plain",
    "quaint",
    "clean",
    "elegant",
    "easy",
    "angry",
    "crazy",
    "helpful",
    "mushy",
    "odd",
    "unsightly",
    "adorable",
    "important",
    "inexpensive",
    "cheap",
    "expensive",
    "fancy",
];
const TBODY_ID: NodeId = NodeId(1);
const ROW_ID: NodeId = NodeId(2);
const TEMP_ID: NodeId = NodeId(3);

const COLOURS_LEN: usize = 11;
const COLOURS: &[&str; COLOURS_LEN] = &[
    "red", "yellow", "blue", "green", "pink", "brown", "purple", "brown", "white", "black",
    "orange",
];

const NOUNS_LEN: usize = 13;
const NOUNS: &[&str; NOUNS_LEN] = &[
    "table", "chair", "house", "bbq", "desk", "car", "pony", "cookie", "sandwich", "burger",
    "pizza", "mouse", "keyboard",
];

fn random(max: usize) -> usize {
    (Math::random() * 1000.0) as usize % max
}

struct Row {
    id: usize,
    row_data: Box<RowData>,
}
struct RowData {
    label: [&'static str; 3],
    excited: u8,
    ptr: u32,
}

impl Row {
    #[inline(always)]
    const fn el(&self) -> NodeId {
        NodeId(self.row_data.ptr)
    }

    #[inline(always)]
    const fn label_node(&self) -> NodeId {
        NodeId(self.row_data.ptr + 1)
    }
}

pub struct Main {
    data: Vec<Row>,
    selected: Option<NodeId>,
    msg: MsgChannel,
}

impl Main {
    pub fn run(&mut self) {
        self.clear();
        self.append_rows(1000)
    }

    pub fn add(&mut self) {
        self.append_rows(1000)
    }

    pub fn update(&mut self) {
        let mut i = 0;
        let l = self.data.len();
        while i < l {
            let row = &mut self.data[i];
            row.row_data.excited += 1;
            self.msg.set_text(
                |w: &mut Vec<u8>| {
                    let label = &row.row_data.label;
                    let adjective = label[0];
                    let colour = label[1];
                    let noun = label[2];
                    adjective.write_as_text(w);
                    ' '.write_as_text(w);
                    colour.write_as_text(w);
                    ' '.write_as_text(w);
                    noun.write_as_text(w);
                    for _ in 0..row.row_data.excited {
                        " !!!".write_as_text(w);
                    }
                },
                MaybeId::Node(row.label_node()),
            );
            i += 10;
        }
        self.msg.flush();
    }

    pub fn unselect(&mut self) {
        if let Some(el) = self.selected.take() {
            self.msg
                .remove_attribute(Attribute::class, MaybeId::Node(el));
        }
    }

    #[inline(never)]
    pub fn select(&mut self, id: usize) {
        self.unselect();
        if let Some(row) = self.data.iter().find(|r| r.id == id) {
            self.msg
                .set_attribute(Attribute::class, "danger", MaybeId::Node(row.el()));
            self.selected = Some(row.el());
            self.msg.flush();
        } else {
            panic!("Row not found");
        }
    }

    pub fn delete(&mut self, id: usize) {
        let row = match self.data.iter().position(|row| row.id == id) {
            Some(i) => self.data.remove(i),
            None => panic!("Row not found"),
        };
        self.msg.remove(MaybeId::Node(row.el()));
        self.msg.flush();
    }

    pub fn clear(&mut self) {
        self.data.clear();
        self.msg.set_text("", MaybeId::Node(TBODY_ID));
        self.unselect();
        self.msg.flush();
    }

    pub fn run_lots(&mut self) {
        self.clear();
        self.append_rows(10000)
    }

    pub fn swap_rows(&mut self) {
        if self.data.len() <= 998 {
            return;
        }
        let row1 = &self.data[1];
        let row2 = &self.data[2];
        let row998 = &self.data[998];
        let row999 = &self.data[999];

        self.msg
            .insert_before(MaybeId::Node(row2.el()), MaybeId::Node(row998.el()));
        self.msg
            .insert_before(MaybeId::Node(row999.el()), MaybeId::Node(row1.el()));

        self.msg.flush();
        self.data.swap(1, 998);
    }

    pub fn append_rows(&mut self, count: usize) {
        // web_sys::console::log_1(&format!("append_rows {}", count).into());
        // self.data
        //     .reserve((count + self.rows).saturating_sub(self.data.capacity()));
        // const BATCH_SIZE: usize = 5000;
        // for x in 0..(count / BATCH_SIZE) {
        for x in 0..count {
            // for y in 0..BATCH_SIZE {
            // let i = self.rows + y + x * BATCH_SIZE;
            let i = x;
            let id = i + 1;

            let label = [
                ADJECTIVES[random(ADJECTIVES_LEN)],
                COLOURS[random(COLOURS_LEN)],
                NOUNS[random(NOUNS_LEN)],
            ];

            let el = i as u32 * 2 + 1 + TEMP_ID.0;
            let el_id = NodeId(el);
            let label_node = NodeId(el + 1);
            self.msg
                .clone_node(MaybeId::Node(ROW_ID), MaybeId::Node(el_id));
            self.msg.set_attribute("data-id", id, MaybeId::LastNode);
            self.msg
                .append_child(MaybeId::Node(TBODY_ID), MaybeId::Node(el_id));
            self.msg.first_child();
            self.msg.set_text(id, MaybeId::LastNode);
            self.msg.next_sibling();
            self.msg.first_child();
            self.msg.store_with_id(label_node);
            self.msg.set_text(
                |w: &mut Vec<u8>| {
                    let adjective = label[0];
                    let colour = label[1];
                    let noun = label[2];
                    adjective.write_as_text(w);
                    ' '.write_as_text(w);
                    colour.write_as_text(w);
                    ' '.write_as_text(w);
                    noun.write_as_text(w);
                },
                MaybeId::LastNode,
            );

            self.data.push(Row {
                id,
                row_data: Box::new(RowData {
                    label,
                    excited: 0,
                    ptr: el,
                }),
            });
        }
        self.msg.flush();
        // }
    }
}

pub fn init() -> Main {
    console::log_1(&format!("size of Maybe Id: {}", std::mem::size_of::<MaybeId>()).into());
    console::log_1(&format!("size of NodeId: {}", std::mem::size_of::<NodeId>()).into());
    console::log_1(&format!("size of Row: {}", std::mem::size_of::<Box<Row>>()).into());
    console::log_1(&format!("size of MsgChannel: {}", std::mem::size_of::<MsgChannel>()).into());
    console::log_1(&format!("size of Node: {}", std::mem::size_of::<Node>()).into());
    console::log_1(
        &format!(
            "size of ElementBuilder: {}",
            std::mem::size_of::<ElementBuilder<'static>>()
        )
        .into(),
    );
    console::log_1(&format!("size of NOUNS: {}", size_of_(NOUNS)).into());
    console::log_1(&format!("size of COLOURS: {}", size_of_(COLOURS)).into());
    console::log_1(&format!("size of ADJECTIVES: {}", size_of_(ADJECTIVES)).into());

    fn size_of_<T>(_: T) -> usize {
        std::mem::size_of::<T>()
    }

    let window = web_sys::window().expect("window");
    let document = window.document().expect("document");

    const EL: ElementBuilder<'static> = ElementBuilder::new(Element::tr.any_element_const())
        .id(ROW_ID)
        .children(&[
            NodeBuilder::Element(
                ElementBuilder::new(Element::td.any_element_const())
                    .attrs(&[(Attribute::class.any_attr_const(), "col-md-1")]),
            ),
            NodeBuilder::Element(
                ElementBuilder::new(Element::td.any_element_const())
                    .attrs(&[(Attribute::class.any_attr_const(), "col-md-4")])
                    .children(&[NodeBuilder::Element(
                        ElementBuilder::new(Element::a.any_element_const())
                            .attrs(&[(Attribute::class.any_attr_const(), "lbl")]),
                    )]),
            ),
            NodeBuilder::Element(
                ElementBuilder::new(Element::td.any_element_const())
                    .attrs(&[(Attribute::class.any_attr_const(), "col-md-1")])
                    .children(&[NodeBuilder::Element(
                        ElementBuilder::new(Element::a.any_element_const())
                            .attrs(&[(Attribute::class.any_attr_const(), "remove")])
                            .children(&[NodeBuilder::Element(
                                ElementBuilder::new(Element::span.any_element_const()).attrs(&[
                                    (
                                        Attribute::class.any_attr_const(),
                                        "remove glyphicon glyphicon-remove",
                                    ),
                                    (Attribute::aria_hidden.any_attr_const(), "true"),
                                ]),
                            )]),
                    )]),
            ),
            NodeBuilder::Element(
                ElementBuilder::new(Element::td.any_element_const())
                    .attrs(&[(Attribute::class.any_attr_const(), "col-md-6")]),
            ),
        ]);

    let tbody = document.get_element_by_id("tbody").expect("tbody");
    let mut msg = MsgChannel::default();
    msg.set_node(TBODY_ID, tbody.into());
    msg.build_full_element(EL);
    msg.flush();

    Main {
        data: Vec::new(),
        selected: None,
        msg,
    }
}

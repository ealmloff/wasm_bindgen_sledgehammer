use core::panic;

use js_sys::Math;
use sledgehammer::builder::MaybeId;
use sledgehammer::element::Element;
use sledgehammer::*;
use ufmt::uwrite;

const ADJECTIVES_LEN: usize = 25;
const ADJECTIVES: [&str; ADJECTIVES_LEN] = [
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
const COLOURS: [&str; COLOURS_LEN] = [
    "red", "yellow", "blue", "green", "pink", "brown", "purple", "brown", "white", "black",
    "orange",
];

const NOUNS_LEN: usize = 13;
const NOUNS: [&str; NOUNS_LEN] = [
    "table", "chair", "house", "bbq", "desk", "car", "pony", "cookie", "sandwich", "burger",
    "pizza", "mouse", "keyboard",
];

fn random(max: usize) -> usize {
    (Math::random() * 1000.0) as usize % max
}

struct Row {
    id: usize,
    label: String,
    excited: u8,
    ptr: u32,
}

impl Row {
    #[inline]
    const fn el(&self) -> NodeId {
        NodeId(self.ptr)
    }

    #[inline]
    const fn label_node(&self) -> NodeId {
        NodeId(self.ptr + 1)
    }
}

pub struct Main {
    data: Vec<Row>,
    rows: usize,
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

    #[inline(never)]
    pub fn update(&mut self) {
        let mut i = 0;
        let l = self.data.len();
        while i < l {
            let row = &mut self.data[i];
            row.excited += 1;
            let mut label = row.label.to_string();
            for _ in 0..row.excited {
                label += " !!!";
            }
            self.msg
                .set_text(label.as_str(), MaybeId::Node(row.label_node()));
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

    pub fn select(&mut self, id: usize) {
        self.unselect();
        for row in &self.data {
            if row.id == id {
                self.msg
                    .set_attribute(Attribute::class, "danger", MaybeId::Node(row.el()));
                self.selected = Some(row.el());
                self.msg.flush();
                return;
            }
        }
        panic!("Row not found");
    }

    pub fn delete(&mut self, id: usize) {
        let row = match self.data.iter().position(|row| row.id == id) {
            Some(i) => self.data.remove(i),
            None => panic!("Row not found"),
        };
        self.msg.remove(MaybeId::Node(row.el()));
        self.msg.flush();
        self.rows -= 1;
    }

    pub fn clear(&mut self) {
        self.data.clear();
        self.msg.set_text("", MaybeId::Node(TBODY_ID));
        self.unselect();
        self.msg.flush();
        self.rows = 0;
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
            .insert_before(MaybeId::Node(row2.el()), row998.el());
        self.msg
            .insert_before(MaybeId::Node(row999.el()), row1.el());

        self.msg.flush();
        self.data.swap(1, 998);
    }

    pub fn append_rows(&mut self, count: usize) {
        // web_sys::console::log_1(&format!("append_rows {}", count).into());
        self.data
            .reserve((count + self.rows).saturating_sub(self.data.capacity()));
        // const BATCH_SIZE: usize = 5000;
        // for x in 0..(count / BATCH_SIZE) {
        for x in 0..count {
            // for y in 0..BATCH_SIZE {
            // let i = self.rows + y + x * BATCH_SIZE;
            let i = self.rows + x;
            let id = i + 1;

            let adjective = ADJECTIVES[random(ADJECTIVES_LEN)];
            let colour = COLOURS[random(COLOURS_LEN)];
            let noun = NOUNS[random(NOUNS_LEN)];
            let capacity = adjective.len() + colour.len() + noun.len() + 2;
            let mut label = String::with_capacity(capacity);
            label.push_str(adjective);
            label.push(' ');
            label.push_str(colour);
            label.push(' ');
            label.push_str(noun);

            let el = i as u32 * 2 + 1 + TEMP_ID.0;
            let el_id = NodeId(el);
            let label_node = NodeId(el + 1);
            self.msg
                .clone_node(MaybeId::Node(ROW_ID), MaybeId::Node(el_id));
            self.msg.set_attribute("data-id", id, MaybeId::LastNode);
            self.msg.append_child(MaybeId::Node(TBODY_ID), el_id);
            self.msg.first_child();
            self.msg.set_text(id, MaybeId::LastNode);
            self.msg.next_sibling();
            self.msg.first_child();
            self.msg.store_with_id(label_node);
            self.msg.set_text(label.as_str(), MaybeId::LastNode);

            let row = Row {
                id,
                label,
                ptr: el,
                excited: 0,
            };

            self.data.push(row);
        }
        self.msg.flush();
        // }
        self.rows += count;
    }
}

pub fn init() -> Main {
    let window = web_sys::window().expect("window");
    let document = window.document().expect("document");

    const EL: ElementBuilder<
        Element,
        (),
        (
            ElementBuilder<Element, ((Attribute, &str),), ()>,
            ElementBuilder<
                Element,
                ((Attribute, &str),),
                (ElementBuilder<Element, ((Attribute, &str),), ()>,),
            >,
            ElementBuilder<
                Element,
                ((Attribute, &str),),
                (
                    ElementBuilder<
                        Element,
                        ((Attribute, &str),),
                        (ElementBuilder<Element, ((Attribute, &str), (Attribute, &str)), ()>,),
                    >,
                ),
            >,
            ElementBuilder<Element, ((Attribute, &str),), ()>,
        ),
    > = ElementBuilder::new(
        MaybeId::Node(ROW_ID),
        Element::tr,
        (),
        (
            ElementBuilder::new(
                MaybeId::LastNode,
                Element::td,
                ((Attribute::class, "col-md-1"),),
                (),
            ),
            ElementBuilder::new(
                MaybeId::LastNode,
                Element::td,
                ((Attribute::class, "col-md-4"),),
                (ElementBuilder::new(
                    MaybeId::LastNode,
                    Element::a,
                    ((Attribute::class, "lbl"),),
                    (),
                ),),
            ),
            ElementBuilder::new(
                MaybeId::LastNode,
                Element::td,
                ((Attribute::class, "col-md-1"),),
                (ElementBuilder::new(
                    MaybeId::LastNode,
                    Element::a,
                    ((Attribute::class, "remove"),),
                    (ElementBuilder::new(
                        MaybeId::LastNode,
                        Element::span,
                        (
                            (Attribute::class, "remove glyphicon glyphicon-remove"),
                            (Attribute::aria_hidden, "true"),
                        ),
                        (),
                    ),),
                ),),
            ),
            ElementBuilder::new(
                MaybeId::LastNode,
                Element::td,
                ((Attribute::class, "col-md-6"),),
                (),
            ),
        ),
    );

    let tbody = document.get_element_by_id("tbody").expect("tbody");
    let mut msg = MsgChannel::new(tbody.clone());
    msg.build_full_element(EL);
    msg.set_node(TBODY_ID, tbody.into());
    msg.flush();

    Main {
        data: Vec::new(),
        rows: 0,
        selected: None,
        msg,
    }
}

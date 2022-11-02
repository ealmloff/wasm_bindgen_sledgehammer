use core::panic;

use js_sys::Math;
use wasm_bindgen::JsCast;
use web_sys::{Element, Node};

const ADJECTIVES_LEN: usize = 25;
const ADJECTIVES_LEN_F64: f64 = ADJECTIVES_LEN as f64;
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

const COLOURS_LEN: usize = 11;
const COLOURS_LEN_F64: f64 = COLOURS_LEN as f64;
const COLOURS: [&str; COLOURS_LEN] = [
    "red", "yellow", "blue", "green", "pink", "brown", "purple", "brown", "white", "black",
    "orange",
];

const NOUNS_LEN: usize = 13;
const NOUNS_LEN_F64: f64 = NOUNS_LEN as f64;
const NOUNS: [&str; NOUNS_LEN] = [
    "table", "chair", "house", "bbq", "desk", "car", "pony", "cookie", "sandwich", "burger",
    "pizza", "mouse", "keyboard",
];

fn random(max: f64) -> usize {
    ((Math::random() * 1000.0) % max) as usize
}

struct Row {
    id: usize,
    label: String,
    el: Element,
    label_node: Node,
}

const ROW_TEMPLATE: &str = "<td class='col-md-1'></td><td class='col-md-4'><a class='lbl'></a></td><td class='col-md-1'><a class='remove'><span class='remove glyphicon glyphicon-remove' aria-hidden='true'></span></a></td><td class='col-md-6'></td>";

pub struct Main {
    data: Vec<Row>,
    row_template: Node,
    tbody: Node,
    selected: Option<Element>,
}

impl Main {
    pub fn run(&mut self) {
        self.clear();
        self.append_rows(1000);
    }

    pub fn add(&mut self) {
        self.append_rows(1000);
    }

    #[inline(never)]
    pub fn update(&mut self) {
        let mut i = 0;
        let l = self.data.len();
        while i < l {
            let row = &mut self.data[i];
            row.label.push_str(" !!!");
            // row.label_node.set_text_content(Some(row.label.as_str()));
            row.label_node.set_text_content(Some(row.label.as_str()));
            i += 10;
        }
    }

    pub fn unselect(&mut self) {
        if let Some(el) = self.selected.take() {
            el.set_class_name("");
        }
    }

    pub fn select(&mut self, id: usize) {
        self.unselect();
        for row in &self.data {
            if row.id == id {
                row.el.set_class_name("danger");
                self.selected = Some(row.el.clone());
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
        row.el.remove();
    }

    pub fn clear(&mut self) {
        self.data = Vec::new();
        self.tbody.set_text_content(None);
        self.unselect();
    }

    pub fn run_lots(&mut self) {
        self.clear();
        self.append_rows(10000);
    }

    pub fn swap_rows(&mut self) {
        if self.data.len() <= 998 {
            return;
        }
        let row1 = &self.data[1];
        let row998 = &self.data[998];
        let a = &row1.el;
        let b = a.next_sibling().unwrap();
        let c = &row998.el;
        let d = c.next_sibling().unwrap();
        self.tbody.insert_before(c, Some(&b)).unwrap();
        self.tbody.insert_before(a, Some(&d)).unwrap();
        self.data.swap(1, 998);
    }

    pub fn append_rows(&mut self, count: usize) {
        self.data.reserve(count);
        for i in 0..count {
            let id = i + 1;

            let adjective = ADJECTIVES[random(ADJECTIVES_LEN_F64)];
            let colour = COLOURS[random(COLOURS_LEN_F64)];
            let noun = NOUNS[random(NOUNS_LEN_F64)];
            let capacity = adjective.len() + colour.len() + noun.len() + 2;
            let mut label = String::with_capacity(capacity);
            label.push_str(adjective);
            label.push(' ');
            label.push_str(colour);
            label.push(' ');
            label.push_str(noun);

            let node = self.row_template.clone_node_with_deep(true).unwrap();
            let id_node = node.first_child().unwrap();
            let label_node = id_node.next_sibling().unwrap().first_child().unwrap();
            let id_string = id.to_string();
            let id_str = id_string.as_str();
            id_node.set_text_content(Some(id_str));
            label_node.set_text_content(Some(label.as_str()));

            let el = JsCast::unchecked_into::<Element>(node);
            el.set_attribute("data-id", id_str).unwrap();
            let row = Row {
                id,
                label,
                el,
                label_node,
            };

            self.tbody.append_child(&row.el).unwrap();
            self.data.push(row);
        }
    }
}

pub fn init() -> Main {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    let row_template = document.create_element("tr").unwrap();
    row_template.set_inner_html(ROW_TEMPLATE);

    let tbody = document.get_element_by_id("tbody").unwrap();

    Main {
        data: Vec::new(),
        row_template: row_template.into(),
        tbody: tbody.into(),
        selected: None,
    }
}

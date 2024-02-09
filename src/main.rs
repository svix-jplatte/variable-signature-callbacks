mod from_value;
mod visitor;

use from_value::{StringFirstChar, StringLastChar};

use self::visitor::Visitor;

#[derive(Clone, Debug)]
enum Value {
    I32(i32),
    String(String),
}

type BoxedVisitor = Box<dyn Fn(&Value)>;

struct ValueVisitorRegistry {
    visitors: Vec<BoxedVisitor>,
}

impl ValueVisitorRegistry {
    fn new() -> Self {
        Self {
            visitors: Vec::new(),
        }
    }

    fn add<T: 'static>(&mut self, visitor: impl Visitor<T>) {
        self.visitors
            .push(Box::new(move |value| visitor.visit(value)));
    }

    fn visit(&self, value: &Value) {
        for visitor in &self.visitors {
            visitor(value);
        }
    }
}

fn main() {
    let mut registry = ValueVisitorRegistry::new();
    registry.add(|_: Value| {
        println!("got a value!");
    });
    registry.add(|value: i32| {
        println!("got an integer.");
        value
    });
    registry.add(
        |StringFirstChar(f): StringFirstChar, StringLastChar(l): StringLastChar| {
            println!("string value: {f}...{l}");
        },
    );

    registry.visit(&Value::I32(10));
    registry.visit(&Value::String("hello".to_owned()));
}

// Factory Pattern

pub fn factory() {
    let f = Factory;
    let product_closure = || ConcreteProductX;
    println!("{}", f.convert(String::from("hoge -> nyan"), product_closure));
}

trait Product {
    fn convert(&self, String) -> String;
}

struct Factory;

impl Factory {
    fn convert<P, F>(&self, s: String, create_product: F) -> String
        where P: Product,
              F: FnOnce() -> P
    {
        create_product().convert(s)
    }
}

struct ConcreteProductX;

impl Product for ConcreteProductX {
    fn convert(&self, s: String) -> String {
        s.to_uppercase()
    }
}

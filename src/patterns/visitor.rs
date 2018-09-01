pub fn visitor() {
    use self::Entity::*;
    let entity = Dir(
        "/".to_string(),
        vec![
            File("etc".to_string()),
            File("usr".to_string()),
            Dir("var/".to_string(), vec![File("log".to_string())]),
        ],
    );
    let mut visitor = ConcreteFileVisitor;
    visitor.visit(&entity);
}

trait Visitor<T> {
    fn visit(&mut self, &T);
}

enum Entity {
    File(String),
    Dir(String, Vec<Entity>),
}

struct ConcreteFileVisitor;

impl Visitor<Entity> for ConcreteFileVisitor {
    fn visit(&mut self, entity: &Entity) {
        use self::Entity::*;
        match *entity {
            File(ref name) => println!("file: {}", name),
            Dir(ref name, ref files) => {
                println!("dir: {}", name);
                for file in files {
                    self.visit(file)
                }
            }
        }
    }
}

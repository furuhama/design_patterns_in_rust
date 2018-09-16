// Builder pattern
//
// Builder: has generation logic
// Director: generates Object

pub fn builder() {
    let bonjin_builder = Box::new(BonjinBuilder::new());
    let mut bonjin_director = Director::new(bonjin_builder);
    let bonjin = bonjin_director.build();

    let superman_builder = Box::new(SupermanBuilder::new());
    let mut superman_director = Director::new(superman_builder);
    let superman = superman_director.build();

    println!("{:?}", bonjin);
    println!("{:?}", superman);
}

#[derive(Clone, Debug)]
struct Person<'a> {
    name: &'a str,
    age: usize,
}

impl<'a> Person<'a> {
    fn new() -> Self {
        Self {
            name: Default::default(),
            age: Default::default(),
        }
    }

    fn set_name(&mut self, name: &'a str) {
        self.name = name;
    }

    fn set_age(&mut self, age: usize) {
        self.age = age;
    }
}

trait PersonBuilder {
    fn build_name(&mut self);
    fn build_age(&mut self);
    fn build(&mut self) -> Person;
}

struct BonjinBuilder<'a> {
    obj: Person<'a>,
}

impl<'a> BonjinBuilder<'a> {
    fn new() -> Self {
        Self {
            obj: Person::new()
        }
    }
}

impl<'a> PersonBuilder for BonjinBuilder<'a> {
    fn build_name(&mut self) {
        self.obj.set_name("凡人");
    }

    fn build_age(&mut self) {
        self.obj.set_age(23);
    }

    fn build(&mut self) -> Person {
        self.obj.clone()
    }
}

struct SupermanBuilder<'a> {
    obj: Person<'a>,
}

impl<'a> SupermanBuilder<'a> {
    fn new() -> Self {
        Self {
            obj: Person::new()
        }
    }
}

impl<'a> PersonBuilder for SupermanBuilder<'a> {
    fn build_name(&mut self) {
        self.obj.set_name("スーパーマン");
    }

    fn build_age(&mut self) {
        self.obj.set_age(200);
    }

    fn build(&mut self) -> Person {
        self.obj.clone()
    }
}

struct Director {
    // DIrector takes a trait object as a attribute.
    builder: Box<PersonBuilder>,
}

impl Director {
    fn new(builder: Box<PersonBuilder>) -> Self {
        Self {
            builder,
        }
    }

    fn build(&mut self) -> Person {
        self.builder.build_name();
        self.builder.build_age();
        self.builder.build()
    }
}

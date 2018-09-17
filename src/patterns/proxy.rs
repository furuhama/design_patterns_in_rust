// Proxy pattern

pub fn proxy() {
    let mut rs = RealSubject::new();
    println!("Create RealSubject");
    println!("{}", rs.get_something());

    let mut p1 = Proxy::new();
    println!("Create Proxy p1");
    let mut p2 = Proxy::new();
    println!("Create Proxy p2");

    println!("Get value from Proxy p1: {}", p1.get_something());
    println!("Get value from Proxy p2: {}", p2.get_something());
}

trait Subject {
    fn get_something(&mut self) -> usize;
}

struct RealSubject {
    value: usize,
}

impl RealSubject {
    fn new() -> Self {
        let mut real_subject = RealSubject { value: 0 };
        real_subject.load_something();

        real_subject
    }

    fn load_something(&mut self) {
        println!("Try to load something, it is an extremely heavy process.");

        self.value = 100;
    }
}

impl Subject for RealSubject {
    fn get_something(&mut self) -> usize {
        self.value
    }
}

struct Proxy {
    subject: Option<RealSubject>,
}

impl Proxy {
    fn new() -> Self {
        Self {
            subject: None,
        }
    }
}

impl Subject for Proxy {
    fn get_something(&mut self) -> usize {
        match self.subject {
            Some(ref mut subject) => subject.get_something(),
            None => {
                let mut real_subject = RealSubject::new();
                let val = real_subject.get_something();
                self.subject = Some(real_subject);

                val
            },
        }
    }
}

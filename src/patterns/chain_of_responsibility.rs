// Chain of Responsibility (= CoR) pattern

pub fn cor() {
    let high = ImplCoR::new(Level::High, None);
    let middle = ImplCoR::new(Level::Middle, Some(Box::new(high)));
    let low = ImplCoR::new(Level::Low, Some(Box::new(middle)));

    let mut r1 = ConcreteRequest::new(Level::High, 1);
    let mut r2 = ConcreteRequest::new(Level::Middle, 2);
    let mut r3 = ConcreteRequest::new(Level::Low, 3);

    low.process_request(&mut r3);
    low.process_request(&mut r2);
    low.process_request(&mut r1);
}

trait CoR {
    fn process_request(&self, &mut Request);
}

trait Request {
    fn get_level(&self) -> &Level;
    fn get_something(&self) -> usize;
}

struct ConcreteRequest {
    level: Level,
    v: usize,
}

impl ConcreteRequest {
    fn new(level: Level, v: usize) -> Self {
        Self {
            level,
            v,
        }
    }
}

impl Request for ConcreteRequest {
    fn get_level(&self) -> &Level {
        &self.level
    }

    fn get_something(&self) -> usize {
        self.v
    }
}

#[derive(Debug, PartialEq)]
enum Level {
    High,
    Middle,
    Low,
}

struct ImplCoR {
    next: Option<Box<CoR>>,
    allowable_level: Level,
}

impl ImplCoR {
    fn new(level: Level, next: Option<Box<CoR>>) -> Self {
        Self {
            next,
            allowable_level: level,
        }
    }
}

impl CoR for ImplCoR {
    fn process_request(&self, req: &mut Request) {
        print!("{:?} ", self.allowable_level);
        if self.allowable_level == *req.get_level() {
            println!("Request accepted: v = {}", req.get_something());
        } else {
            if let Some(ref next) = self.next {
                println!("Pass to the next");
                next.process_request(req);
            } else {
                println!("Chain finished.");
            }
        }
    }
}

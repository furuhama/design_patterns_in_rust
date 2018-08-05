// Subject manages all Events
// Observer is called from Subject when Event occurs

pub fn observer() {
    let mut subject = ConcreteSubject::new();
    subject.register_observer(Box::new(ConcreteObserver(1)));
    subject.register_observer(Box::new(ConcreteObserver(2)));
    subject.register_observer(Box::new(ConcreteObserver(3)));

    let event1 = EventObject::new(String::from("first event"));
    let event2 = EventObject::new(String::from("second event"));

    subject.notify_observers(&event1);
    subject.notify_observers(&event2);
}

// Event

trait Event {
    fn get_title(&self) -> &str;
}

#[derive(Debug, Clone)]
struct EventObject{
    title: String,
}

impl EventObject {
    fn new(title: String) -> EventObject {
        EventObject {
            title: title,
        }
    }
}

impl Event for EventObject {
    fn get_title(&self) -> &str {
        &self.title
    }
}

// Subject

trait Subject<T: Clone> {
    fn notify_observers(&self, &T);
    fn register_observer(&mut self, Box<Observer<T>>) -> usize;
    fn unregister_observer(&mut self, usize);
}

struct ConcreteSubject {
    observers: Vec<(bool, Box<Observer<EventObject>>)>,
}

impl ConcreteSubject {
    fn new() -> ConcreteSubject {
        ConcreteSubject {
            observers: Vec::new(),
        }
    }
}

impl Subject<EventObject> for ConcreteSubject {
    fn notify_observers(&self, event: &EventObject) {
        for observer in self.observers.iter() {
            if observer.0 {
                observer.1.on_notify(event);
            }
        }
    }

    fn register_observer(&mut self, observer: Box<Observer<EventObject>>) -> usize {
        self.observers.push((true, observer));
        self.observers.len() - 1
    }

    fn unregister_observer(&mut self, index: usize) {
        self.observers[index].0 = false;
    }
}

// Observer

trait Observer<T: Clone> {
    fn on_notify(&self, &T);
}

struct ConcreteObserver(usize);

impl Observer<EventObject> for ConcreteObserver {
    fn on_notify(&self, event: &EventObject) {
        println!("ConcreteObserver {} gets event: [{}]", self.0, event.get_title());
    }
}

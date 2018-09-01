pub fn iterator() {
    let mut c: Container<usize> = Container::<usize>::new();
    c.add(10);
    c.add(20);
    c.add(30);

    for i in c {
        println!("container value: {}", i);
    }

    let mut c2: Container<&str> = Container::<&str>::new();
    c2.add("no.1");
    c2.add("no.2");
    c2.add("no.3");

    for i in c2 {
        println!("container2 value: {}", i);
    }
}

// struct Container<T: Sized + Copy> {
struct Container<T: Copy> {
    buf: Vec<T>,
    index: usize,
}

impl<T: Copy> Container<T> {
    fn new() -> Container<T> {
        Container {
            buf: Vec::new(),
            index: 0,
        }
    }

    fn add(&mut self, t: T) {
        self.buf.push(t);
    }
}

impl<T: Copy> Iterator for Container<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.buf.len() {
            let t = Some(self.buf[self.index]);
            self.index += 1;
            t
        } else {
            None
        }
    }
}

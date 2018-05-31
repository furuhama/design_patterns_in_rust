// Command Pattern

pub fn command() {
    let mut r = Robot::new();

    let mut invoker = Invoker::new(&mut r);

    assert_eq!(
        *invoker.target(),
        Robot {
            x: 0,
            y: 0,
            dx: 0,
            dy: 1,
        }
    );

    {
        use self::RobotCommand::*;
        invoker.append_command(TurnRight);
        invoker.append_command(TurnLeft);
        invoker.append_command(MoveForward);
    }

    invoker.execute_all_commands();
    assert_eq!(
        *invoker.target(),
        Robot {
            x: 0,
            y: 1,
            dx: 0,
            dy: 1,
        }
    );

    invoker.undo();
    assert_eq!(
        *invoker.target(),
        Robot {
            x: 0,
            y: 0,
            dx: 0,
            dy: 1,
        }
    );

    invoker.undo();
    assert_eq!(
        *invoker.target(),
        Robot {
            x: 0,
            y: 0,
            dx: 1,
            dy: 0,
        }
    );
}

trait Command<T> {
    fn execute(&self, &mut T);
    fn undo(&self, &mut T);
}

struct Invoker<'a, Cmd, T: 'a> {
    commands: Vec<Cmd>,
    target: &'a mut T,
    current_index: usize,
}

impl<'a, Cmd, T> Invoker<'a, Cmd, T> {
    fn new(t: &'a mut T) -> Self {
        Invoker {
            commands: Vec::new(),
            target: t,
            current_index: 0,
        }
    }

    fn target(&self) -> &T {
        self.target
    }

    fn append_command(&mut self, c: Cmd) {
        self.commands.push(c);
    }
}

impl<'a, Cmd, T> Invoker<'a, Cmd, T>
where
    Cmd: Command<T>,
{
    fn execute_command(&mut self) {
        if self.commands.len() <= self.current_index {
            // Nothing to do.
            return;
        }

        let c = &self.commands[self.current_index];
        let t = &mut *self.target;
        c.execute(t);

        self.current_index += 1;
    }

    fn execute_all_commands(&mut self) {
        for _ in self.current_index..self.commands.len() {
            self.execute_command();
        }
    }

    fn undo(&mut self) {
        if self.current_index == 0 {
            return;
        }

        self.current_index -= 1;

        let c = &self.commands[self.current_index];
        let t = &mut *self.target;
        c.undo(t);
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Robot {
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
}

impl Robot {
    fn new() -> Robot {
        Robot {
            x: 0,
            y: 0,
            dx: 0,
            dy: 1,
        }
    }

    fn move_forward(&mut self) {
        self.x += self.dx;
        self.y += self.dy;
    }

    fn set_direction(&mut self, d: (i32, i32)) {
        self.dx = d.0;
        self.dy = d.1;
    }

    fn get_direction(&self) -> (i32, i32) {
        (self.dx, self.dy)
    }
}

enum RobotCommand {
    MoveForward,
    TurnRight,
    TurnLeft,
}

impl Command<Robot> for RobotCommand {
    fn execute(&self, r: &mut Robot) {
        use self::RobotCommand::*;
        match *self {
            MoveForward => r.move_forward(),
            TurnRight => {
                let (dx, dy) = r.get_direction();
                r.set_direction((dy, -dx))
            }
            TurnLeft => {
                let (dx, dy) = r.get_direction();
                r.set_direction((-dy, dx))
            }
        }
    }

    fn undo(&self, r: &mut Robot) {
        use self::RobotCommand::*;
        match *self {
            MoveForward => {
                let c = TurnRight;
                c.execute(r);
                c.execute(r);
                self.execute(r);
                c.execute(r);
                c.execute(r);
            }
            TurnRight => {
                let c = TurnLeft;
                c.execute(r);
            }
            TurnLeft => {
                let c = TurnRight;
                c.execute(r);
            }
        }
    }
}

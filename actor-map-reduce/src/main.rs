extern crate crux;

use crux::{spawn_actor, Actor, ActorStore};

#[derive(Clone)]
enum PrintAction {
    Set(i32),
    Print,
}

#[derive(Clone)]
struct IncrementActor {
    next: ActorStore<SquareActor>
}
impl Actor for IncrementActor {
    type Action = i32;
    fn receive(&mut self, action: i32) {
        self.next.dispatch_sync(action + 1);
    }
}

#[derive(Clone)]
struct SquareActor {
    next: ActorStore<SumActor>
}
impl Actor for SquareActor {
    type Action = i32;
    fn receive(&mut self, action: i32) {
        self.next.dispatch_sync(action * action);
    }
}

#[derive(Clone)]
struct SumActor {
    sum: i32,
    next: ActorStore<PrintActor>,
}
impl Actor for SumActor {
    type Action = i32;
    fn receive(&mut self, action: i32) {
        self.sum += action;
        self.next.dispatch_sync(PrintAction::Set(self.sum));
    }
}

#[derive(Clone)]
struct PrintActor {
    value: i32
}
impl Actor for PrintActor {
    type Action = PrintAction;
    fn receive(&mut self, action: Self::Action) {
        match action {
            PrintAction::Set(value) => self.value = value,
            PrintAction::Print => println!("{}", self.value),
        }
    }
}

fn main() {
    let mut print_actor = spawn_actor(PrintActor { value: 0 });
    let sum_actor = spawn_actor(SumActor {
        sum: 0, next: print_actor.clone()
    });
    let square_actor = spawn_actor(SquareActor {
        next: sum_actor.clone()
    });
    let mut increment_actor = spawn_actor(IncrementActor {
        next: square_actor.clone()
    });

    print_actor.dispatch_sync(PrintAction::Print);
    increment_actor.dispatch_sync(0);
    increment_actor.dispatch_sync(1);
    increment_actor.dispatch_sync(2);
    increment_actor.dispatch_sync(3);
    print_actor.dispatch_sync(PrintAction::Print);
}

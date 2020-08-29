use std::collections::HashMap;
use std::cell::{RefCell, Cell};

pub trait Observer<E> {
    fn notify(&self, event: &E);
}

#[derive(Copy, Clone, Hash, Eq, PartialEq, Default)]
pub struct ObserverRegistration(usize);

type ObserversStore<'a, E> = HashMap<ObserverRegistration, Box<dyn Observer<E> + 'a>>;

pub struct Observers<'a, E> {
    observers: Box<RefCell<ObserversStore<'a, E>>>,
    next: Cell<ObserverRegistration>,
}

impl<'a, E> Observers<'a, E> {
    pub fn new() -> Self {
        Observers {
            observers: Box::new(RefCell::new(Default::default())),
            next: Default::default(),
        }
    }

    pub fn register(&self, observer: Box<dyn Observer<E> + 'a>) -> ObserverRegistration {
        let mut observers = self.observers.borrow_mut();
        let panic_point = self.next.get();
        observers.insert(panic_point, observer);
        while observers.contains_key(&self.next.get()) {
            self.next.set(match self.next.get().0.checked_add(1) {
                Some(next) => ObserverRegistration(next),
                None => ObserverRegistration(0),
            });
            if self.next.get() == panic_point {
                panic!("ran out of observers space... what the hell are you doing?");
            }
        }
        self.next.get()
    }

    pub fn deregister(&self, registration: &ObserverRegistration) -> Option<Box<dyn Observer<E> + 'a>> {
        self.observers.borrow_mut().remove(registration)
    }
}

pub trait Observable<'a, E> {
    fn observers(&self) -> &Observers<'a, E>;

    fn notify_all(&self, event: &E)
    {
        for (_, observer) in self.observers().observers.borrow().iter() {
            observer.notify(event);
        }
    }
}



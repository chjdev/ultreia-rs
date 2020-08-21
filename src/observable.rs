use std::cell::{Cell, RefCell, RefMut};
use std::collections::HashMap;
use std::ops::Deref;
use std::rc::{Rc, Weak};

pub trait Observer<E> {
    fn notify(&self, event: &E) {}
}

#[derive(Copy, Clone, Hash, Eq, PartialEq, Default)]
pub struct ObserverRegistration(usize);

type ObserversStore<E> = HashMap<ObserverRegistration, Box<dyn Observer<E>>>;

pub struct Observers<E> {
    observers: Box<RefCell<ObserversStore<E>>>,
    next: Cell<ObserverRegistration>,
}

impl<E> Observers<E> {
    pub fn new() -> Self {
        Observers {
            observers: Box::new(RefCell::new(Default::default())),
            next: Default::default(),
        }
    }

    pub fn register(&self, observer: Box<dyn Observer<E>>) -> Result<ObserverRegistration, &str> {
        let mut observers = self.observers.borrow_mut();
        let panic_point = self.next.get();
        observers.insert(panic_point, observer);
        while observers.contains_key(&self.next.get()) {
            self.next.set(match self.next.get().0.checked_add(1) {
                Some(next) => ObserverRegistration(next),
                None => ObserverRegistration(0),
            });
            if self.next.get() == panic_point {
                return Err("ran out of observers space... what the hell are you doing?");
            }
        }
        Ok(self.next.get())
    }

    pub fn deregister(&self, registration: &ObserverRegistration) -> Option<Box<dyn Observer<E>>> {
        self.observers.borrow_mut().remove(registration)
    }
}

pub trait Observable<E> {
    fn observers(&self) -> &Observers<E>;

    fn notify_all(&self, event: &E)
    {
        for (_, observer) in self.observers().observers.borrow().iter() {
            observer.notify(event);
        }
    }
}



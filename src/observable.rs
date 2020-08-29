use std::collections::HashMap;
use std::cell::{RefCell, Cell};
use std::rc::Weak;

pub trait Observer<E> {
    fn notify(&self, event: &E);
}

#[derive(Copy, Clone, Hash, Eq, PartialEq, Default)]
pub struct ObserverRegistration(usize);

pub type WeakObserver<E> = Weak<dyn Observer<E>>;

type ObserversStore<E> = HashMap<ObserverRegistration, WeakObserver<E>>;

pub struct Observers<E> {
    observers: RefCell<ObserversStore<E>>,
    next: Cell<ObserverRegistration>,
}

impl<E> Observers<E> {
    pub fn new() -> Self {
        Observers {
            observers: RefCell::new(Default::default()),
            next: Default::default(),
        }
    }

    pub fn register(&self, observer: WeakObserver<E>) -> ObserverRegistration {
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

    pub fn deregister(&self, registration: &ObserverRegistration) -> Option<WeakObserver<E>> {
        self.observers.borrow_mut().remove(registration)
    }
}

pub trait Observable<E> {
    fn observers(&self) -> &Observers<E>;

    fn notify_all(&self, event: &E)
    {
        for (registration, maybe_observer) in self.observers().observers.borrow().iter() {
            if let Some(observer) = maybe_observer.upgrade() {
                observer.notify(event);
            } else {
                self.observers().deregister(registration);
            }
        }
    }
}



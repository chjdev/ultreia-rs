use std::collections::HashMap;
use std::sync::{Weak, RwLock};
use std::sync::atomic::{AtomicUsize, Ordering};

pub trait Observer<E> {
    fn notify(&self, event: &E);
}

#[derive(Copy, Clone, Hash, Eq, PartialEq, Default)]
pub struct ObserverRegistration(usize);

pub type WeakObserver<E> = Weak<dyn Observer<E> + Send + Sync>;

type ObserversStore<E> = HashMap<ObserverRegistration, WeakObserver<E>>;

pub struct Observers<E> {
    observers: RwLock<ObserversStore<E>>,
    next: AtomicUsize,
}

impl<E> Observers<E> {
    pub fn new() -> Self {
        Observers {
            observers: RwLock::new(Default::default()),
            next: Default::default(),
        }
    }

    pub fn register(&self, observer: WeakObserver<E>) -> ObserverRegistration {
        let mut observers = self.observers.write().expect("failed to acquire write lock in register");
        let safeguard = self.next.load(Ordering::SeqCst);
        let next_observer = ObserverRegistration(self.next.fetch_add(1, Ordering::AcqRel));
        if next_observer.0 < safeguard {
            panic!("ran out of observer registrations, what the hell are you doing?")
        }
        observers.insert(next_observer, observer);
        next_observer
    }

    pub fn deregister(&self, registration: &ObserverRegistration) -> Option<WeakObserver<E>> {
        self.observers.write().expect("failed to acquire write lock in deregister").remove(registration)
    }
}

pub trait Observable<E> {
    fn observers(&self) -> &Observers<E>;

    fn notify_all(&self, event: &E)
    {
        for (registration, maybe_observer) in self.observers().observers.read().expect("failed to acquire read lock").iter() {
            if let Some(observer) = maybe_observer.upgrade() {
                observer.notify(event);
            } else {
                self.observers().deregister(registration);
            }
        }
    }
}



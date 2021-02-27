use rayon::prelude::*;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::sync::{Arc, RwLock, Weak};

pub trait Observer<E>: Send + Sync {
    fn notify(&self, event: &E);
}

/// weak pointer so it will deregister itself automatically when dropped
type WeakObserver<E> = Weak<dyn Observer<E>>;

pub struct ObserverRegistration<E> {
    weak: WeakObserver<E>,
    // todo hmm is this safe? only used for the hash value
    ptr: usize,
}

impl<E> Clone for ObserverRegistration<E> {
    fn clone(&self) -> Self {
        ObserverRegistration {
            weak: Weak::clone(&self.weak),
            ptr: self.ptr,
        }
    }
}

impl<E> Hash for ObserverRegistration<E> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.ptr.hash(state);
    }
}

impl<E> Eq for ObserverRegistration<E> {}

impl<E> PartialEq for ObserverRegistration<E> {
    fn eq(&self, other: &Self) -> bool {
        self.ptr == other.ptr
    }
}

type ObserversStore<E> = HashSet<ObserverRegistration<E>>;

#[derive(Default)]
pub struct Observers<E> {
    observers: RwLock<ObserversStore<E>>,
}

impl<E> Observers<E> {
    pub fn new() -> Self {
        Observers {
            observers: Default::default(),
        }
    }

    // register/derigster doesn't need locking back off since there are no cycles
    pub fn register<SO>(&self, observer: &Arc<SO>) -> ObserverRegistration<E>
    where
        SO: 'static + Observer<E>,
    {
        let mut observers = self.observers.write().unwrap();
        let next_observer = ObserverRegistration {
            weak: Arc::downgrade(observer) as WeakObserver<E>,
            ptr: Arc::as_ptr(observer) as usize,
        };
        observers.insert(next_observer.clone());
        next_observer
    }

    pub fn deregister(&self, registration: &ObserverRegistration<E>) -> bool {
        self.observers.write().unwrap().remove(registration)
    }
}

pub trait Observable<E: Sync>: Sync {
    fn observers(&self) -> &Observers<E>;

    fn notify_all(&self, event: E) {
        self.observers()
            .observers
            .read()
            .unwrap()
            .par_iter()
            .for_each(|registration| {
                if let Some(observer) = registration.weak.upgrade() {
                    observer.notify(&event);
                } else {
                    // the observer has since freed
                    self.observers().deregister(registration);
                }
            });
    }
}

use std::collections::HashSet;
use std::sync::{Weak, RwLock, Arc};
use std::hash::{Hash, Hasher};

pub trait Observer<E> {
    fn notify(&self, event: &E);
}

type SomeObserver<E> = dyn Observer<E> + Send + Sync;

/// weak pointer so it will deregister itself automatically when dropped
type WeakObserver<E> = Weak<SomeObserver<E>>;

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

pub struct Observers<E> {
    observers: RwLock<ObserversStore<E>>,
}

impl<E> Observers<E> {
    pub fn new() -> Self {
        Observers {
            observers: RwLock::new(Default::default()),
        }
    }

    pub fn register<SO>(&self, observer: &Arc<SO>) -> ObserverRegistration<E> where SO: 'static + Observer<E> + Send + Sync {
        let mut observers = self.observers.write().expect("failed to acquire write lock in register");
        let next_observer = ObserverRegistration {
            weak: Arc::downgrade(observer) as WeakObserver<E>,
            ptr: Arc::as_ptr(observer) as usize,
        };
        observers.insert(next_observer.clone());
        next_observer
    }

    pub fn deregister(&self, registration: &ObserverRegistration<E>) -> bool {
        self.observers.write().expect("failed to acquire write lock in deregister").remove(registration)
    }
}

pub trait Observable<E> {
    fn observers(&self) -> &Observers<E>;

    fn notify_all(&self, event: &E)
    {
        for registration in self.observers().observers.read().expect("failed to acquire read lock").iter() {
            if let Some(observer) = registration.weak.upgrade() {
                observer.notify(event);
            } else {
                self.observers().deregister(registration);
            }
        }
    }
}



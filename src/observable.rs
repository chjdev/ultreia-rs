use crossbeam::channel::{bounded, Receiver, Sender};
use rayon::prelude::*;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::sync::{Arc, RwLock, Weak};
use std::thread;
use std::thread::JoinHandle;

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

pub struct Observers<E> {
    tx: Sender<E>,
    observers: Arc<RwLock<ObserversStore<E>>>,
}

impl<E: 'static + Send + Sync> Default for Observers<E> {
    fn default() -> Self {
        Self::new()
    }
}

impl<E: 'static + Send + Sync> Observers<E> {
    pub fn new() -> Self {
        Self::with_capacity(100)
    }

    pub fn with_capacity(capacity: usize) -> Self {
        let (tx, rx) = bounded(capacity);
        let observer = Observers {
            tx,
            observers: Default::default(),
        };
        observer.consume_event(rx);
        observer
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

    fn queue_event(&self, event: E) {
        self.tx.send(event).unwrap();
    }

    fn consume_event(&self, rx: Receiver<E>) {
        let weak_observers = Arc::downgrade(&self.observers);
        thread::spawn(move || loop {
            let maybe_event = rx.recv();
            if maybe_event.is_err() {
                // we have become disconnected
                return;
            }
            let event = maybe_event.unwrap();
            let maybe_observers = weak_observers.upgrade();
            if maybe_observers.is_none() {
                // doesn't really happen since we'd be disconnected anyway
                return;
            }
            let observers = maybe_observers.unwrap();

            observers
                .read()
                .unwrap()
                .par_iter()
                .for_each(|registration| {
                    if let Some(observer) = registration.weak.upgrade() {
                        observer.notify(&event);
                    } else {
                        // the observer has since freed
                        // todo move into own function
                        observers.write().unwrap().remove(registration);
                    }
                });
        });
    }
}

pub trait Observable<E: 'static + Send + Sync> {
    fn observers(&self) -> &Observers<E>;

    fn notify_all(&self, event: E) {
        // self.observers()
        //     .observers
        //     .read()
        //     .unwrap()
        //     .par_iter()
        //     .for_each(|registration| {
        //         if let Some(observer) = registration.weak.upgrade() {
        //             observer.notify(&event);
        //         } else {
        //             // the observer has since freed
        //             self.observers().deregister(registration);
        //         }
        //     });
        self.observers().queue_event(event);
    }
}

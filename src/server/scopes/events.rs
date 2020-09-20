use std::pin::Pin;
use std::sync::{Mutex, Weak, Arc};
use std::task::{Context, Poll};
use std::time::Duration;

use actix_web::rt::time::{interval_at, Instant};
use actix_web::web::{Bytes, Data, ServiceConfig};
use actix_web::{web, Error, HttpResponse, Responder};
use futures::{Stream, StreamExt};
use tokio::sync::mpsc::{channel, Receiver, Sender};
use crate::observable::Observer;
use crate::clock::{Tick, Tock};
use crate::game::Game;
use serde::Serialize;
use serde_json::to_string;

pub fn config(game: &Game) -> impl FnOnce(&mut ServiceConfig) {
    let broadcaster = Broadcaster::create(game);
    move |cfg: &mut web::ServiceConfig| {
        cfg.service(
            web::scope("/events")
                .app_data(broadcaster.clone())
                .route("", web::get().to(new_client))
        );
    }
}

async fn new_client(wrapped_broadcaster: Data<Mutex<Broadcaster>>) -> impl Responder {
    let mut broadcaster = wrapped_broadcaster.lock().unwrap();
    let rx = broadcaster.new_client();

    HttpResponse::Ok()
        .header("content-type", "text/event-stream")
        .streaming(rx)
}

struct EventBridge {
    broadcaster: Weak<Mutex<Broadcaster>>
}

#[derive(Serialize)]
struct EventWrapperEmpty<'a> {
    event: &'a str,
}

#[derive(Serialize)]
struct EventWrapper<'a, T: Serialize> {
    event: &'a str,
    data: &'a T,
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

impl EventBridge {
    fn notify<T: Serialize>(&self, data: &T) {
        if let Some(broadcaster) = self.broadcaster.upgrade() {
            broadcaster.lock().unwrap().send_json(&EventWrapper { event: std::any::type_name::<T>(), data });
        }
    }

    fn notify_empty<T>(&self) {
        if let Some(broadcaster) = self.broadcaster.upgrade() {
            broadcaster.lock().unwrap().send_json(&EventWrapperEmpty { event: std::any::type_name::<T>() });
        }
    }
}

impl Observer<Tick> for EventBridge {
    fn notify(&self, data: &Tick) {
        self.notify(data);
    }
}

impl Observer<Tock> for EventBridge {
    fn notify(&self, data: &Tock) {
        self.notify(data);
    }
}

struct Broadcaster {
    clients: Vec<Sender<Bytes>>,
    event_bridge: Option<Arc<EventBridge>>,
}

impl Broadcaster {
    fn create(game: &Game) -> Data<Mutex<Self>> {
        // Data ≃ Arc
        let me = Data::new(Mutex::new(Broadcaster::new()));

        let weak_broadcaster: Weak<Mutex<Broadcaster>> = Arc::downgrade(&me);
        let event_bridge = Arc::new(EventBridge { broadcaster: weak_broadcaster });
        game.clock().tickers().register(&event_bridge);
        me.lock().unwrap().connect_bridge(event_bridge);

        // ping clients every 10 seconds to see if they are alive
        Broadcaster::spawn_ping(me.clone());

        me
    }

    fn new() -> Self {
        Broadcaster {
            clients: Vec::new(),
            event_bridge: None,
        }
    }

    fn connect_bridge(&mut self, event_bridge: Arc<EventBridge>) {
        self.event_bridge = Some(event_bridge);
    }

    fn spawn_ping(me: Data<Mutex<Self>>) {
        actix_web::rt::spawn(async move {
            let mut task = interval_at(Instant::now(), Duration::from_secs(10));
            while task.next().await.is_some() {
                me.lock().unwrap().remove_stale_clients();
            }
        })
    }

    fn remove_stale_clients(&mut self) {
        let mut ok_clients = Vec::new();
        for client in self.clients.iter() {
            let result = client.clone().try_send(Bytes::from("data: ping\n\n"));

            if let Ok(()) = result {
                ok_clients.push(client.clone());
            }
        }
        self.clients = ok_clients;
    }

    fn new_client(&mut self) -> Client {
        let (tx, rx) = channel(100);

        tx.clone()
            .try_send(Bytes::from("data: connected\n\n"))
            .unwrap();

        self.clients.push(tx);
        Client(rx)
    }

    fn send_json<T: Serialize>(&self, obj: &T) {
        self.send(to_string(obj).unwrap().as_str());
    }

    fn send(&self, msg: &str) {
        let msg = Bytes::from(["data: ", msg, "\n\n"].concat());

        for client in self.clients.iter() {
            client.clone().try_send(msg.clone()).unwrap_or(());
        }
    }
}

// wrap Receiver in own type, with correct error type
struct Client(Receiver<Bytes>);

impl Stream for Client {
    type Item = Result<Bytes, Error>;

    fn poll_next(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        match Pin::new(&mut self.0).poll_next(cx) {
            Poll::Ready(Some(v)) => Poll::Ready(Some(Ok(v))),
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}

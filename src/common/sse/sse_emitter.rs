use std::collections::HashMap;
use std::convert::Infallible;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use axum::response::Sse;
use axum::response::sse::{Event, KeepAlive};
use futures::{SinkExt, Stream};
use tokio::spawn;
use tokio::sync::mpsc;
use tokio::sync::mpsc::Sender;
use tokio::time::interval;
use tokio_stream::StreamExt;
use tokio_stream::wrappers::ReceiverStream;

use crate::common::api_response::ApiResponse;

#[derive(Debug)]
pub struct SseBroadcaster {
    inner: Mutex<SseBroadcasterInner>,
}

#[derive(Debug, Clone, Default)]
pub struct SseBroadcasterInner {
    /**
    *   each user subscribe to channel<br>
    *   each user device subscribe to child of channel <br>
    *   example:<br>
    *   [<br>
    *        "+6281235623":[<br>
    *           "device1:"event",<br>
    *            "device2:"event"<br>
    *        ]<br>
    *    ]
    */
    clients: HashMap<String, HashMap<String, Sender<Event>>>,
}

impl SseBroadcaster {
    pub fn create() -> Arc<Self> {
        let this = Arc::new(SseBroadcaster {
            inner: Mutex::new(SseBroadcasterInner::default())
        });
        SseBroadcaster::spawn_ping(Arc::clone(&this));
        this
    }

    /// Pings clients every 10 seconds to see if they are alive and remove them from the broadcast
    /// list if not.'
    fn spawn_ping(this: Arc<Self>) {
        spawn(async move {
            let mut interval = interval(Duration::from_secs(10));
            loop {
                interval.tick().await;
                this.remove_stale_client().await;
            }
        });
    }

    /// Removes all non-responsive clients from broadcast list.
    async fn remove_stale_client(&self) {
        let clients = self
            .inner
            .lock()
            .unwrap()
            .clients
            .clone();
        let mut ok_client = HashMap::new();

        for (key, channel) in clients {
            let mut ok_subs: HashMap<String, Sender<Event>> = HashMap::new();
            for (user_id, client) in channel {
                if client.send(Event::default().comment("ping")).await.is_ok() {
                    ok_subs.insert(user_id, client);
                }
            }
            ok_client.insert(key, ok_subs.clone());
        }
        self.inner.lock().unwrap().clients = ok_client;
    }

    pub async fn new_client(&self, channel_id: String, key: String) -> Sse<impl Stream<Item=Result<Event, Infallible>>> {
        let (tx, rx) = mpsc::channel(10);
        let event = Event::default()
            .event("connected")
            .json_data(
                ApiResponse::ok("Tes".to_string(), "sasa")
            )
            .unwrap();

        tx.send(event).await.unwrap();
        let stream = ReceiverStream::<Event>::new(rx).map(|res| Ok(res));


        let mut subs = HashMap::new();

        match self.inner.lock().unwrap().clients.get(&channel_id) {
            None => {
                subs = HashMap::new();
            }
            Some(client) => {
                subs = client.clone();
            }
        }
        subs.insert(key.clone(), tx.clone());
        self.inner.lock().unwrap().clients.insert(channel_id.clone(), subs);

        let sse = Sse::new(stream).keep_alive(KeepAlive::default());
        sse
    }

    pub async fn reject_client(&self) -> Result<String, String> {
        let (tx, rx) = mpsc::channel(10);

        let event = Event::default()
            .event("connection")
            .json_data(
                ApiResponse::ok("Rejected".to_string(), "Failed to subscribe")
            )
            .unwrap();

        let _ = tx.send(event).await.unwrap();
        tx.closed().await;

        Ok("".to_string())
    }

    pub async fn broadcast_all<T: serde::Serialize>(
        &self,
        event_name: &str,
        data: &ApiResponse<T>,
    ) {
        let clients = self.inner.lock().unwrap().clients.clone();
        let event = Event::default()
            .event(event_name)
            .json_data(data)
            .unwrap();

        if !clients.is_empty() {
            for (_, channel) in clients {
                for (_, client) in channel {
                    let _ = client
                        .send(event.clone())
                        .await;
                }
            }
        }
    }

    pub async fn send_to_channel<T: serde::Serialize>(
        &self,
        channel_id: String,
        event_name: String,
        data: &ApiResponse<T>,
    ) {
        let clients = self.inner.lock().unwrap().clients.clone();
        let search = clients.contains_key(&channel_id);
        if search {
            match clients.get(&channel_id) {
                None => {}
                Some(client) => {
                    let event = Event::default()
                        .event(event_name)
                        .json_data(data)
                        .unwrap();

                    for (_, user) in client {
                        let _ = user.send(event.clone()).await;
                    }
                }
            }
        }
    }

    pub async fn send_to_user<T: serde::Serialize>(
        &self,
        channel_id: String,
        user_id: String,
        event_name: String,
        data: &ApiResponse<T>,
    ) {
        let clients = self.inner.lock().unwrap().clients.clone();
        let search = clients.get(&channel_id);
        if search.is_some() {
            let event = Event::default()
                .event(event_name)
                .json_data(data)
                .unwrap();

            let client = search.unwrap().get(&user_id);
            if client.is_some() {
                let _ = client
                    .unwrap()
                    .send(event.clone())
                    .await;
            }
        }
    }
}
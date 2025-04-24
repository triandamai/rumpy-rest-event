use std::collections::HashMap;
use std::convert::Infallible;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use axum::response::Sse;
use axum::response::sse::{Event, KeepAlive};
use futures::Stream;
use tokio::spawn;
use tokio::sync::mpsc;
use tokio::sync::mpsc::Sender;
use tokio::time::interval;
use tokio_stream::StreamExt;
use tokio_stream::wrappers::ReceiverStream;

use crate::common::api_response::ApiResponse;
use crate::common::sse::sse_builder::SseBuilder;

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
            inner: Mutex::new(SseBroadcasterInner::default()),
        });
        SseBroadcaster::spawn_ping(Arc::clone(&this));
        this
    }

    /// Pings clients every 10 seconds to see if they are alive and remove them from the broadcast
    /// list if not.'
    fn spawn_ping(this: Arc<Self>) {
        spawn(async move {
            let mut interval = interval(Duration::from_secs(15));
            loop {
                interval.tick().await;
                this.remove_stale_client().await;
            }
        });
    }

    /// Removes ALL non-responsive clients from broadcast list.
    async fn remove_stale_client(&self) {
        let clients = self.inner.lock().unwrap().clients.clone();
        let mut ok_client = HashMap::new();

        for (key, users) in clients {
            let mut ok_subs: HashMap<String, Sender<Event>> = HashMap::new();
            for (device, client) in users {
                if client
                    .send(Event::default().event(":ping").comment("ping"))
                    .await
                    .is_ok()
                {
                    ok_subs.insert(device, client);
                }
            }
            ok_client.insert(key, ok_subs.clone());
        }
        self.inner.lock().unwrap().clients = ok_client;
    }

    pub async fn new_client<'a>(
        &self,
        user_id: String,
        device_id: String,
    ) -> Sse<impl Stream<Item = Result<Event, Infallible>> + use<'a>> {
        let (tx, rx) = mpsc::channel(10);
        let event = Event::default()
            .event("connected")
            .json_data(ApiResponse::ok("CONNECTED".to_string(), "Success"))
            .unwrap();

        tx.send(event).await.unwrap();
        let stream = ReceiverStream::<Event>::new(rx).map(|res| Ok(res));

        let mut subs = match self.inner.lock().unwrap().clients.get(&user_id) {
            None => HashMap::new(),
            Some(client) => client.clone(),
        };

        subs.insert(device_id.clone(), tx.clone());
        self.inner
            .lock()
            .unwrap()
            .clients
            .insert(user_id.clone(), subs);

        Sse::new(stream).keep_alive(KeepAlive::default())
    }

    pub async fn reject_client(&self) -> Result<String, String> {
        let (tx, _) = mpsc::channel(10);

        let event = Event::default()
            .event("connection")
            .json_data(ApiResponse::ok(
                "Rejected".to_string(),
                "Failed to subscribe",
            ))
            .unwrap();

        let _ = tx.send(event).await.unwrap();
        tx.closed().await;

        Ok("Ok".to_string())
    }

    pub async fn send<T: serde::Serialize>(&self, builder: SseBuilder<T>) {
        let target = builder.get_target();
        if target.is_broadcast() {
            self.broadcast(&target.even_name(), &builder.data).await;
        } else {
            if target.is_to_device() {
                self.send_to_user_device(
                    target.user_id(),
                    target.device_id(),
                    &target.even_name(),
                    &builder.data,
                )
                .await;
            } else {
                self.send_to_user(target.user_id(), &target.even_name(), &builder.data)
                    .await;
            }
        }
    }

    async fn broadcast<T: serde::Serialize>(&self, event_name: &String, data: &T) {
        let clients = self.inner.lock().unwrap().clients.clone();
        let event = Event::default().event(event_name).json_data(data).unwrap();

        if !clients.is_empty() {
            for (_, users) in clients {
                for (_, client) in users {
                    let _ = client.send(event.clone()).await;
                }
            }
        }
    }

    async fn send_to_user<T: serde::Serialize>(
        &self,
        user_id: &String,
        event_name: &String,
        data: &T,
    ) {
        let clients = self.inner.lock().unwrap().clients.clone();
        let search = clients.contains_key(user_id);
        if search {
            match clients.get(user_id) {
                None => {}
                Some(users) => {
                    let event = Event::default().event(event_name).json_data(data).unwrap();

                    for (_, client) in users {
                        let _ = client.send(event.clone()).await;
                    }
                }
            }
        }
    }

    async fn send_to_user_device<T: serde::Serialize>(
        &self,
        user_id: &String,
        device_id: &String,
        event_name: &String,
        data: &T,
    ) {
        let clients = self.inner.lock().unwrap().clients.clone();
        let search_users = clients.get(user_id);
        if search_users.is_some() {
            let event = Event::default().event(event_name).json_data(data).unwrap();

            let client = search_users.unwrap().get(device_id);
            if client.is_some() {
                let _ = client.unwrap().send(event.clone()).await;
            }
        }
    }

    pub async fn get_list_client(&self) -> Option<HashMap<String, Vec<String>>> {
        let clients = self.inner.lock().unwrap().clients.clone();

        let mut data: HashMap<String, Vec<String>> = HashMap::new();

        let _ = clients.iter().for_each(|(key, sub)| {
            let items = sub.iter().map(|(key, _)| key.clone()).collect();
            data.insert(key.clone(), items);
        });

        Some(data)
    }
}

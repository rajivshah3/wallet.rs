// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_client::{
    node_api::participation::types::{
        ParticipationEvent, ParticipationEventId, ParticipationEventStatus, ParticipationEventType,
    },
    node_manager::node::Node,
    Client,
};

use crate::account_manager::AccountManager;

impl AccountManager {
    /// Stores participation information locally and returns the event.
    ///
    /// This will NOT store the node url and auth inside the client options.
    pub async fn register_participation_event(
        &self,
        id: ParticipationEventId,
        nodes: Vec<Node>,
    ) -> crate::Result<ParticipationEvent> {
        let mut client_builder = Client::builder().with_ignore_node_health();
        for node in &nodes {
            client_builder = client_builder.with_node_auth(node.url.as_str(), node.auth.clone())?;
        }
        let client = client_builder.finish()?;

        let event_data = client.event(&id).await?;

        let event = ParticipationEvent { id, data: event_data };

        self.storage_manager
            .lock()
            .await
            .insert_participation_event(id, event.clone(), nodes)
            .await?;

        Ok(event)
    }

    /// Removes a previously registered participation event from local storage.
    pub async fn deregister_participation_event(&self, id: &ParticipationEventId) -> crate::Result<()> {
        self.storage_manager.lock().await.remove_participation_event(id).await?;
        Ok(())
    }

    /// Retrieves corresponding information for a participation event from local storage.
    pub async fn get_participation_event(
        &self,
        id: ParticipationEventId,
    ) -> crate::Result<Option<(ParticipationEvent, Vec<Node>)>> {
        Ok(self
            .storage_manager
            .lock()
            .await
            .get_participation_events()
            .await?
            .get(&id)
            .cloned())
    }

    /// Retrieves information for all registered participation events.
    pub async fn get_participation_events(&self) -> crate::Result<Vec<ParticipationEvent>> {
        Ok(self
            .storage_manager
            .lock()
            .await
            .get_participation_events()
            .await?
            .values()
            .map(|(event, _nodes)| event.clone())
            .collect())
    }

    /// Retrieves IDs of all events tracked by the client options node.
    pub async fn get_participation_event_ids(
        &self,
        event_type: Option<ParticipationEventType>,
    ) -> crate::Result<Vec<ParticipationEventId>> {
        let accounts = self.accounts.read().await;
        let events = match accounts.first() {
            Some(account) => account.client.events(event_type).await?,
            None => {
                self.client_options
                    .read()
                    .await
                    .clone()
                    .finish()?
                    .events(event_type)
                    .await?
            }
        };
        Ok(events.event_ids)
    }

    /// Retrieves the latest status of a given participation event.
    pub async fn get_participation_event_status(
        &self,
        id: &ParticipationEventId,
    ) -> crate::Result<ParticipationEventStatus> {
        let events = self.storage_manager.lock().await.get_participation_events().await?;

        let event = events
            .get(id)
            .ok_or_else(|| crate::Error::Storage(format!("event {id} not found")))?;

        let mut client_builder = Client::builder().with_ignore_node_health();
        for node in &event.1 {
            client_builder = client_builder.with_node_auth(node.url.as_str(), node.auth.clone())?;
        }
        let client = client_builder.finish()?;

        let events_status = client.event_status(id, None).await?;

        Ok(events_status)
    }
}

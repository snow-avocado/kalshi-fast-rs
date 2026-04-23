use crate::ws::types::{WsMessageV2, WsSubscriptionParamsV2, WsUpdateSubscriptionParamsV2};

use std::collections::HashMap;

#[derive(Default)]
pub(crate) struct SubscriptionTracker {
    pending: HashMap<u64, WsSubscriptionParamsV2>,
    active: HashMap<u64, WsSubscriptionParamsV2>,
}

impl SubscriptionTracker {
    pub(crate) fn record_subscribe_cmd(&mut self, id: u64, params: WsSubscriptionParamsV2) {
        self.pending.insert(id, params);
    }

    pub(crate) fn handle_message(&mut self, msg: &WsMessageV2) {
        match msg {
            WsMessageV2::Subscribed {
                id: Some(id),
                sid: Some(sid),
            } => {
                self.handle_subscribed(Some(*id), Some(*sid));
            }
            WsMessageV2::Unsubscribed { sid: Some(sid), .. } => {
                self.handle_unsubscribed(Some(*sid));
            }
            _ => {}
        }
    }

    pub(crate) fn handle_subscribed(&mut self, id: Option<u64>, sid: Option<u64>) {
        let (id, sid) = match (id, sid) {
            (Some(id), Some(sid)) => (id, sid),
            _ => return,
        };
        if let Some(params) = self.pending.remove(&id) {
            self.active.insert(sid, params);
        }
    }

    pub(crate) fn handle_unsubscribed(&mut self, sid: Option<u64>) {
        if let Some(sid) = sid {
            self.active.remove(&sid);
        }
    }

    pub(crate) fn drop_active(&mut self, sid: u64) {
        self.active.remove(&sid);
    }

    pub(crate) fn apply_update(&mut self, update: &WsUpdateSubscriptionParamsV2) {
        use crate::ws::types::WsUpdateAction;

        let sid = match update.target_sid() {
            Some(sid) => sid,
            None => return,
        };

        let Some(params) = self.active.get_mut(&sid) else {
            return;
        };

        let mut incoming_tickers = update.market_tickers.clone().unwrap_or_default();
        if let Some(single) = update.market_ticker.clone() {
            incoming_tickers.push(single);
        }

        let mut incoming_ids = update.market_ids.clone().unwrap_or_default();
        if let Some(single) = update.market_id.clone() {
            incoming_ids.push(single);
        }

        let apply_vec =
            |target: &mut Option<Vec<String>>, incoming: Vec<String>, action: WsUpdateAction| {
                if incoming.is_empty() {
                    return;
                }
                let values = target.get_or_insert_with(Vec::new);
                match action {
                    WsUpdateAction::AddMarkets => {
                        for value in incoming {
                            if !values.iter().any(|v| v == &value) {
                                values.push(value);
                            }
                        }
                    }
                    WsUpdateAction::DeleteMarkets => {
                        values.retain(|current| !incoming.iter().any(|value| value == current));
                        if values.is_empty() {
                            *target = None;
                        }
                    }
                }
            };

        apply_vec(&mut params.market_tickers, incoming_tickers, update.action);
        apply_vec(&mut params.market_ids, incoming_ids, update.action);

        if let Some(value) = update.send_initial_snapshot {
            params.send_initial_snapshot = Some(value);
        }
        if let Some(value) = update.skip_ticker_ack {
            params.skip_ticker_ack = Some(value);
        }
    }

    pub(crate) fn prepare_resubscribe(&mut self) -> Vec<WsSubscriptionParamsV2> {
        let mut params: Vec<WsSubscriptionParamsV2> = self.active.values().cloned().collect();
        params.extend(self.pending.values().cloned());
        self.active.clear();
        self.pending.clear();
        params
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ws::types::{WsChannelV2, WsUpdateAction};

    #[test]
    fn subscription_tracker_moves_pending_to_active() {
        let mut tracker = SubscriptionTracker::default();
        let params = WsSubscriptionParamsV2 {
            channels: vec![WsChannelV2::Ticker],
            ..Default::default()
        };
        tracker.record_subscribe_cmd(1, params.clone());
        tracker.handle_message(&WsMessageV2::Subscribed {
            id: Some(1),
            sid: Some(42),
        });

        assert!(tracker.pending.is_empty());
        assert_eq!(tracker.active.len(), 1);
        assert_eq!(tracker.active.get(&42), Some(&params));
    }

    #[test]
    fn subscription_tracker_prepare_resubscribe_clears_state() {
        let mut tracker = SubscriptionTracker::default();
        let params = WsSubscriptionParamsV2 {
            channels: vec![WsChannelV2::Ticker],
            ..Default::default()
        };
        tracker.record_subscribe_cmd(1, params.clone());
        tracker.handle_message(&WsMessageV2::Subscribed {
            id: Some(1),
            sid: Some(42),
        });

        let params = tracker.prepare_resubscribe();
        assert_eq!(params.len(), 1);
        assert!(tracker.pending.is_empty());
        assert!(tracker.active.is_empty());
    }

    #[test]
    fn subscription_tracker_apply_update_changes_fields() {
        let mut tracker = SubscriptionTracker::default();
        let params = WsSubscriptionParamsV2 {
            channels: vec![WsChannelV2::OrderbookDelta],
            market_tickers: Some(vec!["A".to_string()]),
            ..Default::default()
        };
        tracker.active.insert(10, params);

        let update = WsUpdateSubscriptionParamsV2 {
            action: WsUpdateAction::AddMarkets,
            sid: Some(10),
            sids: None,
            market_ticker: None,
            market_tickers: Some(vec!["B".to_string()]),
            market_id: None,
            market_ids: None,
            send_initial_snapshot: Some(true),
            skip_ticker_ack: Some(true),
        };
        tracker.apply_update(&update);

        let updated = tracker.active.get(&10).unwrap();
        assert!(
            updated
                .market_tickers
                .as_ref()
                .unwrap()
                .contains(&"A".to_string())
        );
        assert!(
            updated
                .market_tickers
                .as_ref()
                .unwrap()
                .contains(&"B".to_string())
        );
        assert_eq!(updated.send_initial_snapshot, Some(true));
        assert_eq!(updated.skip_ticker_ack, Some(true));
    }
}

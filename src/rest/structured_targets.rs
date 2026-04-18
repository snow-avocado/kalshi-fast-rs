//! Structured targets and search-filter endpoints.
//!
//! Structured targets are references to real-world entities (teams, players,
//! competitions) used as settlement anchors in multivariate or sports events.
//! Also includes the two "search" helpers: tags-by-categories and
//! filters-by-sport.

use crate::KalshiError;
use crate::rest::client::KalshiRestClient;
use crate::rest::pagination::{CursorPager, stream_items};
use crate::types::deserialize_null_as_empty_vec;
use futures::stream::Stream;
use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Debug, Clone, Default, Serialize)]
pub struct GetStructuredTargetsParams {
    pub ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    pub target_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub competition: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetStructuredTargetsResponse {
    #[serde(default, deserialize_with = "deserialize_null_as_empty_vec")]
    pub structured_targets: Vec<StructuredTarget>,
    #[serde(default)]
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetStructuredTargetResponse {
    pub structured_target: StructuredTarget,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StructuredTarget {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default, rename = "type")]
    pub target_type: Option<String>,
    #[serde(default)]
    pub details: Option<Map<String, Value>>,
    #[serde(default)]
    pub source_id: Option<String>,
    #[serde(default)]
    pub source_ids: Option<Map<String, Value>>,
    #[serde(default)]
    pub last_updated_ts: Option<String>,
    #[serde(default, flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetTagsForSeriesCategoriesResponse {
    #[serde(default)]
    pub tags_by_categories: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetFiltersBySportsResponse {
    #[serde(default)]
    pub filters_by_sports: Map<String, Value>,
    #[serde(default, deserialize_with = "deserialize_null_as_empty_vec")]
    pub sport_ordering: Vec<String>,
}

impl KalshiRestClient {
    pub async fn get_structured_targets(
        &self,
        params: GetStructuredTargetsParams,
    ) -> Result<GetStructuredTargetsResponse, KalshiError> {
        let path = Self::full_path("/structured_targets");
        let query = Self::structured_targets_query(&params);
        self.send(Method::GET, &path, Some(&query), Option::<&()>::None, false)
            .await
    }

    pub async fn get_structured_target(
        &self,
        structured_target_id: &str,
    ) -> Result<GetStructuredTargetResponse, KalshiError> {
        let path = Self::full_path(&format!("/structured_targets/{structured_target_id}"));
        self.send(
            Method::GET,
            &path,
            Option::<&()>::None,
            Option::<&()>::None,
            false,
        )
        .await
    }

    pub async fn get_tags_by_categories(
        &self,
    ) -> Result<GetTagsForSeriesCategoriesResponse, KalshiError> {
        let path = Self::full_path("/search/tags_by_categories");
        self.send(
            Method::GET,
            &path,
            Option::<&()>::None,
            Option::<&()>::None,
            false,
        )
        .await
    }

    pub async fn get_filters_by_sport(&self) -> Result<GetFiltersBySportsResponse, KalshiError> {
        let path = Self::full_path("/search/filters_by_sport");
        self.send(
            Method::GET,
            &path,
            Option::<&()>::None,
            Option::<&()>::None,
            false,
        )
        .await
    }

    /// Page through structured targets.
    pub fn structured_targets_pager(
        &self,
        params: GetStructuredTargetsParams,
    ) -> CursorPager<StructuredTarget> {
        let client = self.clone();
        let base_params = params.clone();
        CursorPager::new(params.cursor.clone(), move |cursor| {
            let client = client.clone();
            let mut page_params = base_params.clone();
            page_params.cursor = cursor;
            Box::pin(async move {
                let resp = client.get_structured_targets(page_params).await?;
                Ok((resp.structured_targets, resp.cursor))
            })
        })
    }

    /// Stream structured targets one by one.
    pub fn stream_structured_targets(
        &self,
        params: GetStructuredTargetsParams,
        max_items: Option<usize>,
    ) -> impl Stream<Item = Result<StructuredTarget, KalshiError>> + Send {
        stream_items(self.structured_targets_pager(params), max_items)
    }

    /// Fetch all pages for structured targets using cursor pagination.
    pub async fn get_structured_targets_all(
        &self,
        params: GetStructuredTargetsParams,
    ) -> Result<Vec<StructuredTarget>, KalshiError> {
        self.paginate_cursor(params.cursor.clone(), |cursor| {
            let mut page_params = params.clone();
            page_params.cursor = cursor;
            async move {
                let resp = self.get_structured_targets(page_params).await?;
                Ok((resp.structured_targets, resp.cursor))
            }
        })
        .await
    }
}

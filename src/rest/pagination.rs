use crate::KalshiError;
use futures::future::BoxFuture;
use futures::stream::{self, Stream};
use std::collections::VecDeque;

/// Manual page-by-page cursor pagination.
///
/// Use `CursorPager` when you need:
/// - Explicit control over when to fetch the next page
/// - Access to page boundaries (e.g., for batch processing)
/// - Custom termination logic based on page contents
///
/// For item-by-item iteration, see the `stream_*` methods on [`crate::KalshiRestClient`].
///
/// # Example
/// ```no_run
/// # use kalshi_fast::{KalshiEnvironment, KalshiRestClient, GetMarketsParams};
/// # async fn example() -> Result<(), kalshi_fast::KalshiError> {
/// let client = KalshiRestClient::new(KalshiEnvironment::demo());
/// let mut pager = client.markets_pager(GetMarketsParams::default());
///
/// while let Some(markets) = pager.next_page().await? {
///     println!("Got {} markets", markets.len());
/// }
/// # Ok(())
/// # }
/// ```
pub struct CursorPager<T> {
    cursor: Option<String>,
    done: bool,
    fetch: Box<
        dyn FnMut(
                Option<String>,
            ) -> BoxFuture<'static, Result<(Vec<T>, Option<String>), KalshiError>>
            + Send,
    >,
}

impl<T> CursorPager<T> {
    pub fn new<F>(cursor: Option<String>, fetch: F) -> Self
    where
        F: FnMut(
                Option<String>,
            ) -> BoxFuture<'static, Result<(Vec<T>, Option<String>), KalshiError>>
            + Send
            + 'static,
    {
        Self {
            cursor: cursor.filter(|c| !c.is_empty()),
            done: false,
            fetch: Box::new(fetch),
        }
    }

    /// Fetch the next page of results.
    ///
    /// Returns `Ok(Some(items))` if there are more results, `Ok(None)` when
    /// pagination is complete, or `Err` on failure.
    pub async fn next_page(&mut self) -> Result<Option<Vec<T>>, KalshiError> {
        if self.done {
            return Ok(None);
        }

        let (items, next) = (self.fetch)(self.cursor.clone()).await?;
        self.cursor = next.filter(|c| !c.is_empty());
        if self.cursor.is_none() {
            self.done = true;
        }

        Ok(Some(items))
    }

    /// Returns the cursor for the next page fetch.
    ///
    /// Useful for checkpointing/resuming pagination across sessions.
    pub fn current_cursor(&self) -> Option<&str> {
        self.cursor.as_deref()
    }

    /// Returns true if pagination is complete.
    pub fn is_done(&self) -> bool {
        self.done
    }
}

struct StreamState<T> {
    pager: CursorPager<T>,
    buffer: VecDeque<T>,
    remaining: Option<usize>,
    done: bool,
}

/// Stream items one-by-one from paginated endpoints.
///
/// Streams provide lazy, item-level iteration built on [`CursorPager`].
/// Pages are fetched on-demand; use `max_items` for early termination.
///
/// # Pagers vs Streams
///
/// | Aspect | Pager | Stream |
/// |--------|-------|--------|
/// | Returns | Full pages (`Vec<T>`) | Individual items |
/// | Control | Manual `next_page()` | Async iterator |
/// | Early stop | Stop calling `next_page()` | `max_items` or `.take()` |
/// | Use case | Batch processing, checkpointing | Item processing, collecting subsets |
pub(crate) fn stream_items<T>(
    pager: CursorPager<T>,
    max_items: Option<usize>,
) -> impl Stream<Item = Result<T, KalshiError>> + Send
where
    T: Send + 'static,
{
    let state = StreamState {
        pager,
        buffer: VecDeque::new(),
        remaining: max_items,
        done: false,
    };

    stream::unfold(state, |mut state| async move {
        if state.done {
            return None;
        }
        if let Some(remaining) = state.remaining
            && remaining == 0
        {
            return None;
        }

        loop {
            if let Some(item) = state.buffer.pop_front() {
                if let Some(remaining) = state.remaining.as_mut() {
                    *remaining -= 1;
                }
                return Some((Ok(item), state));
            }

            match state.pager.next_page().await {
                Ok(Some(items)) => {
                    state.buffer = items.into();
                    if state.buffer.is_empty() && state.pager.done {
                        return None;
                    }
                }
                Ok(None) => {
                    return None;
                }
                Err(err) => {
                    state.done = true;
                    return Some((Err(err), state));
                }
            }
        }
    })
}

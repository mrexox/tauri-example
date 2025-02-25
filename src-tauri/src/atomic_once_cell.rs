use std::fmt::Debug;
use std::sync::Arc;
use tokio::sync::{Notify, OnceCell, SetError};

/// A simple wrapper around OnceCell that allows for async initialization.
///
/// # Examples
///
/// ```rust
/// use tauri_example::atomic_once_cell::AtomicOnceCell;
///
/// #[tokio::main]
/// async fn main() {
///     let cell = AtomicOnceCell::<i32>::new();
///     let cell_clone = cell.clone();
///
///     tauri::async_runtime::spawn(async move {
///         let value = cell.init(42).expect("failed to set value");
///     });
///
///     assert_eq!(42, *cell_clone.get().await);
/// }
/// ```
pub struct AtomicOnceCell<T>(Arc<AtomicOnceCellInner<T>>);

struct AtomicOnceCellInner<T> {
    value: OnceCell<T>,
    notify: Notify,
}

impl<T> AtomicOnceCell<T>
where
    T: Debug,
{
    pub fn new() -> Self {
        Self(Arc::new(AtomicOnceCellInner {
            value: OnceCell::new(),
            notify: Notify::new(),
        }))
    }

    pub fn init(&self, value: T) -> Result<(), SetError<T>> {
        if self.0.value.get().is_some() {
            return Err(SetError::AlreadyInitializedError(value));
        }

        self.0.value.set(value)?;
        self.0.notify.notify_waiters();

        Ok(())
    }

    pub async fn get(&self) -> &T {
        if let Some(value) = self.0.value.get() {
            return value;
        }

        self.0.notify.notified().await;
        self.0.value.get().unwrap()
    }
}

impl<T> Clone for AtomicOnceCell<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_atomic_once_cell() {
        let cell = AtomicOnceCell::<i32>::new();
        let cell_clone = cell.clone();

        tauri::async_runtime::spawn(async move {
            cell.init(42).expect("failed to set value");
        });

        assert_eq!(42, *cell_clone.get().await);
    }

    #[tokio::test]
    async fn test_atomic_once_cell_multi_consumers() {
        let cell = AtomicOnceCell::<i32>::new();
        let cell1 = cell.clone();
        let cell2 = cell.clone();
        let cell3 = cell.clone();

        let h1 = tauri::async_runtime::spawn(async move {
            assert_eq!(42, *cell1.get().await);
        });

        let h2 = tauri::async_runtime::spawn(async move {
            assert_eq!(42, *cell2.get().await);
        });

        let h3 = tauri::async_runtime::spawn(async move {
            assert_eq!(42, *cell3.get().await);
        });

        let h4 = tauri::async_runtime::spawn(async move {
            cell.init(42).expect("failed to set value");
        });

        assert!(h1.await.is_ok());
        assert!(h2.await.is_ok());
        assert!(h3.await.is_ok());
        assert!(h4.await.is_ok());
    }
}

use crate::prelude::*;

pub struct JoinAll;

impl JoinAll {
    pub fn run<T>(
        &self,
        func: impl AsyncFn(&String) -> T,
        keys: &[String],
    ) -> impl Future<Output = MAP<String, T>> {
        async move {
            join_all(keys.iter().map(|v| async { (v.clone(), func(v).await) }))
                .await
                .into_iter()
                .collect()
        }
    }
}

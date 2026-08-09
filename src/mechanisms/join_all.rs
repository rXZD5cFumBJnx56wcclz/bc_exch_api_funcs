use crate::prelude::*;

pub struct JoinAll;

impl JoinAll {
    pub fn run<T, F: Future<Output = T>>(
        &self,
        funcs: impl IntoIterator<Item = (String, F)>,
    ) -> impl Future<Output = MAP<String, T>> {
        async move {
            join_all(funcs.into_iter().map(|(v, f)| async move { (v, f.await) }))
                .await
                .into_iter()
                .collect()
        }
    }
}

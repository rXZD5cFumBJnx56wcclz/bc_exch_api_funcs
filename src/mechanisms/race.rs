use crate::prelude::*;

#[derive(Default)]
pub struct Race<T> {
    pub futures: RefCell<FuturesUnordered<BoxFuture<'static, T>>>,
}

impl<T> Race<T> {
    pub fn run<Conn: Clone, F: Fn(Conn) -> BoxFuture<'static, T>>(
        &self,
        func: F,
        connections: &[Conn],
    ) -> impl Future<Output = T> {
        async move {
            if self.futures.borrow().is_empty() {
                self.futures
                    .borrow_mut()
                    .extend(connections.iter().map(|v| func(v.clone())));
            }
            self.futures.borrow_mut().next().await.unwrap()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::prelude_tests::prelude::*;

    #[tokio::test]
    async fn run_res_1() {
        let race = Race::default();
        let connections = [2, 1, 3]
            .into_iter()
            .enumerate()
            .collect::<Vec<(usize, u64)>>();
        assert_eq_pr!(
            race.run(
                |conn| {
                    Box::pin(async move {
                        tokio::time::sleep(Duration::from_secs(conn.1)).await;
                        conn
                    })
                },
                &connections,
            )
            .await,
            (1, 1),
        );
    }
}

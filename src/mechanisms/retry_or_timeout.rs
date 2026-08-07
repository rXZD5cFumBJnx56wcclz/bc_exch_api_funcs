use crate::prelude::*;

#[derive(Default)]
pub struct RetryOrTimeout {
    pub timeout: Duration,
}

impl RetryOrTimeout {
    pub fn run<T, E: Error>(
        &self,
        func: impl AsyncFn() -> Result<T, E>,
    ) -> impl Future<Output = Result<T, E>> {
        async move {
            loop {
                let inst = Instant::now();
                let req = func().await;
                if let Ok(res) = req {
                    return Ok(res);
                }
                if Instant::now().duration_since(inst) >= self.timeout {
                    return req;
                }
            }
        }
    }
}

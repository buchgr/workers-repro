use worker::*;


#[allow(dead_code)]
#[durable_object]
pub struct ActionCache {
    state: State,
    env: Env,
}

#[durable_object]
impl DurableObject for ActionCache {
    fn new(state: State, env: Env) -> Self {
        Self {
            state: state,
            env: env,
        }
    }

    async fn fetch(&mut self, _req: Request) -> Result<Response> {
        Response::ok("hello world")
    }
}

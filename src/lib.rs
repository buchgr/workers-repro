use worker::*;
pub use console_error_panic_hook::set_once as set_panic_hook;

pub mod cache;

#[event(fetch, respond_with_errors)]
async fn main(_req: Request, env: Env, _ctx: Context) -> Result<Response> {
    set_panic_hook();

    let namespace = env.durable_object("AC")?;
    let stub = namespace.id_from_name("A")?.get_stub()?;
    stub.fetch_with_str("/get_cached").await
}
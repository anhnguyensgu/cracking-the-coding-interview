use crate::auth;

#[derive(Clone)]
pub struct ApplicationContext {}

impl ApplicationContext {
    pub async fn login(&self) {
        auth::login::login().await;
    }
}

use crate::addon_transport::AddonTransport;
use crate::runtime::{EnvError, EnvFutureExt, TryEnvFuture};
use crate::types::addon::{Manifest, ResourcePath, ResourceResponse};
use futures::future;
use url::Url;

pub struct UnsupportedTransport {
    transport_url: Url,
}

impl UnsupportedTransport {
    pub fn new(transport_url: Url) -> Self {
        UnsupportedTransport { transport_url }
    }
    fn result<
        #[cfg(target_arch = "wasm32")] T: Sized + 'static,
        #[cfg(not(target_arch = "wasm32"))] T: Sized + Send + 'static,
    >(
        &self,
    ) -> TryEnvFuture<T> {
        future::err(EnvError::AddonTransport(format!(
            "Unsupported addon transport: {}",
            self.transport_url.scheme()
        )))
        .boxed_env()
    }
}

impl AddonTransport for UnsupportedTransport {
    fn resource(&self, _path: &ResourcePath) -> TryEnvFuture<ResourceResponse> {
        self.result::<ResourceResponse>()
    }
    fn manifest(&self) -> TryEnvFuture<Manifest> {
        self.result::<Manifest>()
    }
}

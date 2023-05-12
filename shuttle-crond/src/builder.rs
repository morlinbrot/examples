use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use shuttle_common::resource::Type;
use shuttle_runtime::{Factory, ResourceBuilder};

use crate::*;

#[derive(Serialize, Deserialize)]
pub struct CrondConfig {}

// Crond Builder
pub struct Crond {
    config: CrondConfig,
}

#[async_trait]
impl ResourceBuilder<CrondInstance> for Crond {
    /// The type of resource this creates
    // TODO: Need to add Type::Demon or something.
    const TYPE: Type = Type::Persist;

    /// The internal config being constructed by this builder. This will be used to find cached [Self::Output].
    type Config = CrondConfig;
    // type Config = ();

    /// The output type used to build this resource later
    type Output = CrondInstance;

    /// Create a new instance of this resource builder
    fn new() -> Self {
        Self {
            config: CrondConfig {},
        }
    }

    /// Get the internal config state of the builder
    ///
    /// If the exact same config was returned by a previous deployement that used this resource, then [Self::output()]
    /// will not be called to get the builder output again. Rather the output state of the previous deployment
    /// will be passed to [Self::build()].
    fn config(&self) -> &Self::Config {
        &self.config
    }

    /// Get the config output of this builder
    ///
    /// This method is where the actual resource provisioning should take place and is expected to take the longest. It
    /// can at times even take minutes. That is why the output of this method is cached and calling this method can be
    /// skipped as explained in [Self::config()].
    async fn output(
        self,
        _factory: &mut dyn Factory,
    ) -> Result<Self::Output, shuttle_service::Error> {
        Ok(CrondInstance {})
    }

    /// Build this resource from its config output
    async fn build(build_data: &Self::Output) -> Result<CrondInstance, shuttle_service::Error> {
        // Here, `build_data` == `CrondInstance`!
        Ok(build_data.clone())
    }
}

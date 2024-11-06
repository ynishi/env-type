use crate::context::{Context, ContextMarker};
use crate::types::{EnvError, EnvType};
use std::any::{Any, TypeId};
use std::collections::HashMap;

/// Environment type that holds contexts, and the current environment.
/// The current environment is the environment type.
/// The contexts are the context type.
/// The context type is a key-value pair of the environment type and the value.
/// The value is the value for the environment type.
pub struct Environment {
    current: EnvType,
    contexts: HashMap<TypeId, Box<dyn Any + Send + Sync>>,
}

/// Environment struct implementation
/// The Environment struct has the current environment and the contexts.
impl Environment {
    /// Get the current environment
    pub fn current_env(&self) -> &EnvType {
        &self.current
    }

    /// Get the context for the context marker
    pub fn context<M: ContextMarker>(&self) -> Option<&Context<M>> {
        self.contexts
            .get(&TypeId::of::<M>())
            .and_then(|ctx| ctx.downcast_ref())
    }

    /// Get the current value for the context marker
    pub fn current_value<M: ContextMarker>(&self) -> Option<M::Value> {
        self.value::<M>(self.current_env())
    }

    /// Get the value for the context marker and the environment type
    pub fn value<M: ContextMarker>(&self, env: &EnvType) -> Option<M::Value> {
        self.context::<M>().and_then(|ctx| ctx.get_for_env(env))
    }
}

/// Environment builder
/// The EnvironmentBuilder is used to create an environment with the current environment and contexts.
#[derive(Default)]
pub struct EnvironmentBuilder {
    current: Option<EnvType>,
    contexts: HashMap<TypeId, Box<dyn Any + Send + Sync>>,
}

/// EnvironmentBuilder implementation
/// Create a new EnvironmentBuilder with the current environment and contexts.
///
/// # Example
///
/// ```
/// use env_type::types::EnvType;
/// use std::str::FromStr;
/// use std::collections::HashMap;
///
/// use env_type::environment::EnvironmentBuilder;
/// use env_type::context::{ContextBuilder, ContextMarker};
///
/// struct TestContext;
///
/// impl ContextMarker for TestContext {
///   type Value = String;
/// }
///
/// struct TestConfig;
///
/// impl From<TestConfig> for EnvType {
///     fn from(_: TestConfig) -> Self {
///        EnvType::Dev
///    }
/// }
///
/// let context = ContextBuilder::<TestContext>::default()
///  .with_value(EnvType::Dev, "dev".to_string())
///  .with_value(EnvType::Test, "test".to_string())
///  .with_default("default".to_string())
///  .build();
///
/// let env = EnvironmentBuilder::default()
///  .current_from(TestConfig)
///  .with_context(context)
///  .build();
///
/// assert!(env.is_ok());
/// let env = env.unwrap();
/// assert_eq!(EnvType::Dev, *env.current_env());
/// assert_eq!(Some("dev".to_string()), env.current_value::<TestContext>());
/// ```
impl EnvironmentBuilder {
    pub fn current_env(mut self, env: EnvType) -> Self {
        self.current = Some(env);
        self
    }

    pub fn current_from<T>(mut self, config: T) -> Self
    where
        EnvType: From<T>,
    {
        self.current = Some(EnvType::from(config));
        self
    }

    pub fn with_context<M: ContextMarker>(mut self, context: Context<M>) -> Self {
        self.contexts.insert(TypeId::of::<M>(), Box::new(context));
        self
    }

    pub fn build(self) -> Result<Environment, EnvError> {
        let current = self.current.ok_or(EnvError::NoCurrentEnv)?;

        Ok(Environment {
            current,
            contexts: self.contexts,
        })
    }
}

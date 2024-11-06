use crate::types::{EnvError, EnvType};
use std::collections::HashMap;
use std::marker::PhantomData;

/// Context marker trait for type-safe context values
/// The Value type must be Clone, Send, Sync, and 'static strictly.
pub trait ContextMarker: Send + Sync + 'static {
    type Value: Clone + Send + Sync + 'static;
}

/// Context is Generic context container
/// The context is a key-value store for environment values.
#[derive(Clone)]
pub struct Context<M: ContextMarker> {
    /// Environment values and values for each environment
    env_values: HashMap<EnvType, M::Value>,
    /// Default value for the context, if no value is found for the environment
    default: Option<M::Value>,
    /// Marker for the context type
    _marker: PhantomData<M>,
}

/// Context implementation for ContextMarker
///
/// # Example
///
/// ```
/// use env_type::context::{ContextMarker, Context};
/// use env_type::types::EnvType;
///
/// struct TestContext;
///
/// impl ContextMarker for TestContext {
///   type Value = String;
/// }
///
/// let mut context = Context::<TestContext>::default();
/// assert!(context.get_for_env(&EnvType::Dev).is_none());
/// ```

impl<M: ContextMarker> Default for Context<M> {
    fn default() -> Self {
        Self {
            env_values: HashMap::new(),
            default: None,
            _marker: PhantomData,
        }
    }
}

impl<M: ContextMarker> Context<M> {
    /// Get the value for the current environment
    /// If no value is found, return the default value(optional)
    pub fn get_for_env(&self, env: &EnvType) -> Option<M::Value> {
        self.env_values
            .get(env)
            .cloned()
            .or_else(|| self.default.clone())
    }

    /// Try to get the value for the current environment
    /// If no value is found, return an error
    pub fn try_get_for_env(&self, env: &EnvType) -> Result<M::Value, EnvError> {
        self.get_for_env(env).ok_or(EnvError::ContextValueNotFound)
    }
}

/// Builder for type-safe context configuration
/// The builder is used to create a context with environment values and a default value.
///
/// # Example
///
/// ```
/// use env_type::context::{ContextBuilder, ContextMarker};
/// use env_type::types::{EnvType};
///
/// struct TestContext;
///
/// impl ContextMarker for TestContext {
///  type Value = String;
/// }
///
/// let context = ContextBuilder::<TestContext>::default()
///  .with_value(EnvType::Dev, "dev".to_string())
///  .with_value(EnvType::Test, "test".to_string())
///  .with_default("default".to_string())
///  .build();
///
/// assert_eq!(context.get_for_env(&EnvType::Dev), Some("dev".to_string()));
/// assert_eq!(context.get_for_env(&EnvType::Test), Some("test".to_string()));
/// assert_eq!(context.get_for_env(&EnvType::Stg), Some("default".to_string()));
/// ```
pub struct ContextBuilder<M: ContextMarker> {
    env_values: HashMap<EnvType, M::Value>,
    default: Option<M::Value>,
    _marker: PhantomData<M>,
}

impl<M: ContextMarker> Default for ContextBuilder<M> {
    fn default() -> Self {
        Self {
            env_values: HashMap::new(),
            default: None,
            _marker: PhantomData,
        }
    }
}

/// ContextBuilder implementation
/// Create a new ContextBuilder with the environment values and default value.
impl<M: ContextMarker> ContextBuilder<M> {
    pub fn with_value(mut self, env: EnvType, value: M::Value) -> Self {
        self.env_values.insert(env, value);
        self
    }

    pub fn with_values<I>(mut self, envs: I, value: M::Value) -> Self
    where
        I: IntoIterator<Item = EnvType>,
        M::Value: Clone,
    {
        for env in envs {
            self.env_values.insert(env, value.clone());
        }
        self
    }

    pub fn with_default(mut self, value: M::Value) -> Self {
        self.default = Some(value);
        self
    }

    pub fn build(self) -> Context<M> {
        Context {
            env_values: self.env_values,
            default: self.default,
            _marker: PhantomData,
        }
    }
}

/// This is used to determine if the current environment is a debug environment.
/// The IsDebug trait is used to check if the environment type is debug.
///
/// # Example
///
/// ```
/// use env_type::types::EnvType;
/// use env_type::is_debug::{IsDebug, IsDebugContext, debug_context};
/// use env_type::environment::EnvironmentBuilder;
///
/// let is_debug_context = debug_context().build();
///
/// let env = EnvironmentBuilder::default()
///  .current_env(EnvType::Dev)
///  .with_context(is_debug_context)
///  .build();
///
/// assert!(env.is_ok());
/// let env = env.unwrap();
/// assert_eq!(EnvType::Dev, *env.current_env());
/// assert_eq!(true, env.is_debug());
/// ```
use crate::context::{ContextBuilder, ContextMarker};
use crate::environment::Environment;
use crate::types::EnvType;

pub struct IsDebugContext;

impl ContextMarker for IsDebugContext {
    type Value = bool;
}

pub fn debug_context() -> ContextBuilder<IsDebugContext> {
    ContextBuilder::<IsDebugContext>::default()
        .with_value(EnvType::Dev, true)
        .with_default(false)
}

pub trait IsDebug {
    fn is_debug(&self) -> bool;
}

impl IsDebug for Environment {
    fn is_debug(&self) -> bool {
        self.current_value::<IsDebugContext>().unwrap_or(false)
    }
}

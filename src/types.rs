use std::str::FromStr;

/// EnvType is an enum that represents the environment type.
/// EnvType is derived from the strum crate, which provides the ability to convert the string to the enum.
///
/// # Example
///
/// ```
/// use env_type::types::EnvType;
/// use std::str::FromStr;
///
/// let env = EnvType::from_str("d").unwrap();
/// assert_eq!(EnvType::Dev, env);
/// ```
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    strum::EnumString,
    strum::EnumIs,
    strum::VariantArray,
    Default,
)]
#[strum(serialize_all = "PascalCase")]
pub enum EnvType {
    #[default]
    #[strum(
        serialize = "develop",
        serialize = "Develop",
        serialize = "dev",
        serialize = "Dev",
        serialize = "DEV",
        serialize = "d",
        serialize = "D"
    )]
    Dev,
    #[strum(
        serialize = "test",
        serialize = "Test",
        serialize = "TEST",
        serialize = "t",
        serialize = "T"
    )]
    Test,
    #[strum(
        serialize = "staging",
        serialize = "Staging",
        serialize = "stg",
        serialize = "Stg",
        serialize = "STG",
        serialize = "s",
        serialize = "S"
    )]
    Stg,
    #[strum(
        serialize = "production",
        serialize = "Production",
        serialize = "prod",
        serialize = "Prod",
        serialize = "PROD",
        serialize = "p",
        serialize = "P"
    )]
    Prod,
}

/// EnvKey is a trait that represents the environment key.
///
/// # Example
///
/// ```
/// use env_type::types::{EnvType, EnvKey};
/// use std::str::FromStr;
///
/// struct NewEnvKey;
///
/// impl EnvKey for NewEnvKey {
///    fn key() -> &'static str {
///       "NEW_ENV"
///   }
/// }
///
/// std::env::set_var("NEW_ENV", "Production");
/// let env = EnvType::from_env_key::<NewEnvKey>();
/// assert_eq!(EnvType::Prod, env);
/// ```
pub trait EnvKey {
    fn key() -> &'static str;
}

/// EnvType is an implementation of the EnvKey trait.
/// The default environment key is "ENV".
impl EnvKey for EnvType {
    fn key() -> &'static str {
        "ENV"
    }
}

impl EnvType {
    /// EnvType::from_env is a function that returns the environment type from the environment variable.
    /// This is deligated to from_env_key with EnvType as default from env key.
    /// The default environment type is Dev.
    ///
    /// # Example
    ///
    /// ```
    /// use env_type::types::EnvType;
    /// use std::str::FromStr;
    ///
    /// std::env::set_var("ENV", "Production");
    /// let env = EnvType::from_env();
    /// assert_eq!(EnvType::Prod, env);
    /// ```
    pub fn from_env() -> Self {
        Self::from_env_key::<Self>()
    }

    /// EnvType::from_env_key is a function that returns the environment type from the environment variable.
    /// The default environment type is Dev.
    ///     /// # Example
    ///
    /// ```
    /// use env_type::types::EnvType;
    /// use std::str::FromStr;
    ///
    /// std::env::set_var("ENV", "Test");
    /// let env = EnvType::from_env_key::<EnvType>();
    /// assert_eq!(EnvType::Test, env);
    /// ```
    pub fn from_env_key<T: EnvKey>() -> Self {
        match std::env::var(T::key()) {
            Ok(env) => Self::from_str(&env).unwrap_or_default(),
            Err(_) => Self::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_env_type() {
        assert_eq!(EnvType::from_str("Develop").unwrap(), EnvType::Dev);
        assert_eq!(EnvType::from_str("develop").unwrap(), EnvType::Dev);
        assert_eq!(EnvType::from_str("DEV").unwrap(), EnvType::Dev);
        assert_eq!(EnvType::from_str("Dev").unwrap(), EnvType::Dev);
        assert_eq!(EnvType::from_str("dev").unwrap(), EnvType::Dev);
        assert_eq!(EnvType::from_str("D").unwrap(), EnvType::Dev);
        assert_eq!(EnvType::from_str("d").unwrap(), EnvType::Dev);
        assert_eq!(EnvType::from_str("TEST").unwrap(), EnvType::Test);
        assert_eq!(EnvType::from_str("Test").unwrap(), EnvType::Test);
        assert_eq!(EnvType::from_str("test").unwrap(), EnvType::Test);
        assert_eq!(EnvType::from_str("T").unwrap(), EnvType::Test);
        assert_eq!(EnvType::from_str("t").unwrap(), EnvType::Test);
        assert_eq!(EnvType::from_str("Staging").unwrap(), EnvType::Stg);
        assert_eq!(EnvType::from_str("staging").unwrap(), EnvType::Stg);
        assert_eq!(EnvType::from_str("STG").unwrap(), EnvType::Stg);
        assert_eq!(EnvType::from_str("Stg").unwrap(), EnvType::Stg);
        assert_eq!(EnvType::from_str("stg").unwrap(), EnvType::Stg);
        assert_eq!(EnvType::from_str("S").unwrap(), EnvType::Stg);
        assert_eq!(EnvType::from_str("s").unwrap(), EnvType::Stg);
        assert_eq!(EnvType::from_str("Production").unwrap(), EnvType::Prod);
        assert_eq!(EnvType::from_str("production").unwrap(), EnvType::Prod);
        assert_eq!(EnvType::from_str("PROD").unwrap(), EnvType::Prod);
        assert_eq!(EnvType::from_str("Prod").unwrap(), EnvType::Prod);
        assert_eq!(EnvType::from_str("prod").unwrap(), EnvType::Prod);
        assert_eq!(EnvType::from_str("P").unwrap(), EnvType::Prod);
        assert_eq!(EnvType::from_str("p").unwrap(), EnvType::Prod);
    }

    #[test]
    fn test_is_debug() {
        assert!(EnvType::Dev.is_dev());
        assert!(!EnvType::Test.is_dev());
        assert!(!EnvType::Stg.is_dev());
        assert!(!EnvType::Prod.is_dev());

        assert!(EnvType::Test.is_test());
        assert!(EnvType::Stg.is_stg());
        assert!(EnvType::Prod.is_prod());
    }

    #[test]
    fn test_from_env() {
        std::env::set_var("ENV", "d");
        assert_eq!(EnvType::from_env(), EnvType::Dev);

        struct TestEnv;
        impl EnvKey for TestEnv {
            fn key() -> &'static str {
                "TEST_ENV"
            }
        }
        std::env::set_var("TEST_ENV", "t");
        assert_eq!(EnvType::from_env_key::<TestEnv>(), EnvType::Test);
        // fallback to default
        assert_eq!(EnvType::from_env_key::<EnvType>(), EnvType::Dev);
    }
}

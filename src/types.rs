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

/// AsEnvTypeStr is a trait that covert some type to a string with Key Type, which is the environment type.
/// This trait can extend the existing configuration struct to get the environment type.
///
/// # Example
///
/// ```
/// use env_type::types::{EnvType, EnvKey, AsEnvStr};
/// use std::collections::HashMap;
///
/// struct Config {
///    map: HashMap<&'static str, String>,
/// }
///
/// impl AsEnvStr for Config {
///   fn as_env_str<T: EnvKey>(&self) -> String {
///      self.map.get(T::key()).map(|v|v.to_string()).unwrap_or_default()
///  }
/// }
/// let mut map = HashMap::new();
/// map.insert("ENV", "test".to_string());
/// let config = Config {
///   map,
/// };
/// assert_eq!("test", config.as_env_str::<EnvType>());
///
/// ```
pub trait AsEnvStr {
    fn as_env_str<T: EnvKey>(&self) -> String;
}

/// EnvType is an implementation of the AsEnvStr trait.
/// EnvType is baesd on env var.
impl AsEnvStr for EnvType {
    fn as_env_str<T: EnvKey>(&self) -> String {
        std::env::var(T::key()).unwrap_or_default()
    }
}

/// FromKey<V, S> is a trait like From<T> with a key.
pub trait FromKey<V, S> {
    fn from_key<K: EnvKey>(value: V) -> S;
}

/// AsEnvTypeStr is a trait that covert some type to a string, which is the environment type.
/// This trait can extend the existing configuration struct to get the environment type.
///
/// # Example
///
/// ```
/// use env_type::types::{EnvType, AsEnvTypeStr};
///
/// struct Config {
///    env_str: String,
/// }
///
/// impl AsEnvTypeStr for Config {
///   fn as_env_type_str(&self) -> Option<String> {
///      Some(self.env_str.clone())
///  }
/// }
/// let config = Config {
///   env_str: "dev".to_string(),
/// };
/// assert_eq!("dev", config.as_env_type_str().unwrap());
///
/// ```
pub trait AsEnvTypeStr {
    fn as_env_type_str(&self) -> Option<String>;
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
        Self::from_env_types::<Self, Self>(Self::default())
    }

    /// EnvType::from_env_key is a function that returns the environment type from the environment variable.
    /// The default environment type is Dev.
    ///
    /// # Example
    ///
    /// ```
    /// use env_type::types::EnvType;
    /// use std::str::FromStr;
    ///
    /// std::env::set_var("ENV", "Test");
    /// let env = EnvType::from_env_key::<EnvType>();
    /// assert_eq!(EnvType::Test, env);
    /// ```
    pub fn from_env_key<K: EnvKey>() -> Self {
        Self::from_env_types::<Self, K>(Self::default())
    }

    /// EnvType::from_env_types is a function that returns the EnvType from AsEnvStr and EnvKey.
    pub fn from_env_types<S: AsEnvStr, K: EnvKey>(s: S) -> Self {
        Self::from_str(&s.as_env_str::<K>()).unwrap_or_default()
    }

    /// EnvType::from_env_str is a function that returns the environment type from the string.
    /// The default environment type is Dev.
    ///
    /// # Example
    ///
    /// ```
    /// use env_type::types::{EnvType, AsEnvTypeStr};
    /// use std::str::FromStr;
    ///
    /// struct Config {
    ///   env_str: String,
    /// }
    /// let config = Config {
    ///  env_str: "Production".to_string(),
    /// };
    ///
    /// impl AsEnvTypeStr for Config {
    ///   fn as_env_type_str(&self) -> Option<String> {
    ///     Some(self.env_str.clone())
    ///   }
    /// }
    /// let env = EnvType::from_env_str(config);
    /// assert_eq!(EnvType::Prod, env);
    pub fn from_env_str<T: AsEnvTypeStr>(t: T) -> Self {
        Self::from_str(t.as_env_type_str().unwrap_or_default().as_str()).unwrap_or_default()
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

    #[test]
    fn test_from_env_str() {
        struct TestEnv(&'static str);

        impl AsEnvTypeStr for TestEnv {
            fn as_env_type_str(&self) -> Option<String> {
                Some(self.0.to_string())
            }
        }

        assert_eq!(EnvType::from_env_str(TestEnv("d")), EnvType::Dev);
        assert_eq!(EnvType::from_env_str(TestEnv("t")), EnvType::Test);
        assert_eq!(EnvType::from_env_str(TestEnv("s")), EnvType::Stg);
        assert_eq!(EnvType::from_env_str(TestEnv("p")), EnvType::Prod);
    }
}

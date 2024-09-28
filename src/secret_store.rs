use crate::types::*;
use shuttle_runtime::SecretStore;

/// From<SecretStore> is an implementation of the EnvType trait.
/// This implementation is used to get the environment type from the secret store.
/// The default environment type is Dev.
impl From<SecretStore> for EnvType {
    fn from(secret: SecretStore) -> Self {
        Self::from_env_types::<SecretStore, Self>(secret)
    }
}

/// FromKey<SecretStore, EnvType> is an implementation of the EnvType trait.
/// This implementation is used to get the environment type from the secret store.
impl FromKey<SecretStore, EnvType> for EnvType {
    fn from_key<K: EnvKey>(secret: SecretStore) -> Self {
        EnvType::from_env_types::<SecretStore, K>(secret)
    }
}

/// AsEnvTypeStr is an implementation of the AsEnvTypeStr trait.
/// This implementation is used to get the environment type from the secret store.
impl AsEnvStr for SecretStore {
    fn as_env_str<T>(&self) -> String
    where
        T: EnvKey,
    {
        self.get(T::key()).unwrap_or_default()
    }
}

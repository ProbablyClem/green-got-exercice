use std::collections::HashMap;

/// LogoService is responsible for returning the logo of a company
pub trait LogoService {
    ///get the logo from a counterpart
    fn get_logo(&self, counterpart: &str) -> Option<String>;
}

/// LogoServiceMap is a simple implementation of LogoService using a HashMap
pub struct LogoServiceMap {
    map: HashMap<String, String>,
}

impl LogoServiceMap {
    pub fn new() -> Self {
        let mut map = HashMap::new();
        map.insert("sncf".to_string(), "/companies/logo-sncf.svg".to_string());
        map.insert("scnf".to_string(), "/companies/logo-sncf.svg".to_string());
        map.insert("ratp".to_string(), "/companies/logo-ratp.svg".to_string());
        map.insert("edf".to_string(), "/companies/logo-edf.svg".to_string());
        map.insert("gdf".to_string(), "/companies/logo-gdf.svg".to_string());
        map.insert("free".to_string(), "/companies/logo-free.svg".to_string());
        LogoServiceMap { map }
    }
}

impl LogoService for LogoServiceMap {
    fn get_logo(&self, counterpart: &str) -> Option<String> {
        self.map.get(&counterpart.to_lowercase()).cloned()
    }
}

#[cfg(test)]
mod test{
    use crate::services::logo_service::LogoService;

    #[test]
    fn test_get_logo() {
        let logo_service = super::LogoServiceMap::new();
        assert_eq!(logo_service.get_logo("sncf"), Some("/companies/logo-sncf.svg".to_string()));
    }

    #[test]
    fn test_get_logo_lowercase() {
        let logo_service = super::LogoServiceMap::new();
        assert_eq!(logo_service.get_logo("Sncf"), Some("/companies/logo-sncf.svg".to_string()));
    }

    #[test]
    fn test_get_logo_not_found() {
        let logo_service = super::LogoServiceMap::new();
        assert_eq!(logo_service.get_logo("not_found"), None);
    }
}

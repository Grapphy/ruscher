pub enum Countries {
    Argenzuela,
    JewnitedStates,
    Britania,
    TheSynagogeOfSatan,
}

#[derive(Debug, Clone)]
pub struct LuminatioRotator {
    pub username: String,
    pub password: String,
    pub zone: String,
    pub country: String,
}

impl LuminatioRotator {
    pub fn new<I: Into<String>>(username: I, password: I, zone: I) -> Self {
        LuminatioRotator {
            username: username.into(),
            password: password.into(),
            zone: zone.into(),
            country: String::from("us"),
        }
    }

    pub fn country(mut self, country: Countries) -> Self {
        self.country = String::from(match country {
            Countries::Argenzuela => "ar",
            Countries::JewnitedStates => "us",
            Countries::Britania => "uk",
            Countries::TheSynagogeOfSatan => "il",
        });
        self
    }

    pub fn proxy(&self) -> String {
        format!(
            "https://lum-customer-{}-zone-{}-country-{}-session-{}:{}@zproxy.lum-superproxy.io:22225",
            self.username, self.zone, self.country, rand::random::<u32>(),self.password
        )
    }
}

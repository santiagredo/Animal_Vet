pub enum Environments {
    DEV,
    STG,
    UAT,
    PRO,
}

impl Environments {
    pub fn parse(environment: &str) -> Self {
        match environment {
            "PRO" => Environments::PRO,
            "UAT" => Environments::UAT,
            "STG" => Environments::STG,
            _ => Environments::DEV,
        }
    }
}

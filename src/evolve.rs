pub struct Evolution {
    db: Option<crate::Db>,
}

impl Evolution {
    pub fn new(db: Option<crate::Db>) -> Self {
        Self { db }
    }
}

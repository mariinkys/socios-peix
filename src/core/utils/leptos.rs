use leptos::prelude::*;

use crate::core::models::interest::Interest;

#[derive(Debug, Clone, Default)]
pub struct LeptosEmail {
    pub subject: String,
    pub body: String,
}

#[derive(Debug, Clone)]
pub struct LeptosSelectableInterest {
    pub interest: Interest,
    pub selected: RwSignal<bool>,
}

#[derive(Debug, Clone, Default)]
pub struct LeptosCupon {
    pub expires_at: Option<chrono::NaiveDate>,
    pub description: String,
}

#[derive(Debug, Clone, Default)]
pub struct LeptosMemberCupon {
    pub email: LeptosEmail,
    pub cupon: LeptosCupon,
}

use leptos::prelude::*;

use crate::core::models::interest::Interest;

#[derive(Debug, Clone)]
pub struct LeptosSelectableInterest {
    pub interest: Interest,
    pub selected: RwSignal<bool>,
}

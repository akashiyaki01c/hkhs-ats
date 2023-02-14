use crate::api::Ats;

/// ATSプラグインの実体
#[derive(Default)]
pub struct HkhsAts();

impl Ats for HkhsAts {}
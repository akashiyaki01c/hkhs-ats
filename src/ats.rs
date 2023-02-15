use std::ffi::c_int;

use crate::api::{Ats, Handles, ConstantSpeed};

/// ATSプラグインの実体
#[derive(Default)]
pub struct HkhsAts {
    user_notch: UserNotch
}

impl Ats for HkhsAts {

    fn elapse(&mut self, _state: crate::api::VehicleState, _panel: &[c_int], _sound: &[c_int]) -> crate::api::Handles {
        Handles {
            power: self.user_notch.power,
            brake: self.user_notch.brake,
            reverser: self.user_notch.reverser,
            constant_speed: ConstantSpeed::Continue as c_int
        }
    }

    fn set_power(&mut self, _notch: c_int) {
        self.user_notch.power = _notch;
    }
    fn set_brake(&mut self, _notch: c_int) {
        self.user_notch.brake = _notch;
    }
    fn set_reverser(&mut self, _pos: c_int) {
        self.user_notch.reverser = _pos;
    }

}

#[derive(Default)]
pub struct UserNotch {
    pub power: c_int,
    pub brake: c_int,
    pub reverser: c_int
}
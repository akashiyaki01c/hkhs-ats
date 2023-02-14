#![allow(unused)]

use std::ffi::{
    c_int, 
    c_double, 
    c_float
};
use int_enum::IntEnum;

/// 車両諸元
#[derive(Debug, Default, PartialEq)]
#[repr(C)]
pub struct VehicleSpec {
    /// ブレーキノッチ数
    brake_notches: c_int,
    /// 力行ノッチ数
    power_notches: c_int,
    /// ATS確認ノッチ
    ats_notch: c_int,
    /// ブレーキ弁67度に相当するノッチ
    b67_notch: c_int,
    /// 編成両数
    cars: c_int
}

/// 車両の状態量
#[derive(Debug, Default, PartialEq)]
#[repr(C)]
pub struct VehicleState {
    /// 列車位置 [m]
    location: c_double,
    /// 列車速度 [km/h]
    speed: c_float,
    /// 現在時刻 [ms]
    time: c_int,
    /// ブレーキシリンダ圧力 [kPa]
    bc_pressure: c_float,
    /// 元空気ダメ圧力 [kPa]
    mr_pressure: c_float,
    /// 釣り合い空気ダメ圧力 [kPa]
    er_pressure: c_float,
    /// ブレーキ管圧力 [kPa]
    bp_pressure: c_float,
    /// 直通管圧力 [kPa]
    sap_pressure: c_float,
    /// 電流 [A]
    current: c_float
}

/// 車上子で受け取った情報
#[derive(Debug, Default, PartialEq)]
#[repr(C)]
pub struct BeaconData {
    /// 地上子種別
    beacon_type: c_int,
    /// 対となるセクションの信号
    signal: c_int,
    /// 対となるセクションまでの距離 [m]
    distance: c_float,
    /// 地上子に設定された任意の値
    optional: c_int
}

/// Bve trainsim に渡すハンドル制御値
#[derive(Debug, Default, PartialEq)]
#[repr(C)]
pub struct Handles {
    /// ブレーキノッチ
    brake: c_int,
    /// 力行ノッチ
    power: c_int,
    /// レバーサー位置
    reverser: c_int,
    /// 定速制御の状態
    constant_speed: c_int
}

/// ATS Plug-in Version
pub const API_VERSION: c_int = 0x00020000;

#[derive(Clone, Copy, Debug, Eq, PartialEq, IntEnum)]
#[repr(i32)]
pub enum Keys {
    /// S Key
    S = 0,
    /// A1 Key
    A1 = 1, 
    /// A2 Key
    A2 = 2,
    /// B1 Key
    B1 = 3, 
    /// B2 Key
    B2 = 4,
    /// C1 Key
    C1 = 5, 
    /// C2 Key
    C2 = 6,
    /// D Key
    D = 7,
    /// R Key
    R = 8,
    /// F Key
    F = 9,
    /// G Key
    G = 10,
    /// H Key
    H = 11,
    /// I Key
    I = 12,
    /// J Key
    J = 13,
    /// K Key
    K = 14,
    /// L Key
    L = 15,
}

/// Initial Position of Handle
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntEnum)]
#[repr(i32)]
pub enum InitPosition {
    /// Handle Removed
    Removed = 2,
    /// Emergency Brake
    Emg = 1,
    /// Service Brake
    Svc = 0,
}

/// Sound Control Instruction
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntEnum)]
#[repr(i32)]
pub enum Sound {
    /// Stop
    Stop = -10000,
    /// Play Once
    Play = 1,
    /// Play Repeatedly
    PlayLooping = 0,
    /// Continue
    Continue = 2,
}

/// Type of Horn
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntEnum)]
#[repr(i32)]
pub enum Horns {
    /// Horn 1
    Primary = 0,
    /// Horn 2
    Secondary = 1,
    /// Music Horn
    Music = 2,
}

/// Constant Speed Control Instruction
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntEnum)]
#[repr(i32)]
pub enum ConstantSpeed {
    /// Continue
    Continue = 0,
    /// Enable
    Enable = 1,
    /// Disable
    Disable = 2,
}

/// ATS Trait
pub trait Ats : Default {
    /// プラグインが読み込まれたときに呼び出される関数です
    fn load(&mut self) {}
    /// プラグインが解放されたときに呼び出される関数です
    fn dispose(&mut self) {}
    /// この ATS プラグインが準じているフォーマットを返す関数です
    fn get_plugin_version(&mut self) -> c_int { API_VERSION }
    /// 車両読み込み時に呼び出される関数です
    fn set_vehicle_spec(&mut self, _spec: VehicleSpec) {}
    /// ゲーム開始 ([開く] または [はじめから] 選択) 時に呼び出される関数です
    fn initialize(&mut self, _brake: c_int) {}
    /// 1 フレームごとに呼び出される関数です
    fn elapse(&mut self, _state: VehicleState, _panel: &[c_int], _sound: &[c_int]) -> Handles { Handles::default() }
    /// 主ハンドルが扱われたときに呼び出される関数です
    fn set_power(&mut self, _notch: c_int) {}
    /// ブレーキが扱われたときに呼び出される関数です
    fn set_brake(&mut self, _notch: c_int) {}
    /// レバーサーが扱われたときに呼び出される関数です
    fn set_reverser(&mut self, _pos: c_int) {}
    /// ATS キーが押されたときに呼び出される関数です
    fn key_down(&mut self, _key: Keys) {}
    /// ATS キーが離されたときに呼び出される関数です
    fn key_up(&mut self, _key: Keys) {}
    /// 警笛が扱われたときに呼び出される関数です
    fn horn_blow(&mut self, _type: Horns) {}
    /// 客室ドアが開いたときに呼び出される関数です
    fn door_open(&mut self) {}
    /// 客室ドアが閉まったときに呼び出される関数です
    fn door_close(&mut self) {}
    /// 現在の閉そくの信号が変化したときに呼び出される関数です
    fn set_signal(&mut self, _signal: c_int) {}
    /// 地上子を越えたときに呼び出される関数です
    fn set_beacon_data(&mut self, _beacon_data: BeaconData) {}
}

#[macro_export]
macro_rules! ats_main {
    ($t: ty) => {
        use ::once_cell::sync::Lazy;
        use crate::api::Ats;
        use ::std::ffi::c_int;
        use ::std::sync::Mutex;
        use ::int_enum::IntEnum;

        static ATS: Lazy<Mutex<$t>> = Lazy::new(|| Mutex::new(<$t>::default()));

        #[no_mangle]
        #[allow(non_snake_case)]
        pub extern "C" fn Load() {
            ATS.lock().unwrap().load()
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub extern "C" fn Dispose() {
            ATS.lock().unwrap().dispose()
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub extern "C" fn GetPluginVersion() -> c_int {
            ATS.lock().unwrap().get_plugin_version()
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub extern "C" fn SetVehicleSpec(_spec: crate::api::VehicleSpec) {
            ATS.lock().unwrap().set_vehicle_spec(_spec)
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub extern "C" fn Initialize(_brake: i32) {
            ATS.lock().unwrap().initialize(_brake)
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub extern "C" fn Elapse(_state: crate::api::VehicleState, _panel: *mut i32, _sound: *mut i32) -> crate::api::Handles {
            ATS.lock().unwrap().elapse(
                _state, 
                unsafe { ::std::slice::from_raw_parts_mut(_panel, 256) }, 
                unsafe { ::std::slice::from_raw_parts_mut(_sound, 256) }
            )
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub extern "C" fn SetPower(_notch: i32) {
            ATS.lock().unwrap().set_power(_notch)
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub extern "C" fn SetBrake(_notch: i32) {
            ATS.lock().unwrap().set_brake(_notch)
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub extern "C" fn SetReverser(_pos: i32) {
            ATS.lock().unwrap().set_reverser(_pos)
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub extern "C" fn KeyDown(_key: i32) {
            let key = crate::api::Keys::from_int(_key).unwrap();
            ATS.lock().unwrap().key_down(key)
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub extern "C" fn KeyUp(_key: i32) {
            let key = crate::api::Keys::from_int(_key).unwrap();
            ATS.lock().unwrap().key_up(key)
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub extern "C" fn HornBlow(_type: i32) {
            ATS.lock().unwrap().horn_blow(crate::api::Horns::from_int(_type).unwrap());
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub extern "C" fn DoorOpen() {
            ATS.lock().unwrap().door_open()
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub extern "C" fn DoorClose() {
            ATS.lock().unwrap().door_close()
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub extern "C" fn SetSignal(_signal: i32) {
            ATS.lock().unwrap().set_signal(_signal)
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub extern "C" fn SetBeaconData(_data: crate::api::BeaconData) {
            ATS.lock().unwrap().set_beacon_data(_data)
        }
    };
}
#[cxx::bridge(namespace = bluetooth::common::init_flags)]
#[allow(unsafe_op_in_unsafe_fn)]
mod ffi {
    struct InitFlagWithValue {
        flag: &'static str,
        value: String,
    }

    extern "Rust" {
        fn load(flags: Vec<String>);
        fn set_all_for_testing();

        fn dump() -> Vec<InitFlagWithValue>;

        fn classic_discovery_only_is_enabled() -> bool;
        fn dynamic_avrcp_version_enhancement_is_enabled() -> bool;
        fn get_hci_adapter() -> i32;
        fn sdp_skip_rnr_if_known_is_enabled() -> bool;
        fn set_min_encryption_is_enabled() -> bool;
        fn subrating_is_enabled() -> bool;
        fn use_unified_connection_manager_is_enabled() -> bool;
        fn sdp_return_classic_services_when_le_discovery_fails_is_enabled() -> bool;
        fn use_rsi_from_cached_inqiry_results_is_enabled() -> bool;
        fn get_att_mtu_default() -> i32;
    }
}

use crate::init_flags::ffi::InitFlagWithValue;

fn dump() -> Vec<InitFlagWithValue> {
    bt_common::init_flags::dump()
        .into_iter()
        .map(|(flag, value)| InitFlagWithValue { flag, value })
        .collect()
}

use bt_common::init_flags::*;

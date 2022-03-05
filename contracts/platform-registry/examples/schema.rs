use std::env::current_dir;
use std::fs::create_dir_all;

use cosmwasm_schema::{export_schema, remove_schemas, schema_for};
pub use platform_registry::{AddressBaseInfoResponse, PlatformRegistryQueryMsg};

fn main() {
    let mut out_dir = current_dir().unwrap();
    out_dir.push("schema");
    create_dir_all(&out_dir).unwrap();
    remove_schemas(&out_dir).unwrap();

    export_schema(&schema_for!(PlatformRegistryQueryMsg), &out_dir);
    export_schema(&schema_for!(AddressBaseInfoResponse), &out_dir);
}

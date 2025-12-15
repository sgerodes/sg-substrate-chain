use sg_solochain_runtime as runtime;
use sc_service::ChainType;
use runtime::WASM_BINARY;
use sc_service::Properties;

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec;

pub fn development_chain_spec() -> Result<ChainSpec, String> {
	let properties = build_default_chain_properties();

	Ok(ChainSpec::builder(
		WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?,
		None,
	)
	.with_name("Development")
	.with_id("dev")
	.with_chain_type(ChainType::Development)
	.with_genesis_config_preset_name(sp_genesis_builder::DEV_RUNTIME_PRESET)
		.with_properties(properties)
	.build())
}

pub fn local_chain_spec() -> Result<ChainSpec, String> {
	let properties = build_default_chain_properties();

	Ok(ChainSpec::builder(
		WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?,
		None,
	)
	.with_name("Local Testnet")
	.with_id("local_testnet")
	.with_chain_type(ChainType::Local)
	.with_genesis_config_preset_name(sp_genesis_builder::LOCAL_TESTNET_RUNTIME_PRESET)
	.with_properties(properties)
	.build())
}

fn build_default_chain_properties() -> Properties {
	let mut properties = Properties::new();
	properties.insert("tokenSymbol".into(), network_constants::TOKEN_SYMBOL.into());
	properties.insert("tokenDecimals".into(), network_constants::TOKEN_DECIMALS.into());
	properties.insert("ss58Format".into(), network_constants::SS58FORMAT.into());
	properties
}
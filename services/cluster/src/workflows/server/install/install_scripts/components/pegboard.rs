use chirp_workflow::prelude::*;

use super::{fdb::FDB_VERSION, tivet::TUNNEL_API_EDGE_PORT};

pub const TUNNEL_PEGBOARD_WS_PORT: u16 = 5030;

pub async fn install(
	config: &tivet_config::Config,
	flavor: pegboard::protocol::ClientFlavor,
) -> GlobalResult<String> {
	let provision_config = &config.server()?.tivet.provision()?;

	Ok(include_str!("../files/pegboard_install.sh")
		.replace("__FLAVOR__", &flavor.to_string())
		.replace(
			"__PEGBOARD_MANAGER_BINARY_URL__",
			provision_config.manager_binary_url.as_ref(),
		)
		.replace(
			"__CONTAINER_RUNNER_BINARY_URL__",
			provision_config.container_runner_binary_url.as_ref(),
		)
		.replace(
			"__ISOLATE_V8_RUNNER_BINARY_URL__",
			provision_config.isolate_runner_binary_url.as_ref(),
		)
		.replace("__FDB_VERSION__", FDB_VERSION))
}

pub fn configure(
	config: &tivet_config::Config,
	flavor: pegboard::protocol::ClientFlavor,
) -> GlobalResult<String> {
	let provision_config = config.server()?.tivet.provision()?;

	let origin_api =
		util::url::to_string_without_slash(&config.server()?.tivet.api_public.public_origin());

	let pb_reserved_memory = match flavor {
		pegboard::protocol::ClientFlavor::Container => {
			server_spec::PEGBOARD_CONTAINER_RESERVE_MEMORY_MIB
		}
		pegboard::protocol::ClientFlavor::Isolate => {
			server_spec::PEGBOARD_ISOLATE_RESERVE_MEMORY_MIB
		}
	};

	Ok(include_str!("../files/pegboard_configure.sh")
		.replace("__FLAVOR__", &flavor.to_string())
		.replace("__ORIGIN_API__", &origin_api)
		.replace(
			"__TUNNEL_API_EDGE_API__",
			&format!("http://127.0.0.1:{TUNNEL_API_EDGE_PORT}"),
		)
		.replace(
			"__PEGBOARD_ENDPOINT__",
			&format!("ws://127.0.0.1:{TUNNEL_PEGBOARD_WS_PORT}"),
		)
		.replace(
			"__RESERVED_MEMORY__",
			&(server_spec::RESERVE_LB_MEMORY_MIB + pb_reserved_memory).to_string(),
		)
		// HACK: Hardcoded to Linode
		.replace("__PUBLIC_IFACE__", "eth0")
		// HACK: Hardcoded to Linode
		.replace("__VLAN_IFACE__", "eth1")
		.replace(
			"__GG_VLAN_SUBNET__",
			&provision_config.pools.gg.vlan_ip_net().to_string(),
		)
		.replace(
			"__ATS_VLAN_SUBNET__",
			&provision_config.pools.ats.vlan_ip_net().to_string(),
		)
		.replace(
			"__MIN_WAN_PORT__",
			&provision_config.pools.pegboard.min_wan_port().to_string(),
		)
		.replace(
			"__MAX_WAN_PORT__",
			&provision_config.pools.pegboard.max_wan_port().to_string(),
		))
}

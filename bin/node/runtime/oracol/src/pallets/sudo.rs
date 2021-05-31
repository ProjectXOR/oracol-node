// --- substrate ---
use pallet_sudo::Config;
// --- oracol ---
use crate::*;

impl Config for Runtime {
	type Event = Event;
	type Call = Call;
}

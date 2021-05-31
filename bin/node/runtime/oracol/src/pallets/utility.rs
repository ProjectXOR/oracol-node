// --- substrate ---
use pallet_utility::Config;
// --- oracol ---
use crate::*;

impl Config for Runtime {
	type Event = Event;
	type Call = Call;
	type WeightInfo = ();
}

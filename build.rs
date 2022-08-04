mod feature_gates;

use feature_gates::{Feature, Status};
use std::fmt::Display;

const FEATURE_LIST: [Feature; 4] = [
    Feature::new(
        "add_pair",
        "Allows you to add two numbers together",
        Some(123),
        Status::New,
    ),
    Feature::new(
        "about_to_launch",
        "Live in NextNet. If we stabilise, will go to mainnet",
        Some(123),
        Status::Testing,
    ),
    Feature::new(
        "live_feature",
        "This feature has been stabilised and is live!",
        Some(150),
        Status::Active,
    ),
    Feature::new(
        "old_feature",
        "We decided not to go with this featuree",
        Some(145),
        Status::Removed,
    ),
];

pub enum Target {
    Dibbler,
    NextNet,
    MainNet,
}

impl Display for Target {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Target::Dibbler => f.write_str("Dibbler"),
            Target::NextNet => f.write_str("NextNet"),
            Target::MainNet => f.write_str("MainNet"),
        }
    }
}

pub fn main() {
    let target = identify_target();
    println!("Building for {}", target);
    list_active_features();
    list_removed_features();
    if let Err(e) = resolve_features(target) {
        eprintln!(
            "Could not build Tari due to issues with the feature flag set.\n{}",
            e.to_string()
        );
        panic!();
    }
    // Build as usual
}

// Identify the target network by
// 1. Checking whether --config tari-network=xxx was passed in as a config flag to cargo (or from Cargo.toml)
// 2. Checking the environment variable TARI_NETWORK is set
// 3. default to mainnet
fn identify_target() -> Target {
    check_envar("CARGO_CFG_TARI_NETWORK")
        .or_else(|| check_envar("TARI_NETWORK"))
        .unwrap_or(Target::MainNet)
}

fn check_envar(envar: &str) -> Option<Target> {
    match std::env::var(envar) {
        Ok(s) => match s.to_lowercase().as_str() {
            "mainnet" => Some(Target::MainNet),
            "nextnet" => Some(Target::NextNet),
            "dibbler" => Some(Target::Dibbler),
            s => {
                println!("Unknown network, {}", s);
                None
            }
        },
        _ => None,
    }
}

fn list_active_features() {
    println!("These features are ACTIVE on mainnet (no special code handling is done)");
    FEATURE_LIST
        .iter()
        .filter(|f| f.is_active())
        .for_each(|f| println!("{}", f));
}

fn list_removed_features() {
    println!("These features are DEPRECATED and will never be compiled");
    FEATURE_LIST
        .iter()
        .filter(|f| f.was_removed())
        .for_each(|f| println!("{}", f));
}

fn resolve_features(target: Target) -> Result<(), String> {
    match target {
        Target::MainNet => { /* No features are active at all */ }
        Target::NextNet => FEATURE_LIST
            .iter()
            .filter(|f| f.is_active_in_nextnet())
            .for_each(|f| activate_feature(f)),
        Target::Dibbler => FEATURE_LIST
            .iter()
            .filter(|f| f.is_active_in_dibbler())
            .for_each(|f| activate_feature(f)),
    }
    Ok(())
}

fn activate_feature(feature: &Feature) {
    println!("** Activating {} **", feature);
    println!("cargo:rustc-cfg={}", feature.attr_name());
}

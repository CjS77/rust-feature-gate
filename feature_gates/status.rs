pub enum Status {
    New,     // new feature, may not even be working or compiling. Will be present on dibbler testnet
    Testing, // potentially active feature, but still gated. Potentially present on nextnet,
    Active,  // feature is live and gate has been removed. Will be running on stagenet and mainnet
    Removed, // feature has been cancelled. Can not be invoked anywhere
}

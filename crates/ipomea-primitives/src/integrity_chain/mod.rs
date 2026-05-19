use fixedstr::{str128, str256};

pub struct IpomeaIntegrityChain {
    pub dashboard: IpomeaIntegrityChainDashboard,
    pub chain: Vec<IpomeaIntegrityChainBlock>,
}

pub struct IpomeaIntegrityChainDashboard {
    pub name: str128,
    pub description: str256,
}

pub struct IpomeaIntegrityChainBlock {
    pub id: u64,
    pub previous_hash: str128,
    pub data: Vec<IpomeaIntegrityChainData>,
}

pub struct IpomeaIntegrityChainData {
    pub content: str256,
}
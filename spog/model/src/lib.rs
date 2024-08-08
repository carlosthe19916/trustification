pub mod config;
pub mod csaf;
pub mod cve;
pub mod package_info;
pub mod pkg;
pub mod search;
pub mod suggestion;
pub mod vuln;

pub mod prelude {
    pub use crate::{config::*, cve::*, package_info::*, pkg::*, search::*, suggestion::*, vuln::*};
}

#[derive(Clone, serde::Deserialize, serde::Serialize)]
pub struct DashboardStatus {
    pub sbom_summary: SbomSummary,
    pub csaf_summary: CSAFSummary,
    pub cve_summary: CveSummary,
}

#[derive(Clone, serde::Deserialize, serde::Serialize)]
pub struct SbomSummary {
    /// Total number of all documents
    pub total_sboms: Option<u64>,
    /// Id of last updated doc
    pub last_updated_sbom_id: Option<String>,
    /// name of last updated doc
    pub last_updated_sbom_name: Option<String>,
    /// Updated time of last updated doc
    pub last_updated_date: Option<String>,
}

#[derive(Clone, serde::Deserialize, serde::Serialize)]
pub struct CSAFSummary {
    /// Total number of all documents
    pub total_csafs: Option<u64>,
    /// Id of last updated doc
    pub last_updated_csaf_id: Option<String>,
    /// name of last updated doc
    pub last_updated_csaf_name: Option<String>,
    /// Updated time of last updated doc
    pub last_updated_date: Option<String>,
}

#[derive(Clone, serde::Deserialize, serde::Serialize)]
pub struct CveSummary {
    /// Total number of all documents
    pub total_cves: Option<u64>,
    /// Name of last updated doc
    pub last_updated_cve: Option<String>,
    /// Updated time of last updated doc
    pub last_updated_date: Option<String>,
}

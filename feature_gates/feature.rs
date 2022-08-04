use super::Status;
use std::fmt::{Display, Formatter};

pub struct Feature {
    name: &'static str,
    description: &'static str,
    tracking_issue: Option<usize>,
    status: Status,
}

impl Feature {
    pub const fn new(
        name: &'static str,
        description: &'static str,
        tracking_issue: Option<usize>,
        status: Status,
    ) -> Self {
        Feature {
            name,
            description,
            tracking_issue,
            status,
        }
    }

    pub fn issue_url(&self) -> String {
        match self.tracking_issue {
            Some(n) => format!("https://github.com/tari-project/tari/issues/{}", n),
            None => "None".into(),
        }
    }

    pub fn attr_name(&self) -> String {
        format!("tari_feature_{}", self.name)
    }

    pub fn is_active_in_dibbler(&self) -> bool {
        match self.status {
            Status::New | Status::Testing => true,
            _ => false,
        }
    }

    pub fn is_active_in_nextnet(&self) -> bool {
        match self.status {
            Status::Testing => true,
            _ => false,
        }
    }

    pub fn is_active(&self) -> bool {
        matches!(self.status, Status::Active)
    }

    pub fn was_removed(&self) -> bool {
        matches!(self.status, Status::Removed)
    }
}

impl Display for Feature {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}. {}. Tracking issue: {}",
            self.name,
            self.description,
            self.issue_url()
        )
    }
}

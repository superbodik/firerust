use fr_dom::tree::document::Document;
use url::Url;

use crate::csp::allows_inline::allows_inline;
use crate::csp::allows_resource::allows_resource;
use crate::csp::content_policy::ContentPolicy;
use crate::csp::policies_from_document::policies_from_document;
use crate::fetch::block_reason::BlockReason;
use crate::fetch::check_fetch::check_fetch;
use crate::fetch::resource_kind::ResourceKind;

#[derive(Debug, Clone)]
pub struct PageGuard {
    url: Url,
    policies: Vec<ContentPolicy>,
}

impl PageGuard {
    pub fn new(document: &Document, url: &Url) -> Self {
        Self {
            url: url.clone(),
            policies: policies_from_document(document),
        }
    }

    pub fn check(&self, kind: ResourceKind, target: &Url) -> Result<(), BlockReason> {
        check_fetch(&self.url, target)?;
        let allowed = self
            .policies
            .iter()
            .all(|policy| allows_resource(policy, kind, &self.url, target));
        if allowed {
            Ok(())
        } else {
            Err(BlockReason::ContentPolicy)
        }
    }

    pub fn permit(&self, kind: ResourceKind, target: &Url) -> bool {
        match self.check(kind, target) {
            Ok(()) => true,
            Err(reason) => {
                eprintln!("blocked {kind} {target}: {reason}");
                false
            }
        }
    }

    pub fn allows_inline(&self, kind: ResourceKind) -> bool {
        self.policies
            .iter()
            .all(|policy| allows_inline(policy, kind))
    }
}

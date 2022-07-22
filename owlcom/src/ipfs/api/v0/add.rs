use hyper::{Body, Request};
use serde::Deserialize;

use crate::traits::ToRequest;

#[derive(Debug, Clone, Default)]
pub struct Add {
    optional_params: String,
}
impl ToRequest for Add {
    fn to_request(&self, host: &String) -> Request<Body> {
        hyper::Request::builder()
            .method("POST")
            // .header(key, value)
            .uri(&format!("{}/api/v0/add?{}", host, self.optional_params))
            .body(Body::empty())
            .unwrap()
    }
}
impl Add {
    pub fn new(optional_params: String) -> Self {
        Self { optional_params }
    }
}
#[derive(Debug, Default)]
pub struct Builder {
    optional_params: String,
}

impl Builder {
    pub fn new() -> Self {
        Self::default()
    }
    /// Write minimal output.
    pub fn quiet(self, quiet: bool) -> Self {
        if self.optional_params == String::new() {
            Self {
                optional_params: format!("quiet={}", quiet.to_string()),
            }
        } else {
            Self {
                optional_params: format!("{}&quiet={}", self.optional_params, quiet.to_string()),
            }
        }
    }
    /// Write only final hash.
    pub fn quieter(self, quieter: bool) -> Self {
        if self.optional_params == String::new() {
            Self {
                optional_params: format!("quieter={}", quieter.to_string()),
            }
        } else {
            Self {
                optional_params: format!("{}&quieter={}", self.optional_params, quieter.to_string()),
            }
        }
    }
    /// Write no output.
    pub fn silent(self, silent: bool) -> Self {
        if self.optional_params == String::new() {
            Self {
                optional_params: format!("silent={}", silent.to_string()),
            }
        } else {
            Self {
                optional_params: format!("{}&silent={}", self.optional_params, silent.to_string()),
            }
        }
    }
    /// Stream progress data.
    pub fn progress(self, progress: bool) -> Self {
        if self.optional_params == String::new() {
            Self {
                optional_params: format!("progress={}", progress.to_string()),
            }
        } else {
            Self {
                optional_params: format!("{}&progress={}", self.optional_params, progress.to_string()),
            }
        }
    }
    /// Use trickle-dag format for dag generation.
    pub fn trickle(self, trickle: bool) -> Self {
        if self.optional_params == String::new() {
            Self {
                optional_params: format!("trickle={}", trickle.to_string()),
            }
        } else {
            Self {
                optional_params: format!("{}&trickle={}", self.optional_params, trickle.to_string()),
            }
        }
    }
    /// Only chunk and hash - do not write to disk.
    pub fn only_hash(self, only_hash: bool) -> Self {
        if self.optional_params == String::new() {
            Self {
                optional_params: format!("only-hash={}", only_hash.to_string()),
            }
        } else {
            Self {
                optional_params: format!("{}&only-hash={}", self.optional_params, only_hash.to_string()),
            }
        }
    }
    /// Wrap files with a directory object.
    pub fn wrap_with_directory(self, wrap_with_directory: bool) -> Self {
        if self.optional_params == String::new() {
            Self {
                optional_params: format!("wrap-with-directory={}", wrap_with_directory.to_string()),
            }
        } else {
            Self {
                optional_params: format!(
                    "{}&wrap-with-directory={}",
                    self.optional_params,
                    wrap_with_directory.to_string()
                ),
            }
        }
    }
    /// Chunking algorithm, size-\[bytes\], rabin-\[min\]-\[avg\]-\[max\] or buzhash.
    /// Default to ``size-262144``.
    pub fn chunker(self, chunker: String) -> Self {
        if self.optional_params == String::new() {
            Self {
                optional_params: format!("chunker={}", chunker),
            }
        } else {
            Self {
                optional_params: format!("{}&chunker={}", self.optional_params, chunker.to_string()),
            }
        }
    }
    /// Pin this object when adding. Default to ``true``.
    pub fn pin(self, pin: bool) -> Self {
        if self.optional_params == String::new() {
            Self {
                optional_params: format!("pin={}", pin.to_string()),
            }
        } else {
            Self {
                optional_params: format!("{}&pin={}", self.optional_params, pin.to_string()),
            }
        }
    }
    /// Use raw blocks for leaf nodes.
    pub fn raw_leaves(self, raw_leaves: bool) -> Self {
        if self.optional_params == String::new() {
            Self {
                optional_params: format!("raw-leaves={}", raw_leaves.to_string()),
            }
        } else {
            Self {
                optional_params: format!("{}&raw-leaves={}", self.optional_params, raw_leaves.to_string()),
            }
        }
    }
    /// Add the file using filestore. Implies raw-leaves. (experimental).
    pub fn nocopy(self, nocopy: bool) -> Self {
        if self.optional_params == String::new() {
            Self {
                optional_params: format!("nocopy={}", nocopy.to_string()),
            }
        } else {
            Self {
                optional_params: format!("{}&nocopy={}", self.optional_params, nocopy.to_string()),
            }
        }
    }
    /// Check the filestore for pre-existing blocks. (experimental).
    pub fn fscache(self, fscache: bool) -> Self {
        if self.optional_params == String::new() {
            Self {
                optional_params: format!("fscache={}", fscache.to_string()),
            }
        } else {
            Self {
                optional_params: format!("{}&fscache={}", self.optional_params, fscache.to_string()),
            }
        }
    }
    /// CID version. Defaults to ``0`` unless an option that depends on ``CIDv1`` is passed.
    /// Passing version ``1`` will cause the raw-leaves option to default to ``true``.
    pub fn cid_version(self, cid_version: u64) -> Self {
        if self.optional_params == String::new() {
            Self {
                optional_params: format!("cid-version={}", cid_version.to_string()),
            }
        } else {
            Self {
                optional_params: format!("{}&quiet={}", self.optional_params, cid_version.to_string()),
            }
        }
    }
    /// Hash function to use.
    /// Implies ``CIDv1`` if not ``sha2-256``. (experimental).
    /// Default to ``sha2-256``
    pub fn hash(self, hash: String) -> Self {
        if self.optional_params == String::new() {
            Self {
                optional_params: format!("hash={}", hash),
            }
        } else {
            Self {
                optional_params: format!("{}&hash={}", self.optional_params, hash.to_string()),
            }
        }
    }
    /// Inline small blocks into CIDs. (experimental).
    pub fn inline(self, inline: bool) -> Self {
        if self.optional_params == String::new() {
            Self {
                optional_params: format!("inline={}", inline.to_string()),
            }
        } else {
            Self {
                optional_params: format!("{}&inline={}", self.optional_params, inline.to_string()),
            }
        }
    }
    /// Maximum block size to inline. (experimental).
    pub fn inline_limit(self, inline_limit: bool) -> Self {
        if self.optional_params == String::new() {
            Self {
                optional_params: format!("inline-limit={}", inline_limit.to_string()),
            }
        } else {
            Self {
                optional_params: format!(
                    "{}&inline-limit={}",
                    self.optional_params,
                    inline_limit.to_string()
                ),
            }
        }
    }
    pub fn build(self) -> Add {
        Add {
            optional_params: self.optional_params,
        }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Response {
    bytes: Option<u64>,
    hash: Option<String>,
    name: Option<String>,
    size: Option<String>,
}

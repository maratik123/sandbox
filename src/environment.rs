use std::collections::HashSet;
use std::env;
use std::ffi::OsStr;
use std::os::unix::ffi::OsStrExt;
use std::sync::OnceLock;

pub mod key {
    pub const SANDBOX_TESTING: &str = "__SANDBOX_TESTING";
    pub const SANDBOX_ACTIVE: &str = "SANDBOX_ACTIVE";
    pub const PORTAGE_TMPDIR: &str = "PORTAGE_TMPDIR";
    pub const SANDBOX_WORKDIR: &str = "SANDBOX_WORKDIR";
    pub const TMPDIR: &str = "TMPDIR";
}

const TRUE_VALUES_CONSTS: [&str; 3] = ["1", "true", "yes"];
const FALSE_VALUES_CONSTS: [&str; 3] = ["0", "false", "no"];

const MAX_VALUES_CONST_LEN: usize = 5;

static TRUE_VALUES: OnceLock<HashSet<&str>> = OnceLock::new();
static FALSE_VALUES: OnceLock<HashSet<&str>> = OnceLock::new();

fn values_contains(
    consts: impl IntoIterator<Item = &'static str>,
    values: &OnceLock<HashSet<&str>>,
    value: &OsStr,
) -> bool {
    if let Some(value) = OsStr::from_bytes(&value.as_bytes()[..MAX_VALUES_CONST_LEN]).to_str() {
        let value = value[..MAX_VALUES_CONST_LEN].to_ascii_lowercase();
        values
            .get_or_init(|| HashSet::from_iter(consts))
            .contains(value.as_str())
    } else {
        false
    }
}

#[inline]
pub fn is_env_on(key: &str) -> bool {
    env::var_os(key)
        .map(|value| values_contains(TRUE_VALUES_CONSTS, &TRUE_VALUES, value.as_os_str()))
        .unwrap_or(false)
}

#[inline]
pub fn is_env_off(key: &str) -> bool {
    env::var_os(key)
        .map(|value| values_contains(FALSE_VALUES_CONSTS, &FALSE_VALUES, value.as_os_str()))
        .unwrap_or(false)
}

#[inline]
pub fn has_env(key: &str) -> bool {
    env::var_os(key).is_some()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_values_const_len_should_be_max_for_values() {
        for s in TRUE_VALUES_CONSTS
            .iter()
            .copied()
            .chain(FALSE_VALUES_CONSTS)
        {
            assert!(
                s.len() <= MAX_VALUES_CONST_LEN,
                "'{s}'.len = {} is more than {MAX_VALUES_CONST_LEN}",
                s.len()
            );
        }
    }
}

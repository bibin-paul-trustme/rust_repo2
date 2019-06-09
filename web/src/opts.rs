use log::LevelFilter;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fmt::Display,
    fs::File,
    path::{Path, PathBuf},
};

/// Support tiers: https://forge.rust-lang.org/platform-support.html.
#[derive(Debug, Serialize, Deserialize, Hash, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub enum Tier {
    /// Tier 1 platforms.
    Tier1,
    /// Tier 2 platforms.
    Tier2,
    /// Tier 2.5 platforms.
    Tier25,
    /// Tier 3 platforms.
    Tier3,
    #[doc(hidden)]
    UnknownTier,
}

fn default_verbosity() -> LevelFilter {
    LevelFilter::Warn
}

fn default_channel() -> String {
    String::from("nightly")
}

fn default_additional_days() -> usize {
    0
}

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(flatten)]
    pub html: Html,
    pub days_in_past: usize,
    #[serde(default = "default_additional_days")]
    pub additional_lookup_days: usize,
    #[serde(default = "default_channel")]
    pub channel: String,
    #[serde(default = "default_verbosity")]
    pub verbosity: LevelFilter,
    #[serde(default)]
    pub cache_path: Option<PathBuf>,
}

/// Html-related configuration
#[derive(Debug, Deserialize)]
pub struct Html {
    pub template_path: PathBuf,
    pub output_pattern: String,
    #[serde(default)]
    pub tiers: HashMap<Tier, Vec<String>>,
}

impl Config {
    pub fn load(path: impl AsRef<Path>) -> Result<Self, failure::Error> {
        let path = path.as_ref();
        let f = File::open(path)?;
        Ok(serde_yaml::from_reader(f)?)
    }

    pub fn default_with_comments() -> impl Display {
        format!(
            r#"---
# Path to a handlebars template file:
template_path: /path/to/template.html

# A pattern that will be used to render output files. Any instance of a
# `{{{{target}}}}` will be replaced with a target name.
output_pattern: "/path/to/output/{{{{target}}}}.html"

# For how many days in the past would you like to peek.
days_in_past: 7

# For how many additional days should we look into to calculate "the last available" date.
additional_lookup_days: {additional_lookup_days}

# A release channel to check.
# If ommited, the default level is {channel}.
channel: {channel}

# Verbosity level, might be one of the following: OFF, ERROR, WARN, INFO, DEBUG or TRACE.
# To see any output under normal circumstances set the level to at least INFO.
# If ommited, the default level is {verbosity}.
verbosity: {verbosity}

# A path where to store the downloaded manifests.
# If ommited, no cache will be used, i.e. all the manifests will be
# re-downloaded every time you run the tool.
cache_path: /tmp/manifests/

# Platform tiers lists
tiers:
  Tier1:
    - "i686-apple-darwin"
    - "i686-pc-windows-gnu"
    - "i686-pc-windows-msvc"
    - "i686-unknown-linux-gnu"
    - "x86_64-apple-darwin"
    - "x86_64-pc-windows-gnu"
    - "x86_64-pc-windows-msvc"
    - "x86_64-unknown-linux-gnu"
  Tier2:
    - "aarch64-apple-ios"
    - "aarch64-linux-android"
    - "aarch64-unknown-fuchsia"
    - "aarch64-unknown-linux-gnu"
    - "aarch64-unknown-linux-musl"
    - "arm-linux-androideabi"
    - "arm-unknown-linux-gnueabi"
    - "arm-unknown-linux-gnueabihf"
    - "arm-unknown-linux-musleabi"
    - "arm-unknown-linux-musleabihf"
    - "armv5te-unknown-linux-gnueabi"
    - "armv7-apple-ios"
    - "armv7-linux-androideabi"
    - "armv7-unknown-linux-gnueabihf"
    - "armv7-unknown-linux-musleabihf"
    - "armv7s-apple-ios"
    - "asmjs-unknown-emscripten"
    - "i386-apple-ios"
    - "i586-pc-windows-msvc"
    - "i586-unknown-linux-gnu"
    - "i586-unknown-linux-musl"
    - "i686-linux-android"
    - "i686-unknown-freebsd"
    - "i686-unknown-linux-musl"
    - "mips-unknown-linux-gnu"
    - "mips-unknown-linux-musl"
    - "mips64-unknown-linux-gnuabi64"
    - "mips64el-unknown-linux-gnuabi64"
    - "mipsel-unknown-linux-gnu"
    - "mipsel-unknown-linux-musl"
    - "powerpc-unknown-linux-gnu"
    - "powerpc64-unknown-linux-gnu"
    - "powerpc64le-unknown-linux-gnu"
    - "s390x-unknown-linux-gnu"
    - "sparc64-unknown-linux-gnu"
    - "sparcv9-sun-solaris"
    - "wasm32-unknown-unknown"
    - "wasm32-unknown-emscripten"
    - "x86_64-apple-ios"
    - "x86_64-linux-android"
    - "x86_64-rumprun-netbsd"
    - "x86_64-sun-solaris"
    - "x86_64-unknown-cloudabi"
    - "x86_64-unknown-freebsd"
    - "x86_64-unknown-fuchsia"
    - "x86_64-unknown-linux-gnux32"
    - "x86_64-unknown-linux-musl"
    - "x86_64-unknown-netbsd"
    - "x86_64-unknown-redox"
  Tier25:
    - "aarch64-unknown-cloudabi"
    - "armv7-unknown-cloudabi-eabihf"
    - "i686-unknown-cloudabi"
    - "powerpc-unknown-linux-gnuspe"
    - "sparc-unknown-linux-gnu"
  Tier3:
    - "i686-unknown-haiku"
    - "i686-unknown-netbsd"
    - "le32-unknown-nacl"
    - "mips-unknown-linux-uclibc"
    - "mipsel-unknown-linux-uclibc"
    - "msp430-none-elf"
    - "sparc64-unknown-netbsd"
    - "thumbv6m-none-eabi"
    - "thumbv7em-none-eabi"
    - "thumbv7em-none-eabihf"
    - "thumbv7m-none-eabi"
    - "x86_64-unknown-bitrig"
    - "x86_64-unknown-dragonfly"
    - "x86_64-unknown-haiku"
    - "x86_64-unknown-openbsd"
"#,
            channel = default_channel(),
            verbosity = default_verbosity(),
            additional_lookup_days = default_additional_days(),
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_defaults() {
        let defaults = format!("{}", Config::default_with_comments());
        let defaults: Config = serde_yaml::from_str(&defaults).unwrap();
        assert_eq!(
            Some("/path/to/template.html"),
            defaults.html.template_path.to_str(),
        );
        assert_eq!(
            "/path/to/output/{{target}}.html",
            defaults.html.output_pattern,
        );
        assert_eq!(7, defaults.days_in_past,);
        assert_eq!(default_channel(), defaults.channel,);
        assert_eq!(default_verbosity(), defaults.verbosity,);
        assert_eq!(default_additional_days(), defaults.additional_lookup_days,);
        assert_eq!(
            Some("/tmp/manifests/"),
            defaults.cache_path.as_ref().and_then(|x| x.to_str()),
        );
        assert_eq!(Some(8), defaults.html.tiers.get(&Tier::Tier1).map(Vec::len));
        assert_eq!(
            Some(49),
            defaults.html.tiers.get(&Tier::Tier2).map(Vec::len)
        );
        assert_eq!(
            Some(5),
            defaults.html.tiers.get(&Tier::Tier25).map(Vec::len)
        );
        assert_eq!(
            Some(15),
            defaults.html.tiers.get(&Tier::Tier3).map(Vec::len)
        );
    }
}
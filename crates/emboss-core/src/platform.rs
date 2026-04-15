//! Platform identity primitives shared across the EMBOSS-RS workspace.

/// Immutable identifiers for the EMBOSS-RS platform.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PlatformIdentity {
    /// Canonical CLI binary name.
    pub binary_name: &'static str,
    /// Sister project that provides first-class R access and plotting.
    pub sister_project: &'static str,
    /// Rendering backend for graphical outputs.
    pub plot_backend: &'static str,
}

/// Canonical platform identity for the current workspace.
pub const PLATFORM_IDENTITY: PlatformIdentity = PlatformIdentity {
    binary_name: "emboss-rs",
    sister_project: "emboss-r",
    plot_backend: "R",
};

impl PlatformIdentity {
    /// Returns the governed CLI invocation surface.
    #[must_use]
    pub fn invocation_pattern(self) -> String {
        format!("{} <tool> ...", self.binary_name)
    }
}

#[cfg(test)]
mod tests {
    use super::PLATFORM_IDENTITY;

    #[test]
    fn exposes_single_binary_surface() {
        assert_eq!(
            PLATFORM_IDENTITY.invocation_pattern(),
            "emboss-rs <tool> ..."
        );
    }
}

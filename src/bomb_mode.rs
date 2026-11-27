//! Provider of [`BombMode`].

/// Bomb mode.
#[derive(Debug, Copy, Clone)]
pub(crate) enum BombMode {
    /// No bomb mode.
    None,
    /// Normal bomb mode.
    Normal,
    /// Auto bomb mode.
    Auto,
}

impl BombMode {
    /// Returns `true` if this is bomb.
    pub fn is_bomb(&self) -> bool {
        !matches!(self, BombMode::None)
    }

    /// Returns `true` if this is auto bomb.
    pub fn is_auto_bomb(&self) -> bool {
        matches!(self, BombMode::Auto)
    }

    /// Returns `true` if this bomb explodes.
    pub fn is_explodes(&self, traced: bool) -> bool {
        match self {
            Self::None => false,
            Self::Normal => traced,
            Self::Auto => true,
        }
    }
}

impl Default for BombMode {
    fn default() -> Self {
        Self::None
    }
}

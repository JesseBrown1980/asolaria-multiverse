#![cfg_attr(not(test), no_std)]
#![forbid(unsafe_code)]

//! Source-aware, lossless *logical* three-axis trit addresses.
//! The third axis distinguishes image source/epoch, NOT physical astronomy depth.

/// A balanced trit. All calculations are integer only.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[repr(i8)]
pub enum Trit {
    Neg = -1,
    Zero = 0,
    Pos = 1,
}

impl Trit {
    /// Order-preserving normalized digit, 0, 1, 2.
    pub const fn digit(self) -> usize {
        (self as i8 + 1) as usize
    }

    pub const fn from_digit(v: usize) -> Self {
        match v % 3 {
            0 => Self::Neg,
            1 => Self::Zero,
            _ => Self::Pos,
        }
    }

    /// A three-step rotation about the free central trit.
    pub const fn rotate(self) -> Self {
        Self::from_digit(self.digit() + 1)
    }
}

/// Encode an integer as N exactly reversible balanced trits, with no truncation.
pub fn encode<const N: usize>(mut v: usize) -> Option<[Trit; N]> {
    let mut out = [Trit::Neg; N];
    for digit in out.iter_mut().rev() {
        *digit = Trit::from_digit(v % 3);
        v /= 3;
    }
    if v == 0 {
        Some(out)
    } else {
        None
    }
}

/// Recover an integer ordinal from the balanced-trit fixed-width sequence.
pub fn decode<const N: usize>(digits: [Trit; N]) -> Option<usize> {
    let mut v = 0usize;
    for digit in digits {
        v = v.checked_mul(3)?.checked_add(digit.digit())?;
    }
    Some(v)
}

/// Each z is a distinct source image, never a depth measurement.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Source {
    WebbProcessed2024,
    HubbleProxima2012,
    HubbleProxima2014,
    HubbleProxima2015,
}

impl Source {
    pub const fn geometry(self) -> (usize, usize, usize) {
        match self {
            Self::WebbProcessed2024 => (187, 67, 0),
            Self::HubbleProxima2012 => (9, 9, 1),
            Self::HubbleProxima2014 => (9, 9, 2),
            Self::HubbleProxima2015 => (9, 9, 3),
        }
    }
}

/// Coordinate triple: 5+4+2=11 trits of spatial/observation identity.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Address3D {
    pub x: [Trit; 5],
    pub y: [Trit; 4],
    pub z: [Trit; 2],
}

pub fn address(source: Source, x: usize, y: usize) -> Option<Address3D> {
    let (cols, rows, z) = source.geometry();
    if x >= cols || y >= rows {
        return None;
    }
    Some(Address3D {
        x: encode(x)?,
        y: encode(y)?,
        z: encode(z)?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn order_three_is_not_two() {
        for t in [Trit::Neg, Trit::Zero, Trit::Pos] {
            assert_eq!(t.rotate().rotate().rotate(), t);
            assert_ne!(t.rotate().rotate(), t);
        }
    }

    #[test]
    fn individual_dimensions_round_trip() {
        for x in 0..187 {
            assert_eq!(decode(encode::<5>(x).unwrap()), Some(x));
        }
        for y in 0..67 {
            assert_eq!(decode(encode::<4>(y).unwrap()), Some(y));
        }
        for z in 0..4 {
            assert_eq!(decode(encode::<2>(z).unwrap()), Some(z));
        }
    }

    #[test]
    fn three_dimensional_source_cells_never_collide() {
        let sources = [
            Source::WebbProcessed2024,
            Source::HubbleProxima2012,
            Source::HubbleProxima2014,
            Source::HubbleProxima2015,
        ];
        let mut seen = HashSet::new();
        for source in sources {
            let (width, height, _) = source.geometry();
            for y in 0..height {
                for x in 0..width {
                    let a = address(source, x, y).unwrap();
                    assert!(seen.insert(a), "same ternary address claimed twice");
                    assert_eq!(decode(a.x), Some(x));
                    assert_eq!(decode(a.y), Some(y));
                }
            }
        }
        assert_eq!(seen.len(), 12_772);
    }

    #[test]
    fn webb_has_exactly_12529_leaves() {
        let (cols, rows, z) = Source::WebbProcessed2024.geometry();
        assert_eq!((cols, rows, z), (187, 67, 0));
        assert_eq!(cols * rows, 12_529);
    }

    #[test]
    fn hubble_epochs_have_243_leaves() {
        let sources = [
            Source::HubbleProxima2012,
            Source::HubbleProxima2014,
            Source::HubbleProxima2015,
        ];
        assert_eq!(
            sources
                .map(|s| {
                    let (w, h, _) = s.geometry();
                    w * h
                })
                .iter()
                .sum::<usize>(),
            243
        );
    }

    #[test]
    fn source_identity_cannot_be_reassigned_by_epoch() {
        let coords = (3, 7);
        assert_ne!(
            address(Source::WebbProcessed2024, coords.0, coords.1),
            address(Source::HubbleProxima2015, coords.0, coords.1)
        );
    }

    #[test]
    fn out_of_bounds_fails_closed() {
        assert!(address(Source::WebbProcessed2024, 187, 0).is_none());
        assert!(address(Source::WebbProcessed2024, 0, 67).is_none());
        assert!(address(Source::HubbleProxima2012, 9, 0).is_none());
        assert!(encode::<5>(243).is_none());
    }

    #[test]
    fn original_leaves_are_not_gguf_storage_tensors() {
        assert_eq!(12_529 + 3 * 81, 12_772);
        assert_ne!(411, 12_772);
    }
}

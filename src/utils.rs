pub fn obl(tile_y: u32, tile_h: u32, spritesheet_h: u32) -> u32 {
    spritesheet_h - tile_h - tile_y
}

/// Spritesheet index
pub fn ss_idx(x: u16, y: u16) -> u16 {
    x * 16 + y
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_obl() {
        assert_eq!(obl(496, 16, 512), 0);
        assert_eq!(obl(480, 16, 512), 16);
        assert_eq!(obl(144, 16, 512), 352);
        assert_eq!(obl(192, 32, 512), 352);
    }
}

pub trait SaveBits {
    fn save_bits(&self) -> Box<dyn Iterator<Item = bool>>;
    fn from_bits(iter: &mut dyn Iterator<Item = bool>) -> Self;
}

impl SaveBits for u8 {
    fn save_bits(&self) -> Box<dyn Iterator<Item = bool>> {
        let s = *self;
        Box::new((0..8).rev().map(move |x| (1 << x) & s > 0))
    }

    fn from_bits(iter: &mut dyn Iterator<Item = bool>) -> Self {
        let mut s = 0;
        for x in (0..8).rev() {
            let is_bit_set = iter
                .next()
                .expect("Iterator returned None while loading u8");
            if is_bit_set {
                s += 2u8.pow(x);
            }
        }
        s
    }
}

impl SaveBits for usize {
    fn save_bits(&self) -> Box<dyn Iterator<Item = bool>> {
        assert!(*self > 0);
        let num_bits = std::mem::size_of::<usize>() as u32 * 8 - self.leading_zeros();
        // panic!("{}", num_bits);
        let s = *self;
        Box::new((0..2 * num_bits - 1).map(move |x| {
            // FIXME better ways to do this?
            if x < num_bits - 1 {
                false
            } else if x == num_bits - 1 {
                true
            } else {
                2usize.pow(2 * num_bits - x - 2) & s > 0
            }
        }))
    }

    fn from_bits(iter: &mut dyn Iterator<Item = bool>) -> Self {
        let mut num_bits = 0;
        let mut last_bit = false;
        while !last_bit {
            num_bits += 1;
            last_bit = iter
                .next()
                .expect("Iterator returned None while loading usize");
        }
        let mut s = 2usize.pow(num_bits - 1);
        for x in (0..num_bits - 1).rev() {
            if iter
                .next()
                .expect("Iterator returned None while loading usize")
            {
                s += 2usize.pow(x);
            }
        }

        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_bits_u8() {
        assert_eq!(
            vec![true, false, true, true, false, false, true, false],
            178u8.save_bits().collect::<Vec<_>>()
        );
        assert_eq!(
            vec![true, false, false, true, false, false, true, false],
            146u8.save_bits().collect::<Vec<_>>()
        );
        assert_eq!(vec![true; 8], 255u8.save_bits().collect::<Vec<_>>());
        assert_eq!(vec![false; 8], 0u8.save_bits().collect::<Vec<_>>());
    }

    #[test]
    fn test_to_bits_usize() {
        assert_eq!(
            vec![false, false, false, true, true, false, true],
            13usize.save_bits().collect::<Vec<_>>()
        );
        assert_eq!(vec![true], 1usize.save_bits().collect::<Vec<_>>());
        assert_eq!(
            vec![false, true, false],
            2usize.save_bits().collect::<Vec<_>>()
        );
        assert_eq!(
            vec![false, true, true],
            3usize.save_bits().collect::<Vec<_>>()
        );
        assert_eq!(
            vec![false, false, false, true, false, false, false],
            8usize.save_bits().collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_from_bits_u8() {
        assert_eq!(u8::from_bits(&mut [false; 8].iter().copied()), 0u8);
        assert_eq!(u8::from_bits(&mut [true; 8].iter().copied()), 255u8);
        assert_eq!(
            u8::from_bits(
                &mut [true, false, false, true, true, false, true, false]
                    .iter()
                    .copied()
            ),
            154u8
        );

        for k in 0..=255u8 {
            assert_eq!(k, u8::from_bits(&mut k.save_bits()));
        }
    }

    #[test]
    fn test_from_bits_usize() {
        assert_eq!(usize::from_bits(&mut [true].iter().copied()), 1usize);
        assert_eq!(
            usize::from_bits(&mut [false, true, false].iter().copied()),
            2usize
        );
        assert_eq!(
            usize::from_bits(
                &mut [false, false, false, true, true, false, true]
                    .iter()
                    .copied()
            ),
            13usize
        );

        for k in 1..1000usize {
            assert_eq!(k, usize::from_bits(&mut k.save_bits()));
        }

        for k in 100_000_000..100_000_500usize {
            assert_eq!(k, usize::from_bits(&mut k.save_bits()));
        }
    }
}

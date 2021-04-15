#[macro_export]
macro_rules! bitfield {
    ($get:ident,$set:ident,$type:ty,$bitnum:expr,$bitwidth:expr) => {
        pub fn $get(&self) -> $type {
            let mask: $type = ((1 << $bitwidth) - 1) << $bitnum;
            (self.value & mask) >> $bitnum
        }
        pub fn $set(&mut self, value: $type) {
            let mask: $type = ((1 << $bitwidth) - 1) << $bitnum;
            self.value = (self.value & !mask) | (value << $bitnum);
        }
    }
}

#[test]
fn single_bit() {
    struct Single {
        pub value: u64
    }
    impl Single {
        bitfield!(get_bit, set_bit, u64, 5, 1);
    }

    let mut test: Single = Single { value: 0 };
    assert_eq!(test.get_bit(), 0);
    test.set_bit(1u64);
    assert_eq!(test.get_bit(), 1);
    test.set_bit(0u64);
    assert_eq!(test.get_bit(), 0);
}

#[test]
fn multiple_bit() {
    struct Multi {
        pub value: u64
    }
    impl Multi {
        bitfield!(get_bits, set_bits, u64, 5, 2);
    }

    let mut test: Multi = Multi { value: 0 };
    assert_eq!(test.get_bits(), 0);
    test.set_bits(1u64);
    assert_eq!(test.get_bits(), 1);
    test.set_bits(0u64);
    assert_eq!(test.get_bits(), 0);
    test.set_bits(2u64);
    assert_eq!(test.get_bits(), 2);
    test.set_bits(3u64);
    assert_eq!(test.get_bits(), 3);
}
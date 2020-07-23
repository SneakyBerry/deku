use alloc_counter::AllocCounterSystem;
use deku::prelude::*;
use deku::ctx::Endian;

#[global_allocator]
static A: AllocCounterSystem = AllocCounterSystem;

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(ctx = "endian: Endian")]
struct NestedStruct {
    #[deku(ctx = "endian")]
    pub field_a: u8,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id_type = "u8", ctx = "endian: Endian")]
enum NestedEnum {
    #[deku(id = "0x01")]
    VarA(#[deku(ctx = "endian")] u8),
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "big")]
struct TestDeku {
    pub field_a: u8,
    pub field_b: u16,
    pub field_c: NestedStruct,
    pub field_d: NestedEnum,
    #[deku(count = "1")]
    pub field_e: Vec<u8>, // 1 alloc
    pub field_f: [u8; 3],
    #[deku(bits = "3")]
    pub field_g: u8, // 1 alloc (bits read)
    #[deku(bits = "5")]
    pub field_h: u8, // 1 alloc (bits read)
}

mod tests {

    use super::*;
    use alloc_counter::count_alloc;
    use hex_literal::hex;
    use std::convert::TryFrom;

    #[test]
    fn test_simple() {
        let input = hex!("aabbbbcc0102ddffffffaa").as_ref();

        assert_eq!(
            count_alloc(|| {
                let _ = TestDeku::try_from(input).unwrap();
            })
            .0,
            (3, 0, 3)
        );
    }
}

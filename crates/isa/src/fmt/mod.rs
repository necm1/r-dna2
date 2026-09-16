pub mod sop1;
pub mod sopp;

macro_rules! opcodes {
    ($name:ident { $($code:literal => $variant:ident,)+ }) => {
        #[derive(Debug, PartialEq, Eq, Clone, Copy)]
        pub enum $name {
            $($variant,)+
        }

        impl $name {
            pub fn decode(op: u32) -> Option<Self> {
                match op {
                    $($code => Some(Self::$variant),)+
                    _ => None,
                }
            }
        }
    };
}

pub(crate) use opcodes;

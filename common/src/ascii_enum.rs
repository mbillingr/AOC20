#[macro_export]
macro_rules! ascii_enum {
    ($typename:ident = $($variant:ident($ch:expr))|+) => {
        #[derive(Copy, Clone, Eq, PartialEq)]
        enum $typename {
            $($variant),*
        }

        impl $typename {
            pub fn from_char(ch: char) -> Self {
                match ch {
                    $($ch => $typename::$variant,)*
                    _ => panic!("Unknown cell {:?}", ch)
                }
            }

            pub fn to_char(&self) -> char {
                match self {
                    $($typename::$variant => $ch,)*
                }
            }
        }

        impl std::fmt::Debug for $typename {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}", self.to_char())
            }
        }

        impl From<char> for $typename {
            fn from(ch: char) -> Self {
                Self::from_char(ch)
            }
        }

        impl From<$typename> for char {
            fn from(this: $typename) -> char {
                this.to_char()
            }
        }
    };
}

#![no_std]

#[macro_export]
macro_rules! blob {
    ($vis:vis $name:ident) => {
        $crate::blob!($vis $name: stringify!($name));
    };
    ($vis:vis $name:ident: $link_name:expr) => {
        #[allow(non_camel_case_types)]
        #[non_exhaustive]
        $vis struct $name {}
        $vis const $name: $name = $name {};
        impl ::core::ops::Deref for $name {
            type Target = [u8];
            #[inline(always)]
            fn deref(&self) -> &[u8] {
                Self::get()
            }
        }
        impl $name {
            #[inline(always)]
            pub fn get() -> &'static [u8] {
                extern "C" {
                    #[link_name = concat!("BLOB.DATA.", $link_name)]
                    static DATA: u8;
                    #[link_name = concat!("BLOB.LEN.",  $link_name)]
                    static LEN: usize;
                }
                unsafe { core::slice::from_raw_parts(core::ptr::addr_of!(DATA), LEN) }
            }
        }
        impl AsRef<[u8]> for $name {
            #[inline(always)]
            fn as_ref(&self) -> &[u8] {
                self
            }
        }
    };
}

#[cfg(test)]
mod tests {
    blob!(FOO);

    // just get rid of the dead code warning for FOO
    #[no_mangle]
    fn foo() -> u8 {
        FOO[0]
    }
}

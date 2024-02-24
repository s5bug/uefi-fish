#[repr(transparent)]
pub struct U16String<const LEN: usize> {
    content: [u16; LEN],
}

impl<const LEN: usize> U16String<LEN> {
    pub fn from_str(data: &str) -> Option<U16String<LEN>> {
        let mut content: [u16; LEN] = [0; LEN];
        let nonnull: &mut [u16] = &mut content[0..LEN - 1];

        let mut utf16 = data.encode_utf16();
        for (ret, src) in nonnull.iter_mut().zip(&mut utf16) {
            *ret = src;
        }
        match utf16.next() {
            None => Some(U16String { content }),
            Some(_) => None, // too much string data to fit
        }
    }

    pub unsafe fn as_ptr(&self) -> *const u16 {
        self.content.as_ptr()
    }
}

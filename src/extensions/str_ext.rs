pub trait StringExt {
    fn trim_in_place(self) -> Self;
}

impl StringExt for String {
    fn trim_in_place(mut self) -> Self {
        let trimmed = self.trim();
        if trimmed.len() < self.len() {
            let len = trimmed.len();
            unsafe {
                core::ptr::copy(
                    trimmed.as_ptr(),
                    self.as_bytes_mut().as_mut_ptr(),
                    len,
                );
            }
            self.truncate(len)
        }
        self
    }
}

pub trait StrExt {
    fn is_not_blank(&self) -> bool;
}

impl<'a> StrExt for &'a str {
    fn is_not_blank(&self) -> bool {
        !self.trim().is_empty()
    }
}

impl StrExt for String {
    fn is_not_blank(&self) -> bool {
        !self.trim().is_empty()
    }
}

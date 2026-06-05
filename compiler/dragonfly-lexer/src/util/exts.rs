


pub(crate) trait BufExt {
  fn ends_with_ignore_ascii_case(&self,needle: &[u8])-> bool;
  fn starts_with_ignore_ascii_case(&self,needle: &[u8])-> bool;
}

impl BufExt for &[u8] {
  #[inline(always)]
  fn starts_with_ignore_ascii_case(&self,needle: &[u8])-> bool {
    super::starts_with_ignore_ascii_case(self,needle)
  }

  #[inline(always)]
  fn ends_with_ignore_ascii_case(&self,needle: &[u8])-> bool {
    super::ends_with_ignore_ascii_case(self,needle)
  }
}













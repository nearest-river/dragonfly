
pub(crate) mod exts;

pub(crate) const fn starts_with_ignore_ascii_case(heystack: &[u8],needle: &[u8])-> bool {
  let n=needle.len();
  if heystack.len()<n {
    return false;
  }

  let mut i=0usize;
  while i<n {
    if !heystack[i].eq_ignore_ascii_case(&needle[i]) {
      return false;
    }

    i+=1;
  }

  true
}

pub(crate) const fn ends_with_ignore_ascii_case(heystack: &[u8],needle: &[u8])-> bool {
  let m=heystack.len();
  let n=needle.len();
  if heystack.len()<n {
    return false;
  }

  let start=m-n;
  let mut i=0usize;
  while i<n {
    if !heystack[start+i].eq_ignore_ascii_case(&needle[i]) {
      return false;
    }

    i+=1;
  }


  true
}








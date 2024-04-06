// Generated by `wit-bindgen` 0.21.0. DO NOT EDIT!
// Options used:
#[derive(Clone)]
pub struct OwnedCodeBlock {
    pub language: Option<_rt::String>,
    pub source: _rt::String,
}
impl ::core::fmt::Debug for OwnedCodeBlock {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OwnedCodeBlock")
            .field("language", &self.language)
            .field("source", &self.source)
            .finish()
    }
}
#[derive(Clone)]
pub struct OwnedFrontmatter {
    pub title: Option<_rt::String>,
    pub code_block: Option<OwnedCodeBlock>,
}
impl ::core::fmt::Debug for OwnedFrontmatter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OwnedFrontmatter")
            .field("title", &self.title)
            .field("code-block", &self.code_block)
            .finish()
    }
}
#[derive(Clone)]
pub struct HtmlOutput {
    pub toc: Option<_rt::String>,
    pub content: _rt::String,
    pub frontmatter: Option<OwnedFrontmatter>,
}
impl ::core::fmt::Debug for HtmlOutput {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HtmlOutput")
            .field("toc", &self.toc)
            .field("content", &self.content)
            .field("frontmatter", &self.frontmatter)
            .finish()
    }
}
/// The set of errors which may be raised by the function
#[derive(Clone)]
pub enum HighlightError {
    Nolang,
    Nohighlighter,
    CouldNotBuildHighlighter(_rt::String),
    StringGenerationError(_rt::String),
}
impl ::core::fmt::Debug for HighlightError {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        match self {
            HighlightError::Nolang => f.debug_tuple("HighlightError::Nolang").finish(),
            HighlightError::Nohighlighter => {
                f.debug_tuple("HighlightError::Nohighlighter").finish()
            }
            HighlightError::CouldNotBuildHighlighter(e) => f
                .debug_tuple("HighlightError::CouldNotBuildHighlighter")
                .field(e)
                .finish(),
            HighlightError::StringGenerationError(e) => f
                .debug_tuple("HighlightError::StringGenerationError")
                .field(e)
                .finish(),
        }
    }
}
impl ::core::fmt::Display for HighlightError {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for HighlightError {}
#[doc(hidden)]
#[allow(non_snake_case)]
pub unsafe fn _export_process_markdown_to_html_cabi<T: Guest>(
    arg0: *mut u8,
    arg1: usize,
) -> *mut u8 {
    let len0 = arg1;
    let bytes0 = _rt::Vec::from_raw_parts(arg0.cast(), len0, len0);
    let result1 = T::process_markdown_to_html(_rt::string_lift(bytes0));
    let ptr2 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
    match result1 {
        Ok(e) => {
            *ptr2.add(0).cast::<u8>() = (0i32) as u8;
            let HtmlOutput {
                toc: toc3,
                content: content3,
                frontmatter: frontmatter3,
            } = e;
            match toc3 {
                Some(e) => {
                    *ptr2.add(4).cast::<u8>() = (1i32) as u8;
                    let vec4 = (e.into_bytes()).into_boxed_slice();
                    let ptr4 = vec4.as_ptr().cast::<u8>();
                    let len4 = vec4.len();
                    ::core::mem::forget(vec4);
                    *ptr2.add(12).cast::<usize>() = len4;
                    *ptr2.add(8).cast::<*mut u8>() = ptr4.cast_mut();
                }
                None => {
                    *ptr2.add(4).cast::<u8>() = (0i32) as u8;
                }
            };
            let vec5 = (content3.into_bytes()).into_boxed_slice();
            let ptr5 = vec5.as_ptr().cast::<u8>();
            let len5 = vec5.len();
            ::core::mem::forget(vec5);
            *ptr2.add(20).cast::<usize>() = len5;
            *ptr2.add(16).cast::<*mut u8>() = ptr5.cast_mut();
            match frontmatter3 {
                Some(e) => {
                    *ptr2.add(24).cast::<u8>() = (1i32) as u8;
                    let OwnedFrontmatter {
                        title: title6,
                        code_block: code_block6,
                    } = e;
                    match title6 {
                        Some(e) => {
                            *ptr2.add(28).cast::<u8>() = (1i32) as u8;
                            let vec7 = (e.into_bytes()).into_boxed_slice();
                            let ptr7 = vec7.as_ptr().cast::<u8>();
                            let len7 = vec7.len();
                            ::core::mem::forget(vec7);
                            *ptr2.add(36).cast::<usize>() = len7;
                            *ptr2.add(32).cast::<*mut u8>() = ptr7.cast_mut();
                        }
                        None => {
                            *ptr2.add(28).cast::<u8>() = (0i32) as u8;
                        }
                    };
                    match code_block6 {
                        Some(e) => {
                            *ptr2.add(40).cast::<u8>() = (1i32) as u8;
                            let OwnedCodeBlock {
                                language: language8,
                                source: source8,
                            } = e;
                            match language8 {
                                Some(e) => {
                                    *ptr2.add(44).cast::<u8>() = (1i32) as u8;
                                    let vec9 = (e.into_bytes()).into_boxed_slice();
                                    let ptr9 = vec9.as_ptr().cast::<u8>();
                                    let len9 = vec9.len();
                                    ::core::mem::forget(vec9);
                                    *ptr2.add(52).cast::<usize>() = len9;
                                    *ptr2.add(48).cast::<*mut u8>() = ptr9.cast_mut();
                                }
                                None => {
                                    *ptr2.add(44).cast::<u8>() = (0i32) as u8;
                                }
                            };
                            let vec10 = (source8.into_bytes()).into_boxed_slice();
                            let ptr10 = vec10.as_ptr().cast::<u8>();
                            let len10 = vec10.len();
                            ::core::mem::forget(vec10);
                            *ptr2.add(60).cast::<usize>() = len10;
                            *ptr2.add(56).cast::<*mut u8>() = ptr10.cast_mut();
                        }
                        None => {
                            *ptr2.add(40).cast::<u8>() = (0i32) as u8;
                        }
                    };
                }
                None => {
                    *ptr2.add(24).cast::<u8>() = (0i32) as u8;
                }
            };
        }
        Err(e) => {
            *ptr2.add(0).cast::<u8>() = (1i32) as u8;
            match e {
                HighlightError::Nolang => {
                    *ptr2.add(4).cast::<u8>() = (0i32) as u8;
                }
                HighlightError::Nohighlighter => {
                    *ptr2.add(4).cast::<u8>() = (1i32) as u8;
                }
                HighlightError::CouldNotBuildHighlighter(e) => {
                    *ptr2.add(4).cast::<u8>() = (2i32) as u8;
                    let vec11 = (e.into_bytes()).into_boxed_slice();
                    let ptr11 = vec11.as_ptr().cast::<u8>();
                    let len11 = vec11.len();
                    ::core::mem::forget(vec11);
                    *ptr2.add(12).cast::<usize>() = len11;
                    *ptr2.add(8).cast::<*mut u8>() = ptr11.cast_mut();
                }
                HighlightError::StringGenerationError(e) => {
                    *ptr2.add(4).cast::<u8>() = (3i32) as u8;
                    let vec12 = (e.into_bytes()).into_boxed_slice();
                    let ptr12 = vec12.as_ptr().cast::<u8>();
                    let len12 = vec12.len();
                    ::core::mem::forget(vec12);
                    *ptr2.add(12).cast::<usize>() = len12;
                    *ptr2.add(8).cast::<*mut u8>() = ptr12.cast_mut();
                }
            }
        }
    };
    ptr2
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub unsafe fn __post_return_process_markdown_to_html<T: Guest>(arg0: *mut u8) {
    let l0 = i32::from(*arg0.add(0).cast::<u8>());
    match l0 {
        0 => {
            let l1 = i32::from(*arg0.add(4).cast::<u8>());
            match l1 {
                0 => (),
                _ => {
                    let l2 = *arg0.add(8).cast::<*mut u8>();
                    let l3 = *arg0.add(12).cast::<usize>();
                    _rt::cabi_dealloc(l2, l3, 1);
                }
            }
            let l4 = *arg0.add(16).cast::<*mut u8>();
            let l5 = *arg0.add(20).cast::<usize>();
            _rt::cabi_dealloc(l4, l5, 1);
            let l6 = i32::from(*arg0.add(24).cast::<u8>());
            match l6 {
                0 => (),
                _ => {
                    let l7 = i32::from(*arg0.add(28).cast::<u8>());
                    match l7 {
                        0 => (),
                        _ => {
                            let l8 = *arg0.add(32).cast::<*mut u8>();
                            let l9 = *arg0.add(36).cast::<usize>();
                            _rt::cabi_dealloc(l8, l9, 1);
                        }
                    }
                    let l10 = i32::from(*arg0.add(40).cast::<u8>());
                    match l10 {
                        0 => (),
                        _ => {
                            let l11 = i32::from(*arg0.add(44).cast::<u8>());
                            match l11 {
                                0 => (),
                                _ => {
                                    let l12 = *arg0.add(48).cast::<*mut u8>();
                                    let l13 = *arg0.add(52).cast::<usize>();
                                    _rt::cabi_dealloc(l12, l13, 1);
                                }
                            }
                            let l14 = *arg0.add(56).cast::<*mut u8>();
                            let l15 = *arg0.add(60).cast::<usize>();
                            _rt::cabi_dealloc(l14, l15, 1);
                        }
                    }
                }
            }
        }
        _ => {
            let l16 = i32::from(*arg0.add(4).cast::<u8>());
            match l16 {
                0 => (),
                1 => (),
                2 => {
                    let l17 = *arg0.add(8).cast::<*mut u8>();
                    let l18 = *arg0.add(12).cast::<usize>();
                    _rt::cabi_dealloc(l17, l18, 1);
                }
                _ => {
                    let l19 = *arg0.add(8).cast::<*mut u8>();
                    let l20 = *arg0.add(12).cast::<usize>();
                    _rt::cabi_dealloc(l19, l20, 1);
                }
            }
        }
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub unsafe fn _export_process_markdown_to_html_with_frontmatter_cabi<T: Guest>(
    arg0: *mut u8,
    arg1: usize,
) -> *mut u8 {
    let len0 = arg1;
    let bytes0 = _rt::Vec::from_raw_parts(arg0.cast(), len0, len0);
    let result1 = T::process_markdown_to_html_with_frontmatter(_rt::string_lift(bytes0));
    let ptr2 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
    match result1 {
        Ok(e) => {
            *ptr2.add(0).cast::<u8>() = (0i32) as u8;
            let HtmlOutput {
                toc: toc3,
                content: content3,
                frontmatter: frontmatter3,
            } = e;
            match toc3 {
                Some(e) => {
                    *ptr2.add(4).cast::<u8>() = (1i32) as u8;
                    let vec4 = (e.into_bytes()).into_boxed_slice();
                    let ptr4 = vec4.as_ptr().cast::<u8>();
                    let len4 = vec4.len();
                    ::core::mem::forget(vec4);
                    *ptr2.add(12).cast::<usize>() = len4;
                    *ptr2.add(8).cast::<*mut u8>() = ptr4.cast_mut();
                }
                None => {
                    *ptr2.add(4).cast::<u8>() = (0i32) as u8;
                }
            };
            let vec5 = (content3.into_bytes()).into_boxed_slice();
            let ptr5 = vec5.as_ptr().cast::<u8>();
            let len5 = vec5.len();
            ::core::mem::forget(vec5);
            *ptr2.add(20).cast::<usize>() = len5;
            *ptr2.add(16).cast::<*mut u8>() = ptr5.cast_mut();
            match frontmatter3 {
                Some(e) => {
                    *ptr2.add(24).cast::<u8>() = (1i32) as u8;
                    let OwnedFrontmatter {
                        title: title6,
                        code_block: code_block6,
                    } = e;
                    match title6 {
                        Some(e) => {
                            *ptr2.add(28).cast::<u8>() = (1i32) as u8;
                            let vec7 = (e.into_bytes()).into_boxed_slice();
                            let ptr7 = vec7.as_ptr().cast::<u8>();
                            let len7 = vec7.len();
                            ::core::mem::forget(vec7);
                            *ptr2.add(36).cast::<usize>() = len7;
                            *ptr2.add(32).cast::<*mut u8>() = ptr7.cast_mut();
                        }
                        None => {
                            *ptr2.add(28).cast::<u8>() = (0i32) as u8;
                        }
                    };
                    match code_block6 {
                        Some(e) => {
                            *ptr2.add(40).cast::<u8>() = (1i32) as u8;
                            let OwnedCodeBlock {
                                language: language8,
                                source: source8,
                            } = e;
                            match language8 {
                                Some(e) => {
                                    *ptr2.add(44).cast::<u8>() = (1i32) as u8;
                                    let vec9 = (e.into_bytes()).into_boxed_slice();
                                    let ptr9 = vec9.as_ptr().cast::<u8>();
                                    let len9 = vec9.len();
                                    ::core::mem::forget(vec9);
                                    *ptr2.add(52).cast::<usize>() = len9;
                                    *ptr2.add(48).cast::<*mut u8>() = ptr9.cast_mut();
                                }
                                None => {
                                    *ptr2.add(44).cast::<u8>() = (0i32) as u8;
                                }
                            };
                            let vec10 = (source8.into_bytes()).into_boxed_slice();
                            let ptr10 = vec10.as_ptr().cast::<u8>();
                            let len10 = vec10.len();
                            ::core::mem::forget(vec10);
                            *ptr2.add(60).cast::<usize>() = len10;
                            *ptr2.add(56).cast::<*mut u8>() = ptr10.cast_mut();
                        }
                        None => {
                            *ptr2.add(40).cast::<u8>() = (0i32) as u8;
                        }
                    };
                }
                None => {
                    *ptr2.add(24).cast::<u8>() = (0i32) as u8;
                }
            };
        }
        Err(e) => {
            *ptr2.add(0).cast::<u8>() = (1i32) as u8;
            match e {
                HighlightError::Nolang => {
                    *ptr2.add(4).cast::<u8>() = (0i32) as u8;
                }
                HighlightError::Nohighlighter => {
                    *ptr2.add(4).cast::<u8>() = (1i32) as u8;
                }
                HighlightError::CouldNotBuildHighlighter(e) => {
                    *ptr2.add(4).cast::<u8>() = (2i32) as u8;
                    let vec11 = (e.into_bytes()).into_boxed_slice();
                    let ptr11 = vec11.as_ptr().cast::<u8>();
                    let len11 = vec11.len();
                    ::core::mem::forget(vec11);
                    *ptr2.add(12).cast::<usize>() = len11;
                    *ptr2.add(8).cast::<*mut u8>() = ptr11.cast_mut();
                }
                HighlightError::StringGenerationError(e) => {
                    *ptr2.add(4).cast::<u8>() = (3i32) as u8;
                    let vec12 = (e.into_bytes()).into_boxed_slice();
                    let ptr12 = vec12.as_ptr().cast::<u8>();
                    let len12 = vec12.len();
                    ::core::mem::forget(vec12);
                    *ptr2.add(12).cast::<usize>() = len12;
                    *ptr2.add(8).cast::<*mut u8>() = ptr12.cast_mut();
                }
            }
        }
    };
    ptr2
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub unsafe fn __post_return_process_markdown_to_html_with_frontmatter<T: Guest>(arg0: *mut u8) {
    let l0 = i32::from(*arg0.add(0).cast::<u8>());
    match l0 {
        0 => {
            let l1 = i32::from(*arg0.add(4).cast::<u8>());
            match l1 {
                0 => (),
                _ => {
                    let l2 = *arg0.add(8).cast::<*mut u8>();
                    let l3 = *arg0.add(12).cast::<usize>();
                    _rt::cabi_dealloc(l2, l3, 1);
                }
            }
            let l4 = *arg0.add(16).cast::<*mut u8>();
            let l5 = *arg0.add(20).cast::<usize>();
            _rt::cabi_dealloc(l4, l5, 1);
            let l6 = i32::from(*arg0.add(24).cast::<u8>());
            match l6 {
                0 => (),
                _ => {
                    let l7 = i32::from(*arg0.add(28).cast::<u8>());
                    match l7 {
                        0 => (),
                        _ => {
                            let l8 = *arg0.add(32).cast::<*mut u8>();
                            let l9 = *arg0.add(36).cast::<usize>();
                            _rt::cabi_dealloc(l8, l9, 1);
                        }
                    }
                    let l10 = i32::from(*arg0.add(40).cast::<u8>());
                    match l10 {
                        0 => (),
                        _ => {
                            let l11 = i32::from(*arg0.add(44).cast::<u8>());
                            match l11 {
                                0 => (),
                                _ => {
                                    let l12 = *arg0.add(48).cast::<*mut u8>();
                                    let l13 = *arg0.add(52).cast::<usize>();
                                    _rt::cabi_dealloc(l12, l13, 1);
                                }
                            }
                            let l14 = *arg0.add(56).cast::<*mut u8>();
                            let l15 = *arg0.add(60).cast::<usize>();
                            _rt::cabi_dealloc(l14, l15, 1);
                        }
                    }
                }
            }
        }
        _ => {
            let l16 = i32::from(*arg0.add(4).cast::<u8>());
            match l16 {
                0 => (),
                1 => (),
                2 => {
                    let l17 = *arg0.add(8).cast::<*mut u8>();
                    let l18 = *arg0.add(12).cast::<usize>();
                    _rt::cabi_dealloc(l17, l18, 1);
                }
                _ => {
                    let l19 = *arg0.add(8).cast::<*mut u8>();
                    let l20 = *arg0.add(12).cast::<usize>();
                    _rt::cabi_dealloc(l19, l20, 1);
                }
            }
        }
    }
}
pub trait Guest {
    fn process_markdown_to_html(input: _rt::String) -> Result<HtmlOutput, HighlightError>;
    fn process_markdown_to_html_with_frontmatter(
        input: _rt::String,
    ) -> Result<HtmlOutput, HighlightError>;
}
#[doc(hidden)]

macro_rules! __export_world_femark_cabi{
      ($ty:ident with_types_in $($path_to_types:tt)*) => (const _: () = {

        #[export_name = "process-markdown-to-html"]
        unsafe extern "C" fn export_process_markdown_to_html(arg0: *mut u8,arg1: usize,) -> *mut u8 {
          $($path_to_types)*::_export_process_markdown_to_html_cabi::<$ty>(arg0, arg1)
        }
        #[export_name = "cabi_post_process-markdown-to-html"]
        unsafe extern "C" fn _post_return_process_markdown_to_html(arg0: *mut u8,) {
          $($path_to_types)*::__post_return_process_markdown_to_html::<$ty>(arg0)
        }
        #[export_name = "process-markdown-to-html-with-frontmatter"]
        unsafe extern "C" fn export_process_markdown_to_html_with_frontmatter(arg0: *mut u8,arg1: usize,) -> *mut u8 {
          $($path_to_types)*::_export_process_markdown_to_html_with_frontmatter_cabi::<$ty>(arg0, arg1)
        }
        #[export_name = "cabi_post_process-markdown-to-html-with-frontmatter"]
        unsafe extern "C" fn _post_return_process_markdown_to_html_with_frontmatter(arg0: *mut u8,) {
          $($path_to_types)*::__post_return_process_markdown_to_html_with_frontmatter::<$ty>(arg0)
        }
      };);
    }
#[doc(hidden)]
pub(crate) use __export_world_femark_cabi;
#[repr(align(4))]
struct _RetArea([::core::mem::MaybeUninit<u8>; 64]);
static mut _RET_AREA: _RetArea = _RetArea([::core::mem::MaybeUninit::uninit(); 64]);
mod _rt {
    pub use alloc_crate::string::String;
    pub use alloc_crate::vec::Vec;
    pub unsafe fn string_lift(bytes: Vec<u8>) -> String {
        if cfg!(debug_assertions) {
            String::from_utf8(bytes).unwrap()
        } else {
            String::from_utf8_unchecked(bytes)
        }
    }
    pub unsafe fn cabi_dealloc(ptr: *mut u8, size: usize, align: usize) {
        if size == 0 {
            return;
        }
        let layout = alloc::Layout::from_size_align_unchecked(size, align);
        alloc::dealloc(ptr as *mut u8, layout);
    }
    extern crate alloc as alloc_crate;
    pub use alloc_crate::alloc;
}

/// Generates `#[no_mangle]` functions to export the specified type as the
/// root implementation of all generated traits.
///
/// For more information see the documentation of `wit_bindgen::generate!`.
///
/// ```rust
/// # macro_rules! export{ ($($t:tt)*) => (); }
/// # trait Guest {}
/// struct MyType;
///
/// impl Guest for MyType {
///     // ...
/// }
///
/// export!(MyType);
/// ```
#[allow(unused_macros)]
#[doc(hidden)]

macro_rules! __export_femark_impl {
      ($ty:ident) => (self::export!($ty with_types_in self););
      ($ty:ident with_types_in $($path_to_types_root:tt)*) => (
      $($path_to_types_root)*::__export_world_femark_cabi!($ty with_types_in $($path_to_types_root)*);
      )
    }
#[doc(inline)]
pub(crate) use __export_femark_impl as export;

#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:wit-bindgen:0.21.0:femark:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 500] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07\xf7\x02\x01A\x02\x01\
A\x0f\x01ks\x01r\x02\x08language\0\x06sources\x03\0\x10owned-code-block\x03\0\x01\
\x01k\x02\x01r\x02\x05title\0\x0acode-block\x03\x03\0\x11owned-frontmatter\x03\0\
\x04\x01k\x05\x01r\x03\x03toc\0\x07contents\x0bfrontmatter\x06\x03\0\x0bhtml-out\
put\x03\0\x07\x01q\x04\x06nolang\0\0\x0dnohighlighter\0\0\x1bcould-not-build-hig\
hlighter\x01s\0\x17string-generation-error\x01s\0\x03\0\x0fhighlight-error\x03\0\
\x09\x01j\x01\x08\x01\x0a\x01@\x01\x05inputs\0\x0b\x04\0\x18process-markdown-to-\
html\x01\x0c\x04\0)process-markdown-to-html-with-frontmatter\x01\x0c\x04\x01\x14\
benwis:femark/femark\x04\0\x0b\x0c\x01\0\x06femark\x03\0\0\0G\x09producers\x01\x0c\
processed-by\x02\x0dwit-component\x070.201.0\x10wit-bindgen-rust\x060.21.0";

#[inline(never)]
#[doc(hidden)]
#[cfg(target_arch = "wasm32")]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}

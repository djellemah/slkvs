// Generated by `wit-bindgen` 0.16.0. DO NOT EDIT!
pub mod exports {
  pub mod golem {
    pub mod component {
      
      #[allow(clippy::all)]
      pub mod api {
        #[used]
        #[doc(hidden)]
        #[cfg(target_arch = "wasm32")]
        static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_section;
        const _: () = {
          
          #[doc(hidden)]
          #[export_name = "golem:component/api#add"]
          #[allow(non_snake_case)]
          unsafe extern "C" fn __export_add(arg0: i32,arg1: i32,arg2: i32,arg3: i32,) {
            #[allow(unused_imports)]
            use wit_bindgen::rt::{alloc, vec::Vec, string::String};
            
            // Before executing any other code, use this function to run all static
            // constructors, if they have not yet been run. This is a hack required
            // to work around wasi-libc ctors calling import functions to initialize
            // the environment.
            //
            // This functionality will be removed once rust 1.69.0 is stable, at which
            // point wasi-libc will no longer have this behavior.
            //
            // See
            // https://github.com/bytecodealliance/preview2-prototyping/issues/99
            // for more details.
            #[cfg(target_arch="wasm32")]
            wit_bindgen::rt::run_ctors_once();
            
            let len0 = arg1 as usize;
            let bytes0 = Vec::from_raw_parts(arg0 as *mut _, len0, len0);
            let len1 = arg3 as usize;
            let bytes1 = Vec::from_raw_parts(arg2 as *mut _, len1, len1);
            <_GuestImpl as Guest>::add(wit_bindgen::rt::string_lift(bytes0), wit_bindgen::rt::string_lift(bytes1));
          }
        };
        const _: () = {
          
          #[doc(hidden)]
          #[export_name = "golem:component/api#get"]
          #[allow(non_snake_case)]
          unsafe extern "C" fn __export_get(arg0: i32,arg1: i32,) -> i32 {
            #[allow(unused_imports)]
            use wit_bindgen::rt::{alloc, vec::Vec, string::String};
            
            // Before executing any other code, use this function to run all static
            // constructors, if they have not yet been run. This is a hack required
            // to work around wasi-libc ctors calling import functions to initialize
            // the environment.
            //
            // This functionality will be removed once rust 1.69.0 is stable, at which
            // point wasi-libc will no longer have this behavior.
            //
            // See
            // https://github.com/bytecodealliance/preview2-prototyping/issues/99
            // for more details.
            #[cfg(target_arch="wasm32")]
            wit_bindgen::rt::run_ctors_once();
            
            let len0 = arg1 as usize;
            let bytes0 = Vec::from_raw_parts(arg0 as *mut _, len0, len0);
            let result1 = <_GuestImpl as Guest>::get(wit_bindgen::rt::string_lift(bytes0));
            let ptr2 = _RET_AREA.0.as_mut_ptr() as i32;
            match result1 {
              Some(e) => {
                *((ptr2 + 0) as *mut u8) = (1i32) as u8;
                let vec3 = (e.into_bytes()).into_boxed_slice();
                let ptr3 = vec3.as_ptr() as i32;
                let len3 = vec3.len() as i32;
                ::core::mem::forget(vec3);
                *((ptr2 + 8) as *mut i32) = len3;
                *((ptr2 + 4) as *mut i32) = ptr3;
              },
              None => {
                {
                  *((ptr2 + 0) as *mut u8) = (0i32) as u8;
                }
              },
            };ptr2
          }
          
          const _: () = {
            #[doc(hidden)]
            #[export_name = "cabi_post_golem:component/api#get"]
            #[allow(non_snake_case)]
            unsafe extern "C" fn __post_return_get(arg0: i32,) {
              let l0 = i32::from(*((arg0 + 0) as *const u8));
              match l0 {
                0 => (),
                _ => {
                  let l1 = *((arg0 + 4) as *const i32);
                  let l2 = *((arg0 + 8) as *const i32);
                  wit_bindgen::rt::dealloc(l1, (l2) as usize, 1);
                },
              }
            }
          };
        };
        const _: () = {
          
          #[doc(hidden)]
          #[export_name = "golem:component/api#listpaths"]
          #[allow(non_snake_case)]
          unsafe extern "C" fn __export_listpaths() -> i32 {
            #[allow(unused_imports)]
            use wit_bindgen::rt::{alloc, vec::Vec, string::String};
            
            // Before executing any other code, use this function to run all static
            // constructors, if they have not yet been run. This is a hack required
            // to work around wasi-libc ctors calling import functions to initialize
            // the environment.
            //
            // This functionality will be removed once rust 1.69.0 is stable, at which
            // point wasi-libc will no longer have this behavior.
            //
            // See
            // https://github.com/bytecodealliance/preview2-prototyping/issues/99
            // for more details.
            #[cfg(target_arch="wasm32")]
            wit_bindgen::rt::run_ctors_once();
            
            let result0 = <_GuestImpl as Guest>::listpaths();
            let ptr1 = _RET_AREA.0.as_mut_ptr() as i32;
            let vec3 = result0;
            let len3 = vec3.len() as i32;
            let layout3 = alloc::Layout::from_size_align_unchecked(vec3.len() * 8, 4);
            let result3 = if layout3.size() != 0
            {
              let ptr = alloc::alloc(layout3);
              if ptr.is_null()
              {
                alloc::handle_alloc_error(layout3);
              }
              ptr
            }else {{
              ::core::ptr::null_mut()
            }};
            for (i, e) in vec3.into_iter().enumerate() {
              let base = result3 as i32 + (i as i32) * 8;
              {
                let vec2 = (e.into_bytes()).into_boxed_slice();
                let ptr2 = vec2.as_ptr() as i32;
                let len2 = vec2.len() as i32;
                ::core::mem::forget(vec2);
                *((base + 4) as *mut i32) = len2;
                *((base + 0) as *mut i32) = ptr2;
              }
            }
            *((ptr1 + 4) as *mut i32) = len3;
            *((ptr1 + 0) as *mut i32) = result3 as i32;
            ptr1
          }
          
          const _: () = {
            #[doc(hidden)]
            #[export_name = "cabi_post_golem:component/api#listpaths"]
            #[allow(non_snake_case)]
            unsafe extern "C" fn __post_return_listpaths(arg0: i32,) {
              let l2 = *((arg0 + 0) as *const i32);
              let l3 = *((arg0 + 4) as *const i32);
              let base4 = l2;
              let len4 = l3;
              for i in 0..len4 {
                let base = base4 + i *8;
                {
                  let l0 = *((base + 0) as *const i32);
                  let l1 = *((base + 4) as *const i32);
                  wit_bindgen::rt::dealloc(l0, (l1) as usize, 1);
                }
              }
              wit_bindgen::rt::dealloc(base4, (len4 as usize) * 8, 4);
            }
          };
        };
        const _: () = {
          
          #[doc(hidden)]
          #[export_name = "golem:component/api#addtree"]
          #[allow(non_snake_case)]
          unsafe extern "C" fn __export_addtree(arg0: i32,arg1: i32,arg2: i32,arg3: i32,) {
            #[allow(unused_imports)]
            use wit_bindgen::rt::{alloc, vec::Vec, string::String};
            
            // Before executing any other code, use this function to run all static
            // constructors, if they have not yet been run. This is a hack required
            // to work around wasi-libc ctors calling import functions to initialize
            // the environment.
            //
            // This functionality will be removed once rust 1.69.0 is stable, at which
            // point wasi-libc will no longer have this behavior.
            //
            // See
            // https://github.com/bytecodealliance/preview2-prototyping/issues/99
            // for more details.
            #[cfg(target_arch="wasm32")]
            wit_bindgen::rt::run_ctors_once();
            
            let len0 = arg1 as usize;
            let bytes0 = Vec::from_raw_parts(arg0 as *mut _, len0, len0);
            let len1 = arg3 as usize;
            let bytes1 = Vec::from_raw_parts(arg2 as *mut _, len1, len1);
            <_GuestImpl as Guest>::addtree(wit_bindgen::rt::string_lift(bytes0), wit_bindgen::rt::string_lift(bytes1));
          }
        };
        const _: () = {
          
          #[doc(hidden)]
          #[export_name = "golem:component/api#delete"]
          #[allow(non_snake_case)]
          unsafe extern "C" fn __export_delete(arg0: i32,arg1: i32,) {
            #[allow(unused_imports)]
            use wit_bindgen::rt::{alloc, vec::Vec, string::String};
            
            // Before executing any other code, use this function to run all static
            // constructors, if they have not yet been run. This is a hack required
            // to work around wasi-libc ctors calling import functions to initialize
            // the environment.
            //
            // This functionality will be removed once rust 1.69.0 is stable, at which
            // point wasi-libc will no longer have this behavior.
            //
            // See
            // https://github.com/bytecodealliance/preview2-prototyping/issues/99
            // for more details.
            #[cfg(target_arch="wasm32")]
            wit_bindgen::rt::run_ctors_once();
            
            let len0 = arg1 as usize;
            let bytes0 = Vec::from_raw_parts(arg0 as *mut _, len0, len0);
            <_GuestImpl as Guest>::delete(wit_bindgen::rt::string_lift(bytes0));
          }
        };
        const _: () = {
          
          #[doc(hidden)]
          #[export_name = "golem:component/api#crash"]
          #[allow(non_snake_case)]
          unsafe extern "C" fn __export_crash() {
            #[allow(unused_imports)]
            use wit_bindgen::rt::{alloc, vec::Vec, string::String};
            
            // Before executing any other code, use this function to run all static
            // constructors, if they have not yet been run. This is a hack required
            // to work around wasi-libc ctors calling import functions to initialize
            // the environment.
            //
            // This functionality will be removed once rust 1.69.0 is stable, at which
            // point wasi-libc will no longer have this behavior.
            //
            // See
            // https://github.com/bytecodealliance/preview2-prototyping/issues/99
            // for more details.
            #[cfg(target_arch="wasm32")]
            wit_bindgen::rt::run_ctors_once();
            
            <_GuestImpl as Guest>::crash();
          }
        };
        use super::super::super::super::super::Component as _GuestImpl;
        pub trait Guest {
          fn add(path: wit_bindgen::rt::string::String,value: wit_bindgen::rt::string::String,);
          fn get(path: wit_bindgen::rt::string::String,) -> Option<wit_bindgen::rt::string::String>;
          fn listpaths() -> wit_bindgen::rt::vec::Vec::<wit_bindgen::rt::string::String>;
          fn addtree(path: wit_bindgen::rt::string::String,json: wit_bindgen::rt::string::String,);
          fn delete(path: wit_bindgen::rt::string::String,);
          fn crash();
        }
        
        #[allow(unused_imports)]
        use wit_bindgen::rt::{alloc, vec::Vec, string::String};
        
        #[repr(align(4))]
        struct _RetArea([u8; 12]);
        static mut _RET_AREA: _RetArea = _RetArea([0; 12]);
        
      }
      
    }
  }
}

#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:yoyo"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 643] = [3, 0, 4, 121, 111, 121, 111, 0, 97, 115, 109, 13, 0, 1, 0, 7, 166, 1, 1, 65, 2, 1, 66, 14, 1, 64, 2, 4, 112, 97, 116, 104, 115, 5, 118, 97, 108, 117, 101, 115, 1, 0, 4, 0, 3, 97, 100, 100, 1, 0, 1, 107, 115, 1, 64, 1, 4, 112, 97, 116, 104, 115, 0, 1, 4, 0, 3, 103, 101, 116, 1, 2, 1, 112, 115, 1, 64, 0, 0, 3, 4, 0, 9, 108, 105, 115, 116, 112, 97, 116, 104, 115, 1, 4, 1, 64, 2, 4, 112, 97, 116, 104, 115, 4, 106, 115, 111, 110, 115, 1, 0, 4, 0, 7, 97, 100, 100, 116, 114, 101, 101, 1, 5, 1, 64, 1, 4, 112, 97, 116, 104, 115, 1, 0, 4, 0, 6, 100, 101, 108, 101, 116, 101, 1, 6, 1, 64, 0, 1, 0, 4, 0, 5, 99, 114, 97, 115, 104, 1, 7, 4, 1, 19, 103, 111, 108, 101, 109, 58, 99, 111, 109, 112, 111, 110, 101, 110, 116, 47, 97, 112, 105, 5, 0, 11, 9, 1, 0, 3, 97, 112, 105, 3, 0, 0, 7, 194, 1, 1, 65, 2, 1, 65, 2, 1, 66, 14, 1, 64, 2, 4, 112, 97, 116, 104, 115, 5, 118, 97, 108, 117, 101, 115, 1, 0, 4, 0, 3, 97, 100, 100, 1, 0, 1, 107, 115, 1, 64, 1, 4, 112, 97, 116, 104, 115, 0, 1, 4, 0, 3, 103, 101, 116, 1, 2, 1, 112, 115, 1, 64, 0, 0, 3, 4, 0, 9, 108, 105, 115, 116, 112, 97, 116, 104, 115, 1, 4, 1, 64, 2, 4, 112, 97, 116, 104, 115, 4, 106, 115, 111, 110, 115, 1, 0, 4, 0, 7, 97, 100, 100, 116, 114, 101, 101, 1, 5, 1, 64, 1, 4, 112, 97, 116, 104, 115, 1, 0, 4, 0, 6, 100, 101, 108, 101, 116, 101, 1, 6, 1, 64, 0, 1, 0, 4, 0, 5, 99, 114, 97, 115, 104, 1, 7, 4, 1, 19, 103, 111, 108, 101, 109, 58, 99, 111, 109, 112, 111, 110, 101, 110, 116, 47, 97, 112, 105, 5, 0, 4, 1, 20, 103, 111, 108, 101, 109, 58, 99, 111, 109, 112, 111, 110, 101, 110, 116, 47, 121, 111, 121, 111, 4, 0, 11, 10, 1, 0, 4, 121, 111, 121, 111, 3, 2, 0, 0, 164, 1, 12, 112, 97, 99, 107, 97, 103, 101, 45, 100, 111, 99, 115, 0, 123, 34, 105, 110, 116, 101, 114, 102, 97, 99, 101, 115, 34, 58, 123, 34, 97, 112, 105, 34, 58, 123, 34, 100, 111, 99, 115, 34, 58, 34, 83, 101, 101, 32, 104, 116, 116, 112, 115, 58, 47, 47, 103, 105, 116, 104, 117, 98, 46, 99, 111, 109, 47, 87, 101, 98, 65, 115, 115, 101, 109, 98, 108, 121, 47, 99, 111, 109, 112, 111, 110, 101, 110, 116, 45, 109, 111, 100, 101, 108, 47, 98, 108, 111, 98, 47, 109, 97, 105, 110, 47, 100, 101, 115, 105, 103, 110, 47, 109, 118, 112, 47, 87, 73, 84, 46, 109, 100, 32, 102, 111, 114, 32, 109, 111, 114, 101, 32, 100, 101, 116, 97, 105, 108, 115, 32, 97, 98, 111, 117, 116, 32, 116, 104, 101, 32, 87, 73, 84, 32, 115, 121, 110, 116, 97, 120, 34, 125, 125, 125, 0, 70, 9, 112, 114, 111, 100, 117, 99, 101, 114, 115, 1, 12, 112, 114, 111, 99, 101, 115, 115, 101, 100, 45, 98, 121, 2, 13, 119, 105, 116, 45, 99, 111, 109, 112, 111, 110, 101, 110, 116, 6, 48, 46, 49, 56, 46, 50, 16, 119, 105, 116, 45, 98, 105, 110, 100, 103, 101, 110, 45, 114, 117, 115, 116, 6, 48, 46, 49, 54, 46, 48];

#[inline(never)]
#[doc(hidden)]
#[cfg(target_arch = "wasm32")]
pub fn __link_section() {}

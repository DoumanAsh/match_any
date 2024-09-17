//! Macro to match Any pointer by type

#![warn(missing_docs)]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::style))]

#[macro_export]
///Matches any expression by type.
///
///Implementation attempts to find possible type match visiting it on success otherwise defaulting to `_` branch
///
///## Usage
///
///```rust
///use core::any::Any;
///use visit_any::visit_any;
///
///pub fn check_type(ptr: &dyn core::any::Any) -> &'static str {
///    visit_any!(ptr => {
///        _value @ String => {
///            "string"
///        },
///        _value @ () => {
///            "unit"
///        },
///        _ => {
///            "unknown"
///        },
///    })
///}
///
///let mut value = "test".to_owned();
///let value = check_type(&value);
///
///assert_eq!(value, "string");
///
///let mut value = ();
///let value = check_type(&value);
///
///assert_eq!(value, "unit");
///
///let mut value = 1;
///let value = check_type(&value);
///
///assert_eq!(value, "unknown");
///```
macro_rules! visit_any {
    (
        $ptr:expr => {
            $($val:ident @ $typ:ty => $code:block),* $(,)?
            _ => $default:block$(,)?
        }
    ) => {{
        let _ptr = $ptr;
        $( if let Some($val) = _ptr.downcast_ref::<$typ>() $code else)*
        $default
    }}
}

#[macro_export]
///Matches any expression by type.
///
///Implementation attempts to find possible type match visiting it on success otherwise defaulting to `_` branch
///
///## Usage
///
///```rust
///use visit_any::visit_any_mut;
///
///pub fn check_type(ptr: &mut dyn core::any::Any) -> &'static str {
///    visit_any_mut!(ptr => {
///        _value @ String => {
///            "string"
///        },
///        _value @ () => {
///            "unit"
///        },
///        _ => {
///            "unknown"
///        },
///    })
///}
///
///let mut value = "test".to_owned();
///let value = check_type(&mut value);
///
///assert_eq!(value, "string");
///
///let mut value = ();
///let value = check_type(&mut value);
///
///assert_eq!(value, "unit");
///
///let mut value = 1;
///let value = check_type(&mut value);
///
///assert_eq!(value, "unknown");
///```
macro_rules! visit_any_mut {
    (
        $ptr:expr => {
            $($val:ident @ $typ:ty => $code:block),* $(,)?
            _ => $default:block$(,)?
        }
    ) => {{
        let _ptr = $ptr;
        $( if let Some($val) = _ptr.downcast_mut::<$typ>() $code else)*
        $default
    }}
}

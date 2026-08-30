//! Shared declarative macros used across this crate's modules.

/// Generates a single chainable "with"-style builder setter method, with a fully
/// auto-generated docstring skeleton (summary, `# Parameters`, `# Returns`) built around one
/// user-supplied clause describing the parameter.
///
/// `name: Type` generates a method that takes `Type` by value, converts it with `.into()`,
/// assigns the result into the same-named field on `self`, and returns `self`. Append
/// `=> field.path` to target a differently-named or nested field instead of `self.name`.
///
/// The target field path is written without a leading `self` — the macro supplies it — since
/// a caller's own `self` token can't be made to refer to the `self` parameter the macro
/// generates (they carry different hygiene contexts, so `self.foo` written at the call site
/// fails to resolve against a `self` introduced by the macro definition).
///
/// Usually invoked in bulk through [`builder_fns`] rather than directly.
///
/// # Parameters
/// - Doc comment(s) on the invocation are the clause completing "- `$name`: ...`" under the
///   generated `# Parameters` section; the rest of the docstring is generated automatically
///   and should not be written by hand.
/// - `$name`: the method (and, unless `$field` is given, the target field) name; also the
///   name of the method's single parameter, taken by value as `$ty`.
/// - `$field`: optional dot-separated path (relative to `self`) of the field to assign, when
///   it differs from `$name`.
macro_rules! builder_fn {
    ($(#[$clause:meta])* $vis:vis $name:ident: $ty:ty) => {
        $crate::macros::builder_fn!($(#[$clause])* $vis $name: $ty => $name);
    };
    ($(#[$clause:meta])* $vis:vis $name:ident: $ty:ty => $($field:ident).+) => {
        #[doc = concat!("Sets the `", stringify!($name), "`.")]
        #[doc = ""]
        #[doc = "# Parameters"]
        #[doc = concat!("- `", stringify!($name), "`:")]
        $(#[$clause])*
        #[doc = ""]
        /// # Returns
        /// `Self`, for method chaining.
        $vis fn $name(mut self, $name: $ty) -> Self {
            self.$($field).+ = $name.into();
            self
        }
    };
}

/// Generates a full `impl` block of chainable builder setters (see [`builder_fn`]) from one
/// field-spec list, so a type's setters don't need one macro invocation each.
///
/// # Parameters
/// - `impl<generics> Type<targs>`: the impl header, written exactly as it would be for a
///   hand-written `impl` block, with a single-bound-per-param restriction: each generic
///   param is `name` or `name: Bound` (a trailing `where` clause, or a param with more than
///   one bound, isn't supported).
/// - A comma-separated list of field specs, each in [`builder_fn`]'s
///   `[doc] vis name: Type [=> field.path]` form.
macro_rules! builder_fns {
    (
        impl $(<$($gparam:tt $(: $gbound:path)?),+>)? $tyname:ident $(<$($targ:tt),+>)? {
            $($(#[$clause:meta])* $vis:vis $name:ident: $fty:ty $(=> $($field:ident).+)?),* $(,)?
        }
    ) => {
        impl $(<$($gparam $(: $gbound)?),+>)? $tyname $(<$($targ),+>)? {
            $(
                $crate::macros::builder_fn!($(#[$clause])* $vis $name: $fty $(=> $($field).+)?);
            )*
        }
    };
}

pub(crate) use builder_fn;
pub(crate) use builder_fns;

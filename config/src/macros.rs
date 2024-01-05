/// A macro to implement converters from a type to [`Config`] and [`figment::Figment`]
///
/// This can be used to remove some boilerplate code that's necessary to add additional layer(s) to
/// the [`Config`]'s default `Figment`.
///
/// `impl_figment` takes the default `Config` and merges additional `Provider`, therefore the
/// targeted type, requires an implementation of `figment::Profile`.
///
/// # Example
///
/// Use `impl_figment` on a type with a `root: Option<PathBuf>` field, which will be used for
/// [`Config::figment_with_root()`]
///
/// ```rust
/// use std::path::PathBuf;
/// use serde::Serialize;
/// use foxar_config::{Config, impl_figment_convert};
/// use foxar_config::figment::*;
/// use foxar_config::figment::error::Kind::InvalidType;
/// use foxar_config::figment::value::*;
/// #[derive(Default, Serialize)]
/// struct MyArgs {
///     #[serde(skip_serializing_if = "Option::is_none")]
///     root: Option<PathBuf>,
/// }
/// impl_figment_convert!(MyArgs);
///
/// impl Provider for MyArgs {
///     fn metadata(&self) -> Metadata {
///         Metadata::default()
///     }
///
///     fn data(&self) -> Result<Map<Profile, Dict>, Error> {
///         let value = Value::serialize(self)?;
///         let error = InvalidType(value.to_actual(), "map".into());
///         let mut dict = value.into_dict().ok_or(error)?;
///         Ok(Map::from([(Config::selected_profile(), dict)]))
///     }
/// }
///
/// let figment: Figment = From::from(&MyArgs::default());
/// let config: Config = From::from(&MyArgs::default());
///
///  // Use `impl_figment` on a type that has several nested `Provider` as fields but is _not_ a `Provider` itself
///
/// #[derive(Default)]
/// struct Outer {
///     start: MyArgs,
///     second: MyArgs,
///     third: MyArgs,
/// }
/// impl_figment_convert!(Outer, start, second, third);
///
/// let figment: Figment = From::from(&Outer::default());
/// let config: Config = From::from(&Outer::default());
/// ```
#[macro_export]
macro_rules! impl_figment_convert {
    ($name:ty) => {
        impl<'a> From<&'a $name> for $crate::figment::Figment {
            fn from(args: &'a $name) -> Self {
                if let Some(root) = args.root.clone() {
                    $crate::Config::figment_with_root(root)
                } else {
                    $crate::Config::figment_with_root($crate::find_project_root_path().unwrap())
                }
                .merge(args)
            }
        }

        impl<'a> From<&'a $name> for $crate::Config {
            fn from(args: &'a $name) -> Self {
                let figment: $crate::figment::Figment = args.into();
                $crate::Config::from_provider(figment).sanitized()
            }
        }
    };
    ($name:ty, $start:ident $(, $more:ident)*) => {
        impl<'a> From<&'a $name> for $crate::figment::Figment {
            fn from(args: &'a $name) -> Self {
                let mut figment: $crate::figment::Figment = From::from(&args.$start);
                $ (
                  figment =  figment.merge(&args.$more);
                )*
                figment
            }
        }

        impl<'a> From<&'a $name> for $crate::Config {
            fn from(args: &'a $name) -> Self {
                let figment: $crate::figment::Figment = args.into();
                $crate::Config::from_provider(figment).sanitized()
            }
        }
    };
    ($name:ty, self, $start:ident $(, $more:ident)*) => {
        impl<'a> From<&'a $name> for $crate::figment::Figment {
            fn from(args: &'a $name) -> Self {
                let mut figment: $crate::figment::Figment = From::from(&args.$start);
                $ (
                  figment =  figment.merge(&args.$more);
                )*
                figment = figment.merge(args);
                figment
            }
        }

        impl<'a> From<&'a $name> for $crate::Config {
            fn from(args: &'a $name) -> Self {
                let figment: $crate::figment::Figment = args.into();
                $crate::Config::from_provider(figment).sanitized()
            }
        }
    };
}

/// Same as `impl_figment_convert` but also merges the type itself into the figment
///
/// # Example
///
/// Merge several nested `Provider` together with the type itself
///
/// ```rust
/// use std::path::PathBuf;
/// use foxar_config::{Config, merge_impl_figment_convert, impl_figment_convert};
/// use foxar_config::figment::*;
/// use foxar_config::figment::value::*;
///
/// #[derive(Default)]
/// struct MyArgs {
///     root: Option<PathBuf>,
/// }
///
/// impl Provider for MyArgs {
///     fn metadata(&self) -> Metadata {
///         Metadata::default()
///     }
///
///     fn data(&self) -> Result<Map<Profile, Dict>, Error> {
///        todo!()
///     }
/// }
///
/// impl_figment_convert!(MyArgs);
///
/// #[derive(Default)]
/// struct OuterArgs {
///     value: u64,
///     inner: MyArgs
/// }
///
/// impl Provider for OuterArgs {
///     fn metadata(&self) -> Metadata {
///         Metadata::default()
///     }
///
///     fn data(&self) -> Result<Map<Profile, Dict>, Error> {
///             todo!()
///     }
/// }
///
/// merge_impl_figment_convert!(OuterArgs, inner);
/// ```
#[macro_export]
macro_rules! merge_impl_figment_convert {
    ($name:ty, $start:ident $(, $more:ident)*) => {
        impl<'a> From<&'a $name> for $crate::figment::Figment {
            fn from(args: &'a $name) -> Self {
                let mut figment: $crate::figment::Figment = From::from(&args.$start);
                $ (
                  figment =  figment.merge(&args.$more);
                )*
                figment = figment.merge(args);
                figment
            }
        }

        impl<'a> From<&'a $name> for $crate::Config {
            fn from(args: &'a $name) -> Self {
                let figment: $crate::figment::Figment = args.into();
                $crate::Config::from_provider(figment).sanitized()
            }
        }
    };
}

/// A macro to implement converters from a type to [`Config`] and [`figment::Figment`]
#[macro_export]
macro_rules! impl_figment_convert_probe {
    ($name:ty) => {
        impl<'a> From<&'a $name> for $crate::figment::Figment {
            fn from(args: &'a $name) -> Self {
                $crate::Config::figment_with_root($crate::find_project_root_path().unwrap())
                    .merge(args)
            }
        }

        impl<'a> From<&'a $name> for $crate::Config {
            fn from(args: &'a $name) -> Self {
                let figment: $crate::figment::Figment = args.into();
                $crate::Config::from_provider(figment).sanitized()
            }
        }
    };
}

/// Same as `impl_figment_convert` but also implies `Provider` for the given `Serialize` type for
/// convenience. The `Provider` only provides the "root" value for the current profile
#[macro_export]
macro_rules! impl_figment_convert_basic {
    ($name:ty) => {
        $crate::impl_figment_convert!($name);

        impl $crate::figment::Provider for $name {
            fn metadata(&self) -> $crate::figment::Metadata {
                $crate::figment::Metadata::named(stringify!($name))
            }

            fn data(
                &self,
            ) -> Result<
                $crate::figment::value::Map<$crate::figment::Profile, $crate::figment::value::Dict>,
                $crate::figment::Error,
            > {
                let mut dict = $crate::figment::value::Dict::new();
                if let Some(root) = self.root.as_ref() {
                    dict.insert(
                        "root".to_string(),
                        $crate::figment::value::Value::serialize(root)?,
                    );
                }
                Ok($crate::figment::value::Map::from([($crate::Config::selected_profile(), dict)]))
            }
        }
    };
}

macro_rules! array_key {
    (
        $vis: vis
        enum
        $name: ident
        {
            $($variant: ident),*
            $(,)?
        }
    ) => {
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
        $vis enum $name {
            $($variant),*
        }
        impl $name {
            #[allow(dead_code, path_statements)]
            $vis const COUNT: usize = $({Self::$variant; 1} + )* 0;
            #[allow(dead_code)]
            $vis const ARRAY: [Self; Self::COUNT] = [$(Self::$variant),*];

            #[allow(dead_code)]
            pub fn name(&self) -> &'static str {
                match self {
                    $(
                        Self::$variant => stringify!($variant)
                    ),*
                }
            }
        }
    };
}
pub(crate) use array_key;


macro_rules! make_trait_alias {
    (
        $new: ident = [$($old: tt)*] { $($content: tt)* }
    ) => {
        pub trait $new: $($old)* { $($content)* }
        impl<T: $($old)*> $new for T {}
    };
}
pub(crate) use make_trait_alias;

// macro_rules! forget {
//     (
//         $($i: tt)*
//     ) => {
//
//     };
// }
// pub(crate) use forget;

macro_rules! make_deref {
    ($s: ty, $t: ty) => {
        impl std::ops::Deref for $s {
            type Target = $t;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
    };
}
pub(crate) use make_deref;

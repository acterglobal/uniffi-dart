// use uniffi;
// use url::Url;

// pub struct Handle(pub i64);

// pub struct TimeIntervalMs(pub i64);

// pub struct TimeIntervalSecDbl(pub f64);

// pub struct TimeIntervalSecFlt(pub f32);

// impl UniffiCustomTypeConverter for Handle {
//     // The `Builtin` type will be used to marshall values across the FFI
//     type Builtin = i64;

//     // Convert Builtin to our custom type
//     fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
//         Ok(Handle(val))
//     }

//     // Convert our custom type to Builtin
//     fn from_custom(obj: Self) -> Self::Builtin {
//         obj.0
//     }
// }

// impl UniffiCustomTypeConverter for Url {
//     type Builtin = String;

//     fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
//         Ok(Url::parse(&val)?)
//     }

//     fn from_custom(obj: Self) -> Self::Builtin {
//         obj.into()
//     }
// }

// impl UniffiCustomTypeConverter for TimeIntervalMs {
//     // The `Builtin` type will be used to marshall values across the FFI
//     type Builtin = i64;

//     // Convert Builtin to our custom type
//     fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
//         Ok(TimeIntervalMs(val))
//     }

//     // Convert our custom type to Builtin
//     fn from_custom(obj: Self) -> Self::Builtin {
//         obj.0
//     }
// }

// impl UniffiCustomTypeConverter for TimeIntervalSecDbl {
//     // The `Builtin` type will be used to marshall values across the FFI
//     type Builtin = f64;

//     // Convert Builtin to our custom type
//     fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
//         Ok(TimeIntervalSecDbl(val))
//     }

//     // Convert our custom type to Builtin
//     fn from_custom(obj: Self) -> Self::Builtin {
//         obj.0
//     }
// }

// impl UniffiCustomTypeConverter for TimeIntervalSecFlt {
//     // The `Builtin` type will be used to marshall values across the FFI
//     type Builtin = f32;

//     // Convert Builtin to our custom type
//     fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
//         Ok(TimeIntervalSecFlt(val))
//     }

//     // Convert our custom type to Builtin
//     fn from_custom(obj: Self) -> Self::Builtin {
//         obj.0
//     }
// }

// #[derive(uniffi::Record)]
// pub struct CustomTypesDemo {
//     url: Url,
//     handle: Handle,
//     time_interval_ms: TimeIntervalMs,
//     time_interval_sec_dbl: TimeIntervalSecDbl,
//     time_interval_sec_flt: TimeIntervalSecFlt,
// }

// #[uniffi::export]
// pub fn get_custom_types_demo(v: Option<CustomTypesDemo>) -> CustomTypesDemo {
//     v.unwrap_or_else(|| CustomTypesDemo {
//         url: Url::parse("http://example.com/").unwrap(),
//         handle: Handle(123),
//         time_interval_ms: TimeIntervalMs(456000),
//         time_interval_sec_dbl: TimeIntervalSecDbl(456.0),
//         time_interval_sec_flt: TimeIntervalSecFlt(777.0),
//     })
// }

// uniffi::include_scaffolding!("api");

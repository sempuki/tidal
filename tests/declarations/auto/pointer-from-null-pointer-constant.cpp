// tidal-rules: AUTO-19
// tidal-expect: reject
// tidal-error-clang: incompatible initializer of type .std::nullptr_t
// tidal-error-gcc: unable to deduce .auto\*
// tidal-error-edg: cannot deduce .auto. type
// tidal-note: nullptr has type std::nullptr_t, which is not a pointer type.
auto* p = nullptr;

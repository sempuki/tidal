// tidal-rules: PACK-2
// tidal-expect: reject
// tidal-error-clang: parenthesized pack declaration to have a name
// tidal-error-gcc: expected primary-expression before
// tidal-error-edg: unnamed parameter pack declaration cannot be parenthesized
// tidal-note: The unnamed pack of pointers to functions has its ellipsis inside the inner parentheses. With a name after the ellipsis, the declaration is valid (function-pack-forms.cpp).
template <class... R> constexpr int count(R (*...)()) { return sizeof...(R); }

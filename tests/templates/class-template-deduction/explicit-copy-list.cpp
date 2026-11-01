// tidal-rules: CTAD-23
// tidal-expect: reject
// tidal-error-clang: selected an explicit constructor for copy-list-initialization
// tidal-error-gcc: explicit deduction guide selected in copy-list-initialization
// tidal-error-edg: copy-list-initialization cannot use "explicit"
template <class T> struct Strict { explicit Strict(T) {} };
Strict s = {1};

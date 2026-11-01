// tidal-rules: CTAD-22 CTAD-23
// tidal-expect: reject
// tidal-error-clang: selected an explicit constructor for copy-list-initialization
// tidal-error-gcc: explicit deduction guide selected in copy-list-initialization
// tidal-error-edg: copy-list-initialization cannot use "explicit"
// tidal-note: The explicit constructor is the one that is not a template, so its guide wins and copy-list initialization fails.
template <class T> struct Mark { explicit Mark(T) {} template <class U = int> Mark(T, U* = nullptr) {} };
Mark m = {1};

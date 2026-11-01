// tidal-rules: CTAD-23
// tidal-expect: reject
// tidal-error-clang: selected an explicit deduction guide for copy-list-initialization
// tidal-error-gcc: explicit deduction guide selected in copy-list-initialization
// tidal-error-edg: copy-list-initialization cannot use "explicit"
// tidal-note: The explicit user guide is the best guide for a pointer.
template <class T> struct Box { Box(T) {} };
template <class T> explicit Box(T*) -> Box<T>;
int n = 0;
Box b = {&n};

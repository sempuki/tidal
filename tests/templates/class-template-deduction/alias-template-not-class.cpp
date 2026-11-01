// tidal-rules: CTAD-15
// tidal-expect: reject
// tidal-error-clang: requires template arguments; argument deduction only allowed for class templates
// tidal-error-gcc: invalid use of template-name .Pointer. without an argument list
// tidal-error-edg: cannot deduce class template arguments
// tidal-note: Pointer<T> is a pointer type, not a specialization of a class template.
template <class T> struct Box { Box(T) {} };
template <class T> using Pointer = Box<T>*;
Pointer p = static_cast<Box<int>*>(nullptr);

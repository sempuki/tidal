// tidal-rules: TPL-19
// tidal-expect: accept
// tidal-note: Two declarations of one template use different parameter names.
template <class A, int N> struct Array;
template <class Element, int Size> struct Array { Element items[Size]; };
template <class X, int M> struct Array;
static_assert(sizeof(Array<char, 3>) == 3);

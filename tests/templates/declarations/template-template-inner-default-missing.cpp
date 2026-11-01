// tidal-rules: TPL-40 TARG-6
// tidal-expect: reject
// tidal-error-clang: too few template arguments for template template parameter
// tidal-error-gcc: wrong number of template arguments
// tidal-error-edg: too few arguments for template template parameter
// tidal-note: The member definition's head has no inner default, so C<> has too few arguments there.
template <class T> struct Box {};
template <template <class = int> class C> struct Use { int size(); };
template <template <class> class C> int Use<C>::size() { C<> c; return sizeof(c); }

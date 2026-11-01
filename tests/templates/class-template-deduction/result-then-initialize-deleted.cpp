// tidal-rules: CTAD-26 CTAD-25
// tidal-expect: reject
// tidal-error-clang: call to deleted constructor
// tidal-error-gcc: use of deleted function
// tidal-error-edg: deleted function
// tidal-note: Deduction gives Gone<int>, and the initialization selects the deleted constructor.
template <class T> struct Gone { Gone(T) = delete; };
Gone g(1);

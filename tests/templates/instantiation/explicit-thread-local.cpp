// tidal-rules: INST-25
// tidal-expect: accept
// tidal-expect-edg: reject
// tidal-error-edg: a storage class may not be specified here
// tidal-note: The explicit instantiation repeats thread_local. Clang and GCC accept it, EDG rejects it (Q-0233).
template <class T> thread_local int counter = 0;
template thread_local int counter<int>;

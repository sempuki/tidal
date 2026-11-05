// tidal-rules: DED-32 ORD-9
// tidal-expect: accept
// tidal-note: The declared type int(int*) matches both templates, and which(T*) is more specialized, so the explicit specialization belongs to which(T*). The explicit instantiation for long* matches in the same way.
template <class T> constexpr int which(T) { return 1; }
template <class T> constexpr int which(T*) { return 2; }
template <> constexpr int which(int*) { return 3; }
template int which(long*);
static_assert(which(static_cast<int*>(nullptr)) == 3);
static_assert(which(static_cast<char*>(nullptr)) == 2);

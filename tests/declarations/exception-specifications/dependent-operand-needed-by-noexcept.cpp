// tidal-rules: EXS-10
// tidal-expect: reject
// tidal-error-clang: cannot be used prior to .::.
// tidal-error-gcc: is not a member of
// tidal-error-edg: must be a class or namespace
template <class T> struct Box { void put() noexcept(T::safe); };
Box<int> box;
bool quiet = noexcept(box.put());

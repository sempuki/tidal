// tidal-rules: INST-7
// tidal-expect: accept
template <class T> int make() { return T::nothing; }
using Result = decltype(make<int>());
int size = sizeof(make<int>());
bool safe = noexcept(make<int>());

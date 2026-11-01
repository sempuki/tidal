// tidal-rules: INST-22
// tidal-expect: accept
template <class T> void run(T) noexcept {}
template <class T> void walk(T) noexcept {}
template void run(int) noexcept;
template void walk(int);

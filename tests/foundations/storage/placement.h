// The library's non-allocating placement forms, declared as a program
// that includes no header declares them (STG-5).
inline void* operator new(decltype(sizeof 0), void* place) noexcept { return place; }
inline void* operator new[](decltype(sizeof 0), void* place) noexcept { return place; }
inline void operator delete(void*, void*) noexcept {}
inline void operator delete[](void*, void*) noexcept {}

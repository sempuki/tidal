namespace std { using size_t = decltype(sizeof 0); }
inline void* operator new(std::size_t, void* p) noexcept { return p; }
// From round 11's arena_fourth_try.cpp. Storage from the allocation
// function, held as unsigned char*, then a T* to its start. STG-28 gives
// element 0 of an array of T, also when T is not an implicit-lifetime
// type. After placement new, d->id reads the new object. tidal-fe gives
// undefined [STG-23]. With an aggregate T, or with the T* cast straight
// from the void* result, it runs.
struct T { int id; T(int i) : id(i) {} };
int main() {
  unsigned char* b = static_cast<unsigned char*>(::operator new(16));
  T* d = static_cast<T*>(static_cast<void*>(b));
  ::new (d) T(2);
  int r = d->id - 2;
  ::operator delete(b);
  return r;
}

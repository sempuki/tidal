// An explicit destructor call that names the type with decltype (the hash
// map's first try). DTOR-6 names a type name and a type alias.
inline void* operator new(decltype(sizeof 0), void* p) noexcept { return p; }
struct KV { int key; ~KV() {} };
int main() {
  KV kv{1};
  kv.~decltype(kv)();
  ::new (static_cast<void*>(&kv)) KV{2};
  return kv.key == 2 ? 0 : 1;
}

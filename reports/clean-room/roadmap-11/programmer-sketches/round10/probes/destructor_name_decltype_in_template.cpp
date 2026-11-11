// The same inside a class template, as in the hash map.
inline void* operator new(decltype(sizeof 0), void* p) noexcept { return p; }
template <class K> struct Slot {
  struct KV { K key; ~KV() {} } kv;
  void reset(K k) { kv.~decltype(kv)(); ::new (static_cast<void*>(&kv)) KV{k}; }
};
int main() {
  Slot<int> s{{1}};
  s.reset(2);
  return s.kv.key == 2 ? 0 : 1;
}

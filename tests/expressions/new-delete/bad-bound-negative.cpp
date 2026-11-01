// tidal-rules: NEW-35 LSUP-8
// tidal-expect: run
// tidal-expect-clang: run-fail
// tidal-note: Q-0301. Clang calls the allocation function with a size that it cannot meet.
namespace std {
using size_t = decltype(sizeof 0);
class exception {
public:
  exception() noexcept {}
  virtual ~exception();
};
class bad_alloc : public exception {
public:
  bad_alloc() noexcept {}
  ~bad_alloc() override;
};
class bad_array_new_length : public bad_alloc {
public:
  bad_array_new_length() noexcept {}
  ~bad_array_new_length() override;
};
}
int calls = 0;
alignas(16) unsigned char pool[256];
void* operator new[](std::size_t n) {
  ++calls;
  if (n > sizeof pool) throw std::bad_alloc();
  return pool;
}
void operator delete[](void*) noexcept {}
long bound(long n) { return n; }
std::size_t huge(std::size_t n) { return n; }
int* volatile kept = nullptr;
int main() {
  try {
    kept = new int[bound(-1)];
    return 1;
  } catch (std::bad_array_new_length&) {
    return calls == 0 ? 0 : 2;
  } catch (...) {
    return 3;
  }
}

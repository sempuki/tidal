// tidal-rules: THR-13
// tidal-expect: run
using size = decltype(sizeof 0);
int global_calls = 0;
int member_calls = 0;
alignas(64) unsigned char pool[4096];
size used = 0;

void* operator new(size n) {
  ++global_calls;
  void* p = pool + used;
  used += (n + 63) / 64 * 64;
  return p;
}
void operator delete(void*) noexcept {}
void operator delete(void*, size) noexcept {}

struct Error {
  int code;
  static void* operator new(size n) { ++member_calls; return ::operator new(n); }
  static void operator delete(void*) noexcept {}
};

int main() {
  global_calls = 0;
  int seen = 0;
  try {
    throw 7;
  } catch (int v) {
    seen += v;
  }
  try {
    throw Error{5};
  } catch (const Error& e) {
    seen += e.code;
  }
  if (seen != 12) return 1;
  if (global_calls != 0) return 2;
  if (member_calls != 0) return 3;
  return 0;
}

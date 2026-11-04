// tidal-rules: CE-2 CE-17
// tidal-expect: reject
// tidal-error-clang: non-constexpr function .~Log.|non-literal type .Log.
// tidal-error-gcc: call to non-.constexpr. function .Log::~Log\(\).
// tidal-error-edg: cannot call non-constexpr function .Log::~Log.
// tidal-note: CE-17 allows the variable of a type that is not a literal type. The end of the variable calls its destructor, which is not constexpr, so CE-2 rejects the evaluation, although the body of the destructor is empty.
struct Log {
  int v = 1;
  ~Log() {}
};
constexpr int read() {
  Log entry;
  return entry.v;
}
static_assert(read() == 1);

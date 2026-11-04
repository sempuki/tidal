// tidal-rules: CE-2 CE-17
// tidal-expect: reject
// tidal-error-clang: non-constexpr function .~Log.|non-literal type .Log.
// tidal-error-gcc: call to non-.constexpr. function .Log::~Log\(\).|.Log. does not have .constexpr. destructor
// tidal-error-edg: cannot call non-constexpr function .Log::~Log.
// tidal-note: The temporary ends at the end of the full expression, and its destructor, which is not constexpr, runs then.
struct Log {
  int v = 1;
  constexpr Log() {}
  ~Log() {}
};
constexpr int read() { return Log().v; }
static_assert(read() == 1);

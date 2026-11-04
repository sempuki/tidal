// tidal-rules: CNS-19 CNS-21 UNI-3
// tidal-expect: run
// tidal-note: TrivialEnd<T> holds when a union with a T member can be destroyed, which needs a trivial destructor of T (UNI-3). Maybe<int> selects the defaulted destructor and Maybe<Logged> the user-provided one (CNS-19), and the concept sees the difference.
struct Logged { ~Logged() {} };
template <class T> union Shell { T member; char c; };
template <class T> concept TrivialEnd = requires(Shell<T>* p) { p->~Shell<T>(); };
template <class T> struct Maybe {
  union { T value; char none; };
  bool full = false;
  Maybe() : none() {}
  ~Maybe() requires TrivialEnd<T> = default;
  ~Maybe() requires (!TrivialEnd<T>) { if (full) value.~T(); }
};
static_assert(TrivialEnd<int>);
static_assert(!TrivialEnd<Logged>);
static_assert(TrivialEnd<Maybe<int>>);
static_assert(!TrivialEnd<Maybe<Logged>>);
int main() {
  Maybe<int> a;
  Maybe<Logged> b;
  return a.full || b.full ? 1 : 0;
}

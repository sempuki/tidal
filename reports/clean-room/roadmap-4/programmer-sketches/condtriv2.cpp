// The same, observed only through behavior: a union member with a nontrivial destructor.
template <bool B> struct If { static constexpr bool value = B; };
struct D { ~D() {} };
template <class T> concept TrivialDtor = requires { [] { union U { T t; char c; } u{.c = 0}; (void)u; }; };
template <class T> struct Probe { union { T t; char c; }; };
template <class T> concept DtorOK = requires(Probe<T>* p) { p->~Probe<T>(); };
static_assert(DtorOK<int>);
static_assert(!DtorOK<D>);
template <class T> struct Opt {
  union { T v; char none; };
  bool has = false;
  Opt() : none() {}
  ~Opt() requires DtorOK<T> = default;
  ~Opt() requires (!DtorOK<T>) { if (has) v.~T(); }
};
static_assert(DtorOK<Opt<int>>);
int main() { Opt<int> a; Opt<D> b; return 0; }

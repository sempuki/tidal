"""Write the round 11 writer probe grids as JSON. Run from the repository root."""
import json, pathlib
OUT = pathlib.Path("reports/clean-room/roadmap-11/writer-probes")

def put(name, grid):
    (OUT / f"{name}.json").write_text(json.dumps(grid, indent=1) + "\n")

# I6-1, I6-5, I6-9, I6-10: where a call binds. Every row asserts the value 1,
# which the overloads declared before the template give. A reject in the
# "later overload" column means the call saw a declaration after the template.
put("i6-call-binding", {
 "prelude": ("namespace lib { struct S {}; struct Tag {};\n"
             "  constexpr int probe(Tag, long) { return 1; }\n"
             "  constexpr int f(S, long) { return 1; } }\n"
             "namespace lib2 { struct Tag2 {}; }"),
 "template": "{prelude}\n{row}\n{col}\nstatic_assert(X<lib::S>::r() == 1);\nint main() {{ return 0; }}",
 "rows": {
  "I6-1 independent call, ADL at the definition": "template <class T> struct X { static constexpr int r() { return probe(lib::Tag{}, 0); } };",
  "I6-1 independent call, only a later declaration": "template <class T> struct X { static constexpr int r() { return probe2(lib2::Tag2{}); } };",
  "I6-5 f(this->n, 0), n of type lib::S": "template <class T> struct X { lib::S n; constexpr int g() const { return f(this->n, 0); } static constexpr int r() { return X{}.g(); } };",
  "I6-5 f(n, 0), n of type lib::S": "template <class T> struct X { lib::S n; constexpr int g() const { return f(n, 0); } static constexpr int r() { return X{}.g(); } };",
  "I6-5 f(this->n, 0), in a local class of a function template": "template <class T> constexpr int loc() { struct L { lib::S n; constexpr int g() const { return f(this->n, 0); } }; return L{}.g(); }\ntemplate <class T> struct X { static constexpr int r() { return loc<T>(); } };",
  "I6-9 lib::f(T{}, 0), qualified, dependent argument": "template <class T> struct X { static constexpr int r() { return lib::f(T{}, 0); } };",
  "I6-9 f(T{}, 0) where a member f is found": "template <class T> struct X { static constexpr int f(T, long) { return 1; } static constexpr int r() { return f(T{}, 0); } };",
  "control: f(T{}, 0), unqualified, dependent argument": "template <class T> struct X { static constexpr int r() { return f(T{}, 0); } };",
  "I6-10 control, first used before the later overload": "template <class T> struct X { static constexpr int r() { return f(T{}, 0); } };\ninline int first_use() { return X<lib::S>::r(); }"
 },
 "cols": {
  "no later overload": "namespace lib2 { constexpr int probe2(Tag2) { return 1; } }",
  "later overload": "namespace lib { constexpr int probe(Tag, int) { return 2; } constexpr int f(S, int) { return 2; } }\nnamespace lib2 { constexpr int probe2(Tag2) { return 1; } }"
 },
 "flags": []
})

# I6-3, I6-6, I6-7, I6-8, I6-13 to I6-16, I6-18: checks before and at
# instantiation. Each row defines use<>(); the "instantiated" column calls it.
put("i6-instantiation-checks", {
 "prelude": "template <int K> struct B { using type = int; };",
 "template": "{prelude}\n{row}\nint main() {{ {col} return 0; }}",
 "rows": {
  "I6-6 static_assert(sizeof(N) == 1)": "template <int N> struct A { static_assert(sizeof(N) == 1); };\ntemplate <int = 0> void use() { A<1> a; (void)a; }",
  "I6-6 B<sizeof(N)>::type with no typename": "template <int N> struct A { void g() { B<sizeof(N)>::type x = 0; (void)x; } };\ntemplate <int = 0> void use() { A<1>{}.g(); }",
  "I6-6 B<noexcept(N + 1)>::type with no typename": "template <int N> struct A { void g() { B<noexcept(N + 1)>::type x = 0; (void)x; } };\ntemplate <int = 0> void use() { A<1>{}.g(); }",
  "I6-6 control: B<N>::type with no typename": "template <int N> struct A { void g() { B<N>::type x = 0; (void)x; } };\ntemplate <int = 0> void use() { A<1>{}.g(); }",
  "I6-3 typename T::template box b(1), no list": "struct H { template <class U> struct box { box(U) {} }; };\ntemplate <class T> struct C { void g() { typename T::template box b(1); (void)b; } };\ntemplate <int = 0> void use() { C<H>{}.g(); }",
  "I6-7 DEP-6 undeclared name in a body": "template <class T> int h(T) { return undeclared_value; }\ntemplate <int = 0> void use() {}",
  "I6-7 DEP-12 name from a dependent base": "template <class T> struct Base { int m = 0; };\ntemplate <class T> struct D : Base<T> { int g() { return m; } };\ntemplate <int = 0> void use() { D<int>{}.g(); }",
  "I6-8 CNS-13 Q<int> fails its constraint": "template <class T> concept Small = sizeof(T) == 1;\ntemplate <Small T> struct Q {};\ntemplate <class T> void k() { Q<int> q; (void)q; }\ntemplate <int = 0> void use() { k<int>(); }",
  "I6-13 decltype(make()) of an incomplete-able class": "template <class T> struct Bad { static_assert(sizeof(T) == 0); };\nBad<int> make();\ntemplate <int = 0> void use() { using R = decltype(make()); R* p = nullptr; (void)p; }",
  "I6-13 a call that passes the class by value": "template <class T> struct Bad { static_assert(sizeof(T) == 0); };\nvoid take(Bad<int>);\nextern Bad<int>& gr;\ntemplate <int = 0> void use() { take(gr); }",
  "I6-14 hard error beside a fallback": "template <class T> struct Hard { static_assert(sizeof(T) == 0); using type = int; };\ntemplate <class T> typename Hard<T>::type pick(T);\nint pick(...);\ntemplate <int = 0> void use() { (void)pick(1); }",
  "I6-15 static constexpr member, initializer fails": "template <class T> struct A { static constexpr int v = T::missing; };\ntemplate <int = 0> void use() { A<int> a; (void)a; }",
  "I6-15 static const member, initializer fails": "template <class T> struct A { static const int v = T::missing; };\ntemplate <int = 0> void use() { A<int> a; (void)a; }",
  "I6-15 static inline member, initializer fails": "template <class T> struct A { static inline int v = T::missing; };\ntemplate <int = 0> void use() { A<int> a; (void)a; }",
  "I6-16 friend body fails, no call": "template <class T> struct F { friend void ff(F) { T::missing(); } };\ntemplate <int = 0> void use() { F<int> f; (void)f; }",
  "I6-16 friend body fails, called": "template <class T> struct F { friend void ff(F) { T::missing(); } };\ntemplate <int = 0> void use() { F<int> f; ff(f); }",
  "I6-18 C::value_type* q = nullptr": "template <class C> void m() { C::value_type* q = nullptr; }\ntemplate <int = 0> void use() {}",
  "I6-18 T::type value = 0": "template <class T> void m() { T::type value = 0; }\ntemplate <int = 0> void use() {}",
  "I6-18 control: int p = 0; T::x * p;": "template <class T> void m() { int p = 0; T::x * p; }\ntemplate <int = 0> void use() {}"
 },
 "cols": {"not instantiated": "", "instantiated": "use();"},
 "flags": []
})

# G161: a jump into a loop body, in constant evaluation.
put("g161-jump-into-loop", {
 "prelude": "",
 "template": "constexpr int run(int k) {{ int n = 0; int s = 0; {row} return s * 100 + n; }}\n{col}\nint main() {{ return 0; }}",
 "rows": {
  "case into while body": "switch (k) { case 0: while (n < 3) { s += 10; case 1: s += 1; ++n; } }",
  "case into for body, expression init": "switch (k) { case 0: for (n = 1; n < 3; ++n) { s += 10; case 1: s += 1; } }",
  "case into for body, declaration init with initializer": "switch (k) { case 0: for (int i = 0; i < 3; ++i) { s += 10; case 1: s += 1; n = i; } }",
  "case into do body": "switch (k) { case 0: do { s += 10; case 1: s += 1; } while (++n < 3); }"
 },
 "cols": {
  "compiles": "",
  "run(1) == 2303: rest of the body, then step, then condition": "static_assert(run(1) == 2303);",
  "run(0) == 3303": "static_assert(run(0) == 3303);"
 },
 "flags": []
})

# P2-3, P2-6: two equal string literals in constant evaluation, and the
# shape of std::initializer_list.
IL = ("namespace std {{ template <class E> class initializer_list {{ {members} public: constexpr initializer_list() : p(nullptr), n(0) {{}} "
      "constexpr decltype(sizeof 0) size() const {{ return n; }} }}; }}\n")
put("p2-points", {
 "prelude": "",
 "template": "{row}\n{col}\nint main() {{ return 0; }}",
 "rows": {
  "P2-6 private constructor, count as decltype(sizeof 0)": IL.format(members="const E* p; decltype(sizeof 0) n; constexpr initializer_list(const E* q, decltype(sizeof 0) c) : p(q), n(c) {}"),
  "P2-6 public constructor": IL.format(members="const E* p; decltype(sizeof 0) n; public: constexpr initializer_list(const E* q, decltype(sizeof 0) c) : p(q), n(c) {} private:"),
  "P2-6 count first, then pointer": IL.format(members="decltype(sizeof 0) n; const E* p; constexpr initializer_list(const E* q, decltype(sizeof 0) c) : n(c), p(q) {}"),
  "P2-6 count as unsigned int": IL.format(members="const E* p; unsigned n; constexpr initializer_list(const E* q, unsigned c) : p(q), n(c) {}")
 },
 "cols": {
  "a braced list of three": "constexpr int count(std::initializer_list<int> l) { return (int)l.size(); }\nstatic_assert(count({1, 2, 3}) == 3);",
  "P2-3 two equal literals, == in a static_assert": "constexpr const char* a = \"pear\"; constexpr const char* b = \"pear\"; static_assert(a == b || a != b);",
  "P2-3 a literal compared with itself": "constexpr const char* a = \"pear\"; static_assert(a == a);"
 },
 "flags": []
})

# I6-6 and I6-15 without a use inside another template: the columns name
# the only use of the class.
put("i6-6-15-clean", {
 "prelude": "template <int K> struct B { using type = int; };",
 "template": "{prelude}\n{row}\n{col}\nint main() {{ return 0; }}",
 "rows": {
  "I6-6 static_assert(sizeof(N) == 1) in A<N>": "template <int N> struct A { static_assert(sizeof(N) == 1); static constexpr int v = 0; };",
  "I6-6 static_assert(sizeof(N) == 4) in A<N>": "template <int N> struct A { static_assert(sizeof(N) == 4); static constexpr int v = 0; };",
  "I6-6 static_assert(alignof(decltype(N)) == 1) in A<N>": "template <int N> struct A { static_assert(alignof(decltype(N)) == 1); static constexpr int v = 0; };",
  "I6-15 static constexpr int v = N / 0": "template <int N> struct A { static constexpr int v = N / 0 + 1; };",
  "I6-15 static constexpr int v = B<N>::missing": "template <int N> struct A { static constexpr int v = B<N>::missing; };",
  "I6-15 static const int v = B<N>::missing": "template <int N> struct A { static const int v = B<N>::missing; };",
  "I6-15 static inline int v = B<N>::missing": "template <int N> struct A { static inline int v = B<N>::missing; };"
 },
 "cols": {
  "no use": "",
  "an object A<1>": "A<1> a1;",
  "sizeof(A<1>)": "static_assert(sizeof(A<1>) >= 1);",
  "a read of A<1>::v": "int r = A<1>::v;"
 },
 "flags": []
})

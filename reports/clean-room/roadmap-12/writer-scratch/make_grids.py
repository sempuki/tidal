"""Write the round 12 writer probe grids as JSON. Run from the repository root."""
import json, pathlib
OUT = pathlib.Path("reports/clean-room/roadmap-12/writer-probes")

def put(name, grid):
    (OUT / f"{name}.json").write_text(json.dumps(grid, indent=1) + "\n")

IL = ("namespace std { template <class E> class initializer_list {\n"
      "  const E* b_; decltype(sizeof 0) n_;\n"
      "  constexpr initializer_list(const E* b, decltype(sizeof 0) n) : b_(b), n_(n) {}\n"
      "public: constexpr initializer_list() : b_(nullptr), n_(0) {} }; }\n")
SAME = ("template <class A, class B> constexpr bool same = false;\n"
        "template <class A> constexpr bool same<A, A> = true;\n")

# I7 rows that depend on behavior. Each row is one program; the single
# column adds nothing. A static_assert names the type the row expects.
put("i7-ctad", {
 "prelude": IL + SAME,
 "template": "{prelude}\n{row}\n{col}",
 "rows": {
  "I7-1 Plain p, not dependent, elided: Mixed b{1, 2, 3L} is Mixed<long>":
   "struct Plain { int x; int y; };\ntemplate <class T> struct Mixed { Plain p; T z; };\nMixed b{1, 2, 3L};\nstatic_assert(same<decltype(b), Mixed<long>>);",
  "I7-1 int v[2], not dependent, elided: Arr b{1, 2, 3L} is Arr<long>":
   "template <class T> struct Arr { int v[2]; T n; };\nArr b{1, 2, 3L};\nstatic_assert(same<decltype(b), Arr<long>>);",
  "I7-1 char v[3] from a string literal: Named b{\"ab\", 1} is Named<int>":
   "template <class T> struct Named { char v[3]; T n; };\nNamed b{\"ab\", 1};\nstatic_assert(same<decltype(b), Named<int>>);",
  "I7-3 a class that inherits constructors is not an aggregate":
   "template <class T> struct Base { constexpr Base(T) {} };\ntemplate <class T> struct Derived : Base<T> { using Base<T>::Base; };\nstatic_assert(!__is_aggregate(Derived<int>));",
  "I7-6 empty list: guide with no parameters, not the initializer-list guide":
   "template <class T> struct C { constexpr C() {} constexpr C(std::initializer_list<int>) {} };\nC(std::initializer_list<int>) -> C<long>;\nC() -> C<char>;\nC c{};\nstatic_assert(same<decltype(c), C<char>>);",
  "I7-6 empty list: the initializer-list guide":
   "template <class T> struct C { constexpr C() {} constexpr C(std::initializer_list<int>) {} };\nC(std::initializer_list<int>) -> C<long>;\nC() -> C<char>;\nC c{};\nstatic_assert(same<decltype(c), C<long>>);",
  "I7-6 two elements: user guide (initializer_list, int = 0) wins over (int, int)":
   "template <class T> struct C { constexpr C(int, int) {} constexpr C(std::initializer_list<int>, int = 0) {} };\nC(std::initializer_list<int>, int = 0) -> C<long>;\nC(int, int) -> C<char>;\nC c{1, 2};\nstatic_assert(same<decltype(c), C<long>>);",
  "I7-6 two elements, parentheses: (int, int) wins":
   "template <class T> struct C { constexpr C(int, int) {} constexpr C(std::initializer_list<int>, int = 0) {} };\nC(std::initializer_list<int>, int = 0) -> C<long>;\nC(int, int) -> C<char>;\nC c(1, 2);\nstatic_assert(same<decltype(c), C<char>>);",
  "I7-7 IntPair p(1, 2.0) is Pair<int, double>":
   "template <class T, class U> struct Pair { constexpr Pair(T, U) {} };\ntemplate <class U> using IntPair = Pair<int, U>;\nIntPair p(1, 2.0);\nstatic_assert(same<decltype(p), Pair<int, double>>);",
  "I7-7 IntPair q(1L, 2.0): the fixed parameter converts":
   "template <class T, class U> struct Pair { constexpr Pair(T, U) {} };\ntemplate <class U> using IntPair = Pair<int, U>;\nIntPair q(1L, 2.0);\nstatic_assert(same<decltype(q), Pair<int, double>>);",
  "I7-8 alias of a constrained alias, constraint met":
   "template <class T> concept Big = sizeof(T) > 1;\ntemplate <class A, class B> struct Pair { constexpr Pair(A, B) {} };\ntemplate <class T> requires Big<T> using Same = Pair<T, T>;\ntemplate <class T> using Same2 = Same<T>;\nSame2 p(1, 2);\nstatic_assert(same<decltype(p), Pair<int, int>>);",
  "I7-8 alias of a constrained alias, constraint not met":
   "template <class T> concept Big = sizeof(T) > 1;\ntemplate <class A, class B> struct Pair { constexpr Pair(A, B) {} };\ntemplate <class T> requires Big<T> using Same = Pair<T, T>;\ntemplate <class T> using Same2 = Same<T>;\nSame2 p('a', 'b');",
  "I7-8 control: the constrained alias itself, constraint not met":
   "template <class T> concept Big = sizeof(T) > 1;\ntemplate <class A, class B> struct Pair { constexpr Pair(A, B) {} };\ntemplate <class T> requires Big<T> using Same = Pair<T, T>;\nSame p('a', 'b');",
  "I7-9 Derived d; base has the guide with no parameters and a default":
   "template <class T = int> struct Base {};\ntemplate <class T> struct Derived : Base<T> { using Base<T>::Base; };\nDerived d;\nstatic_assert(same<decltype(d), Derived<int>>);",
  "I7-9 Derived d{1}; base is an aggregate":
   "template <class T> struct Base { T v; };\ntemplate <class T> struct Derived : Base<T> { using Base<T>::Base; };\nDerived d{1};",
  "I7-9 Derived d(1); base has a constructor (control)":
   "template <class T> struct Base { constexpr Base(T) {} };\ntemplate <class T> struct Derived : Base<T> { using Base<T>::Base; };\nDerived d(1);\nstatic_assert(same<decltype(d), Derived<int>>);",
  "I7-11 copy init: the only guide needs a user conversion to its parameter":
   "struct Conv { constexpr operator int() const { return 1; } };\ntemplate <class T> struct W { constexpr W(const Conv&) {} };\nW(int) -> W<long>;\nW w = Conv{};\nstatic_assert(same<decltype(w), W<long>>);",
  "I7-11 control: direct init, same guide":
   "struct Conv { constexpr operator int() const { return 1; } };\ntemplate <class T> struct W { constexpr W(const Conv&) {} };\nW(int) -> W<long>;\nW w(Conv{});\nstatic_assert(same<decltype(w), W<long>>);",
  "I7-13 new Box with no initializer":
   "template <class T = int> struct Box { T v; };\nauto p = new Box;",
  "I7-13 control: new Box{1}":
   "template <class T = int> struct Box { T v; };\nauto p = new Box{1L};\nstatic_assert(same<decltype(p), Box<long>*>);",
  "I7-13 static data member, no initializer, not inline":
   "template <class T = int> struct Box { T v; };\nstruct S { static Box b; };",
  "I7-13 static inline data member, no initializer":
   "template <class T = int> struct Box { T v; };\nstruct S { static inline Box b; };",
  "I7-13 control: static inline data member with {}":
   "template <class T = int> struct Box { T v; };\nstruct S { static inline Box b{}; };\nstatic_assert(same<decltype(S::b), Box<int>>);",
  "I7-13 control: variable with no initializer":
   "template <class T = int> struct Box { T v; };\nBox b;\nstatic_assert(same<decltype(b), Box<int>>);",
  "I7-14 copy init: better guide explicit for its T, worse user guide not":
   "template <class T> struct N { constexpr explicit(sizeof(T) == 1) N(T) {} };\nN(int) -> N<int>;\nN b = 'c';\nstatic_assert(same<decltype(b), N<int>>);",
  "I7-14 copy-list init, same classes":
   "template <class T> struct N { constexpr explicit(sizeof(T) == 1) N(T) {} };\nN(int) -> N<int>;\nN b = {'c'};",
  "I7-14 control: direct init picks the explicit guide":
   "template <class T> struct N { constexpr explicit(sizeof(T) == 1) N(T) {} };\nN(int) -> N<int>;\nN b('c');\nstatic_assert(same<decltype(b), N<char>>);",
  "I7-16 host claim: Entry a{.key = 3}":
   "template <class K> struct Entry { K key; };\nEntry a{.key = 3};\nstatic_assert(same<decltype(a), Entry<int>>);",
  "I7-16 host claim: Entry b(4L), parentheses":
   "template <class K> struct Entry { K key; };\nEntry b(4L);\nstatic_assert(same<decltype(b), Entry<long>>);"
 },
 "cols": {"program": ""},
 "flags": []
})

# P3-6 to P3-8 and P3-11: when a member of a class template over the
# derived class is made, and copy initialization through a converting
# constructor template. The "call" column calls the member after the
# derived class is complete.
CUR = "template <class D> struct Cursor { typename D::value_type* p = nullptr; };\n"
put("p3-points", {
 "prelude": "",
 "template": "{row}\n{col}",
 "rows": {
  "P3-6 in-class body returns Cursor<D>":
   CUR + "template <class D> struct Base { Cursor<D> begin() { return Cursor<D>{}; } };\nstruct Derived : Base<Derived> { using value_type = int; };",
  "P3-6 declaration in class returns Cursor<D>, body outside":
   CUR + "template <class D> struct Base { Cursor<D> begin(); };\ntemplate <class D> Cursor<D> Base<D>::begin() { return Cursor<D>{}; }\nstruct Derived : Base<Derived> { using value_type = int; };",
  "P3-6 virtual in-class body returns Cursor<D>":
   CUR + "template <class D> struct Base { virtual Cursor<D> begin() { return Cursor<D>{}; } };\nstruct Derived : Base<Derived> { using value_type = int; };",
  "P3-6 in-class body takes Cursor<D> by value":
   CUR + "template <class D> struct Base { void take(Cursor<D>) {} Cursor<D> begin() { return Cursor<D>{}; } };\nstruct Derived : Base<Derived> { using value_type = int; };",
  "P3-6 deduced return type, body returns Cursor<D>":
   CUR + "template <class D> struct Base { auto begin() { return Cursor<D>{}; } };\nstruct Derived : Base<Derived> { using value_type = int; };",
  "P3-6 static data member of type Cursor<D>, declaration only":
   CUR + "template <class D> struct Base { static Cursor<D> first; Cursor<D> begin() { return Cursor<D>{}; } };\nstruct Derived : Base<Derived> { using value_type = int; };",
  "P3-6 static inline data member of type Cursor<D>":
   CUR + "template <class D> struct Base { static inline Cursor<D> first{}; Cursor<D> begin() { return Cursor<D>{}; } };\nstruct Derived : Base<Derived> { using value_type = int; };",
  "P3-6 control: return type typename D::value_type, in-class body":
   "template <class D> struct Base { typename D::value_type first() { return 0; } Base& begin() { return *this; } };\nstruct Derived : Base<Derived> { using value_type = int; };",
  "P3-6 control: data member of type Cursor<D>":
   CUR + "template <class D> struct Base { Cursor<D> held; Cursor<D> begin() { return Cursor<D>{}; } };\nstruct Derived : Base<Derived> { using value_type = int; };",
  "P3-8 Cursor<D> constructor calls a member of D, from Base<D>::begin":
   "template <class D> struct Cursor { D* d; Cursor(D* p) : d(p) { p->touch(); } };\ntemplate <class D> struct Base { Cursor<D> begin() { return Cursor<D>(static_cast<D*>(this)); } };\nstruct Derived : Base<Derived> { using value_type = int; void touch() {} };"
 },
 "cols": {
  "no call": "",
  "call after Derived is complete": "int use() { Derived d; d.begin(); return 0; }"
 },
 "flags": []
})

ACTION = ("template <class A> struct Action {\n"
          "  template <class F> Action(F) {}\n"
          "  Action(const Action&) = delete;\n"
          "  Action& operator=(const Action&) = delete; };\n"
          "template <class A> struct Strict {\n"
          "  template <class F> explicit Strict(F) {}\n"
          "  Strict(const Strict&) = delete; };\n")
put("p3-copy-init", {
 "prelude": ACTION,
 "template": "{prelude}\n{row}\n{col}",
 "rows": {
  "P3-11 Action<int> a = closure; copies deleted, converting constructor template":
   "int f() { int log = 0; Action<int> a = [&log](int v) { log += v; }; return log; }",
  "P3-11 Action<int> a = Action<int>(closure); a prvalue of the same type":
   "int f() { int log = 0; Action<int> a = Action<int>([&log](int v) { log += v; }); return log; }",
  "P3-11 control: Strict<int> s = closure; explicit converting constructor":
   "int f() { Strict<int> s = [](int) {}; return 0; }",
  "P3-11 control: Action<int> b = a; a copy of an lvalue":
   "int f() { Action<int> a = [](int) {}; Action<int> b = a; return 0; }",
  "P3-3 a function template declared only, named in decltype":
   "template <class T> T&& fake();\nstruct Sq { long operator()(int&) const { return 0; } };\ntemplate <class F, class T> using result = decltype(fake<F&>()(fake<T&>()));\ntemplate <class X, class Y> constexpr bool same = false;\ntemplate <class X> constexpr bool same<X, X> = true;\nstatic_assert(same<result<Sq, int>, long>);"
 },
 "cols": {"program": ""},
 "flags": []
})

# G165: VIRT-13 when one overload is overridden and another is hidden.
G165_ROWS = {
 "one of two base overloads overridden, the other hidden":
  "struct B { virtual void f(int); virtual void f(double); virtual ~B(); };\nstruct D : B { void f(int) override; };",
 "neither overridden, a new f(char) hides both":
  "struct B { virtual void f(int); virtual void f(double); virtual ~B(); };\nstruct D : B { void f(char); };",
 "one base virtual f(double), derived f(int) hides it":
  "struct B { virtual void f(double); virtual ~B(); };\nstruct D : B { void f(int); };",
 "control: both overridden":
  "struct B { virtual void f(int); virtual void f(double); virtual ~B(); };\nstruct D : B { void f(int) override; void f(double) override; };",
 "control: using B::f, one overridden":
  "struct B { virtual void f(int); virtual void f(double); virtual ~B(); };\nstruct D : B { using B::f; void f(int) override; };"
}
put("g165-wall", {"prelude": "", "template": "{row}\n{col}", "rows": G165_ROWS,
                  "cols": {"-Wall -Werror": ""}, "flags": ["-Wall", "-Werror"]})
put("g165-woverloaded", {"prelude": "", "template": "{row}\n{col}", "rows": G165_ROWS,
                         "cols": {"-Woverloaded-virtual -Werror": ""},
                         "flags": ["-Woverloaded-virtual", "-Werror"]})

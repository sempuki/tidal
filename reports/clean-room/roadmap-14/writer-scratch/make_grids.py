"""Write the round 14 writer probe grids as JSON. Run from the repository root.

x24-compile: the X24 compile claims, with each probe of
reports/clean-room/X24/p/ as written, one row per define, and rows that
separate the readings of X24-2, X24-3, X24-6, and X24-7.
x24-warnings: the X24-4 claims on FN-7, one function per row, under
-Wreturn-type -Werror (EDG skips the grid, as it skips warning-flag tests).
rv10-points: the rows RV10-R-1 to RV10-R-5 of the read of the RV9 repair
diff.
grammar-forms: one row per form that rows G166 to G179 name, so that each
form's class is known before the Syntax segments derive it.
writer-scratch/run_probes.py holds the X24 run claims.
"""
import json, pathlib
OUT = pathlib.Path("reports/clean-room/roadmap-14/writer-probes")
X24 = pathlib.Path("reports/clean-room/X24/p")
TESTS = pathlib.Path("tests")


def put(name, grid):
    (OUT / f"{name}.json").write_text(json.dumps(grid, indent=1) + "\n")


def probe(name, defines=()):
    return "".join(f"#define {d}\n" for d in defines) + (X24 / name).read_text()


def program(rows):
    return {"prelude": "", "template": "{row}\n{col}", "rows": rows, "cols": {"program": ""}, "flags": []}


rows = {}
for name, cases in [("c1.cpp", [(), ("GOTO",)]), ("k1.cpp", [()]), ("l1.cpp", [()]), ("l2.cpp", [()]),
                    ("r1.cpp", [(), ("DEP",)]), ("s1.cpp", [()]),
                    ("t1.cpp", [("TEMPLATE",), ("DISCARDED",), ("TEMPLATE_DISCARDED",), ("TEMPLATE_BREAK",)])]:
    for d in cases:
        rows[f"{name} {' '.join(d) or 'as written'}"] = probe(name, d)
for t in ("break-with-label.cpp", "continue-with-label.cpp"):
    rows[f"JMP-17 test {t}"] = (TESTS / "statements/jumps" / t).read_text()
rows.update({
 # X24-2: which jumps JMP-6 covers.
 "X24-2 continue before a later declaration of a class with a constructor and destructor, constant":
  "struct N { int v; constexpr N(int x) : v(x) {} constexpr ~N() {} };\n"
  "constexpr int f() { int n = 0; for (int i = 0; i < 4; ++i) { if (i % 2) continue; N y{i}; n += y.v; } return n; }\n"
  "static_assert(f() == 2);\nint main() {}",
 "X24-2 break before a later initialized declaration":
  "int f() { int n = 0; while (true) { if (n > 2) break; int y = n; n = y + 1; } return n; }\nint main() {}",
 "X24-2 return before a later initialized declaration":
  "int f(int k) { if (k) return 1; int y = k; return y; }\nint main() {}",
 "X24-2 control: goto to the end of the body past an initialized declaration":
  "int f() { int n = 0; for (int i = 0; i < 4; ++i) { if (i % 2) goto end; { int y = i; n += y; } int z = i; n += z; end:; } return n; }\nint main() {}",
 "X24-2 control: goto forward past an initialized declaration in scope at the label":
  "int f(int k) { if (k) goto next; int y = k; next: return y; }\nint main() {}",
 # X24-3: when a jump target is checked in a template.
 "X24-3 goto to a missing label in a member function of a class template, never instantiated":
  "template <class T> struct S { void f() { goto nowhere; } };\nint main() {}",
 "X24-3 goto to a missing label in a generic lambda that is never called":
  "auto g = [](auto) { goto nowhere; };\nint main() {}",
 "X24-3 goto to a missing label in a function template, instantiated":
  "template <class T> void f() { goto nowhere; }\ntemplate void f<int>();\nint main() {}",
 "X24-3 a label declared twice in a function template, never instantiated":
  "template <class T> void f() { a:; a:; }\nint main() {}",
 "X24-3 continue outside a loop in a function template, never instantiated":
  "template <class T> void f() { continue; }\nint main() {}",
 "X24-3 break outside a loop in a discarded branch of an instantiated template":
  "template <class T> void h() { if constexpr (sizeof(T) == 0) { break; } }\nvoid use() { h<int>(); }\nint main() {}",
 "X24-3 goto to a missing label in a discarded branch of a template, never instantiated":
  "template <class T> void h() { if constexpr (sizeof(T) == 0) { goto nowhere; } }\nint main() {}",
 # X24-6 and X24-7: separate functions.
 "X24-6 continue in a member function of a local class, inside a loop":
  "void f() { for (int i = 0; i < 3; ++i) { struct S { void m() { continue; } }; S{}.m(); } }\nint main() {}",
 "X24-6 goto from a member function of a local class to a label of the enclosing function":
  "void f() { struct S { void m() { goto out; } }; S{}.m(); out:; }\nint main() {}",
 "X24-6 control: break in a lambda body inside a loop":
  "void f() { for (int i = 0; i < 3; ++i) { auto g = [] { break; }; g(); } }\nint main() {}",
 "X24-7 one label name in a member function of a local class and in its enclosing function":
  "int f() { int n = 0; struct S { int m() { int k = 0; again: if (++k < 3) goto again; return k; } };\n"
  "again: n += S{}.m(); if (n < 6) goto again; return n; }\nint main() { return f() == 6 ? 0 : 1; }",
 "X24-7 control: one label name twice in one lambda body":
  "auto g = [] { a:; a:; };\nint main() {}",
})
put("x24-compile", program(rows))

FN7 = {
 "a: the break leaves only an inner switch":
  "int a(int k) { while (true) { switch (k) { case 0: break; } if (k > 3) return k; } }",
 "b: a goto leaves the loop":
  "int b(int k) { while (true) { if (k) goto out; } out:; }",
 "c: the break leaves only an inner for":
  "int c(int k) { while (true) { for (;;) { if (k) break; } if (k > 3) return k; } }",
 "d control: a break leaves the loop":
  "int d(int k) { while (true) { if (k) break; } }",
 "e: a goto to a label inside the loop body":
  "int e(int k) { while (true) { if (k) goto next; ++k; next:; if (k > 3) return k; } }",
 "f: for (;;) with a goto that leaves it":
  "int f(int k) { for (;;) { if (k) goto out; } out:; }",
 "g: do while (true) with a break in an inner while":
  "int g(int k) { do { while (k) { break; } if (k > 3) return k; } while (true); }",
 "h control: a loop with constant true and only a return":
  "int h(int k) { while (true) { if (k > 3) return k; ++k; } }",
}
put("x24-warnings", {"prelude": "", "template": "{row}\n{col}", "rows": FN7, "cols": {"program": ""},
                     "flags": ["-Wreturn-type", "-Werror"]})

RV10 = {
 # RV10-R-1: places where CLS-2 counts the class complete that INST-30's list leaves out.
 "R-1 noexcept operand in class reads Holder<Coin>::n, which needs Coin complete":
  "template <class T> struct Holder { T value; static constexpr bool n = sizeof(T) > 0; };\n"
  "struct Coin { int c; void f() noexcept(Holder<Coin>::n); int d; };\n"
  "static_assert(noexcept(Coin{}.f()));\nint main() {}",
 "R-1 noexcept operand in class: Holder<Coin> sees a member declared after the operand":
  "template <class T> struct Holder { T value; static constexpr int n = sizeof(T); };\n"
  "struct Coin { int c; void f() noexcept(Holder<Coin>::n == 2 * sizeof(int)); int d; };\n"
  "static_assert(noexcept(Coin{}.f()));\nint main() {}",
 "R-1 default member initializer reads Holder<Coin>::n":
  "template <class T> struct Holder { T value; static constexpr int n = sizeof(T); };\n"
  "struct Coin { int c; int size = Holder<Coin>::n; };\nint main() { return Coin{}.size == 2 * sizeof(int) ? 0 : 1; }",
 "R-1 default template argument of a member template reads Holder<Coin>::n (Q-0406)":
  "template <class T> struct Holder { T value; static constexpr int n = sizeof(T); };\n"
  "struct Coin { int c; template <int N = Holder<Coin>::n> int f() { return N; } };\nint main() {}",
 "R-1 default template argument names Holder<Coin> with no member use":
  "template <class T> struct Holder { T value; };\n"
  "struct Coin { int c; template <class H = Holder<Coin>> int f() { return 0; } };\nint main() {}",
 # RV10-R-2: a nested class defined after its enclosing class.
 "R-2 nested class defined after its enclosing class returns Holder<Inner>":
  "template <class T> struct Holder { T value; };\n"
  "struct Outer { struct Inner; };\n"
  "struct Outer::Inner { int k; Holder<Inner> wrap() const { return {*this}; } };\n"
  "int main() { return Outer::Inner{3}.wrap().value.k == 3 ? 0 : 1; }",
 "R-2 nested class defined after its enclosing class returns its own type by value":
  "struct Outer { struct Inner; };\n"
  "struct Outer::Inner { int k; Inner next() const { return {k + 1}; } };\n"
  "int main() { return Outer::Inner{3}.next().k == 4 ? 0 : 1; }",
 "R-2 nested class defined after its enclosing class: a friend returns Holder<Inner>":
  "template <class T> struct Holder { T value; };\n"
  "struct Outer { struct Inner; };\n"
  "struct Outer::Inner { int k; friend Holder<Inner> wrap(Inner i) { return {i}; } };\n"
  "int main() { return wrap(Outer::Inner{3}).value.k == 3 ? 0 : 1; }",
 "R-2 control: nested class defined after its enclosing class returns a type completed after it":
  "struct Later;\nstruct Outer { struct Inner; };\n"
  "struct Outer::Inner { Later make() const { throw 0; } };\nstruct Later { int k; };\nint main() {}",
 # RV10-R-3: an assignment through a union member's name after a pseudo-destructor call.
 "R-3 union int member ended, assigned through its name, read, constant":
  "using Int = int;\nunion U { int a; float b; };\n"
  "constexpr int f() { U u{1}; u.a.~Int(); u.a = 3; return u.a; }\nstatic_assert(f() == 3);\nint main() {}",
 "R-3 union int member ended, the other member assigned, read, constant":
  "using Int = int;\nunion U { int a; float b; };\n"
  "constexpr float f() { U u{1}; u.a.~Int(); u.b = 2.0f; return u.b; }\nstatic_assert(f() == 2.0f);\nint main() {}",
 "R-3 element of a union's int array member ended, assigned, read, constant":
  "using Int = int;\nunion U { int a[2]; float b; };\n"
  "constexpr int f() { U u{{1, 2}}; u.a[1].~Int(); u.a[1] = 3; return u.a[1]; }\nstatic_assert(f() == 3);\nint main() {}",
 "R-3 member int of a struct member of a union ended, assigned, read, constant":
  "using Int = int;\nstruct P { int x; int y; };\nunion U { P p; float b; };\n"
  "constexpr int f() { U u{{1, 2}}; u.p.y.~Int(); u.p.y = 3; return u.p.y; }\nstatic_assert(f() == 3);\nint main() {}",
 "R-3 control: struct int member ended, assigned, read, constant":
  "using Int = int;\nstruct S { int a; float b; };\n"
  "constexpr int f() { S s{1, 0}; s.a.~Int(); s.a = 3; return s.a; }\nstatic_assert(f() == 3);\nint main() {}",
 # RV10-R-4: the Q-0449 forms that no test holds.
 "R-4 copy of a constexpr union whose active member is not mutable, beside a mutable one":
  "union U { mutable int m; int n; };\nconstexpr U a{.n = 1};\nconstexpr U b = a;\nstatic_assert(b.n == 1);\nint main() {}",
 "R-4 copy of a constexpr struct with a mutable member":
  "struct S { mutable int m; };\nconstexpr S a{1};\nconstexpr S b = a;\nint main() {}",
 "R-4 read of the mutable member of a constexpr union":
  "union U { mutable int m; int n; };\nconstexpr U a{1};\nconstexpr int r = a.m;\nint main() {}",
 "R-4 copy assignment into a local union from a constexpr union whose active member is mutable":
  "union U { mutable int m; int n; };\nconstexpr U a{1};\n"
  "constexpr int g() { U b{.n = 2}; b = a; return 1; }\nstatic_assert(g() == 1);\nint main() {}",
 # RV10-R-5: CPT-28 in the three places.
 "R-5 requires expression in a concept, parameter type not valid":
  "template <class T> T&& value_of();\ntemplate <class R> using It = decltype(value_of<R&>().begin());\n"
  "template <class R> concept Steps = requires(It<R> it) { ++it; };\nstatic_assert(!Steps<int>);\nint main() {}",
 "R-5 requires expression in a variable template, parameter type not valid":
  "template <class T> T&& value_of();\ntemplate <class R> using It = decltype(value_of<R&>().begin());\n"
  "template <class R> constexpr bool steps = requires(It<R> it) { ++it; };\nstatic_assert(!steps<int>);\nint main() {}",
 "R-5 requires expression in if constexpr, parameter type not valid":
  "template <class T> T&& value_of();\ntemplate <class R> using It = decltype(value_of<R&>().begin());\n"
  "template <class R> constexpr int pick() { if constexpr (requires(It<R> it) { ++it; }) return 1; else return 2; }\n"
  "static_assert(pick<int>() == 2);\nint main() {}",
 "R-5 requires expression in a requires clause, parameter type not valid":
  "template <class T> T&& value_of();\ntemplate <class R> using It = decltype(value_of<R&>().begin());\n"
  "template <class R> requires requires(It<R> it) { ++it; } constexpr int pick() { return 1; }\n"
  "template <class R> constexpr int pick() { return 2; }\nstatic_assert(pick<int>() == 2);\nint main() {}",
}
put("rv10-points", program(RV10))

G = {
 "G166 template-id then braced list": "template <class T> struct Box { T v; };\nint f() { return Box<int>{1}.v; }",
 "G166 template-id then empty braces": "template <class T> struct Box { T v; };\nint f() { return Box<int>{}.v; }",
 "G166 qualified template-id then braced list": "namespace n { template <class T> struct Box { T v; }; }\nint f() { return n::Box<int>{2}.v; }",
 "G166 template-id then parentheses, a cast": "template <class T> struct Box { T v; Box(T x) : v(x) {} };\nint f() { return Box<int>(1).v; }",
 "G167 base written as a template-id": "template <class T> struct B { T v; };\nstruct D : B<int> {};",
 "G167 base written as a qualified template-id": "namespace n { template <class T> struct B {}; }\nstruct D : public n::B<int> {};",
 "G167 pack of bases": "template <int K> struct Part {};\ntemplate <int... K> struct All : Part<K>... {};\nAll<1, 2> a;",
 "G167 base written with decltype": "struct B {}; B b;\nstruct D : decltype(b) {};",
 "G167 member initializer written as a template-id": "template <class T> struct B { B(T) {} };\ntemplate <class T> struct D : B<T> { D(T x) : B<T>(x) {} };\nD<int> d(1);",
 "G167 member initializer written with decltype": "struct B { B(int) {} }; B b(0);\nstruct D : decltype(b) { D() : decltype(b)(1) {} };",
 "G168 using-directive with an unqualified name": "namespace lib { int v; }\nusing namespace lib;\nint f() { return v; }",
 "G168 namespace alias to an unqualified name": "namespace m { int v; }\nnamespace n = m;\nint f() { return n::v; }",
 "G168 using-directive with a qualified name": "namespace a { namespace b { int v; } }\nusing namespace a::b;\nint f() { return v; }",
 "G169 using enum at block scope": "enum class Color { red, green };\nint f() { using enum Color; return red == green; }",
 "G169 using enum in a class": "enum class Color { red };\nstruct S { using enum Color; };\nColor c = S::red;",
 "G169 using enum at namespace scope": "namespace n { enum class Color { red }; }\nusing enum n::Color;\nauto c = red;",
 "G170 attribute after the decl-specifiers": "int [[tidal::note]] c;",
 "G170 side: maybe_unused after the decl-specifiers, which applies to the type": "int [[maybe_unused]] c;",
 "G170 attribute before a parameter": "void f([[maybe_unused]] int p) {}",
 "G170 attribute before a base specifier": "struct B {};\nstruct D : [[tidal::note]] B {};",
 "G171 template-id of operator() after a member access": "struct L { template <class T> int operator()() const { return sizeof(T); } };\nint f(L x) { return x.operator()<int>(); }",
 "G171 address of a qualified operator() template-id": "struct S { template <class T> void operator()(T) {} };\nauto p = &S::operator()<long>;",
 "G171 literal operator template-id": "template <char... C> constexpr int operator\"\"_count() { return sizeof...(C); }\nstatic_assert(operator\"\"_count<'4', '0'>() == 2);",
 "G171 destructor name with a template-id": "template <class T> struct Box { ~Box() {} };\nvoid f(Box<int>* p) { p->~Box<int>(); }",
 "G171 destructor name with decltype": "struct T { ~T() {} };\nvoid f() { T t; t.~decltype(t)(); }",
 "G172 opaque scoped enumeration in a class": "struct S { enum class Kind : short; };\nenum class S::Kind : short { a };",
 "G172 opaque unscoped enumeration in a class": "struct S { enum Kind : int; };\nenum S::Kind : int { a };",
 "G173 member conversion function template declared with no decl-specifiers": "struct S { template <class T> operator T() const; };\ntemplate <class T> S::operator T() const { return T(); }\nlong f(S s) { return s; }",
 "G173 member constructor template declared with no decl-specifiers": "struct S { template <class T> S(T); };\ntemplate <class T> S::S(T) {}\nS s(1);",
 "G174 structured binding in an if init-statement": "struct P { int a, b; };\nint f(P s) { if (auto [a, b] = s; b > 2) return a; return 0; }",
 "G174 structured binding in a switch init-statement": "struct P { int a, b; };\nint f(P s) { switch (auto [a, b] = s; a) { default: return b; } }",
 "G174 structured binding in a for init-statement": "struct P { int a, b; };\nint f(P s) { int n = 0; for (auto [a, b] = s; a < b; ++a) ++n; return n; }",
 "G171 control: destructor name with decltype of a reference": "struct T { ~T() {} };\nvoid f(T* p) { p->~decltype(*p)(); }",
 "G175 throw() after a function declarator": "void f() throw();\nstatic_assert(noexcept(f()));",
 "G176 static_assert message of two string literals": "static_assert(true, \"part one, \" \"part two\");",
 "G177 conversion function to a pointer to member": "struct Base { int x; };\nusing PM = int Base::*;\nstruct S { operator int Base::*() const { return &Base::x; } };\nPM p = S{};",
 "G178 braced list as a template argument": "struct Point { int x; int y; };\ntemplate <Point P> struct At { static constexpr int sum = P.x + P.y; };\nstatic_assert(At<{3, 4}>::sum == 7);",
 "G179 typename T::template name with no argument list, deduced": "struct Has { template <class U> struct box { box(U) {} }; };\ntemplate <class T> void make() { typename T::template box b(1); }\ntemplate void make<Has>();",
 "G179 template template default T::template name": "struct Has { template <class U> struct box { U value; }; };\ntemplate <class T> struct Outer { template <template <class> class Make = T::template box> struct In { Make<int> item; }; };\nOuter<Has>::In<> in{{4}};",
}
put("grammar-forms", {"prelude": "", "template": "{row}\n{col}", "rows": G, "cols": {"program": ""}, "flags": []})

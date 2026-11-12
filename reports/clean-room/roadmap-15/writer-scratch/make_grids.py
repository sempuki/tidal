"""Write the round 15 writer's probe grids for tools/probe_grid.py.
Run from the repository root. Each grid has one column, so each row is one
program. Output: reports/clean-room/roadmap-15/writer-probes/*.json.
"""
import json
from pathlib import Path

OUT = Path("reports/clean-room/roadmap-15/writer-probes")
X26 = Path("reports/clean-room/X26/p")
ONE = {"compile": ""}


def write(name, rows, prelude="", flags=None, template="{prelude}\n{row}\n"):
    grid = {"prelude": prelude, "template": template, "rows": rows, "cols": ONE}
    if flags:
        grid["flags"] = flags
    (OUT / f"{name}.json").write_text(json.dumps(grid, indent=1) + "\n")


# 1. X26: each probe file as written and with each define, compiled on every
#    baseline compiler (EDG takes part), and constant-evaluation forms of the
#    findings X26-1, X26-2, and X26-5.
x26 = {}
for name, defines in [("o1", [""]), ("m1", [""]), ("u1", [""]), ("f1", ["", "TWO"]),
                      ("s1", [""]), ("t1", ["", "USE", "DEFER_CLASS", "DEFER"])]:
    text = (X26 / f"{name}.cpp").read_text()
    for d in defines:
        label = f"{name}.cpp" + (f" -D{d}" if d else "")
        x26[label] = (f"#define {d}\n" if d else "") + text
LOG = """struct Log {
  char s[12]{}; int n = 0;
  constexpr void add(char c) { s[n++] = c; }
  constexpr bool is(const char* w) const {
    int i = 0;
    for (; w[i]; ++i) if (s[i] != w[i]) return false;
    return i == n;
  }
};
struct T {
  Log* l; char c;
  constexpr T(Log* l, char c) : l(l), c(c) { l->add(c); }
  constexpr ~T() { l->add(char(c - 32)); }
};
"""
x26["X26-1: left operand temporary ends at the end of the full expression"] = LOG + \
    "constexpr bool f() { Log g; (T(&g, 'a'), T(&g, 'b'), 0); return g.is(\"abBA\"); }\nstatic_assert(f());"
x26["X26-1: the same with a cast to void"] = LOG + \
    "constexpr bool f() { Log g; ((void)T(&g, 'a'), (void)T(&g, 'b'), 0); return g.is(\"abBA\"); }\nstatic_assert(f());"
x26["X26-1: a class prvalue as an expression statement ends at once"] = LOG + \
    "constexpr bool f() { Log g; T(&g, 'a'); g.add('x'); return g.is(\"aAx\"); }\nstatic_assert(f());"
x26["X26-2: a class prvalue as the iteration expression ends before the condition"] = LOG + \
    "constexpr bool f() { Log g; int i = 0; for (; i < 2; T(&g, 'a')) { ++i; g.add('x'); } return g.is(\"xaAxaA\"); }\nstatic_assert(f());"
x26["X26-2: a comma in the iteration expression keeps its temporary to the end of it"] = LOG + \
    "constexpr bool f() { Log g; int i = 0; for (; i < 1; T(&g, 'a'), ++i, g.add('y')) g.add('x'); return g.is(\"xayA\"); }\nstatic_assert(f());"
x26["X26-5: a right fold over class elements calls the overloaded comma"] = \
    "struct N { int v; };\nconstexpr N operator,(N l, N r) { return {l.v * 10 + r.v}; }\n" \
    "template <class... T> constexpr N fold(T... t) { return (t, ...); }\n" \
    "static_assert(fold(N{1}, N{2}, N{3}).v == 33);"
x26["X26-5: a left fold over class elements calls the overloaded comma"] = \
    "struct N { int v; };\nconstexpr N operator,(N l, N r) { return {l.v * 10 + r.v}; }\n" \
    "template <class... T> constexpr N fold(T... t) { return (..., t); }\n" \
    "static_assert(fold(N{1}, N{2}, N{3}).v == 123);"
write("x26-compile", x26)

# 2. X26-3 and the w1.cpp claims: which forms warn under -Wunused-value
#    -Werror, as a comma's left operand, as an expression statement, and as a
#    for iteration expression. A reject is a warning. EDG skips the grid.
W1 = """struct P { int id; P(int i) : id(i) {} ~P() {} };
struct S { int m; };
int f();
void g();
int (*pf)() = f;
int operator""_k(unsigned long long v) { return int(v); }
volatile int vol = 0;
"""
ops = {
    "name": "a", "literal": "1", "a + 1": "a + 1", "a + f()": "a + f()", "a == 1": "a == 1",
    "volatile name": "vol", "P(1)": "P(1)", "P{1}": "P{1}", "new int": "new int",
    "user-defined literal": "5_k", "lambda not called": "[] { return 1; }",
    "sizeof a": "sizeof a", "s.m": "s.m", "p[0]": "p[0]", "*p": "*p",
    "(long)a": "(long)a", "static_cast<long>(a)": "static_cast<long>(a)",
    "++a": "++a", "a = 1": "a = 1", "a += 1": "a += 1", "f()": "f()", "pf()": "pf()",
    "(void)a": "(void)a", "c ? f() : f()": "c ? f() : f()", "c ? a : b": "c ? a : b",
    "called lambda": "[] { return 1; }()", "g() (void)": "g()", "a++": "a++",
    "(a, f())": "(a, f())", "(f(), a)": "(f(), a)",
}
ctx = {
    "comma left": "int t(int a, int b, S s, int* p, bool c) {{ return ({op}, b); }}",
    "statement": "void t(int a, int b, S s, int* p, bool c) {{ {op}; }}",
    "for iteration": "void t(int a, int b, S s, int* p, bool c) {{ for (int k = 0; k < 1; {op}) ++k; }}",
}
rows = {}
for cl, ct in ctx.items():
    for ol, op in ops.items():
        rows[f"{cl}: {ol}"] = ct.format(op=op)
write("x26-warnings", rows, prelude=W1, flags=["-Wunused-value", "-Werror"])

# 3. X26-6: does a language-support type in Tidal's scope declare operator,?
#    Each row asks, through requires expressions, for a member or a free
#    operator, of the type with itself and with int, under the library
#    headers of each compiler.
pre6 = """#include <initializer_list>
#include <compare>
#include <cstddef>
#include <new>
#include <exception>
#include <typeinfo>
#include <memory>
template <class T, class U> concept member_comma = requires(T& a, U& b) { a.operator,(b); };
template <class T, class U> concept free_comma = requires(T& a, U& b) { operator,(a, b); };
template <class T> constexpr bool none = !member_comma<T, T> && !free_comma<T, T> &&
    !member_comma<T, int> && !free_comma<T, int> && !free_comma<int, T>;
template <class T> struct same { static constexpr bool v = false; };
template <class T> struct same<T&(T&)> { static constexpr bool v = true; };
"""
types = ["std::initializer_list<int>", "std::strong_ordering", "std::weak_ordering",
         "std::partial_ordering", "std::byte", "std::nullptr_t", "std::size_t", "std::align_val_t",
         "std::nothrow_t", "std::exception", "std::bad_alloc", "std::bad_array_new_length",
         "std::bad_cast", "std::type_info", "std::allocator<int>"]
rows = {t: f"static_assert(none<{t}>);" for t in types}
rows["control: a class with a member operator,"] = \
    "struct C { int operator,(C&); };\nstatic_assert(!none<C>);"
rows["control: a class with a free operator,"] = \
    "namespace n { struct D {}; int operator,(D&, D&); }\nstatic_assert(!none<n::D>);"
rows["comma of two std::byte is the built-in one"] = \
    "std::byte x{}, y{};\nstatic_assert(same<decltype((x, y))(std::byte&)>::v);"
write("x26-library", rows, prelude=pre6)

# 4. G182 (union_def) and G183 (LIST-4 with conversion functions).
g182 = {
    "union partial specialization": "template <class T, class U> union S { T t; U u; };\n"
        "template <class T> union S<T, int> { T t; int u; char c[3]; };\nS<char, int> s{};\nstatic_assert(sizeof(S<char, int>) >= sizeof(int));",
    "union explicit specialization": "template <class T> union S { T t; };\ntemplate <> union S<int> { int a; long b; };\nstatic_assert(sizeof(S<int>) == sizeof(long));",
    "union partial specialization with final": "template <class T> union S { T t; };\ntemplate <class T> union S<T*> final { T* p; long n; };\nS<int*> s{};",
    "union defined with a namespace qualifier": "namespace N { union V; }\nunion N::V { int a; float f; };\nN::V v{1};",
    "union defined with a class qualifier": "struct O { union I; };\nunion O::I { int x; };\nO::I i{2};",
    "member union template defined outside": "template <class T> struct O { template <class U> union I; };\n"
        "template <class T> template <class U> union O<T>::I { T t; U u; };\nO<int>::I<char> i{1};",
    "explicit specialization of a member union": "template <class T> struct O { union I { T t; }; };\n"
        "template <> union O<int>::I { int t; long l; };\nstatic_assert(sizeof(O<int>::I) == sizeof(long));",
    "partial specialization: union key for a struct template": "template <class T> struct S {};\ntemplate <class T> union S<T*> { T* p; };",
    "explicit specialization: struct key for a union template": "template <class T> union S { T t; };\ntemplate <> struct S<int> { int a; };",
}
write("g182-union", g182)

pre183 = """struct In { int a; int b; };
struct Out { In in; int z; };
struct DIn : In {};
struct ToIn { constexpr operator In() const { return {7, 8}; } };
struct Both { constexpr operator In() const { return {7, 8}; } constexpr operator int() const { return 1; } };
struct Any {
  template <class T> constexpr operator T() const {
    if constexpr (sizeof(T) == sizeof(int)) return T(1); else return T{7, 8};
  }
};
"""
g183 = {
    "ToIn (converts only to In): takes the member whole":
        "constexpr Out o{ToIn{}, 5};\nstatic_assert(o.in.a == 7 && o.in.b == 8 && o.z == 5);",
    "Both (to In and to int): takes the member whole":
        "constexpr Out o{Both{}, 5};\nstatic_assert(o.in.a == 7 && o.in.b == 8 && o.z == 5);",
    "Both (to In and to int): braces elided":
        "constexpr Out o{Both{}, 5};\nstatic_assert(o.in.a == 1 && o.in.b == 5 && o.z == 0);",
    "Any (template): takes the member whole":
        "constexpr Out o{Any{}, 5};\nstatic_assert(o.in.a == 7 && o.in.b == 8 && o.z == 5);",
    "Any (template): braces elided":
        "constexpr Out o{Any{}, 5};\nstatic_assert(o.in.a == 1 && o.in.b == 5 && o.z == 0);",
    "Any, Any: takes the member whole":
        "constexpr Out o{Any{}, Any{}};\nstatic_assert(o.in.a == 7 && o.z == 1);",
    "Any, Any, Any (three values for two members)":
        "constexpr Out o{Any{}, Any{}, Any{}};",
    "DIn (derived from In): takes the member whole":
        "constexpr Out o{DIn{{7, 8}}, 5};\nstatic_assert(o.in.a == 7 && o.in.b == 8 && o.z == 5);",
    "control: In object takes the member whole":
        "constexpr In i{7, 8};\nconstexpr Out o{i, 5};\nstatic_assert(o.in.a == 7 && o.z == 5);",
    "control: int values fill the member by elision":
        "constexpr Out o{1, 2, 3};\nstatic_assert(o.in.a == 1 && o.in.b == 2 && o.z == 3);",
}
write("g183-list4", g183, prelude=pre183)

# 5. The open RV11-R rows and F7-2.
rv11 = {
    "R-1: friend member template of a class template's specializations, two heads":
        "template <class T> struct A { template <class U> int g(U); };\n"
        "class C { int secret = 3; template <class T> template <class U> friend int A<T>::g(U); };\n"
        "template <class T> template <class U> int A<T>::g(U) { return C{}.secret; }\nint use() { return A<int>{}.g(1); }",
    "R-1: friend member class template of a class template's specializations, two heads":
        "template <class T> struct A { template <class U> struct In; };\n"
        "class C { int secret = 3; template <class T> template <class U> friend struct A<T>::In; };\n"
        "template <class T> template <class U> struct A<T>::In { int get() { return C{}.secret; } };\nint use() { return A<int>::In<char>{}.get(); }",
    "R-1: two heads, the first parameter not in the qualifier":
        "template <class T> struct A { template <class U> int g(U); };\n"
        "class C { template <class T> template <class U> friend int A<int>::g(U); };",
    "R-1: control: two heads before a member declaration":
        "struct C { template <class T> template <class U> void g(T, U); };",
    "R-2: functional cast to an alias template, parentheses":
        "template <class T> struct Box { T v; Box(T x) : v(x) {} };\ntemplate <class T> using B = Box<T>;\n"
        "auto b = B(3);\nstatic_assert(sizeof(b.v) == sizeof(int));",
    "R-2: functional cast to an alias template, braces":
        "template <class T> struct Box { T v; Box(T x) : v(x) {} };\ntemplate <class T> using B = Box<T>;\n"
        "auto b = B{3.0};\nstatic_assert(sizeof(b.v) == sizeof(double));",
    "R-2: functional cast, qualifier not dependent":
        "struct H { template <class U> struct Box { U v; Box(U x) : v(x) {} }; };\nauto b = H::Box(3);\nstatic_assert(sizeof(b.v) == sizeof(int));",
    "R-2: typename T::Box(3), dependent qualifier, no argument list":
        "struct H { template <class U> struct Box { U v; Box(U x) : v(x) {} }; };\n"
        "template <class T> auto f() { return typename T::Box(3); }\nauto x = f<H>();",
    "R-2: typename T::template Box(3), dependent qualifier, no argument list":
        "struct H { template <class U> struct Box { U v; Box(U x) : v(x) {} }; };\n"
        "template <class T> auto f() { return typename T::template Box(3); }\nauto x = f<H>();",
    "R-2: control: typename T::template Box<int>(3)":
        "struct H { template <class U> struct Box { U v; Box(U x) : v(x) {} }; };\n"
        "template <class T> auto f() { return typename T::template Box<int>(3); }\nauto x = f<H>();",
    "R-3: [[]] before a using-directive": "namespace N { int x; }\n[[]] using namespace N;",
    "R-3: [[maybe_unused]] before a using-directive": "namespace N { int x; }\n[[maybe_unused]] using namespace N;",
    "R-3: [[deprecated]] before a using-directive": "namespace N { int x; }\n[[deprecated]] using namespace N;",
    "R-3: [[nodiscard]] before a using-directive": "namespace N { int x; }\n[[nodiscard]] using namespace N;",
    "R-3: [[deprecated]] before a using-declaration": "namespace N { int x; }\n[[deprecated]] using N::x;",
    "R-5: union member of class type, default constructor deleted, written through the chain":
        "struct D { D() = delete; constexpr D(int v) : x(v) {} int x; };\nunion U { int i; D d; };\n"
        "constexpr int f() { U u{.i = 1}; u.d.x = 2; return u.d.x; }\nstatic_assert(f() == 2);",
    "R-5: control: default constructor defaulted, written through the chain":
        "struct D { D() = default; int x; };\nunion U { int i; D d; };\n"
        "constexpr int f() { U u{.i = 1}; u.d.x = 2; return u.d.x; }\nstatic_assert(f() == 2);",
    "R-5 and F7-2: a scalar ended inside a living union member of class type, then assigned":
        "using Int = int;\nstruct S { int a; int b; };\nunion U { S s; float f; };\n"
        "constexpr int f() { U u{.s = {1, 2}}; u.s.a.~Int(); u.s.a = 3; return u.s.a; }\nstatic_assert(f() == 3);",
    "F7-2: the same, read of the untouched member b after the write":
        "using Int = int;\nstruct S { int a; int b; };\nunion U { S s; float f; };\n"
        "constexpr int f() { U u{.s = {1, 2}}; u.s.a.~Int(); u.s.a = 3; return u.s.b; }\nstatic_assert(f() == 2);",
    "R-7: braced value for an int parameter": "template <int N> struct X { static constexpr int v = N; };\nstatic_assert(X<{3}>::v == 3);",
    "R-7: empty braces for an int parameter": "template <int N> struct X { static constexpr int v = N; };\nstatic_assert(X<{}>::v == 0);",
    "R-7: narrowing double in braces for an int parameter": "template <int N> struct X { static constexpr int v = N; };\nint y = X<{3.5}>::v;",
    "R-7: narrowing int constant in braces for a char parameter": "template <char C> struct X { static constexpr char v = C; };\nint y = X<{300}>::v;",
    "R-7: two values in braces for an int parameter": "template <int N> struct X { static constexpr int v = N; };\nint y = X<{3, 4}>::v;",
    "R-7: braced int constant for a double parameter": "template <double D> struct X { static constexpr double v = D; };\nstatic_assert(X<{1}>::v == 1.0);",
    "R-7: braced nullptr for a pointer parameter": "template <int* P> struct X { static constexpr bool v = P == nullptr; };\nstatic_assert(X<{nullptr}>::v);",
    "R-7: control: narrowing double without braces for an int parameter": "template <int N> struct X { static constexpr int v = N; };\nint y = X<3.5>::v;",
    "R-8: decltype(auto) before ::": "struct S { using t = int; };\nS s;\ndecltype(auto)::t x = 0;",
    "R-8: control: decltype(s) before ::": "struct S { using t = int; };\nS s;\ndecltype(s)::t x = 0;",
}
write("rv11-rows", rv11)

# 6. RV11-R-8 again: forms where `::` after decltype(auto) cannot start a
#    qualified declarator, so the first error must name the placeholder.
r8 = {
    "decltype(auto)::t in sizeof": "struct S { using t = int; };\nS s;\nint y = sizeof(decltype(auto)::t);",
    "decltype(auto)::v in an expression": "struct S { static constexpr int v = 1; };\nS s;\nint y = decltype(auto)::v;",
    "control: decltype(s)::v in an expression": "struct S { static constexpr int v = 1; };\nS s;\nint y = decltype(s)::v;",
}
write("rv11-r8", r8)

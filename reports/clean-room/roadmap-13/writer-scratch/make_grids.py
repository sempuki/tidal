"""Write the round 13 writer probe grids as JSON. Run from the repository root.

x25-constant: the X25 probes in reports/clean-room/X25/p/, compile only,
one row per probe and case, with construct.h inlined and a9's test of
validity turned into two static_assert rows.
x25-rules: rows that separate the readings of X25-1, X25-5, and X25-7
under the current rules (writer-scratch/run_probes.py holds X25-4).
p4-points: the P4 host-only claims and the behavior that rows P4-1 to
P4-5 and P4-7 rest on.
p4-4-warnings: the two forms of naming a member call's type under
-Wall -Wextra -Werror.
fe26-named-call: a literal operator template called by its name with
explicit template arguments (front-end report 26, section 5).
q0445-template-id: an attribute after the class key with a template-id
(Q-0445, RV9-R-4).
"""
import json, pathlib, re
OUT = pathlib.Path("reports/clean-room/roadmap-13/writer-probes")
X25 = pathlib.Path("reports/clean-room/X25/p")


def put(name, grid):
    (OUT / f"{name}.json").write_text(json.dumps(grid, indent=1) + "\n")


HEADER = (X25 / "construct.h").read_text()


def probe(name, defines=()):
    text = (X25 / name).read_text().replace('#include "construct.h"\n', HEADER)
    return "".join(f"#define {d}\n" for d in defines) + text


rows = {}
for name, cases in [("a1.cpp", [()]), ("a2.cpp", [("CASE 1",), ("CASE 2",), ("CASE 3",), ("CASE 4",)]),
                    ("a3.cpp", [()]), ("a3b.cpp", [()]), ("a3c.cpp", [()]),
                    ("a4.cpp", [("CASE 1",), ("CASE 2",)]),
                    ("a5.cpp", [("CASE 1",), ("CASE 2",)]), ("a6.cpp", [()]),
                    ("a7.cpp", [("CASE 1",), ("CASE 2",)]),
                    ("a10.cpp", [(), ("END",)]), ("m1.cpp", [()])]:
    for d in cases:
        rows[f"{name} {' '.join(d) or 'as written'}"] = probe(name, d)
a9 = probe("a9.cpp").replace("int main() { return is_constant<int> ? 1 : 0; }", "")
rows["a9.cpp test of validity is true"] = a9 + "static_assert(is_constant<int>);\nint main() {}"
rows["a9.cpp test of validity is false"] = a9 + "static_assert(!is_constant<int>);\nint main() {}"
put("x25-constant", {"prelude": "", "template": "{row}\n{col}", "rows": rows,
                     "cols": {"program": ""}, "flags": []})

NUM = "struct Num { int v; constexpr Num(int x) : v(x) {} constexpr ~Num() {} };\n"
put("x25-rules", {
 "prelude": HEADER + NUM,
 "template": "{prelude}\n{row}\n{col}",
 "rows": {
  "X25-1 construct_at of a whole member that is not active, constant":
   "union U { Num a; int b; constexpr U() : b(0) {} constexpr ~U() {} };\n"
   "constexpr int f() { U u; std::construct_at(&u.a, 4); int r = u.a.v; u.a.~Num(); return r; }\n"
   "static_assert(f() == 4);",
  "X25-1 construct_at of a member of a union with no active member, constant":
   "union U { Num a; constexpr U() {} constexpr ~U() {} };\n"
   "constexpr int f() { U u; std::construct_at(&u.a, 5); int r = u.a.v; u.a.~Num(); return r; }\n"
   "static_assert(f() == 5);",
  "X25-1 construct_at of an element of an array member that is not active, constant":
   "union U { Num a[2]; constexpr U() {} constexpr ~U() {} };\n"
   "constexpr int f() { U u; std::construct_at(&u.a[0], 6); int r = u.a[0].v; u.a[0].~Num(); return r; }\n"
   "static_assert(f() == 6);",
  "X25-5 trivial copy of a union whose active int array has a part with no value":
   "union R { int items[2]; constexpr R() {} };\n"
   "constexpr int f() { R r; r.items[0] = 1; R s = r; return s.items[0]; }\nstatic_assert(f() == 1);",
  "X25-5 the same copy of a struct, not a union":
   "struct R { int items[2]; constexpr R() {} };\n"
   "constexpr int f() { R r; r.items[0] = 1; R s = r; return s.items[0]; }\nstatic_assert(f() == 1);",
  "X25-5 trivial copy assignment of the union":
   "union R { int items[2]; constexpr R() {} };\n"
   "constexpr int f() { R r; r.items[0] = 1; R s; s = r; return s.items[0]; }\nstatic_assert(f() == 1);",
  "X25-5 trivial copy of a union whose active struct member has a field with no value":
   "struct P { int x; int y; };\nunion R { P p; long b; constexpr R() {} };\n"
   "constexpr int f() { R r; r.p.x = 1; R q = r; return q.p.x; }\nstatic_assert(f() == 1);",
  "X25-5 control: read of that field in the copy":
   "struct P { int x; int y; };\nunion R { P p; long b; constexpr R() {} };\n"
   "constexpr int f() { R r; r.p.x = 1; R q = r; return q.p.y; }\nstatic_assert(f() == 0);",
  "X25-7 constexpr union variable, one element ended by a pseudo-destructor":
   "using Int = int;\nunion R { int items[2]; constexpr R() {} };\n"
   "constexpr R make() { R r; r.items[0] = 1; r.items[1] = 2; r.items[1].~Int(); return r; }\n"
   "constexpr R kept = make();",
  "X25-7 constexpr struct variable, one element ended by a pseudo-destructor":
   "using Int = int;\nstruct R { int items[2]; };\n"
   "constexpr R make() { R r{1, 2}; r.items[1].~Int(); return r; }\n"
   "constexpr R kept = make();",
  "X25-7 control: constexpr struct variable, nothing ended":
   "struct R { int items[2]; };\nconstexpr R make() { R r{1, 2}; return r; }\n"
   "constexpr R kept = make();\nstatic_assert(kept.items[1] == 2);"
 },
 "cols": {"program": ""},
 "flags": []
})

IL = ("namespace std { template <class E> class initializer_list {\n"
      "  const E* b_; decltype(sizeof 0) n_;\n"
      "  constexpr initializer_list(const E* b, decltype(sizeof 0) n) : b_(b), n_(n) {}\n"
      "public: constexpr initializer_list() : b_(nullptr), n_(0) {}\n"
      "  constexpr const E* begin() const { return b_; }\n"
      "  constexpr decltype(sizeof 0) size() const { return n_; } }; }\n")
SAME = ("template <class A, class B> constexpr bool same = false;\n"
        "template <class A> constexpr bool same<A, A> = true;\n")
SEQ = ("struct Done {};\n"
       "struct Count { int n;\n"
       "  struct It { int i, n; constexpr int operator*() const { return i; }\n"
       "    constexpr It& operator++() { ++i; return *this; }\n"
       "    constexpr bool operator==(Done) const { return i == n; } };\n"
       "  constexpr It begin() const { return {0, n}; }\n"
       "  constexpr Done end() const { return {}; } };\n")
FILTER = ("template <class S, class F> struct Filtered { S base; F keep;\n"
          "  using BaseIt = decltype(static_cast<const S*>(nullptr)->begin());\n"
          "  using BaseEnd = decltype(static_cast<const S*>(nullptr)->end());\n"
          "  struct It { BaseIt it; BaseEnd end; F keep;\n"
          "    constexpr void skip() { while (!(it == end) && !keep(*it)) ++it; }\n"
          "    constexpr auto operator*() const { return *it; }\n"
          "    constexpr It& operator++() { ++it; skip(); return *this; }\n"
          "    constexpr bool operator==(Done) const { return it == end; } };\n"
          "  constexpr It begin() const { It i{base.begin(), base.end(), keep}; i.skip(); return i; }\n"
          "  constexpr Done end() const { return {}; } };\n")
MAYBE = ("template <class T> struct Maybe {\n"
         "  alignas(T) unsigned char bytes[sizeof(T)]; T* p = nullptr;\n"
         "  Maybe() = default;\n"
         "  Maybe(const Maybe&) = delete; Maybe& operator=(const Maybe&) = delete;\n")
put("p4-points", {
 "prelude": IL + SAME,
 "template": "{prelude}\n{row}\n{col}",
 "rows": {
  "P4-1 in-class body returns Box<Money>, Box has a member of type T":
   "template <class T> struct Box { T item; };\n"
   "struct Money { int cents; Box<Money> boxed() const { return {*this}; } };",
  "P4-1 in-class body takes Box<Money> by value":
   "template <class T> struct Box { T item; };\n"
   "struct Money { int cents; int take(Box<Money> b) const { return b.item.cents; } };",
  "P4-1 in-class body returns List<Money, 16>, List has T items[16]":
   "template <class T, int N> struct List { T items[N]; int count = 0; };\n"
   "struct Money { int cents = 0; List<Money, 16> split() const { return {}; } };",
  "P4-1 declaration only in class returns Box<Money>":
   "template <class T> struct Box { T item; };\n"
   "struct Money { int cents; Box<Money> boxed() const; };",
  "P4-1 control: in-class body returns a class completed only after the class":
   "struct Later;\nstruct User { Later make() const; Later twice() const { throw 0; } };\nstruct Later { int k; };",
  "P4-1 control: data member of type Box<Money> in Money":
   "template <class T> struct Box { T item; };\nstruct Money { int cents; Box<Money> held; };",
  "P4-2 member operator== with an initializer_list parameter, named operand":
   "struct L { int n; constexpr bool operator==(std::initializer_list<int> o) const { return o.size() == 2; } };\n"
   "constexpr bool f() { std::initializer_list<int> il = {1, 2}; return L{1} == il; }\nstatic_assert(f());",
  "P4-2 member operator== with an initializer_list parameter, operand std::initializer_list<int>{1, 2}":
   "struct L { int n; constexpr bool operator==(std::initializer_list<int> o) const { return o.size() == 2; } };\n"
   "static_assert(L{1} == std::initializer_list<int>{1, 2});",
  "P4-2 control: the same operator with a braced list as the right operand":
   "struct L { int n; constexpr bool operator==(std::initializer_list<int> o) const { return o.size() == 2; } };\n"
   "static_assert(L{1} == {1, 2});",
  "P4-2 control: operator[] with a braced list operand":
   "struct L { constexpr int operator[](std::initializer_list<int> o) const { return int(o.size()); } };\n"
   "static_assert(L{}[{1, 2, 3}] == 3);",
  "P4-3 range-for over an end type compared by == only (!= rewritten)":
   SEQ + "constexpr int f() { int s = 0; for (int v : Count{4}) s += v; return s; }\nstatic_assert(f() == 6);",
  "P4-3 adaptor over any sequence, closure type as a template parameter, CTAD":
   SEQ + FILTER +
   "constexpr int f() { int s = 0; auto even = [](int v) { return v % 2 == 0; };\n"
   "  for (int v : Filtered{Count{7}, even}) s += v; return s; }\nstatic_assert(f() == 12);",
  "P4-3 aggregate deduction Window{Count{6}, 3} gives Window<Count>":
   SEQ + "template <class S> struct Window { S base; int width; };\n"
   "constexpr Window w{Count{6}, 3};\nstatic_assert(same<decltype(w), const Window<Count>>);",
  "P4-4 decltype(static_cast<const S*>(nullptr)->begin()) names the iterator type":
   SEQ + "template <class S> using IteratorOf = decltype(static_cast<const S*>(nullptr)->begin());\n"
   "static_assert(same<IteratorOf<Count>, Count::It>);",
  "P4-4 requires-expression parameter of the iterator type":
   SEQ + "template <class S> using IteratorOf = decltype(static_cast<const S*>(nullptr)->begin());\n"
   "template <class S> concept Seq = requires(IteratorOf<S> it, const S& s) { ++it; *it; it == s.end(); };\n"
   "static_assert(Seq<Count>);\nstatic_assert(!Seq<int*>);",
  "P4-4 a declared function in decltype names the iterator type":
   SEQ + "template <class T> T&& fake();\n"
   "template <class S> using IteratorOf = decltype(fake<const S&>().begin());\n"
   "static_assert(same<IteratorOf<Count>, Count::It>);",
  "P4-5 Maybe with deleted copy and no move: return of a local":
   MAYBE + "};\nMaybe<int> take() { Maybe<int> last; return last; }",
  "P4-5 Maybe with deleted copy and a move constructor: return of a local":
   MAYBE + "  Maybe(Maybe&&) noexcept {}\n};\nMaybe<int> take() { Maybe<int> last; return last; }",
  "P4-5 Maybe with deleted copy and no move: return of a prvalue":
   MAYBE + "};\nMaybe<int> take() { return Maybe<int>(); }",
  "P4-7 19.99_usd through the long double form plus one half is 1999":
   "constexpr long long operator\"\"_usd(long double d) { return static_cast<long long>(d * 100 + 0.5L); }\n"
   "static_assert(19.99_usd == 1999);",
  "P4-7 the long double form receives the value of the unsuffixed literal 19.99L":
   "constexpr long double operator\"\"_v(long double d) { return d; }\n"
   "static_assert(19.99_v == 19.99L);\nstatic_assert(0.1_v == 0.1L);\nstatic_assert(1e300_v == 1e300L);",
  "P4-7 the long double form does not receive the double value 19.99":
   "constexpr long double operator\"\"_v(long double d) { return d; }\n"
   "static_assert(19.99_v != 19.99);",
  "P4-7 the integer form receives 18446744073709551615":
   "constexpr unsigned long long operator\"\"_v(unsigned long long d) { return d; }\n"
   "static_assert(18446744073709551615_v == 18446744073709551615ull);\nstatic_assert(0x10_v == 16);"
 },
 "cols": {"program": ""},
 "flags": []
})

# P4-4 with warnings as errors: the null-pointer form of naming a member
# call's type against the declared-function form, used directly and
# through a member of a class template. EDG skips a warning-flag grid.
NULLFORM = "template <class S> using It = decltype(static_cast<const S*>(nullptr)->begin());\n"
DECLFORM = "template <class T> T&& fake();\ntemplate <class S> using It = decltype(fake<const S&>().begin());\n"
USE_DIRECT = "static_assert(same<It<Count>, Count::It>);"
USE_MEMBER = ("template <class S> struct Wrap { S base; struct Cursor { It<S> at; }; };\n"
              "Wrap<Count>::Cursor c{};")
put("p4-4-warnings", {
 "prelude": SAME + SEQ,
 "template": "{prelude}\n{row}\n{col}",
 "rows": {
  "null-pointer form, used directly": NULLFORM + USE_DIRECT,
  "null-pointer form, through a member class of a class template": NULLFORM + USE_MEMBER,
  "declared-function form, used directly": DECLFORM + USE_DIRECT,
  "declared-function form, through a member class of a class template": DECLFORM + USE_MEMBER,
 },
 "cols": {"-Wall -Wextra -Werror": ""},
 "flags": ["-Wall", "-Wextra", "-Werror"]
})

# Front-end report 26, section 5: a literal operator template called by
# its name with explicit template arguments, against the literal form.
LOT = ("template <char... C> constexpr int operator\"\"_w() { return sizeof...(C) * 10 + ((C - '0') + ... + 0); }\n")
put("fe26-named-call", {
 "prelude": LOT,
 "template": "{prelude}\n{row}\n{col}",
 "rows": {
  "the literal 12_w gives 23": "static_assert(12_w == 23);",
  "operator\"\"_w<'1', '2'>() gives 23": "static_assert(operator\"\"_w<'1', '2'>() == 23);",
  "operator\"\"_w<>() gives 0": "static_assert(operator\"\"_w<>() == 0);",
  "operator\"\"_w() with no argument list deduces an empty pack and gives 0": "static_assert(operator\"\"_w() == 0);"
 },
 "cols": {"program": ""},
 "flags": []
})

# Q-0445 and RV9-R-4: an attribute after the class key in a declaration
# that holds only an elaborated type with a template-id.
put("q0445-template-id", {
 "prelude": "template <class T> struct S;\ntemplate <class T> struct S { };\n",
 "template": "{prelude}\n{row}\n{col}",
 "rows": {
  "explicit specialization declared only: template <> struct [[deprecated]] S<int>;":
   "template <class T> struct P;\ntemplate <> struct [[deprecated]] P<int>;",
  "partial specialization declared only: template <class T> struct [[deprecated]] P<T*>;":
   "template <class T> struct P;\ntemplate <class T> struct [[deprecated]] P<T*>;",
  "explicit instantiation: template struct [[deprecated]] S<int>;":
   "template struct [[deprecated]] S<int>;",
  "control: forward declaration struct [[deprecated]] F;":
   "struct [[deprecated]] F;",
  "control: variable struct [[deprecated]] F* p;":
   "struct F;\nstruct [[deprecated]] F* p = nullptr;"
 },
 "cols": {"program": ""},
 "flags": []
})

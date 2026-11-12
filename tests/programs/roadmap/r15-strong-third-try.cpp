// tidal-rules: TARG-10 TARG-18 CNS-35 PACK-5 DEP-25
// tidal-expect: run
// tidal-note: Roadmap round 15, programmer sketch strong_third_try.cpp (reports/clean-room/roadmap-15/programmer.md, section 1.4). The third try builds clean on GCC and Clang, so it is a program test (decision 0061). It differs from the last version in skills that are class templates, passed as a pack of template template parameters and expanded in the base list of Strong, and in a divide skill named through the alias template PerSecond with one parameter, in place of skills that are classes with a member template apply reached through Mix and an alias template. The rules are the ones the report names for this program and its reductions.
// tidal-note: A program test is evidence from use and decides no rule alone (decision 0054).
// Strong types: a value wrapped with a tag, and its operations chosen
// from a list of skills, each a CRTP mixin template. Binary skills
// between two strong types come from a member template of a helper.
// No standard header.

using size_t = decltype(sizeof 0);

namespace st {

template <class T> struct remove_ref { using type = T; };
template <class T> struct remove_ref<T&> { using type = T; };
template <class T> struct remove_ref<T&&> { using type = T; };

template <class T>
constexpr typename remove_ref<T>::type&& move(T&& t) noexcept {
  return static_cast<typename remove_ref<T>::type&&>(t);
}

template <class A, class B> inline constexpr bool same = false;
template <class A> inline constexpr bool same<A, A> = true;

template <class A, class B>
concept same_as = same<A, B>;

template <class N>
concept number = same<N, int> || same<N, long> || same<N, long long> || same<N, unsigned>;

template <class T, class Tag, template <class> class... Skills>
class Strong : public Skills<Strong<T, Tag, Skills...>>... {
 public:
  using value_type = T;
  using tag_type = Tag;

  constexpr Strong() = default;
  constexpr explicit Strong(T v) : v_(move(v)) {}

  constexpr T& get() & { return v_; }
  constexpr const T& get() const& { return v_; }
  constexpr T&& get() && { return move(v_); }

 private:
  T v_{};
};

template <class S> struct Equal {
  friend constexpr bool operator==(const S& a, const S& b) { return a.get() == b.get(); }
};

template <class S> struct Ordered {
  friend constexpr bool operator<(const S& a, const S& b) { return a.get() < b.get(); }
  friend constexpr bool operator>(const S& a, const S& b) { return b < a; }
  friend constexpr bool operator<=(const S& a, const S& b) { return !(b < a); }
  friend constexpr bool operator>=(const S& a, const S& b) { return !(a < b); }
};

template <class S> struct Addable {
  friend constexpr S operator+(const S& a, const S& b) { return S(a.get() + b.get()); }
  friend constexpr S& operator+=(S& a, const S& b) {
    a.get() += b.get();
    return a;
  }
};

template <class S> struct Subtractable {
  friend constexpr S operator-(const S& a, const S& b) { return S(a.get() - b.get()); }
};

template <class S> struct Scalable {
  template <class N>
    requires number<N>
  friend constexpr S operator*(const S& s, N n) { return S(s.get() * n); }
  template <class N>
    requires number<N>
  friend constexpr S operator*(N n, const S& s) { return S(s.get() * n); }
};

template <class S> struct Incrementable {
  constexpr S& operator++(this S& self) {
    ++self.get();
    return self;
  }
  constexpr S operator++(this S& self, int) {
    S old = self;
    ++self.get();
    return old;
  }
};

template <class S> struct Hashable {
  friend constexpr size_t hash(const S& s) {
    size_t h = 14695981039346656037ull;
    auto v = static_cast<unsigned long long>(s.get());
    for (int i = 0; i < 8; ++i) {
      h ^= (v >> (i * 8)) & 0xff;
      h *= 1099511628211ull;
    }
    return h;
  }
};

// A skill that divides one strong type by another and gives a third.
// A program names it through an alias template with one parameter.
template <class S, class Divisor, class Result>
struct Divides {
  friend constexpr Result operator/(const S& a, const Divisor& b) {
    return Result(a.get() / b.get());
  }
};

template <class S>
concept addable = requires(const S& a) { a + a; };
template <class S>
concept ordered = requires(const S& a) { a < a; };
template <class S>
concept hashable = requires(const S& a) { { hash(a) } -> same_as<size_t>; };

}  // namespace st

namespace units {

using st::Strong;

using Seconds = Strong<long long, struct SecondsTag, st::Equal, st::Ordered, st::Addable, st::Subtractable>;
using Speed = Strong<long long, struct SpeedTag, st::Equal, st::Ordered>;
template <class S> using PerSecond = st::Divides<S, Seconds, Speed>;
using Meters = Strong<long long, struct MetersTag, st::Equal, st::Ordered, st::Addable,
                      st::Subtractable, st::Scalable, PerSecond>;
using Id = Strong<unsigned, struct IdTag, st::Equal, st::Incrementable, st::Hashable>;

}  // namespace units

using namespace units;

static_assert(st::addable<Meters>);
static_assert(!st::addable<Id>);
static_assert(st::ordered<Seconds>);
static_assert(!st::ordered<Id>);
static_assert(st::hashable<Id>);
static_assert(!st::hashable<Meters>);
static_assert(sizeof(Meters) == sizeof(long long));
static_assert(st::same<Meters::tag_type, units::MetersTag>);

constexpr Speed average(const Meters* legs, const Seconds* times, int n) {
  Meters d{};
  Seconds t{};
  for (int i = 0; i < n; ++i) {
    d += legs[i];
    t += times[i];
  }
  return d / t;
}

constexpr bool meters_test() {
  Meters a{120}, b{30};
  Meters c = a - b + 2 * b * 1;
  if (c != Meters{150}) return false;
  if (!(b < a) || a <= b || !(a >= a)) return false;
  Meters legs[] = {Meters{100}, Meters{300}, Meters{200}};
  Seconds times[] = {Seconds{10}, Seconds{20}, Seconds{30}};
  return average(legs, times, 3) == Speed{10};
}
static_assert(meters_test());

struct Table {
  static constexpr int cap = 8;
  Id keys[cap]{};
  int vals[cap]{};
  bool used[cap]{};

  constexpr bool put(Id k, int v) {
    for (size_t i = hash(k) % cap, n = 0; n < cap; i = (i + 1) % cap, ++n) {
      if (!used[i] || keys[i] == k) {
        keys[i] = k;
        vals[i] = v;
        used[i] = true;
        return true;
      }
    }
    return false;
  }
  constexpr const int* find(Id k) const {
    for (size_t i = hash(k) % cap, n = 0; n < cap; i = (i + 1) % cap, ++n) {
      if (!used[i]) return nullptr;
      if (keys[i] == k) return &vals[i];
    }
    return nullptr;
  }
};

constexpr int id_test() {
  Table t;
  Id id{1};
  for (int i = 0; i < 6; ++i) t.put(id++, i * i);
  Id probe{3};
  const int* p = t.find(probe);
  const int* q = t.find(++probe);
  const int* none = t.find(Id{40});
  return (p ? *p : -1) * 100 + (q ? *q : -1) + (none ? 1000 : 0);
}
static_assert(id_test() == 409);

int main() {
  if (!meters_test()) return 1;
  if (id_test() != 409) return 2;
  Id a{7};
  Id b = a++;
  if (b.get() != 7 || a.get() != 8) return 3;
  return 0;
}

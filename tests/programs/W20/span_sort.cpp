// tidal-rules: CTAD-1 LAM-1 LAM-12 TPL-13 ADD-6 STG-5
// tidal-expect: run
// tidal-note: Write suite W20, C# persona, program span_sort.cpp (reports/clean-room/W20/report.md). The rules are the ones the report notes before writing for the program's constructs.
// tidal-note: A program test is evidence from use and decides no rule alone (decision 0054).
// Span<T> over a pointer and a length, and a sort over a Span.
// Rules used: DED-P3 (T[N] gives T and N), CTAD-1 (implicit guide from the
// array constructor), TPL-13 (default template argument), LAM-1 and
// LAM-12 (a lambda as the comparator), ADD-6 (pointer arithmetic over
// array elements), and the StaticVector rules of static_vector.cpp.
// STG-P2 and STG-29: the elements of StaticVector live in separate rows,
// so a T* to one element does not reach the next (ADD-10). A Span over a
// StaticVector is therefore a Span<T*> over the vector's array of the
// pointers that placement new returned, and the sort reorders those
// pointers through an Indirect comparator.

namespace std { using size_t = decltype(sizeof 0); }
inline void* operator new(std::size_t, void* at) noexcept { return at; }

template <class T, int N>
class StaticVector {
  alignas(T) unsigned char rows[N][sizeof(T)];
  T* items[N];
  int count = 0;
public:
  StaticVector() = default;
  StaticVector(const StaticVector& o) {
    for (int i = 0; i < o.count; ++i) emplace_back(*o.items[i]);
  }
  StaticVector(StaticVector&& o) {
    for (int i = 0; i < o.count; ++i) emplace_back(static_cast<T&&>(*o.items[i]));
    o.clear();
  }
  StaticVector& operator=(const StaticVector& o) {
    if (this != &o) {
      clear();
      for (int i = 0; i < o.count; ++i) emplace_back(*o.items[i]);
    }
    return *this;
  }
  StaticVector& operator=(StaticVector&& o) {
    if (this != &o) {
      clear();
      for (int i = 0; i < o.count; ++i) emplace_back(static_cast<T&&>(*o.items[i]));
      o.clear();
    }
    return *this;
  }
  ~StaticVector() { clear(); }

  template <class... Args>
  T& emplace_back(Args&&... args) {
    T* p = new (rows[count]) T(static_cast<Args&&>(args)...);
    items[count++] = p;
    return *p;
  }
  void pop_back() { items[--count]->~T(); }
  void clear() { while (count > 0) pop_back(); }
  int size() const { return count; }
  static constexpr int capacity() { return N; }
  T& operator[](int i) { return *items[i]; }
  T** slots() { return items; }   // the pointers, in element order
  const T& operator[](int i) const { return *items[i]; }
};

template <class T>
class Span {
  T* first;
  std::size_t length;
public:
  constexpr Span(T* p, std::size_t n) : first(p), length(n) {}
  template <std::size_t N>
  constexpr Span(T (&a)[N]) : first(a), length(N) {}
  constexpr std::size_t size() const { return length; }
  constexpr T& operator[](std::size_t i) const { return first[i]; }
};

struct LessThan {
  template <class A>
  constexpr bool operator()(const A& a, const A& b) const { return a < b; }
};

// Compares two pointers by the objects they point to.
template <class Less = LessThan>
struct Indirect {
  Less less;
  template <class P>
  constexpr bool operator()(P a, P b) const { return less(*a, *b); }
};

template <class T, int N>
Span<T*> pointers(StaticVector<T, N>& v) { return Span<T*>(v.slots(), v.size()); }

template <class T, class Less = LessThan>
constexpr void sort(Span<T> s, Less less = Less{}) {
  // insertion sort
  for (std::size_t i = 1; i < s.size(); ++i) {
    for (std::size_t j = i; j > 0 && less(s[j], s[j - 1]); --j) {
      T tmp = static_cast<T&&>(s[j]);
      s[j] = static_cast<T&&>(s[j - 1]);
      s[j - 1] = static_cast<T&&>(tmp);
    }
  }
}

template <class T>
constexpr bool is_sorted(Span<T> s) {
  for (std::size_t i = 1; i < s.size(); ++i)
    if (s[i] < s[i - 1]) return false;
  return true;
}

constexpr bool sort_ints() {
  int numbers[] = {5, 3, 9, 1, 4, 1};
  sort(Span(numbers));
  return is_sorted(Span(numbers)) && numbers[0] == 1 && numbers[5] == 9;
}
static_assert(sort_ints());

struct Person {
  const char* name;
  int age;
};

int run() {
  if (!sort_ints()) return 1;
  Person people[] = {{"Ann", 41}, {"Bo", 23}, {"Cy", 35}};
  sort(Span(people), [](const Person& a, const Person& b) { return a.age < b.age; });
  if (people[0].age != 23 || people[2].name[0] != 'A') return 2;

  StaticVector<int, 8> v;
  v.emplace_back(7);
  v.emplace_back(2);
  v.emplace_back(5);
  sort(pointers(v), Indirect<>{});
  if (v[0] != 2 || v[1] != 5 || v[2] != 7) return 3;

  StaticVector<Person, 4> pv;
  pv.emplace_back(Person{"Dee", 50});
  pv.emplace_back(Person{"Ed", 10});
  sort(pointers(pv), Indirect{[](const Person& a, const Person& b) { return a.age < b.age; }});
  if (pv[0].age != 10) return 4;
  return 0;
}

int main() { return run(); }

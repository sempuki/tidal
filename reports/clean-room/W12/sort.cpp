// W12: a sort that takes a comparator.

// Insertion sort. `less` is any callable: a function pointer, a closure,
// or a class with operator() (PFX-17). Taken by value (DED-6).
template <class T, class Less>
constexpr void sort(T* first, T* last, Less less) {
  for (T* i = first; i != last; ++i) {
    T v = *i;
    T* j = i;
    while (j != first && less(v, *(j - 1))) {
      *j = *(j - 1);
      --j;
    }
    *j = v;
  }
}

template <class T, class Less>
constexpr bool is_sorted(const T* first, const T* last, Less less) {
  for (const T* i = first; i != last && i + 1 != last; ++i)
    if (less(*(i + 1), *i)) return false;
  return true;
}

// Sort by a key that a pointer to data member names (MPTR-3).
template <class T, class K>
constexpr void sort_by(T* first, T* last, K T::* key) {
  sort(first, last, [key](const T& a, const T& b) { return a.*key < b.*key; });
}

constexpr bool ascending(int a, int b) { return a < b; }

struct Person {
  int age;
  int height;
  constexpr bool older(const Person& other) const { return age > other.age; }
};

constexpr int check_function_pointer() {
  int a[] = {5, 2, 9, 1, 7};
  sort(a, a + 5, ascending);                 // function decays (DED-6)
  return is_sorted(a, a + 5, ascending) && a[0] == 1 && a[4] == 9;
}
static_assert(check_function_pointer());

constexpr int check_lambda_descending() {
  int a[] = {5, 2, 9, 1, 7};
  sort(a, a + 5, [](int x, int y) { return x > y; });
  return a[0] == 9 && a[4] == 1;
}
static_assert(check_lambda_descending());

constexpr int check_converted_pointer() {
  bool (*cmp)(int, int) = [](int x, int y) { return x < y; };   // LAM-10
  int a[] = {3, 1, 2};
  sort(a, a + 3, cmp);
  return a[0] == 1 && a[1] == 2 && a[2] == 3;
}
static_assert(check_converted_pointer());

constexpr int check_generic_and_capture() {
  double d[] = {2.5, -1.0, 0.5};
  auto generic = [](const auto& x, const auto& y) { return x < y; };  // LAM-15
  sort(d, d + 3, generic);
  int pivot = 10;
  int calls = 0;
  int a[] = {14, 3, 11, 9, 20};
  // order by distance from pivot, and count comparisons by reference
  sort(a, a + 5, [pivot, &calls](int x, int y) {
    ++calls;
    int dx = x < pivot ? pivot - x : x - pivot;
    int dy = y < pivot ? pivot - y : y - pivot;
    return dx < dy;
  });
  return d[0] == -1.0 && d[2] == 2.5 && a[0] == 11 && a[1] == 9 &&
         a[4] == 20 && calls > 0;
}
static_assert(check_generic_and_capture());

constexpr int check_member_pointers() {
  Person p[] = {{40, 170}, {20, 180}, {30, 160}};
  sort_by(p, p + 3, &Person::age);
  bool by_age = p[0].age == 20 && p[2].age == 40;
  sort_by(p, p + 3, &Person::height);
  bool by_height = p[0].height == 160 && p[2].height == 180;
  // a pointer to member function as the comparator
  bool (Person::*older)(const Person&) const = &Person::older;
  sort(p, p + 3, [older](const Person& a, const Person& b) {
    return (a.*older)(b);                    // MPTR-10: parentheses needed
  });
  bool by_older = p[0].age == 40 && p[2].age == 20;
  return by_age && by_height && by_older;
}
static_assert(check_member_pointers());

// A class with operator() as a comparator.
struct ByLastDigit {
  constexpr bool operator()(int a, int b) const { return a % 10 < b % 10; }
};
constexpr int check_function_object() {
  int a[] = {19, 22, 31, 48};
  sort(a, a + 4, ByLastDigit{});
  return a[0] == 31 && a[1] == 22 && a[2] == 48 && a[3] == 19;
}
static_assert(check_function_object());

int main() {
  int failures = 0;
  if (!check_function_pointer()) ++failures;
  if (!check_lambda_descending()) ++failures;
  if (!check_converted_pointer()) ++failures;
  if (!check_generic_and_capture()) ++failures;
  if (!check_member_pointers()) ++failures;
  if (!check_function_object()) ++failures;
  return failures;
}

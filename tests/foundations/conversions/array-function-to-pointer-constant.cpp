// tidal-rules: CONV-13
// tidal-expect: accept
// tidal-note: A string literal and a constexpr array give pointers to element zero, and two function names give pointers in a conditional expression.
constexpr const char* s = "abc";
static_assert(s[2] == 'c');
constexpr int arr[2] = {4, 5};
constexpr const int* p = arr;
static_assert(*p == 4 && p[1] == 5);
void f();
void g();
void (*r)() = true ? f : g;

// tidal-rules: DCL-2
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
int x = 0;
int* p = &x;
int** pp = &p;
void f();
void (*pf)() = f;
void (*pg)() = &f;
static_assert(same<decltype(*pp), int*&>);
int main_value() { return **pp; }

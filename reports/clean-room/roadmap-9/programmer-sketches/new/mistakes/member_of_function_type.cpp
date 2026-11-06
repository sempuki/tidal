template <class F> struct Holder { F f; };
template <class F> struct remove_ref { using type = F; };
template <class F> struct remove_ref<F&> { using type = F; };
template <class F> int keep(F&&) { Holder<typename remove_ref<F>::type> h; (void)h; return 0; }
int square(int x) { return x * x; }
int main() { return keep(square); }

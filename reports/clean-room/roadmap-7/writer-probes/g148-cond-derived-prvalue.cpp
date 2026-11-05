// G148: a conditional expression with a Derived prvalue and a Base xvalue.
// The probe asks whether the result is an xvalue of Base.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
template <class T> T&& move_(T& t) { return static_cast<T&&>(t); }
struct Base {};
struct Derived : Base {};
Base b;
bool c = true;
using T = decltype(c ? Derived{} : move_(b));
static_assert(same<T, Base&&>, "xvalue");
int main() {}

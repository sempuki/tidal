template <class T> int f(T&) = delete;
namespace n { struct C {}; int f(C&) { return 0; } }
template <class T> concept has_f = requires(T& a) { f(a); };
static_assert(has_f<n::C>);
int main() { return 0; }

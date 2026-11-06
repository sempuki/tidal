// ICS-45: an explicit conversion function to Derived& does not serve static_cast<Base&>.
struct Base {};
struct Derived : Base {};
Derived dd;
struct ToDerivedRef { explicit operator Derived&() { return dd; } };
template <class T, class S> concept makes = requires(S s) { static_cast<T>(s); };
int main() { return makes<Base&, ToDerivedRef> ? 1 : 0; }

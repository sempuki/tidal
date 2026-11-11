// tidal-rules: CV-6
// tidal-expect: accept
// tidal-note: Gap review RV6-A-10. A cv-qualifier that reaches an array type or a function type through decltype acts as through an alias. The const applies to the elements of the array, and has no effect on the function type.
template <class A, class B> struct same { static constexpr bool value = false; };
template <class A> struct same<A, A> { static constexpr bool value = true; };
int values[2] = {1, 2};
const decltype(values) fixed = {1, 2};
static_assert(same<decltype(fixed[0]), const int&>::value);
void tick() {}
const decltype(tick)* hook = tick;
static_assert(same<decltype(hook), void (*)()>::value);
int main() { hook(); return fixed[1] == 2 ? 0 : 1; }

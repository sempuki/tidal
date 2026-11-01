// tidal-rules: CTAD-28 CTAD-8 CTAD-24
// tidal-expect: reject
// tidal-expect-edg: accept
// tidal-error-clang: (no viable constructor or deduction guide|requires template arguments; argument deduction only allowed for class templates)
// tidal-error-gcc: class template argument deduction failed
// tidal-note: Through the alias, T&& still names a class parameter, so it is no forwarding reference and does not take the lvalue i. EDG accepts (Q-0278).
template <class T> struct Ref { constexpr Ref(T&&) {} };
template <class T> using Alias = Ref<T>;
int i;
Alias a(i);

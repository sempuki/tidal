// tidal-rules: DEP-34
// tidal-expect: reject
// tidal-expect-edg: accept
// tidal-error-clang: 'template' keyword not permitted in destructor name
// tidal-error-gcc: .template. keyword not permitted in destructor name
// tidal-note: The keyword template comes before a destructor name in a member access with a dot. Clang and GCC reject the template, EDG accepts it (Q-0245).
struct Item {};
template <class T> void end(T& r) { r.template ~T(); }
void use(Item& item) { end(item); }

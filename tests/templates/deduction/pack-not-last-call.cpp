// tidal-rules: DED-20 DED-35
// tidal-expect: reject
// tidal-error-clang: requires 1 argument, but 2 were provided
// tidal-error-gcc: no matching function for call to
// tidal-error-edg: no instance of function template
// tidal-note: The pack deduces nothing and is empty, so lead takes one argument, not two.
template <class... Ts> void lead(Ts..., int);
void use() { lead(1, 2); }

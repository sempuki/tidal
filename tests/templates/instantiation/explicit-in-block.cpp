// tidal-rules: INST-21
// tidal-expect: reject
// tidal-error-clang: expected '<' after 'template'|expected expression
// tidal-error-gcc: expected primary-expression before .template.
// tidal-error-edg: explicit instantiation is not allowed in the current scope
template <class T> int get(T) { return 1; }
void run() { template int get(int); }

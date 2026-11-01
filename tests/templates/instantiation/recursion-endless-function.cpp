// tidal-rules: INST-15
// tidal-expect: reject
// tidal-error-clang: in instantiation of function template specialization 'depth<[0-9]{3,}>'
// tidal-error-gcc: In instantiation of .constexpr int depth\(\) \[with int N = [0-9]{3,}\].
// tidal-error-edg: excessive recursion at instantiation
// tidal-note: depth<N> needs depth<N + 1>, with no end. The behavior is undefined, and every compiler stops at its depth limit.
template <int N> constexpr int depth() { return depth<N + 1>() + 1; }
int x = depth<0>();

// tidal-rules: INST-1 INST-6
// tidal-expect: reject
// tidal-error-clang: static assertion failed
// tidal-error-gcc: static assertion failed
// tidal-error-edg: static assertion failed
// tidal-note: Gap review RV7-B-6. make() is the argument of take, not the whole operand of decltype, so the use instantiates Broken<int> and its static_assert fails. Every compiler rejects, also when take takes a reference to const (writer probe roadmap-11/writer-probes/rv7-points.json).
template <class T> struct Broken { static_assert(sizeof(T) == 0); };
Broken<int> make();
int take(Broken<int>);
using Result = decltype(take(make()));
Result* none = nullptr;

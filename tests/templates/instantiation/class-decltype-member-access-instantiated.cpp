// tidal-rules: INST-1 INST-6
// tidal-expect: reject
// tidal-error-clang: static assertion failed
// tidal-error-gcc: static assertion failed
// tidal-error-edg: static assertion failed
// tidal-note: Gap review RV7-B-6. make() is the object of a member access inside decltype, not the whole operand, so the use instantiates Broken<int> and its static_assert fails. Every compiler rejects, also for make() on either side of a comma (writer probe roadmap-11/writer-probes/rv7-points.json).
template <class T> struct Broken { static_assert(sizeof(T) == 0); int size; };
Broken<int> make();
using Result = decltype(make().size);
Result* none = nullptr;

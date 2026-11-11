// tidal-rules: INST-2
// tidal-expect: accept
// tidal-note: Gap review RV7-B-6. The call make() in parentheses is still the whole operand of decltype, so the use does not instantiate Broken<int>, and its static_assert does not fire. Every compiler accepts (writer probe roadmap-11/writer-probes/rv7-points.json).
template <class T> struct Broken { static_assert(sizeof(T) == 0); };
Broken<int> make();
using Result = decltype((make()));
Result* none = nullptr;

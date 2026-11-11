// tidal-rules: INST-2
// tidal-expect: accept
// tidal-note: I6-13. The call make() appears only as the operand of decltype, and its result type is Broken<int>. The use does not instantiate Broken<int>, so its static_assert does not fire.
template <class T> struct Broken { static_assert(sizeof(T) == 0); };
Broken<int> make();
using Result = decltype(make());
Result* none = nullptr;

// tidal-rules: INST-2 INST-1
// tidal-expect: accept
// tidal-note: Read of the RV7 repair diff, RV8-R-4. make is consteval, so a call to it is an immediate invocation, and its result type is Broken<int>. The call is the whole operand of decltype, with and without parentheses, and every compiler leaves Broken<int> not instantiated, also for a consteval function template. The review reads an immediate invocation as needing the class complete. The sizeof control of the writer's probe instantiates the class.
template <class T> struct Broken { static_assert(sizeof(T) == 0); };
consteval Broken<int> make();
template <class T> consteval Broken<T> build() { return {}; }
using Plain = decltype(make());
using Parenthesized = decltype((make()));
using FromTemplate = decltype(build<int>());
Plain* none = nullptr;

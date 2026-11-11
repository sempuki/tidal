// tidal-rules: INST-2 INST-1
// tidal-expect: accept
// tidal-note: Read of the RV7 repair diff, RV8-R-4, and the read of the RV8 repair diff, RV9-R-2. make is consteval, and its result type is Broken<int>. The call is the whole operand of decltype, with and without parentheses. That operand is not evaluated, so the call is no immediate invocation and needs no complete class. Every compiler leaves Broken<int> not instantiated, also for a consteval function template. The sizeof control of the writer's probe instantiates the class, because sizeof needs a complete type.
template <class T> struct Broken { static_assert(sizeof(T) == 0); };
consteval Broken<int> make();
template <class T> consteval Broken<T> build() { return {}; }
using Plain = decltype(make());
using Parenthesized = decltype((make()));
using FromTemplate = decltype(build<int>());
Plain* none = nullptr;

// tidal-rules: ICS-7
// tidal-expect: accept
// tidal-note: The constructor and the conversion function compete as functions. The argument's sequence to the constructor's parameter and to the conversion function's object parameter decides first: binding s to S& beats binding it to const S&.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
struct Target;
struct Source { operator Target(); };
struct Target { Target(); Target(const Source&); };
Source s;
Target t = s;
One take(Target);
Two take(...);
static_assert(same<decltype(take(s)), One>);
struct Goal;
struct Maker { operator Goal() const; };
struct Goal { Goal(); Goal(Maker&); };
Maker m;
Goal g = m;

// tidal-rules: ICS-7
// tidal-expect: accept
// tidal-note: int to Both has two equally good conversions, so its sequence is the ambiguous sequence. The candidate stays and loses to a standard sequence.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
struct Left { Left(int); };
struct Right { Right(int); };
struct Both { Both(Left); Both(Right); };
One take(Both);
Two take(int);
static_assert(same<decltype(take(1)), Two>);
struct Source { operator int(); operator long(); };
One look(double);
Two look(Source);
static_assert(same<decltype(look(Source{})), Two>);

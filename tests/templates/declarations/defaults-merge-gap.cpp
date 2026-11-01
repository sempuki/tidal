// tidal-rules: TPL-14
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-error-clang: template parameter missing a default argument
// tidal-error-edg: default argument not at end of
// tidal-note: The second declaration gives a default to the first parameter. No declaration gives one to the second parameter, so a parameter with a default comes before one without. GCC checks only the defaults written on one declaration and accepts (Q-0296).
template <class T, class U> struct Pair;
template <class T = char, class U> struct Pair;

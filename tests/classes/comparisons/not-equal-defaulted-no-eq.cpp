// tidal-rules: DCMP-11
// tidal-expect: reject
// tidal-error-clang: operator!=. is implicitly deleted
// tidal-error-gcc: use of deleted function .*operator!=
// tidal-error-edg: no operator .!=. matches
// tidal-note: S has no ==, so the defaulted != is deleted. EDG leaves the deleted function out of the candidates and finds no match, as for secondary-without-spaceship.cpp.
struct S { int x; bool operator!=(const S&) const = default; };
bool b = S{1} != S{2};

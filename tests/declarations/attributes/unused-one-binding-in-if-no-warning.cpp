// tidal-rules: ATTR-6
// tidal-expect: accept
// tidal-flags: -Werror -Wunused
// tidal-note: The condition reads length and never reads start. The declaration counts as read.
struct Span { int start; int length; };
int f(Span s) {
  if (auto [start, length] = s; length > 2) return 1;
  return 0;
}

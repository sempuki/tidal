// tidal-rules: ATTR-18
// tidal-expect: accept
// tidal-note: An attribute specifier after a * with no cv-qualifiers. Q-0123 concerns only the order of an attribute and cv-qualifiers.
int i = 0;
int * [[]] r = &i;
int * [[]] * [[]] s = &r;

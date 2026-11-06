#!/bin/sh
# $1 = test file, $2 = inject header.
# ALIGNED-NEW-WARN: GCC's -Waligned-new=all, which names a new expression for an
#   over-aligned type that calls a class allocation function without an alignment parameter.
# PLACEMENT-OVERALIGNED: with the inject header, a placement new of an over-aligned type
#   through a void* argument selects the deleted aligned placement form.
# INJECT-CHANGED: the inject header changes the compile result for another reason (review by hand).
f="$1"
d=$(dirname "$f")
a=$(g++ -std=c++23 -fsyntax-only -Waligned-new=all -I"$d" "$f" 2>&1 | grep -c "Waligned-new")
g++ -std=c++23 -fsyntax-only -w -I"$d" "$f" >/dev/null 2>&1; s0=$?
out=$(g++ -std=c++23 -fsyntax-only -w -I"$d" -include "$2" "$f" 2>&1); s1=$?
b=$(printf '%s\n' "$out" | grep -c "use of deleted function .*align_val_t.*void\*")
[ "$a" != 0 ] && echo "ALIGNED-NEW-WARN $f $a"
[ "$b" != 0 ] && echo "PLACEMENT-OVERALIGNED $f $b"
[ "$b" = 0 ] && [ "$s0" != "$s1" ] && echo "INJECT-CHANGED $f $s0 $s1"
exit 0

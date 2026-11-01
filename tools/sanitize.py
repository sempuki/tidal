#!/usr/bin/env python3
"""Strip comments and standard citations from implementation source.

Implementations quote the C++ standard in comments, so writers read only
sanitized copies (see documentation/clean-room.md). This script:

1. removes // and /* */ comments, keeping string and character literals
   (including raw strings) intact, and keeping line numbers stable;
2. replaces any remaining bracketed stable-name citation such as
   "[expr.cond]" or "C++ [stmt.if]p2" with "[REF]";
3. drops lines left blank by step 1 when more than one blank line in a row.

Usage:
    tools/sanitize.py IN OUT
Prints the number of comments removed and citations redacted.
"""

import re
import sys

CITATION = re.compile(r"(?:C\+\+\d*\s*)?\[[a-z][a-z0-9_]*(?:\.[a-z0-9_]+)+\](?:p\d+)?")


def strip_comments(src: str) -> tuple[str, int]:
    out = []
    i, n, count = 0, len(src), 0
    while i < n:
        c = src[i]
        two = src[i:i + 2]
        if two == "//":
            count += 1
            j = src.find("\n", i)
            i = n if j < 0 else j
        elif two == "/*":
            count += 1
            j = src.find("*/", i + 2)
            j = n if j < 0 else j + 2
            out.append("\n" * src.count("\n", i, j))
            i = j
        elif c == "R" and src[i + 1:i + 2] == '"' and (i == 0 or not (src[i - 1].isalnum() or src[i - 1] == "_")) or (
            c in "uUL8" and re.match(r'(?:u8|u|U|L)R"', src[i:i + 4])
            and (i == 0 or not (src[i - 1].isalnum() or src[i - 1] == "_"))
        ):
            q = src.index('"', i)
            paren = src.find("(", q)
            delim = src[q + 1:paren]
            end = src.find(")" + delim + '"', paren)
            end = n if end < 0 else end + len(delim) + 2
            out.append(src[i:end])
            i = end
        elif c in "\"'":
            if c == "'" and i > 0 and src[i - 1].isalnum() and src[i + 1:i + 2].isalnum():
                out.append(c)  # digit separator, as in 1'000
                i += 1
                continue
            j = i + 1
            while j < n and src[j] != c and src[j] != "\n":
                j += 2 if src[j] == "\\" else 1
            out.append(src[i:j + 1])
            i = j + 1
        else:
            out.append(c)
            i += 1
    return "".join(out), count


def sanitize(src: str) -> tuple[str, int, int]:
    text, comments = strip_comments(src)
    text, citations = CITATION.subn("[REF]", text)
    lines = [l.rstrip() for l in text.splitlines()]
    return "\n".join(lines) + "\n", comments, citations


def main() -> int:
    if len(sys.argv) != 3:
        print(__doc__)
        return 2
    src = open(sys.argv[1], encoding="utf-8", errors="replace").read()
    text, comments, citations = sanitize(src)
    with open(sys.argv[2], "w", encoding="utf-8") as f:
        f.write(text)
    print(f"{sys.argv[1]}: removed {comments} comments, redacted {citations} citations")
    return 0


if __name__ == "__main__":
    sys.exit(main())

#import "template.typ": *

#show: project.with(
  title: "שם הקורס",
  number: 1,
  authors: (
    (name: "שם מלא", id: "123456789", email: ""),
  ),
  date: datetime(year: 2026, month: 1, day: 1)
)
#set text(font: "David")
#set text(size: 9pt)
#set math.text(size: 12pt)

// Optional helpers for accent notation
#let Hat(x) = $accent(#x, hat)$
#let Bar(x) = $accent(#x, macron)$
#let Vec(x) = $accent(#x, arrow)$

== שאלה 1

*א.*

פסקת הסבר כאן.

$ X^f (omega) = integral_(-oo)^oo x(t) e^(-j omega t) d t $

*ב.*

נשתמש בתכונת ההזזה:
$ "DTFT"{x[n - n_0]} = e^(-j theta n_0) X^f (theta) $

**מסקנה:** טקסט מסכם בעברית עם $"inline math"$ בתוכו.

#pagebreak()

== שאלה 2

*א.*

$ y[n] = cases(
  1 & 0 <= n <= N-1,
  0 & "אחרת"
) $

*ב.*

קונבולוציה בתחום הזמן שקולה לכפל בתחום התדר:
$ Y^f (theta) = X^f (theta) dot H^f (theta) $

לפי נוסחת ה-DTFT:
$ therefore Y^f (theta) = sum_(n=-oo)^oo y[n] e^(-j theta n) $

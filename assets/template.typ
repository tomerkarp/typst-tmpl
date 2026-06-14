#let project(
  title: "",
  number: 0,
  authors: (),
  date: none,
  logo: none,
  body,
) = {
  set document(author: authors.map(a => a.name), title: title)
  set text(font: "David", lang: "he")
  show math.equation: set text(weight: 400)
  set heading(numbering: none)

  // Title page.
  v(0.6fr)
  if logo != none {
    align(right, image(logo, width: 26%))
  }
  v(9.6fr)

  text(2em, weight: 700, title)
  v(1.2em, weak: true)
  text(size: 1.5em)[גיליון #number #date.display(" [day]/[month]/[year]")]

  // Author information.
  pad(
    top: 0.7em,
    right: 0%,
    grid(
      columns: (1fr,) * calc.min(3, authors.len()),
      gutter: 1em,
      ..authors.map(author => align(start)[
        *#author.name* \
        #author.id \
        #author.email
      ]),
    ),
  )

  v(2.4fr)
  pagebreak()

  set page(numbering: "1", number-align: center)

  // Main body.
  set par(justify: true)

  body
}

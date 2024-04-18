# Planung für [Einstieg in Embedded Rust mit dem Raspberry Pico WH](https://www.luga.de/static/LIT-2024/talks/einstieg_in_embedded_rust_mit_dem_raspberry_pico_wh/) für den [Augsburger Linux-Infotag 2024](https://www.luga.de/static/LIT-2024)

um 10:45 im Raum  Raum F für 105min

Tauchen Sie mit unserem Workshop in die Welt der Embedded-Systeme ein, der sich auf Rust und den Raspberry Pi Pico WH konzentriert. Vorkenntnisse in Programmierung (unabhängig von der Sprache) und erste Erfahrungen im Embedded-Bereich sind empfehlenswert. Der Raspberry Pi Pico WH wird aufgrund seines günstigen Preises und der umfangreichen Auswahl an Peripheriegeräten als Lehrmittel eingesetzt.

Teilnehmende lernen, wie sie ihre Entwicklungsumgebung einrichten, einfache Programme wie das 'Blinky'-Beispiel implementieren und Eingaben über Taster verarbeiten.

Dieser Workshop ist darauf ausgelegt, einen praktischen Einstieg in die Programmierung von Embedded-Systemen mit Rust zu bieten. Er richtet sich an alle, die Interesse daran haben, wie man mit Rust effiziente und sichere Software für Mikrocontroller entwickelt. Begleiten Sie uns, um die Grundlagen zu erlernen und erste Schritte in der Entwicklung von Embedded-Projekten mit dem Raspberry Pi Pico WH zu machen.

Ihr könnt einfach zuhören oder mitmachen. Technische Voraussetzungen sind ein Notebook, am besten mit Linux und Rust vorinstalliert, sowie eines Micro-USB-Kabel. Ideal wäre zudem die Verfügbarkeit eines Raspberry Pi Pico oder Pico W oder Pico WH und eines Debug Probes. Für bis zu zehn Teilnehmende können die Picos gestellt werden.

# Kanban Board

Ein [Kanban Board](https://github.com/orgs/rust-augsburg/projects/1/views/1) ist auch schon eingerichtet. Damit könnt ihr den aktuellen Bearbeitungsstand verfolgen und eigene Aufgaben eintragen.


# Verzeichnisstruktur

`praesentation` enthält die Präsentation.
`doc` Dokumentation von Claudia und Memory Game
`memory-game` Code

## Lokales Erstellung der Präsentation

Die Präsentation wird mit [mdBook](https://rust-lang.github.io/mdBook/) erstellt. Falls ihr mdBook noch nicht kennt, seid ihr ihm möglicherweise unbewusst bereits begegnet. Ein Beispiel dafür ist das [Rust Buch](https://doc.rust-lang.org/stable/book/), welches damit erstellt wurde. Weitere Informationen findet ihr unter: <https://rust-lang.github.io/mdBook/>."


```sh
# ggf. Installation mdbook
cargo install mdbook

cd ./praesentation
mdbook serve --open
```

Die Datei [./praesentation/src/SUMMARY.md](./praesentation/src/SUMMARY.md) enthält die Gliederung.

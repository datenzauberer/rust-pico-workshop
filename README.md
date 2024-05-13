# Planung für Einstieg in Embedded Rust mit dem Raspberry Pico WH

Erstmals verwendet: [2024-04-20 Augsburger Linux-Infotag 2024: Einstieg in Embedded Rust mit dem Raspberry Pico WH](https://www.luga.de/static/LIT-2024/talks/einstieg_in_embedded_rust_mit_dem_raspberry_pico_wh/).
Repo: https://github.com/rust-augsburg/2024-04-20-linux-tag-rust-picow-workshop

# Kanban Board

[Kanban Board](https://github.com/orgs/rust-augsburg/projects/1/views/1) ist auch schon eingerichtet. Damit können wir den aktuellen Bearbeitungsstand verfolgen und eigene Aufgaben eintragen.

# Verzeichnisstruktur

`praesentation` enthält die aktuelle Präsentation (in Deutsch).
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

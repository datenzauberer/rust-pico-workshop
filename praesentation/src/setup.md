# Setup Allgemein

**Um sicherzustellen, dass wir im Workshop direkt loslegen können, wäre es super, wenn ihr das Setup der Entwicklungsumgebung schon vorab erledigen könntet. Vielen Dank!**. 

Im Folgenden ist die Installation unter Ubuntu 22.04 LTS exemplarisch beschrieben. Solltet ihr ein anderes Betriebssystem nutzen, lest bitte die entsprechenden Installationsanleitungen.

Zur Information für Neugierige: Im Workshop werden wir zwei Raspberry Pi Pico verwenden. Einer wird als 'Debug-Probe' und der andere als 'Target' verwendet (näheres unter ["Debug Probe"](./debugprobe.md)). 
Auch wenn's ohne Debug-Probe klappt, wird's auf Dauer echt lästig, weil man den Pico vor jedem Flashen immer wieder in den BOOTSEL-Modus bringen muss.

# Setup der Entwicklungsumgebung

Die folgende Beschreibung des Setups setzt voraus, dass Ubuntu 22.04 LTS verwendet wird.

## Ubuntu 22.04 einrichten

**Bitte stellt sicher, dass Ihre Ubuntu 22.04-Installation wie unten beschrieben eingerichtet ist.**

```sh
sudo apt update
sudo apt install -y git
# curl wird für die Installation von rust benötigt
sudo apt install -y curl
# libudev-dev wird für die Fehlersuche benötigt
sudo apt install -y libudev-dev

# für Cargo-Generierung:
sudo apt install -y build-essential
sudo apt install -y pkg-config libssl-dev
```

## Visual Studio Code (VSCode)

Lade VS Code von der offiziellen Website herunter: https://code.visualstudio.com/download

Installiere es, z.B.:

```sh
sudo apt install ~/Downloads/code_1.87.2-1709912201_amd64.deb
```

Installiere die VS Code Erweiterungen (über die Kommandozeile):

```sh
# rust development
code --install-extension rust-lang.rust-analyzer
# debug rust code
code --install-extension vadimcn.vscode-lldb # auf macOS/Linux
#code --install-extension ms-vscode.cpptools # unter Windows
# für die Fehlersuche mit probe-rs:
code --install-extension probe-rs.probe-rs-debugger
```

Starte VS Code:

* Starte VS Code aus dem Terminal mit `code` oder über das Anwendungsmenü.

## Rust installieren

Folge den offiziellen Rust-Installationsanweisungen, um Rust zu installieren, einschließlich des Compilers (rustc) und des Paketmanagers (cargo): https://www.rust-lang.org/tools/install

Führen den folgenden Installationsbefehl aus (mit einer "1 Standard Installation").

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

**Neustart der Shell** oder setzen der  Umgebungsvariablen wie in der Terminalausgabe angegeben (z.B. für Bash: `source $HOME/.cargo/env`).

### Überprüfe Rust-Installation

```sh
rustc --version
```

Dieser Befehl liefert die Version des Rust-Compilers, `rustc`, zurückgeben, welcher aktuell auf Ihrem System verwendet wird. Dies ist zum Beispiel `rustc 1.77.1 (7cf61ebde 2024-03-27)`, wobei die Versionsnummer und das Veröffentlichungsdatum angegeben werden.

Cargo ist der Paketmanager und das Build-System von Rust. Um zu überprüfen, ob Cargo korrekt installiert ist und um seine Version zu sehen:

```sh
cargo --version
```

## Embedded Rust Development Dependencies

Installiere `cargo-generate`:

```sh
cargo install cargo-generate
```

Wenn die Installation aufgrund fehlender Abhängigkeiten fehlschlägt, die benötigten Pakete wie in [Ubuntu 22.04 setup](#ubuntu-2204-setup) beschrieben installieren.

Folge den Installationsanweisunge [`rp-rs/rp-hal` *Getting Started*](https://github.com/rp-rs/rp-hal?tab=readme-ov-file#getting-started):

```sh
rustup self update
rustup update stable
rustup target add thumbv6m-none-eabi
cargo install elf2uf2-rs --locked
cargo install probe-rs --features cli --locked
cargo install flip-link
```

Wenn die Installation aufgrund fehlender Abhängigkeiten fehlschlägt, die benötigten Pakete wie in [Ubuntu 22.04 setup](#ubuntu-2204-setup) beschrieben installieren.


**ACHTUNG: Aktualisiere die `/etc/udev/rules.d`** (wie in der [Probe.rs Dokumentation](https://probe.rs/docs/getting-started/probe-setup/#linux%3A-udev-rules) beschrieben):

```sh
curl -o ~/Downloads/69-probe-rs.rules https://probe.rs/files/69-probe-rs.rules
sudo cp ~/Downloads/69-probe-rs.rules /etc/udev/rules.d
sudo udevadm control --reload
sudo udevadm trigger
```

### Dokumentationserstellung

Zur Erstellung dieser Dokumentation bitte folgende Tools installieren:

```sh
cargo install mdbook
cargo install mdbook-toc
cargo install mdbook-pdf
```

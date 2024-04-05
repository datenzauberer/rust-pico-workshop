# Setup des Comput

Aus Zeitgründen wär's hilfreich, wenn die Teilnehmer das Setup schon vorher durchgeführt hätten. In der Ankündigung ist das Setup allerdings im Workshop mit inbegriffen.
Aufgrund der kurzen Workshopdauer würde wir Euch bitten, Euren Laptop zu Hause vorzubereiten.

Im folgenden ist die Installation unter Ubuntu 22.04 LTS exemplarisch beschrieben.
Solltet ihr ein anderes OS nutzen bitte die entsprechenden Installationsanleitungen lesen.

Das hier beschrieben Setup wird im Workshop verwendet. Das Debuggen erfolgt über probe-rs. 
Zu möglicher Fehleranalyse wär's hilfreich, wenn minicom sowie openocd installiert ist. (Wenn's installiert ist wird's sicher nicht gebraucht).

Trotzdem wärs sinnvoll, wenn openocd auf dem Rechner installiert ist.

Installation der Tools:

 * rustup
 * embedded tools
 * vscode
 * installation serial terminal: minicom
 * OpenOCD
 * debugging gdb


Im Workshop werden wir zwei Picos verwenden,

Aufsetzen des Picoprobe-Umgebunge:
siehe: <https://www.raspberrypi.com/documentation/microcontrollers/debug-probe.html> und mit zwei Picos:
[Getting Started with Pico - Appendix A: Using Picoprobe](https://datasheets.raspberrypi.com/pico/getting-started-with-pico.pdf#page=62)


Die Stromversorgung des PicoB erfolgt über den PicoA.
Daher muss nur PicoA an USB angeschlossen werden.





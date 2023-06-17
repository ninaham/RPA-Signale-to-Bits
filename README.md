# RPA: Konvertierer Signale zu Bitfolgen

dieses Programm erlaubt es eine Reihe von Signalen für eine CPU zu definieren, um diese später in eine Bitfolge umzuwandeln. 

## Usage

1. [Binary](https://github.com/ninaham/RPA-Signale-to-Bits/releases/tag/v0.1.0) laden (aktuell nur Linux)
2. ausführen mit `sigs-to-bits <path_to_input_file>`
3. noch Fragen? `sigs-to-bits -h`

## Input Format

Eine exemplarische Input Datei ist unten angegeben. Die erste Zeile definiert alle verfügbaren Signale. Danach folgt eine Leerzeile (!), darauf folgen die eigentlichen Signalkombinationen. Eine Kommentarzeile beginnt mit c gefolgt von einem Leerzeichen. 

```
ERA ERB ERC SUBE ADDE

ERA ERC SUBE
c ERB
ERC
SUBE
ADDE
```

Der Output zu obigem Input sieht folgendermaßen aus:

```
0000000d
00000004
00000008
00000010
```
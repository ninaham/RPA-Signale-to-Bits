# RPA: Konvertierer Signale zu Bitfolgen

dieses Programm erlaubt es eine Reihe von Signalen für eine CPU zu definieren, um diese später in eine Bitfolge umzuwandeln. 

## Usage

1. [Rust](https://www.rust-lang.org/learn/get-started) sollte installiert sein
2. ausführen mit `cargo run -- <path_to_input_file>`

## Input Format

Eine exemplarische Input Datei ist unten angegeben. Die erste Zeile definiert alle verfügbaren Signale. Danach folgt eine Leerzeile (!), darauf folgen die eigentlichen Signalkombinationen. 

```
ERA ERB ERC SUBE ADDE

ERA ERC SUBE
ERB
ERC
SUBE
ADDE
```

Der Output zu obigem Input sieht folgendermaßen aus:

```
00000000000000000000000000001101
00000000000000000000000000000010
00000000000000000000000000000100
00000000000000000000000000001000
00000000000000000000000000010000
```
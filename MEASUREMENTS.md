# Spike egui/eframe: mediciones

Rama `spike/egui`, código desechable: solo sobreviven estos números en el ADR 0001. Fecha: 2026-11-23.
Máquina: Windows 11 Pro x64, Intel i9-11900H (16 hilos), Intel UHD Graphics (pantalla de 360 Hz) +
NVIDIA RTX 3070 Laptop (Optimus), escala 100 %. El spike de Slint compilaba y medía en paralelo, así
que los tiempos pueden traer ruido.

## 1. Toolkit y features exactas

| Crate | Versión | Features |
| --- | --- | --- |
| eframe | 0.37.0 | `default-features = false`: `accesskit`, `clipboard`, `system_fonts`, `wayland`, `x11`, `glow` (igual que `master`) |
| winit | 0.30.13 | `default-features = false`: `wayland-dlopen`, `wayland-csd-adwaita` |
| syntect | 5.3.0 | `default-features = false`: `default-syntaxes`, `default-themes`, `regex-fancy` (fancy-regex 0.16.2; nada de onig) |
| regex | 1.13.2 | por omisión |

Resueltas: egui/epaint/egui-winit/egui_glow 0.37.0, glow 0.17.0, glutin 0.32.3, accesskit 0.25.2
(accesskit_windows 0.35.2, accesskit_macos 0.27.2, accesskit_unix 0.24.1), egui_system_font_provider
0.37.0 (fontique 0.11.1, skrifa 0.44.0), harfrust 0.12.0 (shaping), ab_glyph 0.2.32. Rust 1.100.0,
perfil release por omisión (sin LTO ni strip). Sin `default_fonts`: el texto sale de fuentes del sistema
y **sí se dibuja** (Consolas para el editor, Segoe UI para la interfaz, respaldo del sistema para CJK y
emoji a color).

## 2. Qué hace el prototipo

`amate-spike [--bench] [--font RUTA] ARCHIVO...`: una pestaña por archivo (solo UTF-8; si no, error en
la barra de estado y no se abre), vista virtualizada propia (`Vec<String>`, una galley por línea
visible, caché de galleys de egui), escribir, Backspace/Supr, Enter, flechas, Inicio/Fin
(Ctrl = documento), RePág/AvPág, clic para el caret, desplazamiento vertical y horizontal, Ctrl+S,
marca ● de modificado, numeración de líneas, IME (preedit en línea subrayado, commit, área de cursor
para la ventana de candidatos), AccessKit (sección 6), Ctrl+F con regex, F3/Enter siguiente, Reemplazar
todo con `$1`, coincidencias resaltadas, resaltado syntect de JSON y C++ solo en líneas visibles,
carga en hilo aparte con barra de progreso, y arriba de 10 MiB modo degradado (sin resaltado ni
ajuste de línea; el spike nunca ajusta). `--font` antepone una fuente a la familia monoespaciada
(para probar ligaduras). Fuera de alcance: selección, deshacer, copiar/cortar, Guardar como,
navegación por palabras.

## 3. Resultados (`measure.ps1`, mediana de 3 corridas de `--bench`)

| Métrica | Valor | Corridas |
| --- | --- | --- |
| Arranque → primer frame | **218 ms** | 246 / 218 / 217 |
| Abrir 1 MB (C++, resaltado) | **136 ms** (hilo de carga: 6.6 ms) | 135 / 136 / 138 |
| Abrir 10 MB (C++, resaltado) | **52 ms** (hilo de carga: 37.7 ms) | 57 / 52 / 50 |
| Frame más largo mientras carga el de 10 MB | 11.9 ms | la interfaz no se congela |
| Tecla → frame, **con resaltado** (n = 500) | **p50 12.1 / p95 13.9 / p99 14.6 ms** (máx. 16.6) | p99: 14.1 / 15.6 / 14.6 |
| Tecla → frame, sin resaltado (n = 500) | **p50 2.8 / p95 3.0 / p99 3.5 ms** | p99: 3.5 / 3.6 / 3.5 |
| Tecla → fin de `App::ui`, con / sin resaltado | p50 10.7 / 0.29 ms, p99 12.7 / 0.44 ms | |
| CPU del frame de la tecla (eframe), con / sin resaltado | p50 11.6 / 2.5 ms, p99 13.8 / 3.2 ms | |
| Intervalo entre frames sin trabajo | p50 2.9 ms | = vsync a 360 Hz |
| syntect: parsear 1 MB completo, C++ / JSON | **5.2 s** / 0.78 s | ~5 300 líneas/s en C++ |
| Reposo con el de 1 MB abierto (30 s tras 5 s) | **111.5 MB privada**, 167.4 MB working set, CPU 125 ms en 30 s | |
| Binario release | **9.1 MB** (9 541 120 bytes) | |
| Compilación release limpia | **82.7 s** | con el otro spike compilando |
| `cargo deny check` | advisories ok, bans ok, licenses ok, sources ok (salida 0) | |
| Dependencias transitivas | Windows **137**, Linux **242**, macOS (aarch64) 147, `--target all` 352 | |

Memoria privada por caso (una muestra a 6 s, fuera de `measure.ps1`): ventana vacía 88.0 MB,
1 MB como `.txt` 92.0 MB, 1 MB como `.cpp` 110.4 MB, 10 MB como `.cpp` 133.8 MB. La base de 88 MB
es la de `master` (86 MB, sobre todo el controlador de OpenGL); el documento suma ~4 MB y la
gramática de C++ ya compilada por syntect ~18 MB.

## 4. Qué incluye cada número

Todo «fin de frame» es el inicio del frame siguiente (`App::raw_input_hook`), al que eframe solo
llega después de pintar y de que `swap_buffers` regrese. Es lo más cerca de «presentado» que eframe
deja ver: no expone marca de tiempo de presentación. El bench pide repintado en cada frame, así que el
siguiente empieza de inmediato. Ventana de 1200×800 puntos (~45 líneas visibles), vsync encendido.

- **Arranque**: de un `Instant` al inicio de `main` al inicio del frame 2. Incluye crear la
  ventana y el contexto GL, iniciar egui, el primer frame (pestaña vacía, barras) y esperar a que
  fontique termine de enumerar las fuentes del sistema (el primer texto lo espera). Excluye la
  creación del proceso y lo previo a `main`. La gramática de syntect se carga en otro hilo.
- **Abrir N MB**: de antes de lanzar el hilo de carga al fin del primer frame que pintó todas las
  líneas visibles con sus colores finales. Incluye leer (archivo ya en caché del SO), validar UTF-8,
  partir en líneas, pasar el documento por canal (se revisa una vez por frame), resaltar las líneas
  visibles, maquetar, pintar y hacer swap. El de 1 MB es el primer archivo C++: los ~130 ms del lado
  de la interfaz son, casi todo, syntect compilando al primer uso las regex de la gramática (el de
  10 MB, abierto después con la misma gramática, gasta ~14 ms del lado de la interfaz; la memoria
  sube 18 MB en el mismo paso). No lo perfilamos más.
- **Tecla → frame**: de inyectar un `egui::Event::Text` imprimible en `raw_input_hook` (un evento
  por frame, 20 de calentamiento descartados, 500 medidos) al inicio del frame siguiente. El caret
  está a la mitad del archivo (línea 13 756 de 27 511, columna 1), centrada en pantalla. Incluye: proceso de entrada de egui, la
  edición, el re-resaltado (con resaltado: desde el punto de control previo, hasta 15 líneas arriba,
  hasta la última línea visible, ~25–40 líneas), maquetado, teselado, pintado GL y la espera de
  vblank en `swap_buffers` (≤ 2.8 ms a 360 Hz). Excluye: teclado → winit → egui-winit (se inyecta
  después), la composición de DWM y el escaneo de la pantalla (al menos un refresco más: 2.8 ms aquí,
  hasta 16.7 ms a 60 Hz). En una pantalla de 60 Hz, el frame con resaltado (11.6 ms de CPU) todavía
  cabe, pero la latencia se cuantiza al vblank de 16.7 ms. Todas las teclas van a la misma línea
  (crece ~520 caracteres por fase). AccessKit apagado (sin lector de pantalla conectado).
- **Tecla → fin de `App::ui`**: solo nuestro código y el pase de egui hasta terminar `ui`.
- **CPU del frame**: `frame.info().cpu_usage` de eframe para el frame de la tecla (sin la espera de
  vsync).
- **Reposo**: `PrivateMemorySize64`, `WorkingSet64` y `TotalProcessorTime` de .NET, 1 muestra por
  segundo durante 30 s, tras 5 s de asentarse. egui es reactivo: sin entrada no repinta. Los 125 ms
  son saltos sueltos de 15.6 ms (la resolución del contador de Windows), 0 en la mayoría de los
  segundos. El caret no parpadea en el spike; parpadear implicaría repintados periódicos.
- **Dependencias**: `cargo tree --target <t> -e normal,build --format "{p}"`, versiones únicas, sin
  contar el propio crate. `--target all` incluye Android, iOS y wasm, que `deny.toml` no revisa.

## 5. `cargo deny check`

`advisories ok, bans ok, licenses ok, sources ok` con el `deny.toml` de `master` (4 objetivos de Q09).
syntect agrega bincode 1.3.3, flate2 1.1.11 (miniz_oxide, Rust puro), fancy-regex, bit-set, fnv,
walkdir, serde, once_cell y thiserror: todas MIT o Apache-2.0. Ni onig, ni cmake, ni bindgen; `cc`
solo entra como build-dep de `wayland-backend` (excepción de Q19) y de `android-activity` (solo
Android, fuera del grafo de `deny.toml`).

## 6. Accesibilidad (AccessKit): qué se logró y qué no

Logrado, verificado en Windows con `a11y-probe.ps1` (cliente UIA de .NET):

- El editor es un elemento UIA `Edit` (`Role::MultilineTextInput`) con nombre
  «Editor: archivo», foco de teclado, ValuePattern y TextPattern.
- El texto se expone con nodos `TextRun` usando `egui::text_selection::accesskit_text::`
  `update_accesskit_for_text_widget`, más `set_value`. Hasta 1 000 líneas va todo el texto; arriba de
  eso, solo las líneas visibles. El caret (`text_selection`) se publica cuando su línea está dentro.
- Con rangos sacados de `DocumentRange`, expandir a carácter, palabra, línea, párrafo y documento
  funciona.

No logrado o limitado por el toolkit:

- **Texto incorrecto para RTL y clústeres**: egui arma el valor de cada `TextRun` desde los glifos
  de la galley, no desde el texto. Con árabe, hebreo, emoji ZWJ y banderas, el TextPattern devuelve
  caracteres duplicados o invertidos («مرحبا» → «ابحرمرحبا», 👨‍👩‍👧 → «👨👨👨‍👧», 🇲🇽 → «🇲🇲»). El
  ValuePattern (nuestro `set_value`) sí es correcto. Pasa igual en el `TextEdit` de egui: es un bug de
  egui 0.37. Arreglo: armar nuestros propios `TextRun` desde el texto fuente.
- **Rangos de `TextPattern.GetSelection()`**: en el cliente .NET, expandir el rango del caret falló
  con HRESULT basura y luego el cliente murió con `AccessViolationException`. Le pasa igual al
  `TextEdit` de egui. No sabemos si NVDA (cliente UIA nativo) lo sufre; NVDA usa ese rango para leer
  la línea al mover el caret. **Es la verificación manual más importante.** Posible bug de
  `accesskit_windows`.
- **Virtualización contra lector de pantalla**: AccessKit no tiene proveedor de texto bajo demanda; el
  texto debe estar en el árbol. En archivos grandes el lector solo ve la ventana visible («leer todo»
  se detiene en el borde). Además, egui reconstruye y envía el árbol completo en cada frame mientras
  hay un lector conectado: el costo crece con el texto expuesto (no medido con lector activo). Para
  exponerlo hay que maquetar una galley extra solo para AccessKit.
- Acciones de AccessKit desde el lector (mover caret, `SetValue`, `ReplaceSelectedText`) no se
  manejan en el spike.

## 7. IME

Implementado con lo que egui ofrece: `ImeEvent::Preedit` se pinta en línea, subrayado, en el caret
(esa línea pierde el resaltado mientras dura la composición); `ImeEvent::Commit` inserta;
`PlatformOutput::ime` publica el rectángulo del caret cada frame con el editor enfocado, y egui-winit
llama a `set_ime_allowed` y `set_ime_cursor_area`. `ImeEvent::DeleteSurrounding` no se maneja. egui-winit
0.37 trae parches por bugs de winit (teclas procesadas por el IME en Windows, posición del cursor con el
IME coreano en Windows, eventos de preedit vacíos repetidos en Wayland): señal de que el IME es la zona
frágil. No se probó con un IME real: queda en la lista manual.

## 8. Limitaciones encontradas

1. **El cuello de botella es syntect con fancy-regex, no egui.** Sin resaltado, egui completo
   (entrada, maquetado, teselado, GL, swap) cuesta 2.8 ms p50 y 3.5 ms p99 por tecla; nuestro `ui`,
   0.3 ms. Con resaltado, ~10 ms por tecla son de syntect re-parseando 25–40 líneas a ~0.2 ms cada
   una. 1 MB de C++ tarda 5.2 s en parsearse completo (JSON: 0.78 s). En un archivo de 10 MB, ir al
   final exige parsear desde arriba: ~50 s de CPU repartidos en rebanadas de 8 ms por frame (la
   interfaz no se congela, pero los colores tardan). La gramática de C++ suma ~18 MB de RAM y ~120 ms
   de compilación al primer uso, en el hilo de la interfaz.
2. **Memoria (Q25, menos de 100 MB)**: 111.5 MB con el archivo de 1 MB en C++. La ventana vacía ya
   cuesta 88 MB (GL). egui deja ~12 MB de margen para todo lo demás.
3. **AccessKit** (sección 6): texto de `TextRun` erróneo con RTL y clústeres; rangos de selección
   dudosos en UIA; nada de texto bajo demanda para una vista virtualizada.
4. **Bidi**: árabe y hebreo se ven bien formados y de derecha a izquierda en la muestra, pero el caret,
   el clic y la selección de egui trabajan con índices lógicos de `char` (`CCursor`): en líneas RTL el
   caret seguramente queda mal. No verificado.
5. **Fuentes de respaldo**: en la misma línea, el CJK queda más arriba y el hebreo y el árabe más abajo
   que el latín (línea base desalineada, en Windows). Los caracteres anchos no miden 2 celdas exactas
   y el katakana de ancho medio mide menos de 1: la rejilla de columnas se rompe. egui no ajusta a
   celdas; habría que posicionar glifos por celda a mano.
6. **Clústeres de grafemas**: el caret y Backspace van por `char`; pueden caer dentro de «e + ◌́» o de
   un emoji ZWJ. Se arregla en nuestro núcleo, egui no lo da.
7. **Coordenadas en f32**: el alto del contenido va en puntos f32. El archivo de 30 MB (816 187 líneas)
   mide ~13.9 M px, cerca de los 16.7 M donde f32 deja de ser exacto. Un editor real necesita su propio
   modelo de desplazamiento (línea + desfase), no los píxeles de `ScrollArea`.
8. **Líneas largas**: una galley por línea, sin virtualización horizontal. Un JSON minificado de 1 MB
   en una sola línea se maquetaría entero en cada cambio de esa línea. No medido.
9. **Presentación**: eframe no da marca de tiempo de presentación; el número más honesto termina
   cuando `swap_buffers` regresa.
10. Todo lo de editor (selección, deshacer, portapapeles, palabras) se construye a mano: el `TextEdit`
    de egui maqueta el texto completo y no sirve para 10 MB.

Lo que funcionó bien: dibujo libre con `Painter` y caché de galleys, cero CPU en reposo, 10 MB en
~52 ms sin frames de más de 12 ms, el modo degradado abre 30 MB (816 187 líneas), las fuentes del
sistema sin `default_fonts`, emoji a color, y ligaduras con `--font C:\Windows\Fonts\CascadiaCode.ttf`.

## 9. Lista de verificación manual

Abre `samples/scripts.txt` (latín, CJK, coreano, emoji, árabe, hebreo, bidi mixto, marcas combinantes,
rejilla de anchos, tabulador) en cada sistema. Windows 11, macOS (Apple Silicon e Intel) y Linux
(GNOME y KDE, cada uno en X11 y en Wayland).

### IME

- [ ] **Windows, IME de Microsoft**: japonés (`nihongo`, Espacio, Enter → 日本語), chino Pinyin
  (`zhongwen` + número), coreano (2 juegos, `gksrmf` → 한글). Revisa: el preedit sale subrayado en el
  caret; la ventana de candidatos se abre junto al caret, no en una esquina; Backspace y flechas actúan
  dentro de la composición; el commit no duplica ni pierde caracteres; en coreano, la sílaba en curso
  no se pinta corrida.
- [ ] Clic en otra línea a media composición y cambio de IME a media composición: nada duplicado ni
  perdido.
- [ ] **macOS**: fuentes de entrada «Japonés – Romaji», «Pinyin – Simplificado» y «Coreano 2 juegos»;
  mantener presionada la «e» (menú de acentos); tecla muerta Opción+e, e → é; Cmd+tecla durante la
  composición.
- [ ] **Linux X11**: fcitx5 (mozc, pinyin, hangul) e IBus (mozc, libpinyin, hangul). Revisa la posición
  de los candidatos y la tecla Compose (Compose ' e → é).
- [ ] **Linux Wayland**: GNOME con IBus (text-input-v3) y KDE con fcitx5. Revisa la posición de los
  candidatos y que los eventos de preedit vacíos no borren texto.

### Lectores de pantalla

- [ ] **NVDA (Windows)**: al enfocar el editor debe decir «Editor: scripts.txt, edición, multilínea»
  y la línea actual. Flecha arriba/abajo: **debe leer la línea nueva** (depende del rango de selección
  de la sección 6). Izquierda/derecha: lee caracteres. Eco de caracteres al escribir. NVDA+flecha
  arriba lee la línea; NVDA+flecha abajo (leer todo) se detiene en el borde visible en archivos de más
  de 1 000 líneas (esperado). En las líneas de árabe, hebreo y emoji, escucha si hay caracteres
  duplicados o invertidos (esperado por el bug de egui). Repite lo básico con Narrador.
- [ ] **VoiceOver (macOS)**: VO+Mayús+flecha abajo para interactuar; debe anunciar «área de entrada
  de texto»; flechas leen línea y carácter; VO+A lee todo; el cursor de VoiceOver sigue al caret; el
  rotor ofrece líneas.
- [ ] **Orca (Linux X11 y Wayland, GNOME)**: al enfocar dice el nombre y «texto»; flechas leen línea y
  carácter; revisión plana; «dónde estoy». Si Orca no ve nada, revisa que el bus de accesibilidad esté
  activo.
- [ ] En los tres: la barra de búsqueda (Ctrl+F) se anuncia con su etiqueta «Find (regex):»; las
  pestañas se anuncian.

### Texto

- [ ] **Respaldo CJK**: japonés, chino simplificado, chino tradicional y coreano sin cuadros vacíos (□);
  con locale japonés vs chino, la variante regional correcta (p. ej. 直). Revisa si la línea base del CJK
  queda desalineada respecto del latín (en Windows, sí).
- [ ] **Emoji**: a color (Segoe UI Emoji, Apple Color Emoji, Noto Color Emoji); la familia ZWJ como un
  solo glifo; el tono de piel combinado; banderas en macOS y Linux (Windows no trae banderas: muestra
  «MX», esperado).
- [ ] **Ligaduras**: `amate-spike --font <ruta a CascadiaCode.ttf o FiraCode-Regular.ttf> samples/scripts.txt`;
  `-> => != === <= >=` deben unirse (en Windows ya se unen). Pon el caret entre `=` y `>` y revisa dónde
  se pinta.
- [ ] **Bidi**: árabe unido y de derecha a izquierda; hebreo de derecha a izquierda; la línea mixta
  `abc שלום 123 def مرحبا 456 ghi` con base izquierda a derecha y los números en orden. Mueve el caret
  con flechas dentro de las palabras RTL y anota dónde se pinta (esperado: mal).
- [ ] **Rejilla**: las barras de `|漢字漢字|`, `|abcdefgh|` y `|ｱｲｳｴｵｶｷｸ|` deberían alinearse si los anchos
  son 2 y 1 celdas (en Windows no se alinean).
- [ ] **Marcas combinantes**: `é ä ñ` descompuestas se ven como una letra; anota si el caret se detiene
  entre la letra y el acento (esperado: sí).
- [ ] Escala 125 %, 150 % y 200 %, y mover la ventana entre monitores con distinto DPI: texto nítido.
- [ ] macOS: corre con OpenGL (obsoleto en macOS) en Apple Silicon; anota consumo de energía.
- [ ] Linux Wayland: decoraciones de ventana en GNOME (CSD de `sctk-adwaita`) y Ctrl+V.

## 10. Cómo reproducir

Desde PowerShell, en la raíz de este worktree:

```powershell
cargo build --release --locked
.\target\release\amate-spike.exe .\samples\scripts.txt
.\target\release\amate-spike.exe --font C:\Windows\Fonts\CascadiaCode.ttf .\samples\scripts.txt
.\target\release\amate-spike.exe --bench          # un JSON en stdout; la ventana debe quedar visible
powershell -NoProfile -File .\measure.ps1          # escribe target\spike-data\results.json
powershell -NoProfile -File .\a11y-probe.ps1       # el cliente .NET puede morir en el paso 5 (sección 6)
```

En macOS y Linux: `cargo run --release -- samples/scripts.txt`. Ojo: el spike de Slint también produce
`amate-spike.exe`; para cerrar procesos, fíltralos por ruta.

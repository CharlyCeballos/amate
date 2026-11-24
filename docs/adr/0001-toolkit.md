# ADR 0001 — Toolkit de la interfaz

Estado: **aceptada** (2026-11-23): egui/eframe con el backend glow. El mantenedor lo eligió sobre
Slint porque la velocidad es la prioridad del proyecto (arranque de 241 ms frente a 1 098 ms). Las
pruebas manuales de IME y de lector de pantalla siguen pendientes: si egui las falla, este ADR se
reabre con las mediciones de Slint ([Q20], [Q24]).
Fecha: 2026-11-23.

## Contexto

El toolkit es la elección que menos se revierte. Tiene que cumplir a la vez [Q16] (nativo, en
proceso, en Rust), [Q19] (nada de C ni C++ compilado en el binario), [Q20] (lector de pantalla e IME
en los tres sistemas, filtro duro), [Q25] (presupuestos de arranque, latencia y RAM) y [Q02]
(licencia compatible con GPL-3.0-or-later). Además tiene que dejar dibujar una vista de editor
propia, virtualizada.

Hallazgo que cambió [Q19]: todos los candidatos con winit dependen de `wayland-backend`, que
declara `cc` como dependencia de compilación siempre, pero solo compila C con su feature `log`.
El dueño aprobó la excepción acotada que ahora registra [Q19] y aplica `deny.toml`.

## Candidatos (datos del 2026-11-23)

| | egui/eframe | Slint | Xilem/Masonry | iced | floem | GPUI |
| --- | --- | --- | --- | --- | --- | --- |
| Último release | 0.37.0, 2026-10-15 | 1.19.0, 2026-11-16 | 0.4.0, 2025-10-29 | 0.14.0, 2025-12-07 | 0.2.0, 2024-11-14 | 0.2.2, 2025-10-22 |
| Commits en 3 meses | 239 | 2 318 | 20 | 402 | 1 | 161 (`crates/gpui`) |
| AccessKit | Sí, 3 sistemas | Sí, 3 sistemas | Sí (AccessKit viejo) | **No** | **No** | Solo en main, parcial |
| IME | Sí; fallas en Linux (sin verificar) | Sí (winit) | Sí (winit) | Sí (0.14) | Sin verificar | Sí |
| C compilado | No* | No* (sin `renderer-skia` ni `backend-qt`) | No* | No* | No* | **Sí** (`bindgen`, `freetype-sys`, `ring`) |
| Licencia | MIT o Apache-2.0 | GPL-3.0-only, royalty-free o comercial | Apache-2.0 | MIT | MIT o Apache-2.0 | Apache-2.0 |
| Avisos RustSec | Solo «unmaintained» | Solo «unmaintained» | Solo «unmaintained» | `lru` unsound | `im`, `sized-chunks` unsound | Solo «unmaintained» |
| Deps transitivas (Win/Linux) | 174 / 267 (wgpu) | 243 / 329 | 158 / 241 | 148 / 222 | 218 / 242 | 410 / 479 |
| Texto | harfrust + skrifa + fontique; sin bidi | parley + fontique + swash; bidi | parley + vello | cosmic-text; bidi | cosmic-text / parley | Del sistema (CoreText, DirectWrite) |
| Vista de editor propia | Fácil (painter) | Difícil (sin dibujo libre desde Rust) | Media | Media | Fácil (Lapce) | Fácil (Zed) |

\* Solo el `cc` sin usar de `wayland-backend`.

Descartados:

- **iced** y **floem**: sin AccessKit upstream ([Q20]). iced #552 abierto desde 2020; floem #8 desde 2023.
- **GPUI**: compila C en su release de crates.io ([Q19]) y su versión vigente no está publicada
  (dependencia git del monorepo de Zed). Sirve como diseño de referencia.
- **Xilem/Masonry**: se declara experimental y su release va un año detrás de parley y AccessKit.
  En observación para una 0.5.

## Decisión

Medir **egui/eframe** y **Slint** en paralelo, en las ramas `spike/egui` y `spike/slint`. La
ventana vacía de `master` usa egui/eframe.

Primeras mediciones de la ventana vacía de `master` (Windows 11 x64, release, GPU NVIDIA, sin
archivo abierto):

| Backend de eframe | Binario | Ventana visible | Memoria privada | Working set | CPU en reposo (10 s) |
| --- | --- | --- | --- | --- | --- |
| wgpu | 13.7 MB | 119 ms | 525 MB | 341 MB | 0 ms |
| **glow** (elegido) | **6.3 MB** | 115 ms | **86 MB** | 130 MB | 0 ms |

Con wgpu, `egui-wgpu` reactiva las fuentes por omisión de epaint (OFL-1.1 y Ubuntu-font-1.0, que
`deny.toml` no permite); con glow y sin `default_fonts` no entran. Riesgo de glow: OpenGL está
obsoleto en macOS; el spike mide macOS.

## Mediciones de los spikes

Windows 11 x64, i9-11900H, GPU NVIDIA, monitor de 360 Hz, build release. Tiempos: mediana de 3
corridas de `--bench`, una después de otra (2026-11-23). Memoria, binario, compilación y
dependencias: `measure.ps1` de cada rama, corrido con el otro spike compilando en paralelo (con
ruido). Detalle y método en el `MEASUREMENTS.md` de cada rama.

| Pregunta | egui/eframe 0.37 (glow) | Slint 1.19 (winit + femtovg) |
| --- | --- | --- |
| Arranque → primer frame (< 300 ms) | **241 ms** | **1 098 ms** (128 ms crear la UI; el resto antes del primer frame, causa sin investigar) |
| Latencia tecla→frame, sin resaltado | p50 2.8 / p99 **3.4 ms** | no medida aparte |
| Latencia tecla→frame, con resaltado de C++ | p50 12.9 / p99 **15.3 ms** (re-parsea 25-40 líneas por tecla) | p50 7.7 / p99 **9.4 ms**, hasta `swap_buffers` (~5 ms son syntect) |
| Abrir 1 MB / 10 MB | 139 ms / 56 ms; frame más largo durante la carga de 10 MB: 12.7 ms | 157 ms / 62 ms |
| Resaltar 1 MB de C++ completo | 5.5-5.8 s en otro hilo; la UI responde | 4.7-4.9 s en el hilo de la UI; **la UI se congela** |
| RAM privada en reposo con 1 MB (< 100 MB) | **111.5 MB** (la ventana vacía: 88 MB; la gramática de C++ ~18 MB) | **117 MiB** |
| Binario release / compilación limpia | 9.1 MB / 83 s | 16.7 MiB / 130 s |
| Dependencias (Windows / Linux / macOS) | 137 / 242 / 147 | 259 / 352 / 262 |
| `cargo deny check` (Q02, Q19) | ok | ok, agregando `GPL-3.0-only` |
| Lector de pantalla (prueba automática de UIA) | Expone el texto; el rango del caret falla con el cliente UIA de .NET, pero NVDA sí lee el caret (fila siguiente) | Edit multilínea con texto; solo las ~38 líneas visibles; el rango del caret de UIA falla, igual que en `LineEdit` |
| IME JA/ZH/KO (manual, 3 sistemas) | Windows 11 (2026-11-23): japonés ok (preedit en línea, conversión, ventana de candidatos pegada al caret, commit exacto del candidato elegido); coreano ok (commit exacto `한글`); chino: preedit y barra de candidatos pegados al caret, commit sin verificar (lo interrumpió el aviso de Bing del IME). macOS y Linux pendientes | Pendiente |
| NVDA / VoiceOver / Orca (manual) | NVDA 2026.3 (2026-11-23): foco «Editor: scripts.txt, edit» y la línea; ↑↓ leen la línea, ←→ el carácter, eco al escribir y al borrar; CJK y coreano bien. Fallan árabe y hebreo (duplicados, al revés y al derecho), la familia ZWJ, las banderas, y la etiqueta de la búsqueda (lee el placeholder). Al entrar con Alt+Tab real anuncia la ventana y el editor, y ↓ lee el archivo (verificado por el mantenedor). VoiceOver y Orca pendientes | Pendiente |
| Texto: respaldo, ligaduras, CJK, emoji, bidi | Fuentes del sistema y ligaduras ok; bidi, emoji compuestos y anchos CJK fallan en caret y en el texto para AccessKit | Dibuja CJK, emoji y RTL; sin API para medir glifos, el caret falla después de CJK y RTL |
| Binario universal de macOS | CI de `master` | — |

### Hallazgos que pesan en la decisión

- **Slint pasa 3.7 veces el presupuesto de arranque** de [Q25], no deja medir ni ubicar glifos
  (la vista de editor tiene que suponer una rejilla), solo entrega el IME a un `TextInput` (el
  spike usó uno transparente encima de la vista y leyó el preedit de una propiedad interna), no
  deja crear nodos de accesibilidad propios, y su macro `slint!` no compila con
  `unsafe_code = "forbid"`.
- **egui** cumple arranque y latencia, y deja dibujar y exponer lo que haga falta, pero el texto que
  manda a AccessKit sale de los glifos dibujados (bidi y emoji compuestos llegan mal) y no tiene
  bidi.
- **Común a los dos:** `syntect` con `fancy-regex` es el cuello de botella (unos 0.14 ms por línea
  de C++). El resaltado tiene que ir en otro hilo y re-parsear solo hasta que el estado coincida.
  Los dos pasan el presupuesto de RAM con 1 MB por 11-17 MB, sobre todo por la gramática de C++.
- **Decisión:** egui/eframe con glow. Las pruebas manuales de IME y lectores de pantalla en los
  tres sistemas siguen siendo condición para la primera versión pública ([Q20]).

## Consecuencias

- La vista del editor es propia en cualquier caso; el núcleo (`amate-core`) no depende del toolkit,
  así que la capa fina entre ambos acota el costo de cambiar.
- egui no tiene bidi: si gana, el texto bidireccional del hito 2 ([Q04]) necesita parley dentro de
  su painter.
- Slint es GPL-3.0-only: el binario distribuido quedaría GPLv3-only, aunque el código propio siga
  GPL-3.0-or-later ([Q02]).

## Si sale mal

Si el elegido falla el IME o el lector de pantalla en una plataforma después del GO, se intenta
corregir upstream; si no se puede, este ADR se reabre con las mediciones del otro candidato.

## Fuentes

API de crates.io (`/api/v1/crates/<nombre>`), rustsec.org, y los repositorios emilk/egui
(CHANGELOG, #8906), slint-ui/slint (LICENSE.md, CHANGELOG, #2723), linebender/xilem, iced-rs/iced
(#552, #3281), lapce/floem (#8), zed-industries/zed (#56065, #67305) y Smithay/wayland-rs
(`wayland-backend/build.rs`). Consultados el 2026-11-23.

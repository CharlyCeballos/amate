# Decisiones

Hoja de respuestas del proyecto Amate. Es la fuente de verdad: el prompt operativo
([PROMPT.md](PROMPT.md)) y el resto de los documentos citan cada decisión por su identificador,
así: `[Q09]`.

- Una línea `Respuesta:` con contenido cierra la decisión.
- Una línea `Respuesta:` vacía aplica la recomendación, y esa consecuencia queda marcada como
  supuesto en el plan.
- Para cambiar una decisión, edita su respuesta y vuelve a correr el prompt. El apartado
  «Si cambias» de cada decisión dice qué parte del plan se mueve.

Contexto fijo: una persona con experiencia sólida en Rust trabaja con un agente de código.
Decisiones tomadas entre el 2026-10-15 y el 2026-10-20. La columna «Origen» enlaza con las
preguntas de [02-cuestionario.md](02-cuestionario.md) que cubre cada decisión.

## Índice

| ID | Tema | Respuesta | Origen |
| --- | --- | --- | --- |
| Q01 | Nombre | `amate` | A4 |
| Q02 | Licencia | `GPL-3.0-or-later`, todo el repo | H1, A5 |
| Q03 | Paridad de la primera versión pública | `diario` | B1 |
| Q04 | Añadidos al hito 1 | básicos baratos | — |
| Q05 | Configuración de Notepad++ | `ninguna` | C1, C2 |
| Q06 | Plugins propios | `despues` | D1, D2, B6 |
| Q07 | Idioma de la interfaz | español e inglés | H5 |
| Q08 | Idioma del repositorio y del manual | README y manual bilingües, commits en inglés | J3 |
| Q09 | Paquetes y plataformas | MSI, ZIP, DMG, `.deb`, AppImage, `.tar.gz` | F1, F2, F4, F5 |
| Q10 | Firma y notarización | `despues` + SignPath | G1 |
| Q11 | Tamaño de archivo | `normal`: 10 MB | B3 |
| Q12 | Parecido visual | *(vacía: `familiar`)* | — |
| Q13 | Actualizaciones | `release`, sin red | G3, G4, F6 |
| Q14 | Integración con el escritorio | `con-el-paquete` | F8 |
| Q15 | Tipo de proyecto | `publico`, por etapas | H2, H4, H6 |
| Q16 | Tecnología de la interfaz | `nativa` | E2 |
| Q17 | Usuario principal y propuesta de valor | quien usa Notepad++ | A2, A3 |
| Q18 | Relación con Notepad++ | sucesor espiritual | A1 |
| Q19 | Código nativo en el binario | Rust estricto | E1, E3, E6 |
| Q20 | Accesibilidad | filtra el toolkit | I3 |
| Q21 | Material de Notepad++ | reimplementación limpia, nada dentro | H1 |
| Q22 | Contribuciones | DCO | H3 |
| Q23 | Atajos en macOS | Cmd por Ctrl | C3 |
| Q24 | Validación en el hito 0 | sí, con mediciones | J1 |
| Q25 | Presupuestos de rendimiento | requisito en CI | I2 |
| Q26 | Lenguajes con resaltado | 13 en el hito 1, 32 en la pública | B4 |
| Q27 | Versionado | SemVer, 0.x hasta firmar | — |
| Q28 | Procedencia de los releases | atestación de GitHub + `SHA256SUMS.txt` | G2 |
| Q29 | Titular | Carlos Ceballos | — |
| Q30 | Finales de línea del repo | LF en `.gitattributes` | — |
| Q31 | Criterio de éxito | adopción externa | J5 |

---

### Q01 — Nombre del producto

Respuesta: `amate`, del náhuatl *āmatl*, el papel de corteza de los códices. En la prosa se
escribe «Amate»; el binario, el comando y el crate son `amate`. La carpeta `notepadrust` es solo
un nombre de trabajo local.

Implica:

- Antes de publicar cualquier artefacto, crear el repo público o reservar el crate, falta
  verificar la marca (IMPI, USPTO, EUIPO) y el dominio.
- Choques revisados el 2026-10-15: crates.io, Debian, Homebrew (cask y fórmula), PyPI, npm, AUR
  y la web están libres. El usuario `amate` de GitHub está ocupado por una cuenta ajena activa;
  la org propuesta es `amate-editor` (a verificar). Hay nombres parecidos: los crates AmateRS
  (`amaters`, una base de datos cifrada) y Amate Audio (bocinas, no software).
- Búsqueda de dominios del dueño (2026-10-20): solo aparecen `amateaudiousa.com`,
  `amateaudio.com` y `amateaudionorway.no` (audio, otro sector) y `amatetecnologia.com` (empresa de
  tecnología: revisar que no sea software de edición), más variantes parecidas. Ningún editor de
  texto. Falta la consulta formal en IMPI, USPTO y EUIPO, y elegir un dominio libre.
- Ningún nombre, binario, crate, paquete, repo, dominio, logo ni icono usa «Notepad++» o un
  derivado confundible ([Q21]).

Si cambias: cambian el comando de la línea de comandos, los identificadores de paquete (bundle
ID de macOS, nombre del `.deb`, UpgradeCode del MSI) y la org.

---

### Q02 — Licencia

Respuesta: `GPL-3.0-or-later`, para todo el repositorio: código, documentos de planeación y el
futuro manual.

Implica:

- Las dependencias deben tener licencia compatible con GPLv3 (MIT, Apache-2.0, BSD, ISC, Zlib,
  MPL-2.0, Unicode y similares). Lo controla `cargo-deny`. Quedan fuera GPL-2.0-only y las
  licencias propietarias.
- El CI genera los avisos de terceros y cada artefacto los lleva.
- Cambiar de licencia exigiría el permiso de cada contribuyente ([Q22]).
- Slint deja de tener conflicto de licencia (verificar sus niveles vigentes).

Si cambias a una licencia permisiva: la reimplementación limpia de [Q21] pasa a ser además una
obligación legal, y Slint vuelve a revisión.

---

### Q03 — Paridad de la primera versión pública

Respuesta: `diario`.

La escala usa nombres y no letras para no chocar con los bloques A–J del cuestionario:

| Nivel | Antes | Contenido |
| --- | --- | --- |
| `familiar` | A | La base del hito 1, pulida. |
| `diario` | B | `familiar` más: sesión con recuperación de pestañas sin guardar, buscar en archivos, edición en columna y multicursor, operaciones de línea (duplicar, mover, borrar, comentar, mayúsculas y minúsculas, sangría), plegado, vista dividida con dos vistas del mismo búfer, carpeta de trabajo, los lenguajes de [Q26], temas y atajos configurables. Sin plugins. |
| `completo` | C | `diario` más: macros, lenguajes definidos por el usuario, mapa del documento, lista de funciones, impresión y autocompletado. |

Implica: `diario` se entrega en el hito 2. Lo de `completo` y la larga cola se deciden por
peticiones reales de usuarios, nunca por la lista de 510 comandos del menú de Notepad++. En la
versión anterior del cuestionario, «Nivel A» equivalía a `diario` con mapa del documento y lista
de funciones.

Si cambias: `familiar` vacía el hito 2; `completo` lo alarga y adelanta macros, UDL e impresión.

---

### Q04 — Qué más tiene que estar en el hito 1

Respuesta: la base del hito 1 de [PROMPT.md](PROMPT.md) más estos básicos, baratos y que evitan
retrabajo:

- coincidencia de paréntesis;
- espacios en blanco visibles;
- seguir el tema claro u oscuro del sistema;
- DPI alto;
- rutas de configuración resueltas al ejecutar, con modo portable (archivo marcador junto al
  ejecutable);
- abrir y guardar en ANSI (Windows-1252 e ISO-8859-1), muy común en archivos en español.

Implica: buscar en archivos, plegado y texto bidireccional siguen en el hito 2. Q04 solo suma a
la base; no la sustituye.

---

### Q05 — Archivos de configuración de Notepad++

Respuesta: `ninguna`.

Implica: Amate tiene su propia configuración (formato recomendado: TOML, ver C5). No hay
importadores en la hoja de ruta. El hito 3 deja escrito el costo estimado de importar temas, UDL,
atajos y sesión, como trabajo no incluido.

Si cambias a `importar-estilos` o más: el hito 3 suma lectores de XML y un corpus de
instalaciones reales. Los archivos se leerían siempre del disco del usuario ([Q21]).

---

### Q06 — Plugins propios

Respuesta: `despues`.

Implica:

- No hay sistema de plugins antes de la 1.0. El plan trae un capítulo corto de cómo se añadiría,
  sin ABI ni diseño detallado.
- Desde el hito 1, cada acción del usuario es un comando con nombre y serializable. Eso habilita
  macros, paleta de comandos, atajos configurables y pruebas automatizadas.
- Por [Q19], Lua queda fuera (`mlua` compila C). Candidatos futuros: Rhai, o WebAssembly con
  `wasmi`, ambos en Rust puro.
- Los plugins DLL de Notepad++ nunca son objetivo de compatibilidad.

---

### Q07 — Idioma de la interfaz

Respuesta: español e inglés en la primera versión pública.

Implica: los textos viven fuera del código desde el hito 1 (recomendado: Fluent con
`fluent-rs`). Desde el inicio se prueba que caben los textos en español, que son más largos.

---

### Q08 — Idioma del repositorio y del manual

Respuesta: README bilingüe (inglés y español). El manual sale en los mismos idiomas que la
interfaz: español e inglés, en el hito 3. Los documentos de planeación, en español. Issues en
español o en inglés. Commits en inglés. Código, identificadores y comentarios en inglés.

Implica: el trabajo de documentación se duplica, y nadie que use la interfaz en inglés se queda
sin manual ([Q31]).

---

### Q09 — Paquetes y plataformas de la primera versión pública

Respuesta:

| Sistema | Arquitectura | Artefactos |
| --- | --- | --- |
| Windows 10 (22H2) y 11 | x64 | MSI y ZIP portable («descomprimir y ejecutar») |
| macOS, desde la versión que soporte el toolkit | universal (arm64 + x86_64) | DMG |
| Debian 12+ y Ubuntu 24.04+ (Linux Mint 22+ por herencia) | amd64 | `.deb` |
| Linux, resto de distribuciones | x86_64 | AppImage y `.tar.gz` de binarios |

Implica:

- Canal único: GitHub Releases ([Q13], [Q28]).
- Linux se compila en un contenedor `debian:bookworm` (glibc 2.36) y se prueba en contenedores
  limpios, no en el runner de CI.
- Windows 10 ya no recibe parches de consumo (verificar la fecha): se sigue prometiendo, pero las
  pruebas principales se hacen en Windows 11.
- Hito 4b, opcional: Flatpak, `.deb` arm64, Windows ARM64, repositorio apt propio, winget y
  Homebrew cask.
- Fuera salvo petición explícita: Windows x86, `.rpm`, web/WASM, FreeBSD, RISC-V, Snap y Mac App
  Store.

---

### Q10 — Firma de código y notarización

Respuesta: `despues` para los certificados de pago (Apple Developer y certificado de Windows).
En el hito 4 se solicita la firma gratuita para código abierto de SignPath Foundation (verificar
requisitos). Si la aprueban, la primera versión pública ya va firmada en Windows.

Implica:

- Construir y firmar son pasos distintos. La firma usa secretos que no viven en el repo ni en el
  CI público.
- El README explica cómo abrir la app sin firma en macOS (Gatekeeper) y en Windows (SmartScreen).
- Smart App Control de Windows 11 bloquea los binarios sin firmar y no admite excepciones: sin
  firma, esas personas no pueden abrir Amate.
- La 1.0 exige firma en Windows y notarización en macOS ([Q27]).

---

### Q11 — Tamaño de archivo

Respuesta: `normal`. Un archivo de 10 MB se abre, se desplaza y se edita con fluidez.

Implica:

- Arriba de 10 MB, modo degradado: abre con barra de progreso y, arriba de un umbral
  configurable, sin resaltado ni ajuste de línea. La interfaz nunca se congela. No hay promesa de
  fluidez.
- El documento vive en un rope desde el hito 1, nunca en un solo `String`.
- Quedan fuera: archivos de varios GB y el modo hexadecimal para binarios.

Si cambias a `grandes` o más: la carga diferida, `mmap` y el índice de líneas en segundo plano
pasan al hito 1.

---

### Q12 — Parecido visual con Notepad++

Respuesta:

Opciones: `familiar` (menús, pestañas arriba, barra de estado y atajos de Notepad++ en las
acciones que existan; iconos y marca propios) · `propia` (mismos comandos, disposición distinta) ·
`clon-visual` (también iconos, diálogos y densidad; sigue prohibido copiar el icono y la marca).

Recomendación: `familiar`.

---

### Q13 — Actualizaciones

Respuesta: `release`.

Implica: la versión nueva se descarga de GitHub Releases. Amate no hace ninguna conexión de red:
ni comprobación de versión ni telemetría. Si algún día se añade un aviso de versión, será opt-in
en el primer arranque, solo enviará la versión y usará el TLS del sistema ([Q19]).

---

### Q14 — Integración con el escritorio

Respuesta: `con-el-paquete`.

Implica: el `.deb` instala `.desktop`, iconos y tipos MIME. En Windows y macOS, el registro en
«Abrir con» entra en el hito 4. Amate nunca se apropia de las asociaciones por defecto. El verbo
«Editar con Amate» del menú contextual de Windows 11 exige un paquete MSIX firmado y queda para
después de la 1.0.

---

### Q15 — Tipo de proyecto

Respuesta: `publico`, por etapas.

| Momento | Se añade |
| --- | --- |
| Primer commit | LICENSE y README (inglés y español) |
| Hito 0, con el primer código | CI y SECURITY.md (reporte privado de vulnerabilidades de GitHub) |
| Primera versión pública | CONTRIBUTING con DCO ([Q22]), código de conducta y plantillas de issues |
| Tres o más contribuyentes recurrentes | GOVERNANCE.md |

---

### Q16 — Tecnología de la interfaz

Respuesta: `nativa`. La interfaz corre dentro del proceso y está escrita en Rust. Sin Electron y
sin un webview como interfaz principal.

---

### Q17 — Usuario principal y propuesta de valor

Respuesta: quien usa Notepad++ en Windows y quiere lo mismo en Linux y macOS.

Propuesta de valor, para el README: «Editor de texto 100 % Rust: arranca al instante, se usa
entero con teclado o lector de pantalla y no se conecta a nada.»

Implica: los archivos de varios GB no son objetivo ([Q11]).

---

### Q18 — Relación con Notepad++

Respuesta: sucesor espiritual.

Implica: misma filosofía (rápido, ligero, abre cualquier archivo de texto sin ceremonia), flujo y
atajos reconocibles, y libertad para corregir lo que está mal diseñado. No es fork ni port.

---

### Q19 — Código nativo en el binario

Respuesta: Rust estricto. Nada de C ni C++ compilado dentro del binario distribuido. Sí se usan
las API y bibliotecas del sistema operativo (Win32, AppKit y CoreText, Wayland y X11, drivers de
GPU).

Implica:

- Resaltado con `syntect` y su backend `fancy-regex`, sin Oniguruma.
- Búsqueda con `regex` y `fancy-regex`, sin PCRE2.
- Fuera: Scintilla, Lexilla, Tree-sitter, gtk4-rs, Qt, `mlua` y TLS con proveedores en C (`ring`,
  `aws-lc-rs`).
- El CI falla si el árbol de dependencias del binario incluye `cc`, `cmake` o `bindgen`.
- Las herramientas que no viajan en el binario (valgrind, libFuzzer) quedan fuera de la regla.

Si cambias: vuelven a la comparación Tree-sitter, PCRE2 y gtk4-rs.

---

### Q20 — Accesibilidad

Respuesta: requisito que filtra el toolkit.

Implica: teclado completo desde el hito 1. Lector de pantalla (NVDA, VoiceOver y Orca) verificado
antes de la primera versión pública. El hito 0 lo mide junto con el IME ([Q24]), y un toolkit que
no cumpla queda descartado.

---

### Q21 — Material de Notepad++

Respuesta: reimplementación limpia, y nada de Notepad++ dentro del proyecto.

Implica:

- No se copia ni se traduce código, comentarios, textos del manual, iconos ni recursos.
- Ningún archivo de Notepad++ viaja en los paquetes (temas, UDL, traducciones), aunque la GPL lo
  permitiría.
- Sí se puede leer su código y su manual para analizar y escribir especificaciones de
  comportamiento. Se cita por enlace y capítulo, y se parafrasea.
- La marca solo aparece en texto descriptivo («usa los atajos de Notepad++») y con este aviso:
  «Notepad++ es una marca de su titular. Amate no está afiliado a Notepad++ ni a sus autores.»

---

### Q22 — Contribuciones

Respuesta: DCO (`Signed-off-by`), sin CLA. No hay planes comerciales.

Implica: el mantenedor revisa y firma con `Signed-off-by` cada commit escrito con ayuda de un
agente. CONTRIBUTING pide lo mismo a quien contribuya código asistido.

---

### Q23 — Atajos en macOS

Respuesta: Cmd donde Notepad++ usa Ctrl, respetando los atajos que reserva el sistema (Cmd+H,
Cmd+Q y similares). En Windows y Linux, los mismos atajos que Notepad++. Los perfiles de teclado
intercambiables llegan después.

---

### Q24 — Validación en el hito 0

Respuesta: sí, dentro del hito 0.

Implica: el hito 0 entrega una ventana vacía empaquetada en los tres sistemas y no se cierra sin
medir IME, lector de pantalla, consumo en reposo, latencia de tecla, costo de empaquetado y build
universal de macOS. Los spikes viven en ramas `spike/` que se borran; lo que queda es el ADR en
`docs/adr/`. Si el toolkit falla el IME o la accesibilidad, se repite con el siguiente candidato.

---

### Q25 — Presupuestos de rendimiento

Respuesta: requisitos medidos en CI, que fallan ante una regresión.

| Métrica | Presupuesto |
| --- | --- |
| Arranque en frío | < 300 ms |
| Latencia de tecla | < 16 ms al p99 |
| RAM en reposo con un archivo de 1 MB | < 100 MB |
| Abrir un archivo de 10 MB | umbral que fija el hito 0 con mediciones |

Implica: en los runners compartidos se bloquea con conteo de instrucciones (Linux); los tiempos
de reloj se reportan sin bloquear.

---

### Q26 — Lenguajes con resaltado

Respuesta:

- Hito 1 (13): texto plano, Rust, C, C++, Python, JavaScript, TypeScript, JSON, HTML, CSS,
  Markdown, Shell y SQL.
- Primera versión pública (32): los 13 anteriores más C#, Go, INI, Java, Kotlin, Lua, PHP, TOML,
  XML, YAML, Dockerfile, diff/patch, Batch, PowerShell, PL/SQL, T-SQL, Makefile, CMake y LaTeX.

Implica: el juego de sintaxis que trae `syntect` no las cubre todas (verificar cuáles faltan).
Hay que sumar definiciones `.sublime-syntax` con licencia compatible con GPL-3.0 y conservar sus
avisos.

---

### Q27 — Versionado

Respuesta: SemVer, con 0.x hasta firmar.

Implica: el hito 1 es la 0.1.0, sin anuncio. La primera versión pública (hito 4) es una 0.N. La
1.0 llega con `diario` completo, firma ([Q10]) y manual. Lo que los documentos anteriores
llamaban «v1.1» es el hito 4b.

---

### Q28 — Procedencia de los releases

Respuesta: atestación de procedencia de GitHub Actions y `SHA256SUMS.txt`.

Implica: cada release publica `SHA256SUMS.txt`, en el formato que acepta `sha256sum -c`, y la
atestación de `actions/attest-build-provenance`, que se verifica con `gh attestation verify`. No
sustituye la firma de código.

---

### Q29 — Titular del copyright y de las cuentas

Respuesta: Carlos Ceballos, como persona: «Copyright © 2026 Carlos Ceballos y colaboradores». La
org de GitHub y el dominio van a su nombre.

Implica: si se usan recursos u horario de Gometrix, conseguir permiso por escrito antes de
publicar.

---

### Q30 — Finales de línea y codificación del repositorio

Respuesta: LF, fijado en `.gitattributes` (`* text=auto eol=lf`; `*.bat` y `*.cmd` con CRLF).
UTF-8 sin BOM.

---

### Q31 — Criterio de éxito

Respuesta: adopción externa. Amate salió bien si gente de fuera lo descarga, abre issues y
contribuye. Fracaso: que nadie de fuera lo use.

Implica:

- Sin telemetría ([Q13]): la adopción se mide con datos públicos de GitHub (descargas por release,
  issues, pull requests y estrellas).
- Cuando haya que recortar, gana lo que decide la primera impresión de quien llega de fuera:
  distribución (hito 4), firma (SignPath, [Q10]), README y manual bilingües ([Q08]) y CONTRIBUTING.
- Los canales del hito 4b (winget, Homebrew, apt) son los primeros candidatos a adelantarse si las
  descargas lo piden.

Origen: pregunta 12 de 04 («Criterios de éxito y prioridades») · J5 del cuestionario.

# Análisis de Notepad++ como referencia de Amate

Este análisis es insumo para la corrida de [PROMPT.md](PROMPT.md): el agente lo verifica y lo
amplía, no lo rehace. Las decisiones viven en [DECISIONES.md](DECISIONES.md) y aquí se citan por
su identificador, así: `[Q21]`. Donde una decisión cambia lo que implica un dato, la consecuencia
aparece junto al dato, en los párrafos «Con las decisiones».

Amate no es un fork ni un port de Notepad++: es su sucesor espiritual y una reimplementación limpia
([Q18], [Q21]). Notepad++ se lee como especificación de comportamiento; su código no es la base.

Los datos se midieron el 2026-10-15 sobre un clon superficial de `main` de
`notepad-plus-plus/notepad-plus-plus` y sobre la API pública de la organización en GitHub, para
calibrar el alcance real del proyecto. Cada dato fechado lleva «(verificado: 2026-10-15)», y lo que
nadie ha confirmado lleva «(a verificar)». Son el punto de partida, no una cita permanente: la
corrida del prompt los vuelve a comprobar el día en que se ejecuta, y la lista de hechos por
re-verificar está en [fusion.md](fusion.md).

Fuentes: la [organización de Notepad++](https://github.com/orgs/notepad-plus-plus/repositories),
su [repositorio principal](https://github.com/notepad-plus-plus/notepad-plus-plus) y la sección
*standalone* de la [página de PhotoCraft](https://getartcraft.com/apps/photocraft).

---

## 1. La organización completa

La organización tiene 9 repositorios públicos (verificado: 2026-10-15). No es un solo proyecto: es
un producto con satélites, y cada satélite es una decisión de alcance que Amate también tuvo que
tomar.

Datos de la API pública de GitHub (verificado: 2026-10-15). La columna «Estrellas» mide
popularidad, no tamaño: las 823 estrellas de `userDefinedLanguages` no son 823 lenguajes, y las
1 701 de `nppPluginList` no son 1 701 plugins. «Sin declarar» quiere decir que el repositorio no
declara licencia.

| Repositorio | Lenguaje | Estrellas | Licencia | Qué es | Respuesta de Amate |
| --- | --- | --- | --- | --- | --- |
| `notepad-plus-plus` | C++ | 29 569 | GPL-3.0; su archivo `LICENSE` agrega texto propio | La aplicación: C++ y Win32, con Scintilla, Lexilla y Boost.Regex incrustados. Solo Windows. | Se lee como especificación de comportamiento; no se porta ni se copia ([Q18], [Q21]). |
| `nppPluginList` | Python | 1 701 | Sin declarar | Catálogo firmado de plugins que alimenta el *Plugins Admin*. Es el ecosistema, y sus plugins son DLL de Windows. | Sin sistema de plugins antes de la 1.0; los plugins DLL nunca son objetivo de compatibilidad ([Q06]). |
| `userDefinedLanguages` | XML | 823 | Sin declarar | Colección comunitaria de lenguajes definidos por el usuario (UDL), en XML declarativo. | No se importa ([Q05]) ni viaja en los paquetes ([Q21]). En Amate, los lenguajes definidos por el usuario son del nivel `completo` ([Q03]). |
| `npp-usermanual` | HTML | 257 | Sin declarar | Manual de usuario publicado como sitio estático. Es la especificación de comportamiento más completa. | Se cita por enlace y capítulo, y se parafrasea; sus textos no se copian ([Q21]). Amate escribe su propio manual, en español e inglés ([Q08]). |
| `nppThemes` | Python | 169 | GPL-3.0 | Colección de temas (`stylers.xml`). | Ningún tema de Notepad++ viaja en los paquetes ([Q21]). |
| `wingup` | C | 83 | LGPL-3.0 | WinGup, actualizador genérico solo para Windows. | No se porta. Amate no tiene actualizador ni se conecta a la red; la versión nueva se descarga de GitHub Releases ([Q13]). |
| `nppShell` | C++ | 47 | GPL-3.0 | Extensión de shell del Explorador («Edit with Notepad++»). Puro Win32/COM. | No se porta. El registro en «Abrir con» entra en el hito 4; el verbo «Editar con Amate» del menú contextual de Windows 11 exige un paquete MSIX firmado y queda para después de la 1.0 ([Q14]). |
| `old-releases` | — | 4 | Sin declarar | Archivo histórico de binarios. | No aporta al diseño. |
| `VPAT-for-Npp` | — | 3 | Sin declarar | Informe de accesibilidad (VPAT). Señal de que la accesibilidad es un requisito real, no opcional. | La accesibilidad filtra el toolkit ([Q20]). |

**Lectura:** el valor de Notepad++ no está solo en el binario. Está también en el catálogo de
plugins, la colección de UDL, los temas, el manual, el actualizador y la integración con el
Explorador. Competir exige una respuesta para cada una de esas piezas, no solo para el editor; la
última columna resume la que ya dan las decisiones.

---

## 2. Anatomía del repositorio principal

Medido sobre el clon superficial de `main` (verificado: 2026-10-15):

```text
notepad-plus-plus/
├── PowerEditor/      35 MB   ← la aplicación (UI Win32, diálogos, configuración, instalador)
├── lexilla/         7.8 MB   ← 128 lexers de resaltado de sintaxis
├── scintilla/       6.3 MB   ← el componente de edición de texto real
├── boostregex/      1.8 MB   ← motor de expresiones regulares (Boost.Regex recortado)
└── BUILD.md, SUPPORTED_SYSTEM.md, LICENSE, appveyor.yml
```

Líneas de código, contando solo `.cpp`, `.h` y `.cxx`, sin recursos ni XML
(verificado: 2026-10-15):

| Componente | LOC | Nota |
| --- | --- | --- |
| `PowerEditor/src` | 158 145 (285 archivos) | La capa de aplicación. Amate la reimplementa entera, sin copiar código ([Q21]). |
| `scintilla` | 73 252 | Motor de edición. Existe desde 1999; backends Win32, GTK, Cocoa y Qt. |
| `lexilla` | 79 583 | 128 lexers. Traducirlos a mano no es viable. |
| **Total aproximado** | **~311 000 LOC** | Sin contar Boost.Regex, los recursos `.rc` ni los XML de configuración. |

`boostregex/` es una copia recortada de Boost.Regex. La sintaxis que ve el usuario es la de
Boost.Regex, de estilo Perl según `npp-usermanual` (a verificar). Boost es C++, así que [Q19] la
deja fuera: Amate busca con `regex` y `fancy-regex`, sin PCRE2, que también es C. Los patrones que
un usuario guardó en Notepad++ se escribieron para Boost.Regex y las diferencias de dialecto se
notan; el plan las documenta con tres ejemplos que ese usuario notaría ([PROMPT.md](PROMPT.md)).

### 2.1 Notepad++ es un *shell* alrededor de Scintilla

Esta es la conclusión más importante del análisis. `PowerEditor/src/ScintillaComponent/` contiene
`ScintillaEditView`, `Buffer`, `AutoCompletion`, `FindReplaceDlg`, `DocTabView`,
`SmartHighlighter`, `UserDefineDialog`, `columnEditor`, `Printer`, `FunctionCallTip` y
`xmlMatchedTagsHighlighter` (verificado: 2026-10-15). Es decir: **Notepad++ no implementa edición
de texto.** Implementa una aplicación Win32 que conduce a Scintilla.

Por eso la pregunta «¿qué motor de edición uso?» era la decisión arquitectónica número uno: definía
si había que reescribir 158 k LOC (solo la aplicación) o 311 k LOC (también el motor y los lexers).

**Con las decisiones.** [Q19] la cerró por el lado largo: nada de C ni C++ compilado dentro del
binario, así que Scintilla y Lexilla quedan descartadas y Amate pone también el motor de edición y
el resaltado. No los reescribe línea por línea: el documento vive en un rope desde el hito 1
([Q11]) y el resaltado usa `syntect` con gramáticas `.sublime-syntax`, que son datos y no código
([Q19], [Q26]). Aun así, la referencia de tamaño es el total, no `PowerEditor` solo.

### 2.2 La interfaz es Win32 pura y no es portable

`PowerEditor/src/WinControls/` tiene 30 subdirectorios (verificado: 2026-10-15), todos diálogos y
controles nativos de Windows definidos en archivos de recursos `.rc`:

```text
AboutDlg          AnsiCharPanel     ClipboardHistory   ColourPicker      ContextMenu
DockingWnd        DocumentMap       DoubleBuffer       FileBrowser       FindCharsInRange
FunctionList      Grid              ImageListSet       OpenSaveFileDialog PluginsAdmin
Preference        ProjectPanel      ReadDirectoryChanges shortcut        SplitterContainer
StaticDialog      StatusBar         TabBar             TaskList          ToolBar
ToolTip           TrayIcon          TreeView           VerticalFileSwitcher WindowsDlg
```

A eso se suman `NppDarkMode.cpp` (modo oscuro implementado por *subclassing* de Win32),
`dpiManagerV2` (gestión de DPI por monitor), `notepad++.exe.manifest`, `TrayIcon`,
`ReadDirectoryChanges` (API de Windows para vigilar el disco) y `RegExt` (asociación de extensiones
en el registro).

**Nada de esto se porta. Todo se reinventa.** El modo oscuro, el DPI, el *docking* de paneles, la
bandeja del sistema y la vigilancia de archivos son cinco subsistemas completos que en Rust
multiplataforma se resuelven desde cero o eligiendo bien el toolkit.

**Con las decisiones.** La interfaz es nativa: corre dentro del proceso y está escrita en Rust, sin
Electron y sin un webview como interfaz principal ([Q16]). Las API del sistema (Win32, AppKit y
CoreText, Wayland y X11) sí se usan; lo prohibido es compilar C o C++ dentro del binario ([Q19]).
La accesibilidad filtra el toolkit: el hito 0 mide lector de pantalla e IME, y un toolkit que no
cumpla queda descartado ([Q20], [Q24]). El teclado completo llega en el hito 1, y el lector de
pantalla se verifica con NVDA, VoiceOver y Orca antes de la primera versión pública ([Q20]). Lo que
en Notepad++ hace `RegExt`, en Amate es el registro en «Abrir con», sin apropiarse nunca de las
asociaciones por defecto ([Q14]).

### 2.3 Superficie funcional: 510 comandos de menú

El menú principal (`IDR_M30_MENU` en `Notepad_plus.rc`) tiene **510 elementos** repartidos en 13
menús de nivel superior: File, Edit, Search, View, Encoding, Language, Settings, Tools, Macro, Run,
Plugins, Window y Help (verificado: 2026-10-15).

Ejemplos del nivel de granularidad que acumuló el producto en dos décadas
(verificado: 2026-10-15):

- `Edit → Line Operations` tiene **26 comandos**, de los cuales 14 son variantes de ordenar líneas
  (lexicográfico, ignorando mayúsculas, orden de la configuración regional, como enteros, como
  decimales con coma, como decimales con punto y por longitud; cada uno ascendente y descendente).
- `Edit → Convert Case to` tiene 8 variantes, incluida `ranDOm CasE`.
- `Search` tiene familias completas de *Style all occurrences of token* (5 estilos),
  *Style one token*, *Clear style*, *Jump up*, *Jump down* y *Copy styled text*.
- `Edit → Paste Special` incluye pegado de HTML, RTF y contenido binario.

**Lectura:** «paridad con Notepad++» no es un objetivo verificable. Sin datos de uso no se puede
decir cuántos de esos 510 comandos usa la mayoría; lo probable es que sea una fracción pequeña, y
el resto es la larga cola que hace al producto insustituible para nichos concretos.

**Con las decisiones.** [Q03] fija tres niveles de paridad con nombre: `familiar`, `diario` y
`completo` (el contenido de cada uno está en [DECISIONES.md](DECISIONES.md)). La primera versión
pública tiene paridad `diario`, que se entrega en el hito 2. Lo de `completo` y la larga cola se
deciden por peticiones reales de usuarios, nunca por la lista de 510 comandos.

### 2.4 Los datos: XML en todas partes

Conteos del repositorio principal (verificado: 2026-10-15):

| Activo | Cantidad | Archivo o ruta |
| --- | --- | --- |
| Lenguajes con resaltado integrado | 95 | `langs.model.xml` |
| Lexers disponibles en Lexilla | 128 | `lexilla/lexers/` |
| Traducciones de la interfaz | 94 | `installer/nativeLang/` |
| Temas | 22 | `installer/themes/` |
| Definiciones de *Function List* | 48 | `installer/functionList/` |

Estos archivos parecían el mayor regalo del proyecto original: son datos declarativos, no código.
Leerlos cuesta poco comparado con reescribir lógica, y por eso la compatibilidad de configuración
parecía la palanca de migración más barata.

**Con las decisiones.** [Q05] es `ninguna`: Amate tiene su propia configuración (formato
recomendado: TOML) y no hay importadores en la hoja de ruta. El hito 3 deja escrito el costo
estimado de importar temas, UDL, atajos y sesión, como trabajo no incluido. Si [Q05] cambiara, los
archivos se leerían siempre del disco del usuario: leer un archivo que el usuario ya tiene no es
redistribuirlo, y ningún archivo de Notepad++ viaja en los paquetes de Amate ([Q21]). Las licencias
de cada pieza están en la sección 3. Frente a estas cantidades, Amate resalta 13 lenguajes en el
hito 1 y 32 en la primera versión pública ([Q26]), su interfaz llega en español e inglés ([Q07]) y
la lista de funciones es del nivel `completo` ([Q03]).

### 2.5 El modelo de plugins es una frontera dura

`PowerEditor/src/MISC/PluginsManager/` define la API pública: `PluginInterface.h` y
`Notepad_plus_msgs.h`, este último con **326 mensajes** `NPPM_*` (comandos hacia Notepad++) y
`NPPN_*` (notificaciones hacia el plugin) (verificado: 2026-10-15).

El mecanismo es: una DLL de Windows exporta funciones C, y la comunicación ocurre mediante
`SendMessage` de Win32 sobre `HWND`, pasando punteros a estructuras y `HWND` de Scintilla
directamente al plugin.

**Esto es incompatible con cualquier producto multiplataforma en Rust.** No hay camino de
compatibilidad parcial: los plugins existentes manipulan handles de ventana de Win32. El catálogo
`nppPluginList` representa un ecosistema que **no se puede heredar**; sus 1 701 estrellas miden la
popularidad del catálogo, no cuántos plugins tiene. Hacía falta un modelo de extensibilidad nuevo,
y esa decisión (WebAssembly, Lua, procesos externos, LSP, o ninguno en la primera versión pública)
condicionaba la arquitectura desde el primer día.

**Con las decisiones.** [Q06] es `despues`: no hay sistema de plugins antes de la 1.0, y el plan
trae solo un capítulo corto de cómo se añadiría, sin ABI ni diseño detallado; el hito 5 es ese
anexo, nunca trabajo programado. Lo que sí condiciona la arquitectura desde el primer día ya está
fijado: desde el hito 1, cada acción del usuario es un comando con nombre y serializable, lo que
habilita macros, paleta de comandos, atajos configurables y pruebas automatizadas ([Q06]). Lua
queda fuera porque `mlua` compila C ([Q19]); los candidatos futuros son Rhai, o WebAssembly con
`wasmi`, ambos en Rust puro. Los plugins DLL de Notepad++ nunca son objetivo de compatibilidad.

### 2.6 Compatibilidad de plataformas: el punto de partida es solo Windows

`SUPPORTED_SYSTEM.md` solo enumera versiones de Windows, con 8.1, 10 y 11 como soportadas
(verificado: 2026-10-15). No hay Linux ni macOS, ni siquiera a nivel experimental. El proyecto se
construye con Visual Studio (`visual.net/`), con un `makefile` de GCC/MinGW como alternativa, y se
empaqueta con NSIS (`nppSetup.nsi`) y WiX/MSI (verificado: 2026-10-15).

Lo relevante: **no hay «puerto» que hacer.** Amate es una reimplementación, no una portabilidad.
Eso es una buena noticia en libertad de diseño y una mala en cuánto trabajo es realmente nuevo.

**Con las decisiones.** Amate es sucesor espiritual, ni fork ni port ([Q18]). Desde el hito 0
compila con un workspace de Cargo en Windows, Linux y macOS. Promete Windows 10 (22H2) y 11 en x64;
macOS con binario universal, desde la versión que soporte el toolkit; Debian 12+ y Ubuntu 24.04+ en
amd64, y el resto de Linux en x86_64 ([Q09]). Windows 8.1, que Notepad++ todavía soporta, no está
en esa lista. Windows 10 ya no recibe parches de consumo (a verificar): se sigue prometiendo, pero
las pruebas principales se hacen en Windows 11 ([Q09]). En Windows, los artefactos son MSI y ZIP
portable ([Q09]).

---

## 3. Licencia, material de Notepad++ y marca

Los riesgos legales que este análisis planteaba antes de escribir código ya tienen decisión
([Q01], [Q02], [Q21]). El marco para leerlos distingue tres cosas:

- **Inspirarse** en el comportamiento observable: sí. El código y el manual de Notepad++ se pueden
  leer para analizar y escribir especificaciones de comportamiento; se citan por enlace y capítulo,
  y se parafrasean ([Q21]).
- **Reutilizar dependencias** con licencia compatible con GPLv3: sí, siempre que no metan C ni C++
  compilado en el binario ([Q02], [Q19]). Lo controla `cargo-deny`.
- **Copiar** o traducir código, comentarios, textos del manual, iconos o recursos de Notepad++: no,
  en ningún caso ([Q21]).

### 3.1 Licencia del código

Notepad++ es GPL-3.0; su archivo `LICENSE` agrega texto propio (verificado: 2026-10-15). Falta
confirmar qué agrega ese texto y si cambia algo de lo que sigue (a verificar).

Amate es `GPL-3.0-or-later` en todo el repositorio ([Q02]). Con esa licencia, copiar o derivar
código de Notepad++ sería compatible en principio, pero [Q21] lo prohíbe: la reimplementación es
limpia y nada de Notepad++ entra en el proyecto. Hoy eso es una regla del proyecto, no una
obligación legal; si [Q02] cambiara a una licencia permisiva, pasaría a ser además una obligación
legal.

### 3.2 Configuración, traducciones y temas

- Las traducciones (`installer/nativeLang/`), los temas (`installer/themes/`) y las definiciones de
  *Function List* (`installer/functionList/`) viven dentro del repositorio principal, así que los
  cubre su licencia: GPL-3.0 con el texto propio de `LICENSE`.
- `nppThemes` es GPL-3.0 (verificado: 2026-10-15).
- `userDefinedLanguages` no declara licencia (verificado: 2026-10-15). Sin licencia declarada, por
  defecto no hay permiso para redistribuir sus archivos.
- `npp-usermanual` tampoco declara licencia (verificado: 2026-10-15). Sus textos se citan por
  enlace y capítulo y se parafrasean, nunca se copian ([Q21]).
- Ninguno de estos archivos viaja en los paquetes de Amate, aunque la GPL lo permitiría para los
  que son GPL ([Q21]).

Leer un archivo que el usuario ya tiene en su disco y redistribuirlo dentro de un paquete son dos
cosas distintas. Hoy Amate no lee ninguno, porque [Q05] es `ninguna`; si cambiara, los leería
siempre del disco del usuario ([Q05], [Q21]).

### 3.3 Marca

«Notepad++» es una marca registrada (a verificar). Ningún nombre, binario, crate, paquete, repo,
dominio, logo ni icono de Amate usa «Notepad++» o un derivado confundible ([Q01], [Q21]). Ejemplos
de lo que queda prohibido: llamarse `Notepad++ Rust` o `notepad-plus-plus-rs`, o usar el logotipo
del camaleón. La carpeta `notepadrust` es solo un nombre de trabajo local ([Q01]).

La marca solo aparece en texto descriptivo, como «usa los atajos de Notepad++», y con este aviso
([Q21]):

> Notepad++ es una marca de su titular. Amate no está afiliado a Notepad++ ni a sus autores.

El posicionamiento es el de [Q18]: sucesor espiritual, con la misma filosofía y atajos
reconocibles, nunca fork ni port. Amate no se presenta como «compatible con la configuración de
Notepad++»: con [Q05] en `ninguna`, esa promesa sería falsa. El parecido visual aplica, como
supuesto, la recomendación `familiar` de [Q12]: menús, pestañas arriba, barra de estado y atajos de
Notepad++ en las acciones que existan, con iconos y marca propios.

Antes de publicar cualquier artefacto, crear el repo público o reservar el crate, falta verificar
la marca «Amate» (IMPI, USPTO, EUIPO) y el dominio ([Q01]).

### 3.4 Scintilla y Lexilla

Scintilla y Lexilla llevan una licencia permisiva propia, estilo HPND, con el aviso de copyright de
Neil Hodgson en `scintilla/License.txt` (verificado: 2026-10-15). Al ser permisiva, no choca con
`GPL-3.0-or-later`, y por eso «reutilizar Scintilla» era una opción legalmente viable. Queda
descartada por una razón que no es legal: [Q19] prohíbe compilar C o C++ dentro del binario.

---

## 4. Lo que PhotoCraft (ArtCraft) enseña sobre distribución

La sección *standalone* de la página de PhotoCraft es un buen modelo porque resuelve el mismo
problema que Amate: una aplicación nativa en Rust, de código abierto, para tres sistemas
operativos. Es referencia de distribución, no de alcance.

Matriz de artefactos que publicaba esa sección, en la versión 0.7.0 (verificado: 2026-10-15):

| Plataforma | Artefactos ofrecidos |
| --- | --- |
| macOS | `.dmg` universal (Apple Silicon + Intel) + CLI universal en `.zip` |
| Windows | `.msi` x64 · portable `.zip` x64 · `.msi` x86 · portable x86 · `.msi` ARM64 · portable ARM64 |
| Linux | Flatpak x86_64 (`flatpak install --user`) |
| FreeBSD | binarios planos en `.tar.gz` |
| Web | build WebAssembly en `.zip` para autohospedar |
| Todas | `SHA256SUMS.txt` para verificar cualquier descarga |

Ese día el release de GitHub iba por delante de la página: en la página 0.7.0 solo Flatpak; el
release 0.9.0 publica más archivos de Linux (a verificar). La página sirve como modelo de
presentación; la lista real de artefactos es la del release vigente.

Más dos cosas que no son artefactos pero sí producto (verificado: 2026-10-15):

- **Un instalador-lanzador propio** («ArtCraft Launcher») que elige el *build* correcto para la
  máquina, instala y mantiene actualizadas todas las apps de la familia. Detecta instalaciones
  standalone previas y las adopta.
- **Una vía «ejecútalo hoy desde el código»**: versión mínima de Rust declarada (1.90+),
  `git clone`, `cargo run --release -p <paquete>`. Tres líneas, sin dependencias de sistema que
  explicar.

**Dos observaciones para Amate.** Primera, Linux: sea cual sea la matriz real de PhotoCraft, la de
Amate sale del requisito de Debian y derivados, no de PhotoCraft. Flatpak no viene instalado por
defecto en Debian (a verificar), la comunidad Debian suele preferir `.deb`, y el sandbox de Flatpak
hace pasar el acceso al disco por portales XDG, una fricción constante para un editor de texto. Por
eso [Q09] pone el `.deb` amd64 como artefacto de primera clase para Debian 12+ y Ubuntu 24.04+
(Linux Mint 22+ por herencia), AppImage y `.tar.gz` para el resto de distribuciones, y deja Flatpak
para el hito 4b.

Segunda: la variante *portable* de Windows no es un lujo. Es un requisito cultural del público de
Notepad++, que es el usuario principal de Amate ([Q17]) y que históricamente lo lleva en memorias
USB y lo usa sin permisos de administrador. [Q09] la incluye como ZIP portable («descomprimir y
ejecutar»).

Lo que Amate toma del modelo, en corto: la aplicación se instala sola, sin lanzador; los binarios
viven en GitHub Releases, que es el canal único ([Q09]); la versión nueva se obtiene descargando el
release nuevo ([Q13]); cada release publica `SHA256SUMS.txt` y la atestación de procedencia de
GitHub ([Q28]); y hay una vía para compilar desde el código con la MSRV declarada. No toma el
lanzador ni el alcance de PhotoCraft (lienzo GPU, PSD, web, FreeBSD). El detalle de qué copiar y
qué no está en [04-distribucion-y-empaquetado.md](04-distribucion-y-empaquetado.md).

---

## 5. Calibración honesta del esfuerzo

Para fijar expectativas sin hablar de plazos, que es lo que no se puede estimar de forma confiable.
La columna «Notepad++ hoy» usa los datos medidos arriba (verificado: 2026-10-15); la última dice
qué cambió al tomar las decisiones.

| Dimensión | Notepad++ hoy | Implicación para una primera versión pública razonable | Con las decisiones |
| --- | --- | --- | --- |
| Código de aplicación | 158 k LOC | Una primera versión pública con el 20 % de los comandos ya es un proyecto grande por sí solo. | La primera versión pública (0.N, hito 4) tiene paridad `diario`, sin plugins ([Q03], [Q27]). |
| Motor de edición | 73 k LOC (Scintilla, desde 1999) | Reescribirlo es un proyecto en sí; reutilizarlo vía FFI era una decisión estratégica legítima. | [Q19] descarta el FFI. Motor propio en Rust, con el documento en un rope desde el hito 1; un archivo de 10 MB se edita con fluidez y arriba de eso entra el modo degradado ([Q11]). |
| Lexers | 128 (79 k LOC) | Inviables a mano. Había que elegir entre Lexilla vía FFI, Tree-sitter o gramáticas TextMate. | `syntect` con gramáticas `.sublime-syntax` y su backend `fancy-regex`, sin Oniguruma; Lexilla y Tree-sitter quedan fuera porque meten C o C++ compilado en el binario ([Q19]). 13 lenguajes en el hito 1 y 32 en la primera versión pública; el juego que trae `syntect` no los cubre todos y falta saber cuáles (a verificar), así que se suman definiciones `.sublime-syntax` con licencia compatible con GPL-3.0, conservando sus avisos ([Q26]). |
| Comandos de menú | 510 | Hay que definir niveles de paridad y aceptar que la larga cola llega tarde o nunca. | Niveles `familiar`, `diario` y `completo`; la larga cola se decide por peticiones reales, nunca por la lista de 510 ([Q03]). |
| Plataformas | 1 (Windows) | Pasar a 3 multiplica el costo de empaquetado, firma, pruebas y soporte, no lo suma. | Tres sistemas y seis tipos de artefacto: MSI, ZIP portable, DMG, `.deb`, AppImage y `.tar.gz` ([Q09]). El costo de empaquetar se mide en el hito 0 ([Q24]). Firma según [Q10]: certificados de pago después; en el hito 4 se solicita la firma gratuita para código abierto de SignPath Foundation, cuyos requisitos hay que revisar (a verificar). Si la aprueban, la primera versión pública ya va firmada en Windows. |
| Arquitecturas | x86, x64, ARM64 | PhotoCraft publica 6 artefactos solo para Windows (página 0.7.0). | Windows solo x64; macOS universal (arm64 + x86_64); Linux x86_64 (`amd64` en el `.deb`). Windows ARM64 y `.deb` arm64 en el hito 4b; Windows x86 fuera salvo petición explícita ([Q09]). |
| Idiomas de interfaz | 94 | Define desde el inicio el flujo de i18n o nunca lo tendrás. | Español e inglés en la primera versión pública; textos fuera del código desde el hito 1 ([Q07]). |
| Ecosistema de plugins | Catálogo `nppPluginList` (1 701 estrellas: popularidad, no número de plugins) | No heredable. Es un hueco competitivo real que hay que mitigar o aceptar. | Se acepta: sin plugins antes de la 1.0, y la capa de comandos del hito 1 deja la puerta abierta ([Q06]). |

El riesgo dominante no es técnico, es de alcance. La forma de gestionarlo es decidir por adelantado
qué *no* se hace, y eso es lo que hacen las decisiones de [DECISIONES.md](DECISIONES.md). El
razonamiento y las opciones de cada una están en [02-cuestionario.md](02-cuestionario.md).

---

## 6. Lo que falta por analizar

Este análisis mide tamaño, superficie, formatos y dependencias. La corrida de
[PROMPT.md](PROMPT.md) lo verifica y lo completa con lo que todavía no cubre:

- El mapa de módulos completo: ventana y pestañas, búfer, búsqueda, codificaciones, lenguajes y
  estilos, configuración, sesión y macros, parámetros de línea de comandos, plugins y
  actualización. Aquí solo están `ScintillaComponent`, `WinControls` y `PluginsManager`.
- Qué partes son Win32 y hay que rehacer, y qué comportamiento es independiente del sistema, más
  allá de los subsistemas de la sección 2.2.
- La lista real de lenguajes con lexer, sacada de `langs.model.xml`. Aquí solo está el conteo.
- Los parámetros de línea de comandos documentados.
- Cómo decide la codificación al abrir y qué conserva al guardar (UTF-8, BOM, UTF-16, páginas
  ANSI).
- Qué hace con un archivo que cambia en disco, con uno de solo lectura y con uno muy grande.
- Qué agrega el texto propio de `LICENSE`, y la confirmación de las licencias de `scintilla/` y
  `lexilla/`.
- El inventario de comportamiento por áreas de `npp-usermanual`. La lista corta de funciones del
  wiki está incompleta y no sirve como inventario (verificado: 2026-10-15).
- Qué capacidades viven fuera del núcleo según `nppPluginList`, cuántos plugins tiene en realidad
  y cómo se agrupan para el plan.
- El formato de los XML de `userDefinedLanguages` y `nppThemes`, solo para estimar el costo de
  importarlos ([Q05]).
- Qué integración de escritorio da `nppShell` y qué cubre el informe de `VPAT-for-Npp`.
- La automatización propia de Notepad++: `appveyor.yml`, pruebas, compilación y publicación.

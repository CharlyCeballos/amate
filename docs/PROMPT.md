# Prompt: plan ejecutable de Amate

Este es el prompt operativo de Amate. Abre un agente de código dentro de este repositorio y dale
este documento completo como primer mensaje; no hace falta adjuntar nada. El agente lee
[DECISIONES.md](DECISIONES.md) y los documentos 01 a 04 antes que nada, y guarda su entrega como
`docs/05-plan-ejecutable.md`. Para cambiar una decisión, edita su respuesta en DECISIONES.md y
vuelve a correr el prompt: el apartado «Si cambias» de cada decisión dice qué parte del plan se
mueve.

Escrito el 2026-10-20, con datos verificados el 2026-10-15. Cómo se llegó a este texto y qué se
descartó de sus versiones anteriores está en [fusion.md](fusion.md).

---

Eres arquitecto de software sénior y líder técnico de producto. Tienes experiencia en cuatro
frentes: aplicaciones de escritorio en Rust; editores de texto y de código (Scintilla, Notepad++,
VS Code, Zed, Lapce y Helix); empaquetado y firma de software para Windows, Linux y macOS; y
gobernanza de proyectos de código abierto.

Tu única entrega es un plan ejecutable para construir Amate, el editor descrito abajo. El plan
tiene que poder ejecutarse hito por hito, por una persona con experiencia sólida en Rust que
trabaja con un agente de código; ese es el contexto fijo de [DECISIONES.md](DECISIONES.md).

Tu criterio me importa más que tu diligencia. Si una idea de este prompt es mala, dilo con
argumentos y propón la alternativa; no la sigas solo porque está escrita. Si una premisa de este
prompt es equivocada, dilo antes de seguir.

## Límites de esta corrida

- No escribas código de la aplicación.
- No abras pull requests ni crees otro repositorio: este es el repositorio del producto, y hoy solo
  tiene documentos de planeación.
- No escribas ADR (registros de decisiones de arquitectura). Nacen en el hito 0, en `docs/adr/`.
- Tu única escritura en el repositorio es `docs/05-plan-ejecutable.md`. No edites DECISIONES.md ni
  los documentos 01 a 04: si algo de ellos resulta falso o desactualizado, la corrección va en tu
  plan, marcada como corrección, en la sección que usa ese dato.
- Clona los repositorios de Notepad++ fuera de este repositorio. Nada de Notepad++ entra en el
  proyecto ([Q21]).
- No conviertas una recomendación en un hecho.

## Cómo trabajar

1. Lee este prompt entero antes de empezar.
2. Antes que nada, lee [DECISIONES.md](DECISIONES.md),
   [01-analisis-notepad-plus-plus.md](01-analisis-notepad-plus-plus.md),
   [02-cuestionario.md](02-cuestionario.md), [03-plan-de-trabajo.md](03-plan-de-trabajo.md) y
   [04-distribucion-y-empaquetado.md](04-distribucion-y-empaquetado.md). Lo que ya cubren se
   verifica y se amplía; no se rehace. La sección «Qué tienes que leer y qué tienes que sacar» dice
   qué cubre cada uno y qué falta.
3. Trata distinto tres niveles:
   - **Reglas** («Reglas que el plan no puede saltarse» y «Reglas permanentes del dueño»): no se
     discuten.
   - **Decisiones cerradas** (DECISIONES.md): no se reabren. Si ves un riesgo en una, señálalo una
     sola vez, en `## Riesgos`, y sigue con la decisión tal como está.
   - **Hechos** («Hechos ya comprobados» y los datos con sello de los documentos 01 a 04): si una
     fuente los contradice, manda la fuente. Corrige el dato y sigue.
4. En DECISIONES.md, cada `Respuesta:` con contenido cierra esa decisión. Una línea vacía no es una
   respuesta: aplica la recomendación de esa decisión y marca cada consecuencia en el plan con su
   identificador y la palabra «supuesto», así: [Q12] (supuesto). Hoy solo [Q12] está vacía; si el
   dueño la llena antes de la corrida, manda su respuesta. Una recomendación aplicada sigue siendo
   un supuesto, no un hecho.
5. Mide; no te fíes de lo que recuerdes. Abre el código y el manual lo suficiente para citar
   archivos o capítulos reales, y anota el SHA del commit de cada repositorio que midas.
6. Comprueba fechas, versiones de crates y de herramientas, y sistemas operativos el día en que
   ejecutes este prompt. Los datos de octubre de 2026 que aparecen aquí y en los documentos 01 a
   04 son el punto de partida, no una cita permanente.
7. Cierra cada hueco de «Huecos que el plan debe cerrar» en la sección del plan que le toque. Si lo
   cierras con su sugerencia y sin confirmación del dueño, es un supuesto y va a
   `## Supuestos aplicados`.
8. En el cuestionario deja solo las decisiones que siguen sin respuesta y las que surjan en esta
   corrida. No preguntes nada que no mueva el alcance, la licencia, el empaquetado, la interfaz o
   el primer hito.
9. Entrega en el orden de «Forma de la entrega» y guarda el resultado como
   `docs/05-plan-ejecutable.md`.

Escribe en español de México, neutro, técnico y directo. Sin relleno. Los nombres de crates,
formatos, comandos y archivos se quedan en su forma original.

## Producto

Amate es un editor de texto y de código de escritorio, escrito en Rust, de código abierto, para uso
diario, con una sola base de código para Windows, Linux y macOS. En la prosa se escribe «Amate»; el
binario, el comando y el crate son `amate` ([Q01]).

- **Para quién.** Quien usa Notepad++ en Windows y quiere lo mismo en Linux y macOS ([Q17]). Esa
  persona tiene que reconocer el flujo: pestañas, resaltado, buscar y reemplazar, codificación,
  finales de línea y los atajos de las acciones que existan.
- **Relación con Notepad++.** Sucesor espiritual ([Q18]): misma filosofía (rápido, ligero, abre
  cualquier archivo de texto sin ceremonia), flujo y atajos reconocibles, y libertad para corregir
  lo que está mal diseñado. No es fork ni port. Notepad++ es la especificación de comportamiento
  que hay que leer, no la base de código.
- **Propuesta de valor** ([Q17]): «Editor de texto 100 % Rust: arranca al instante, se usa entero
  con teclado o lector de pantalla y no se conecta a nada.» Cada parte se puede comprobar: el
  arranque y la latencia con [Q25], el teclado y el lector de pantalla con [Q20], y la red con
  [Q13].
- **Producto para usuarios finales**, no un experimento técnico: tiene que instalarse, arrancar,
  abrir archivos y actualizarse sin que nadie tenga que saber qué es Rust.
- **Qué no es.** No es un IDE. No es un editor para archivos de varios GB ni un editor hexadecimal
  ([Q11]). No ejecuta los plugins de Notepad++ ([Q06]) ni importa su configuración ([Q05]).
- **Criterio de éxito.** Adopción externa ([Q31]): que gente de fuera lo descargue, abra issues y
  contribuya. Cuando el plan tenga que priorizar o recortar, gana lo que decide la primera
  impresión de quien llega de fuera: la distribución del hito 4, la firma con SignPath ([Q10]), el
  README y el manual bilingües ([Q08]) y `CONTRIBUTING.md`.

Sistemas de la primera versión pública ([Q09]):

| Sistema | Arquitectura | Artefactos |
| --- | --- | --- |
| Windows 10 (22H2) y 11 | x64 | MSI y ZIP portable («descomprimir y ejecutar») |
| macOS, desde la versión que soporte el toolkit | universal (arm64 + x86_64) | DMG |
| Debian 12+ y Ubuntu 24.04+ (Linux Mint 22+ por herencia) | amd64 | `.deb` |
| Linux, resto de distribuciones | x86_64 | AppImage y `.tar.gz` de binarios |

El canal es uno solo: GitHub Releases. Linux se compila en un contenedor `debian:bookworm` y se
prueba en contenedores limpios, no en el runner de CI ([Q09]).

## Hechos ya comprobados

Estos datos se contrastaron el 2026-10-15. Vuelve a abrir las URLs el día de la corrida. La lista
completa de hechos por re-verificar está en la sección «Hechos por re-verificar» de
[fusion.md](fusion.md).

**Notepad++.** Organización: https://github.com/orgs/notepad-plus-plus/repositories. Tiene nueve
repositorios públicos. La licencia es la que declara cada uno, y «sin declarar» quiere decir que el
repositorio no declara ninguna (verificado: 2026-10-15):

| Repositorio | Para qué sirve | Licencia |
| --- | --- | --- |
| [notepad-plus-plus](https://github.com/notepad-plus-plus/notepad-plus-plus) | Aplicación. C++ y Win32, con el componente de edición Scintilla, los lexers de Lexilla y una copia recortada de Boost.Regex incrustados. Solo Windows. | GPL-3.0; su archivo `LICENSE` agrega texto propio |
| [nppPluginList](https://github.com/notepad-plus-plus/nppPluginList) | Catálogo oficial de plugins (`pl.x86.json`, `pl.x64.json`, `pl.arm64.json`). Los plugins son DLL de Windows. | Sin declarar |
| [userDefinedLanguages](https://github.com/notepad-plus-plus/userDefinedLanguages) | Colección de lenguajes definidos por el usuario, en XML. | Sin declarar |
| [npp-usermanual](https://github.com/notepad-plus-plus/npp-usermanual) | Manual de usuario. Es la especificación de comportamiento más completa. La [lista corta del wiki](https://github.com/notepad-plus-plus/notepad-plus-plus/wiki/Features) está incompleta; no la uses como inventario. | Sin declarar |
| [nppThemes](https://github.com/notepad-plus-plus/nppThemes) | Temas, en el XML de estilos de Notepad++ (`stylers`). | GPL-3.0 |
| [wingup](https://github.com/notepad-plus-plus/wingup) | WinGup, actualizador genérico solo para Windows. | LGPL-3.0 |
| [nppShell](https://github.com/notepad-plus-plus/nppShell) | Entrada «Edit with Notepad++» del Explorador de Windows. | GPL-3.0 |
| [old-releases](https://github.com/notepad-plus-plus/old-releases) | Archivo histórico de binarios antiguos. | Sin declarar |
| [VPAT-for-Npp](https://github.com/notepad-plus-plus/VPAT-for-Npp) | Informe de accesibilidad (VPAT). | Sin declarar |

Scintilla y Lexilla llevan una licencia permisiva propia, estilo HPND, con el aviso de copyright de
Neil Hodgson en `scintilla/License.txt` (verificado: 2026-10-15). No choca con
`GPL-3.0-or-later`, pero [Q19] las deja fuera: son C++ compilado dentro del binario.

La sintaxis de búsqueda que ve quien usa Notepad++ es la de Boost.Regex, de estilo Perl, con
retrorreferencias y lookaround, según `npp-usermanual` (a verificar).

**Instalación standalone de PhotoCraft.** Página: https://getartcraft.com/apps/photocraft, sección
«Just PhotoCraft? Download it standalone.». El 2026-10-15 esa sección mostraba la versión 0.7.0, y
el release de GitHub ya iba en la 0.9.0 (verificado: 2026-10-15). Para Linux, en la página 0.7.0
solo Flatpak; el release 0.9.0 publica más archivos de Linux (a verificar). Usa la página como
modelo de presentación y el release vigente como lista real de artefactos. La matriz de Linux de
Amate no sale de PhotoCraft, sino del requisito de Debian y derivados ([Q09]).

El modelo, aplicado a Amate:

- La aplicación se instala sola. No depende de un lanzador.
- Windows tiene instalador MSI y, además, un ZIP portable que se descomprime y se ejecuta ([Q09]).
- macOS tiene una imagen de disco universal ([Q09]).
- Linux tiene un paquete nativo de la distribución (`.deb`) y formatos que no exigen instalación
  (AppImage y `.tar.gz`) ([Q09]).
- Los binarios viven como assets de GitHub Releases, el canal único ([Q09]).
- Cada release publica `SHA256SUMS.txt` y la atestación de procedencia de GitHub ([Q28]).
- La versión nueva se obtiene descargando el release nuevo ([Q13]).
- Quien desarrolla puede compilar con Rust vía rustup y `cargo run --release`. La MSRV la fijas
  después de elegir crates, con la versión estable reciente como suelo.
- Construir el paquete y firmarlo son pasos distintos. La firma de Windows y la notarización de
  macOS son la compuerta de la 1.0, no de la primera versión pública ([Q10], [Q27]).

## Reglas que el plan no puede saltarse

Las reglas que citan una decisión siguen a esa decisión: si el dueño cambia la respuesta, la regla
cambia con ella. Las que no citan ninguna no dependen de una respuesta.

- **Reimplementación limpia** ([Q21]). El código nuevo se escribe en Rust, desde cero. El código y
  el manual de Notepad++ se leen para analizar y escribir especificaciones de comportamiento; se
  citan por enlace y capítulo, y se parafrasean. Distingue tres cosas: inspirarse en el
  comportamiento observable (sí); reutilizar dependencias con licencia compatible con GPLv3 (sí, si
  cumplen [Q02] y [Q19]); copiar o traducir código, comentarios, textos del manual, iconos o
  recursos (no, en ningún caso). Ningún archivo de Notepad++ (temas, UDL, traducciones) viaja en
  los paquetes, aunque la GPL lo permitiría.
- **Marca** ([Q01], [Q21]). «Notepad++» solo aparece en texto descriptivo, por uso nominativo
  («usa los atajos de Notepad++»), y con este aviso: «Notepad++ es una marca de su titular. Amate
  no está afiliado a Notepad++ ni a sus autores.» Ningún nombre, binario, crate, paquete,
  repositorio, dominio, logo ni icono usa «Notepad++» o un derivado confundible. Amate no se
  presenta como fork ni como port ([Q18]), ni como «compatible con la configuración de Notepad++»
  ([Q05]).
- **Rust estricto** ([Q19]). Nada de C ni C++ compilado dentro del binario distribuido. Sí se usan
  las API y bibliotecas del sistema operativo (Win32, AppKit y CoreText, Wayland y X11, drivers de
  GPU). El CI falla si el árbol de dependencias del binario incluye `cc`, `cmake` o `bindgen`; las
  herramientas que no viajan en el binario (valgrind, libFuzzer) quedan fuera de la regla.
  Scintilla, Lexilla, Tree-sitter, PCRE2, gtk4-rs, Qt, `mlua` (Lua), y TLS con `ring` o `aws-lc-rs`
  solo aparecen en el plan como alternativas descartadas, con su motivo.
- **Interfaz nativa** ([Q16]). Corre dentro del proceso y está escrita en Rust. Sin Electron y sin
  un webview como interfaz principal; Tauri queda fuera por eso.
- **Sin red** ([Q13]). Amate no hace ninguna conexión de red: ni comprobación de versión ni
  telemetría. La versión nueva se obtiene descargando el release nuevo. WinGup no se porta. Si
  algún día se agrega un aviso de versión, será opt-in en el primer arranque, solo enviará la
  versión y usará el TLS del sistema.
- **Plugins de Notepad++.** Un plugin DLL de Notepad++ no es un objetivo de compatibilidad
  ([Q06]): está escrito contra la API Win32 de Notepad++ y de Scintilla. Esto sigue así aunque
  [Q06] cambie.
- **La accesibilidad filtra** ([Q20]). Un toolkit que falle el IME o el lector de pantalla en las
  mediciones del hito 0 queda descartado, y la medición se repite con el siguiente candidato
  ([Q24]).
- **Una recomendación, con su comparación.** El plan recomienda un toolkit y un motor de texto, con
  la comparación que pide «Arquitectura recomendada», y el hito 0 los confirma con mediciones
  ([Q24]). Si dos opciones quedan empatadas, el cuestionario lo dice con una sola pregunta nueva y
  el hito 0 mide las dos; no repartas el empate en varias preguntas.
- **Cada hito corre.** Cada hito termina en algo que se puede ejecutar o instalar. Un hito que solo
  «prepara la arquitectura» no cuenta.
- **El hito 1 es el de la base.** El primer hito usable es el de la sección «Base del hito 1», más
  lo que [Q04] añade. No adelantes a ese hito la paridad `diario` de [Q03].

## Reglas permanentes del dueño

El dueño del proyecto, que también es su mantenedor, fijó estas reglas para todo el proyecto, no
solo para esta corrida.

- **Dependencias nuevas.** Antes de proponer una, revisa si la biblioteca estándar o algo ya
  elegido lo cubre. Cada crate o herramienta nueva se propone con al menos tres candidatos y una
  comparación de seguridad por candidato: estado de mantenimiento (último release y mantenedores
  activos), avisos de seguridad conocidos, adopción, número de dependencias transitivas y licencia.
  Entra solo cuando el dueño la aprueba. En el plan, toda elección de crate o herramienta es una
  recomendación pendiente de esa aprobación, no un hecho.
- **Páginas web.** Toda página que el proyecto publique en una URL (por ejemplo, una página de
  descargas en GitHub Pages o el manual como sitio) lleva metadatos Open Graph: título,
  descripción, imagen, URL y tipo, con `og:image` y `og:url` como URL absolutas leídas de la
  configuración, nunca fijas en el código. Dependen del dominio de [Q01], que falta verificar.

## Qué tienes que leer y qué tienes que sacar

Abre el código y el manual lo suficiente para citar archivos o capítulos reales. No hace falta un
inventario línea a línea.

**Lo que ya está en este repositorio.** Se verifica y se amplía; no se rehace.

- [01-analisis-notepad-plus-plus.md](01-analisis-notepad-plus-plus.md) ya mide la organización (los
  nueve repositorios, sus licencias y su popularidad), las líneas de código por componente, las
  clases de `ScintillaComponent`, los subsistemas Win32 de `WinControls`, la superficie funcional
  (510 elementos en 13 menús), los datos XML, los mensajes de la API de plugins, los sistemas que
  soporta Notepad++ y cómo se compila y se empaqueta, más las licencias, la marca y PhotoCraft.
  Re-mide lo que el plan use. Su sección 6, «Lo que falta por analizar», es tu lista de pendientes.
- [02-cuestionario.md](02-cuestionario.md) tiene el razonamiento y las opciones de cada pregunta,
  con su estado. Las que siguen abiertas aplican su recomendación como supuesto.
- [03-plan-de-trabajo.md](03-plan-de-trabajo.md) es la hoja de ruta por hitos, con su tabla de
  riesgos. Tu `## Hitos` parte de ahí y la baja a criterios comprobables; tu `## Riesgos` actualiza
  esa tabla.
- [04-distribucion-y-empaquetado.md](04-distribucion-y-empaquetado.md) tiene la matriz de
  artefactos, el modo portable, la firma, la integración con el escritorio, las herramientas y el
  pipeline de release. Tu `## Empaquetado` parte de ahí.
- [fusion.md](fusion.md) tiene la tabla de hechos por re-verificar.

**Del repositorio principal de Notepad++ saca**, además de lo que ya mide 01:

- Mapa de módulos: ventana y pestañas, búfer, búsqueda, codificaciones, lenguajes y estilos,
  configuración, sesión y macros, parámetros de línea de comandos, plugins y actualización. El
  análisis solo cubre `ScintillaComponent`, `WinControls` y `PluginsManager`.
- Qué partes son Win32 y por eso hay que rehacerlas, y qué comportamiento es independiente del
  sistema. Para cada subsistema Win32 de la sección 2.2 de 01 (modo oscuro, DPI, *docking* de
  paneles, bandeja del sistema, vigilancia de archivos, asociación de extensiones), la estrategia
  multiplataforma que lo reemplaza en Amate.
- La lista real de lenguajes con lexer, a partir de `langs.model.xml`, no de memoria. El análisis
  solo trae el conteo.
- Los parámetros de línea de comandos documentados.
- Cómo decide la codificación al abrir y qué ocurre al guardar (UTF-8, BOM, UTF-16, páginas ANSI).
- Qué hace con un archivo que cambia en disco, con uno de solo lectura y con uno muy grande.
- Cómo guarda la sesión, la configuración y las macros. Sirve para diseñar las de Amate, no para
  leer sus archivos ([Q05]).
- Su propia automatización: `appveyor.yml`, pruebas, compilación y publicación.
- Licencia: qué agrega el texto propio de `LICENSE`, y la licencia de `scintilla/` y `lexilla/`.

**De los otros repositorios saca solo esto:**

- `npp-usermanual`: el inventario de comportamiento por áreas (documentos, edición, búsqueda,
  vista, codificación, lenguajes, autocompletado, macros, ejecución de comandos, impresión,
  preferencias, atajos, línea de comandos, plugins, copias de seguridad). Comprueba esas áreas
  contra los capítulos reales del manual antes de usarlas como filas del inventario. De aquí salen
  también la tabla de atajos del hito 1, el dialecto de búsqueda (capítulo de búsqueda) y la
  convención de columnas.
- `nppPluginList`: qué capacidades viven fuera del núcleo. Agrupa los plugins en tres cubos para el
  plan: «hace falta dentro del editor porque no habrá DLL», «puede esperar» y «fuera de producto».
  Basa el agrupamiento en el manual y en la presencia en el catálogo, y di cuándo un cubo es un
  juicio de producto. Cuenta los plugins del catálogo: las estrellas del repositorio no son
  plugins.
- `userDefinedLanguages` y `nppThemes`: formato de los XML y qué costaría leerlos, sin implementar
  el lector, y qué formatos convendría leer y cuáles no. Solo sirve para el costo de importación
  que el hito 3 deja escrito como trabajo no incluido ([Q05]).
- `nppShell`: qué integración de escritorio existe en Windows, para traducirla a «Abrir con», a un
  `.desktop` y a los tipos de documento de macOS ([Q14]). No portes esa DLL.
- `wingup`: confirma que es un actualizador de Windows y cítalo al justificar [Q13].
- `VPAT-for-Npp`: qué cubre el informe, como lista de comprobación del teclado completo del hito 1
  y de la verificación con lector de pantalla antes de la primera versión pública ([Q20]).
- `old-releases`: no aporta al diseño. Una línea basta.

**PhotoCraft** se usa solo para distribución. Vuelve a abrir la página y el release vigente, y
verifica la sección 1 de [04-distribucion-y-empaquetado.md](04-distribucion-y-empaquetado.md). No
copies su alcance (lienzo GPU, PSD, web, FreeBSD, RISC-V) ni su lanzador.

**Zed, Lapce, Helix y Xi.** Lee lo necesario de su código, de su documentación o de lo que
escribieron sus autores para la sección de lecciones de la arquitectura. Cita cada fuente.

## Decisiones

Todas las decisiones de producto están en [DECISIONES.md](DECISIONES.md), que es la fuente de
verdad. Esta tabla es solo un índice: el detalle, lo que implica cada una y qué parte del plan se
mueve si cambia están allá. Cuando dudes, cita y enlaza en vez de parafrasear.

| ID | Tema | Respuesta, en corto |
| --- | --- | --- |
| Q01 | Nombre | `amate`; «Amate» en la prosa. Antes de publicar cualquier artefacto, crear el repo público o reservar el crate, falta verificar la marca y el dominio. |
| Q02 | Licencia | `GPL-3.0-or-later` para todo el repositorio. Dependencias con licencia compatible con GPLv3, controladas con `cargo-deny`; el CI genera los avisos de terceros. |
| Q03 | Paridad de la primera versión pública | `diario`, que se entrega en el hito 2. Niveles: `familiar`, `diario` y `completo`. |
| Q04 | Añadidos al hito 1 | Coincidencia de paréntesis, espacios en blanco visibles, tema del sistema, DPI alto, rutas resueltas al ejecutar con modo portable, y ANSI (Windows-1252 e ISO-8859-1). |
| Q05 | Configuración de Notepad++ | `ninguna`: configuración propia (TOML recomendado) y sin importadores; el hito 3 deja escrito el costo de importar. |
| Q06 | Plugins propios | `despues`: ningún sistema de plugins antes de la 1.0; capa de comandos con nombre y serializable desde el hito 1. |
| Q07 | Idioma de la interfaz | Español e inglés en la primera versión pública; textos fuera del código desde el hito 1. |
| Q08 | Idioma del repositorio y del manual | README bilingüe; manual en español e inglés en el hito 3; planeación en español; commits, código y comentarios en inglés. |
| Q09 | Paquetes y plataformas | La tabla de «Producto». Hito 4b, opcional: Flatpak, `.deb` arm64, Windows ARM64, repositorio apt propio, winget y cask de Homebrew. |
| Q10 | Firma y notarización | Certificados de pago después; solicitud a SignPath Foundation en el hito 4. Construir y firmar son pasos distintos. |
| Q11 | Tamaño de archivo | `normal`: un archivo de 10 MB con fluidez; arriba, modo degradado sin que la interfaz se congele; el documento vive en un rope. |
| Q12 | Parecido visual | Sin respuesta: aplica `familiar` como supuesto. |
| Q13 | Actualizaciones | `release`: ninguna conexión de red. |
| Q14 | Integración con el escritorio | `con-el-paquete`; «Abrir con» en Windows y macOS en el hito 4; nunca se apropia de las asociaciones por defecto. |
| Q15 | Tipo de proyecto | `publico`, por etapas: CI y `SECURITY.md` en el hito 0; `CONTRIBUTING.md`, código de conducta y plantillas de issues en la primera versión pública. |
| Q16 | Tecnología de la interfaz | `nativa`: en proceso y en Rust, sin Electron ni webview. |
| Q17 | Usuario principal y propuesta de valor | Quien usa Notepad++ en Windows y quiere lo mismo en Linux y macOS. |
| Q18 | Relación con Notepad++ | Sucesor espiritual, ni fork ni port. |
| Q19 | Código nativo en el binario | Rust estricto: nada de C ni C++ compilado dentro del binario; resaltado con `syntect` y `fancy-regex`; búsqueda con `regex` y `fancy-regex`. |
| Q20 | Accesibilidad | Filtra el toolkit: teclado completo desde el hito 1; lector de pantalla verificado antes de la primera versión pública. |
| Q21 | Material de Notepad++ | Reimplementación limpia; nada de Notepad++ dentro del proyecto. |
| Q22 | Contribuciones | DCO (`Signed-off-by`), sin CLA. |
| Q23 | Atajos en macOS | Cmd donde Notepad++ usa Ctrl, respetando los atajos que reserva el sistema. |
| Q24 | Validación en el hito 0 | Sí, con mediciones, en ramas `spike/`; lo que queda es un ADR en `docs/adr/`. |
| Q25 | Presupuestos de rendimiento | Requisitos medidos en CI: arranque en frío < 300 ms, latencia de tecla < 16 ms al p99, RAM en reposo con un archivo de 1 MB < 100 MB; el umbral para abrir 10 MB lo fija el hito 0. |
| Q26 | Lenguajes con resaltado | 13 en el hito 1 y 32 en la primera versión pública. |
| Q27 | Versionado | SemVer, con 0.x hasta firmar: el hito 1 es la 0.1.0 y la primera versión pública, una 0.N. |
| Q28 | Procedencia de los releases | Atestación de GitHub Actions y `SHA256SUMS.txt`. |
| Q29 | Titular | Carlos Ceballos, como persona. |
| Q30 | Finales de línea del repositorio | LF en `.gitattributes`, con CRLF para `*.bat` y `*.cmd`; UTF-8 sin BOM. |
| Q31 | Criterio de éxito | Adopción externa. |

Reglas de lectura:

- **[Q04]** solo suma funciones a la base del hito 1; no la sustituye. Una función entra al hito 1
  solo si el dueño la agrega a [Q04], y el criterio es que sin ella no usaría Amate. Si alguien pide
  FTP, comparar archivos, mapa del documento (minimapa) o vista de carpeta, colócalo en el hito que
  corresponda y dilo.
- **[Q12]** no tiene respuesta. Aplica la recomendación `familiar` (menús, pestañas arriba, barra de
  estado y atajos de Notepad++ en las acciones que existan; iconos y marca propios) y marca cada
  consecuencia como [Q12] (supuesto).
- **[Q10]**: si algún día cambia a firmar con certificados de pago, basta con que el dueño confirme
  que la cuenta y el certificado existen (titular, tipo y vencimiento). Nunca pidas ni aceptes
  archivos de certificado, llaves privadas ni contraseñas en el mensaje ni en el repositorio: la
  firma usa secretos que no viven en el repo ni en el CI público.

## Base del hito 1

Esta es la primera versión que una persona puede usar para editar código. [Q04] suma; nada de
`diario` entra aquí solo por estar en Notepad++.

- Crear, abrir, guardar, guardar como y cerrar pestaña. Indicador de modificado.
- Varias pestañas en una ventana.
- Deshacer y rehacer.
- Cortar, copiar, pegar y seleccionar todo.
- Buscar y reemplazar en el documento actual, en literal y con expresión regular. El dialecto es el
  de `regex` y `fancy-regex` ([Q19]), no el de Boost.Regex que usa Notepad++. El plan escribe tres
  ejemplos de patrones cuya diferencia notaría un usuario de Notepad++.
- Números de línea, resalte de la línea actual y ajuste de línea opcional.
- Barra de estado: línea, columna, codificación, fin de línea y lenguaje.
- Resaltado con `syntect` ([Q19]) de los 13 lenguajes de [Q26]: texto plano, Rust, C, C++, Python,
  JavaScript, TypeScript, JSON, HTML, CSS, Markdown, Shell y SQL.
- Al abrir: UTF-8 con BOM, UTF-8 sin BOM, UTF-16 LE y UTF-16 BE. Al guardar se conserva la
  codificación detectada.
- Conversión explícita entre LF, CRLF y CR.
- Tema claro y tema oscuro.
- Atajos de Notepad++ para las acciones de esta lista que tengan atajo en el manual. El plan incluye
  una tabla atajo → acción sacada del manual, no de memoria, con una columna para macOS: Cmd donde
  Notepad++ usa Ctrl, respetando los atajos que reserva el sistema, como Cmd+H y Cmd+Q ([Q23]).
- Línea de comandos: `amate archivo` y `amate archivo:línea`.
- Teclado completo ([Q20]): abrir, guardar, cerrar, cambiar de pestaña, buscar, moverse por el
  texto y llegar a cada función de esta lista sin ratón.
- Textos de la interfaz fuera del código desde el primer texto ([Q07]; recomendado: Fluent con
  `fluent-rs`), con una prueba de que caben los textos en español, que son más largos.
- El documento vive en un rope, nunca en un solo `String` ([Q11]).
- Cada acción del usuario es un comando con nombre y serializable ([Q06]).
- Paquetes de [Q09] generados en CI, sin firma ([Q10]).

Añadidos de [Q04], que también son parte del hito 1:

- coincidencia de paréntesis;
- espacios en blanco visibles;
- seguir el tema claro u oscuro del sistema;
- DPI alto;
- rutas de configuración resueltas al ejecutar, con modo portable (archivo marcador junto al
  ejecutable);
- abrir y guardar en ANSI (Windows-1252 e ISO-8859-1).

Buscar en archivos, plegado y texto bidireccional siguen en el hito 2 ([Q04]). Lo que el hito 1
exige por otras decisiones está en su definición, en «Hitos».

El hito 0, anterior, deja el workspace de Cargo compilando en los tres sistemas en CI, con una
ventana vacía empaquetada de forma basta y el toolkit medido. El hito 1 se apoya en eso.

## Qué debe contener el plan

### 1. Análisis

- Qué es Notepad++ en una página: plataforma, licencia, stack, y por qué no se puede extender ese
  repositorio a Linux y macOS recompilándolo. Parte de 01 y enlázalo; no lo copies.
- Inventario de comportamiento por áreas del manual (las de «Qué tienes que leer», comprobadas
  contra sus capítulos), con el nivel de [Q03] y el hito propuesto para cada área: hito 1, hito 2,
  hito 3, hito 4, hito 4b, después de la 1.0, o nunca (o «se reemplaza», diciendo por qué). Una
  fila por área, con las familias de comandos como subfilas, no una fila por cada ítem del menú.
- Tres o cuatro flujos que un usuario habitual de Notepad++ echaría de menos si faltaran, y en qué
  hito aparecen.
- El mapa de módulos y la estrategia multiplataforma de cada subsistema Win32.
- Riesgos de verdad: el dialecto de regex, las codificaciones ANSI que Notepad++ sí abre, la
  diferencia entre Scintilla y un búfer en Rust, la cobertura de `syntect` frente a los lexers de
  Notepad++, el CI del binario universal de macOS y el sandbox de Flatpak cuando llegue en el hito
  4b. Su detalle va a `## Riesgos`.

### 2. Arquitectura recomendada

Compara opciones vivas el día de la corrida. Descarta las abandonadas y dilo. Para cada decisión
importante incluye:

- las alternativas evaluadas, con sus ventajas y desventajas;
- madurez y mantenimiento del ecosistema;
- compatibilidad con cada sistema operativo;
- impacto en rendimiento, en tamaño del binario y en accesibilidad;
- riesgos técnicos y legales;
- qué cuesta equivocarse;
- la recomendación final, justificada.

Cuando recomiendes una tecnología, cita la alternativa descartada y el motivo. Cada crate o
herramienta que propongas sigue la regla de los tres candidatos y la comparación de seguridad
(«Reglas permanentes del dueño»).

Como mínimo:

- **Interfaz.** Compara los toolkits que pueden cumplir a la vez [Q16], [Q19] y [Q20]: egui, Iced,
  Slint con sus renderizadores en Rust, Xilem/Masonry, floem y GPUI. De cada uno, verifica el día
  de la corrida la accesibilidad en los tres sistemas (AccessKit u otra vía), el soporte de IME y
  que no compile C ni C++ dentro del binario. De Slint revisa además sus niveles de licencia
  vigentes (a verificar). Las mediciones del hito 0 son las de [Q24]; súmales el renderizado de
  texto (fuentes de respaldo, ligaduras, texto bidireccional, ancho de caracteres CJK y emojis que
  no rompan la rejilla), que es donde más decepcionan los toolkits de Rust.
- **Texto.** Rope frente a *piece table*. Parte de `ropey` y compáralo al menos con `crop` y con
  una implementación propia; sustitúyelo solo si los bancos de prueba demuestran que es el cuello
  de botella. Requisitos: inserción y borrado en O(log n), índice por líneas, conversión entre
  desplazamiento de bytes, de caracteres y de columna visual, clústeres de grafemas, nunca el
  archivo entero en un `String` ([Q11]), y varias vistas por documento desde el hito 1, con búfer y
  deshacer compartidos y cursor, desplazamiento y selección propios de cada vista. Di desde qué
  hito el modelo de selección admite varias selecciones y modo columna, que `diario` exige en el
  hito 2: agregarlos tarde obliga a reescribir el núcleo.
- **Resaltado.** `syntect` con su backend `fancy-regex`, sin Oniguruma, está decidido ([Q19]). El
  plan verifica que ese backend compila sin C y carga las gramáticas que hacen falta (a verificar);
  dice cuáles de los 32 lenguajes de [Q26] faltan en el juego de sintaxis que trae `syntect`
  (a verificar), de dónde salen las definiciones `.sublime-syntax` con licencia compatible con
  GPL-3.0 y cómo se conservan sus avisos; cómo se limita el repintado al área visible; y cómo se
  cubrirían después los lenguajes definidos por el usuario, que son del nivel `completo` ([Q03]).
- **Expresiones regulares de la búsqueda del usuario.** `regex` y `fancy-regex` ([Q19]): cuándo se
  usa cada uno, con qué límite de retroceso y cómo se cancela una búsqueda.
- **Codificación de archivos.** `encoding_rs` u otra, justificada, y la detección (`chardetng` u
  otra), también justificada, incluidas Windows-1252 e ISO-8859-1 al abrir y al guardar ([Q04]).
- **Configuración.** Formato (TOML recomendado, [Q05]) y ruta por sistema, resuelta en tiempo de
  ejecución, con el modo portable por archivo marcador ([Q04]): qué va en el directorio de usuario y
  qué junto al ejecutable.
- **Textos de la interfaz.** Fluent con `fluent-rs` es la recomendación de [Q07]; compárala como
  cualquier otro crate.

Incluye también:

- **Lecciones de Zed, Lapce, Helix y Xi** sobre estructuras de texto, renderizado y rendimiento:
  qué aplica a Amate y qué no, con su fuente.
- **Modelo de datos y flujo de eventos**: documentos, vistas, búferes y comandos; cómo viaja una
  tecla hasta la pantalla; qué hilos hay (interfaz, resaltado, búsqueda, vigilancia de archivos) y
  cómo se cancelan los trabajos.
- **Modelo de proceso**: una ventana, varias ventanas, y qué pasa al abrir un archivo desde la
  terminal (o, en macOS, desde Finder) cuando Amate ya está abierto.
- **Lista de descartes**, cada uno con su motivo: Scintilla y Lexilla (C++, [Q19]); Tree-sitter
  (gramáticas en C, [Q19]); PCRE2 y Boost.Regex (C y C++, [Q19]); gtk4-rs (enlaza GTK, que es C,
  [Q19]); Qt (C++, [Q19]); Electron, Tauri y cualquier webview como interfaz principal ([Q16]);
  `mlua` y Lua (compila C, [Q06], [Q19]); `ring` y `aws-lc-rs` (TLS con proveedores en C, [Q19]).

Cierra con una caja de crates concreta: qué crate, para qué, qué queda en código propio y su
comparación de seguridad. Dibuja los límites de crates del workspace en una lista corta (`app`,
`buffer`, `highlight`, `config`, `xtask`, o los nombres que recomiendes), y di si el acceso al
sistema de archivos vive en un crate aparte para que el núcleo se pruebe en memoria.

### 3. Hitos

Usa estos hitos y este orden. Puedes partir un hito en dos si cada parte sigue siendo demostrable.
No añadas un hito de investigación: la validación medida vive dentro del hito 0 ([Q24]). La
distribución no espera al hito 4: arranca en el hito 0 con paquetes bastos y sigue en el hito 1 con
los paquetes de [Q09] generados en CI.

**Hito 0 — Cimientos y validación medida.** Workspace de Cargo que compila en Windows, Linux y
macOS en CI: `fmt`, `clippy -D warnings`, pruebas, `cargo-deny` y la comprobación de [Q19] (el CI
falla si el árbol de dependencias del binario incluye `cc`, `cmake` o `bindgen`). Una ventana vacía
empaquetada de forma basta en cada sistema, todavía sin ser el MSI, el DMG ni el `.deb` finales.
Spikes del toolkit en ramas `spike/`, que se borran al terminar, para medir IME, lector de
pantalla, consumo en reposo, latencia de tecla, costo de empaquetado y el build universal de macOS
([Q20], [Q24]); lo que queda es un ADR en `docs/adr/` con la decisión GO/PIVOT. Si el toolkit falla
el IME o la accesibilidad, se repite con el siguiente candidato: es un resultado válido del hito,
no un fracaso del proyecto. Una decisión técnica que no se resuelve con argumentos en una sesión
pasa a los spikes, que la resuelven con mediciones. El CI y `SECURITY.md`, con el reporte privado
de vulnerabilidades de GitHub, llegan aquí ([Q15]). El hito fija con mediciones el umbral para
abrir un archivo de 10 MB ([Q25]). Salida: CI verde en los tres sistemas, ADR con números y un
artefacto basto descargable desde el CI.

**Hito 1 — Primera versión usable (0.1.0, sin anuncio, [Q27]).** La «Base del hito 1» más los
añadidos de [Q04]. Buscar en archivos, plegado y texto bidireccional siguen en el hito 2. También:
teclado completo ([Q20]), textos fuera del código ([Q07]), los 13 lenguajes de [Q26], el documento
en un rope ([Q11]), la capa de comandos serializable ([Q06]), un modelo de documento que ya admite
varias vistas del mismo búfer (la vista dividida llega en el hito 2), el mapeo de atajos de macOS
([Q23]) y los paquetes de [Q09] generados en CI, sin firma ([Q10]). Los presupuestos de [Q25] se
miden en CI desde aquí. Dentro del hito, el núcleo va primero y la aplicación se construye encima.
Entre los criterios: el corpus hostil (I5 en [02-cuestionario.md](02-cuestionario.md)) pasa entero
y el búfer tiene pruebas de propiedades. La salida incluye el uso diario: el mantenedor usa Amate
como editor principal para las tareas básicas durante dos semanas.

**Hito 2 — Flujo diario.** Todo lo de `diario` ([Q03]) que no entró en el hito 1: sesión con
recuperación de pestañas sin guardar, buscar en archivos, edición en columna y multicursor,
operaciones de línea (duplicar, mover, borrar, comentar, mayúsculas y minúsculas, sangría),
plegado, vista dividida con dos vistas del mismo búfer, carpeta de trabajo, los 32 lenguajes de
[Q26], temas y atajos configurables, y el renderizado de texto bidireccional ([Q04]). Sin plugins.
Salida: un usuario habitual de Notepad++ trabaja un día completo sin volver atrás, comprobado con
usuarios reales.

**Hito 3 — Manual y preferencias.** Preferencias y temas editables, el manual de usuario en
español y en inglés ([Q08]), la interfaz completa en los dos idiomas ([Q07]) y, por escrito, el
costo estimado de importar temas, UDL, atajos y sesión de Notepad++, como trabajo no incluido
([Q05]). El manual se escribe para quien llega de Notepad++ («vengo de Notepad++, ¿dónde está
X?») y no copia textos del suyo ([Q21]).

**Hito 4 — Distribución y primera versión pública (0.N).** Los artefactos finales de [Q09],
`SHA256SUMS.txt` y la atestación de procedencia ([Q28]), la integración con el escritorio ([Q14]),
la sección de descargas del README con los pasos para abrir builds sin firma, la solicitud a
SignPath Foundation ([Q10]) y `CONTRIBUTING.md` con DCO, código de conducta y plantillas de issues
([Q15], [Q22]). Salida: un `git tag` produce todo sin pasos manuales, y los paquetes se instalan y
se verifican en máquinas limpias. Si SignPath exige aprobar a mano cada firma (a verificar), el
plan resuelve ese choque con la salida antes de cerrar el hito.

**Hito 4b — opcional, después del hito 4.** Flatpak, `.deb` arm64, Windows ARM64, repositorio apt
propio, winget, cask de Homebrew, firma de pago y notarización de macOS ([Q09], [Q10]). Va aparte
del cierre del hito 4. Si las descargas lo piden, winget, Homebrew y apt son los primeros
candidatos a adelantarse ([Q31]).

**La 1.0.** No es un hito sino una compuerta: `diario` completo, firma en Windows, notarización en
macOS y el manual ([Q27]).

**Hito 5 — Extensiones.** Con [Q06] en `despues`, es solo un anexo de una página del plan: cómo se
añadiría después una interfaz de extensiones (candidatos: Rhai, o WebAssembly con `wasmi`, ambos
en Rust puro), sin ABI nativa ni diseño detallado. Nunca es trabajo programado.

Para cada hito, salvo el anexo del hito 5, escribe:

- Objetivo en una frase y tamaño relativo, sin calendario.
- Alcance y lo que explícitamente queda fuera.
- Criterios de aceptación que se puedan comprobar: un comando, un archivo de prueba, un paquete
  instalable. Los criterios humanos (las dos semanas de uso del hito 1, el día con usuarios reales
  del hito 2) son listas de comprobación que se suman a los objetivos; nunca son la única salida.
- Definición de terminado del hito, a partir de la que recomienda I1 de
  [02-cuestionario.md](02-cuestionario.md) para cada función.
- Módulos o crates que toca.
- Dependencias de hitos anteriores.
- Riesgos de ese hito, y qué se hace si el toolkit elegido deja de cubrir una plataforma después
  del hito 0.

Incluye lo que no es código: licencia y avisos de terceros, gobernanza por etapas ([Q15]),
documentación, internacionalización y comunidad.

### 4. Primera tanda de implementación

Baja el hito 0 y el hito 1 a una lista ordenada de tareas. Cada tarea cabe en una sesión de
implementación y es lo bastante pequeña para convertirse en un issue: un resultado visible, los
archivos o crates que crea, y cómo se prueba. La primera tarea produce una ventana en los tres
sistemas. Las tareas del hito 0 incluyen los spikes en ramas `spike/` y el ADR con los números. La
última tarea del hito 1 cumple los criterios de aceptación de ese hito.

### 5. Pruebas y CI

- Qué se prueba con tests de Rust: búfer (con pruebas de propiedades), codificaciones, finales de
  línea, regex, abrir y guardar.
- El corpus hostil que recomienda I5 de [02-cuestionario.md](02-cuestionario.md): archivos con BOM,
  UTF-16 LE y BE, CP1252, Shift-JIS, finales de línea mezclados, líneas de 10 MB sin salto,
  archivos sin salto final, binarios, enlaces simbólicos, archivos de solo lectura, rutas con
  emojis y rutas de más de 260 caracteres en Windows. Cada fallo reportado se vuelve un archivo
  nuevo del corpus.
- Fuzzing de los analizadores y del motor de búsqueda: qué objetivos y dónde corren.
- Qué se automatiza en la interfaz y con qué herramienta.
- Qué se prueba a mano en cada sistema: el IME en los tres, y el lector de pantalla con NVDA,
  VoiceOver y Orca antes de la primera versión pública ([Q20]).
- Compuertas del CI desde el hito 0: `fmt`, `clippy -D warnings`, pruebas, `cargo-deny`
  (licencias compatibles con GPLv3 y avisos de seguridad, [Q02]) y la comprobación de [Q19].
- Presupuestos de [Q25] desde el hito 1: en los runners compartidos se bloquea con conteo de
  instrucciones (Linux); los tiempos de reloj se reportan sin bloquear.
- Matriz de GitHub Actions. Consulta los runners hospedados vigentes ese día. Cubre Windows x64,
  Linux x64 y macOS arm64. Explica cómo se produce la mitad x86_64 del binario universal de macOS
  y cómo se prueba.
- Los paquetes de Linux se prueban en contenedores limpios, nunca en el runner de CI ([Q09]).
- La firma usa secretos que no viven en el repo ni en el CI público ([Q10]). El CI de cada pull
  request nunca los ve; solo el flujo de release, disparado por una etiqueta, firma cuando hay
  firma disponible.

### 6. Empaquetado

Parte de [04-distribucion-y-empaquetado.md](04-distribucion-y-empaquetado.md). Para cada artefacto
de [Q09]: herramienta concreta (`dist`, `cargo-wix`, `cargo-deb`, `appimagetool`, `create-dmg`,
`cargo-packager`, `cargo-bundle` u otra que verifiques el día de la corrida), nombre del archivo,
arquitecturas y sistema mínimo. Incluye el ZIP portable de Windows como «descomprimir y ejecutar».
Incluye `SHA256SUMS.txt`, en el formato que acepta `sha256sum -c`, y la atestación de procedencia
de `actions/attest-build-provenance`, que se verifica con `gh attestation verify` ([Q28]).

Sistema mínimo prometido: no bajes de Windows 10 (22H2), Debian 12, Ubuntu 24.04 y la versión de
macOS que el toolkit elegido soporte de verdad ([Q09]). Si el toolkit exige algo más nuevo, el plan
sube el mínimo y lo justifica. Windows 10 ya no recibe parches de consumo (a verificar): se sigue
prometiendo, pero las pruebas principales se hacen en Windows 11 ([Q09]).

Incluye también:

- la integración con el escritorio de [Q14], paquete por paquete;
- la sección de descargas del README: los pasos para abrir builds sin firma en macOS (Gatekeeper)
  y en Windows (SmartScreen), y el aviso de que, con Smart App Control activo, Windows 11 no deja
  abrir un binario sin firma (a verificar) ([Q10]);
- la solicitud a SignPath Foundation, cuyos requisitos hay que revisar (a verificar) ([Q10]);
- la vía para compilar desde el código, con la MSRV declarada;
- lo que agrega el hito 4b, separado del cierre del hito 4.

### 7. Riesgos

Un registro de riesgos con probabilidad, impacto y mitigación, ordenado por probabilidad. Parte de
la tabla de [03-plan-de-trabajo.md](03-plan-de-trabajo.md) y actualízala con lo que encuentres.
Incluye los riesgos de seguridad de la propia aplicación: abrir archivos que no son de confianza,
expresiones regulares del usuario sobre archivos grandes y el ayudante para guardar con permisos
elevados, si lo hay. Aquí va también, una sola vez, cada riesgo que veas en una decisión cerrada.

## Huecos que el plan debe cerrar

Ninguna decisión los cierra. Salen de la auditoría de la fusión de este repositorio, cada uno con
la sugerencia de esa auditoría ajustada a las decisiones. Ciérralos en la sección del plan que
corresponda. Una sugerencia aplicada sin confirmación del dueño es un supuesto y va a
`## Supuestos aplicados`; si además mueve el alcance, la licencia, el empaquetado, la interfaz o el
primer hito, pasa al cuestionario como pregunta nueva. Una sugerencia que nombra un crate o una
herramienta propone un candidato, no una elección: entra con la regla de los tres candidatos.

### Núcleo y edición

- **Modelo de proceso.** Una instancia o varias, comunicación entre procesos, varias ventanas, y el
  evento de abrir documento de macOS, porque Finder no pasa los archivos como argumentos —
  sugerencia: una instancia por sesión de usuario que recibe los archivos nuevos como pestañas en
  la última ventana activa, por una tubería con nombre en Windows y un socket Unix en el resto;
  opción `--new-instance`; varias ventanas después de la 1.0; comprobar que el toolkit recibe el
  evento de macOS.
- **Argumentos de línea de comandos.** `amate archivo:línea` es ambiguo en Windows (letras de
  unidad, flujos alternos de NTFS); falta decidir la compatibilidad con opciones de Notepad++ como
  `-n`, y el subsistema gráfico frente a la salida de consola — sugerencia: quitar un
  `:línea[:columna]` numérico final solo si el archivo existe sin él; aceptar `-n<línea>` y un
  subconjunto pequeño de las opciones documentadas de Notepad++; subsistema gráfico con
  `AttachConsole` para `--help`.
- **Qué es «guardar» correcto.** El renombrado atómico reemplaza enlaces simbólicos, rompe enlaces
  duros y pierde las ACL y los flujos alternos de Windows; faltan también las unidades de red y los
  archivos de solo lectura — sugerencia: desde el hito 1, escritura atómica que conserva los
  permisos y rechaza con aviso un archivo de solo lectura; resolver los enlaces simbólicos antes de
  escribir; `ReplaceFileW` en Windows; escritura en el lugar con copia de respaldo para enlaces
  duros y unidades de red.
- **Deshacer.** Lineal o en árbol, tope de memoria, reglas de agrupación y qué pasa al recargar
  desde disco — sugerencia: lineal, con agrupación y puntos de guardado; el árbol puede llegar
  después; recargar desde disco es un paso que se deshace.
- **Finales de línea.** Valor por defecto de los archivos nuevos en cada sistema, archivos con
  finales mezclados, salto de línea final y cómo los muestra la barra de estado — sugerencia:
  conservar lo que trae el archivo salvo conversión explícita; CRLF por defecto en Windows y LF en
  el resto, configurable; la barra de estado dice «Mixto»; nunca agregar ni quitar el salto final
  en silencio.
- **Codificaciones más allá de [Q04].** Salida en UTF-16, que `encoding_rs` no codifica
  (a verificar), páginas OEM (437, 850…), UTF-16 sin BOM, errores de decodificación y el resto del
  juego de `encoding_rs` — sugerencia: un codificador UTF-16 propio y pequeño; páginas OEM solo
  tras aprobar la dependencia; un archivo que no se decodifica sin pérdida abre en solo lectura con
  aviso; ISO-8859-15 junto a las de [Q04], y el resto del juego antes de la 1.0.
- **Codificación de respaldo en Linux y macOS**, que no tienen página «ANSI» — sugerencia: UTF-8 si
  los bytes son UTF-8 válido; si no, la conjetura del detector con un respaldo configurable
  (Windows-1252, o el que se derive del locale); siempre «reabrir con otra codificación».
- **Cambios externos.** Cómo se detecta que un archivo abierto cambió, se borró o se renombró —
  sugerencia: en el hito 1, comparar fecha de modificación y tamaño al recibir el foco y antes de
  guardar; un vigilante del sistema después, como dependencia nueva que requiere aprobación; un
  archivo borrado sigue abierto y marcado como faltante.
- **Hilos y flujo de eventos.** Hilo de la interfaz, hilos de trabajo (índice de líneas,
  resaltado, búsqueda, vigilancia de archivos), instantáneas y cancelación — sugerencia: el hilo de
  la interfaz es dueño de los documentos; instantáneas inmutables del rope viajan a los hilos de
  trabajo por canales; contadores de generación para cancelar; un solo proceso (la lección de Xi).
- **Regex sobre un rope.** `fancy-regex` necesita un `&str` contiguo; faltan además las
  coincidencias de varias líneas, la búsqueda hacia atrás, la sintaxis de reemplazo y el modo
  «Extendido» de Notepad++ (`\n`, `\t`) — sugerencia: `regex-automata` sobre los trozos del rope
  (por ejemplo `regex-cursor` (a verificar)) para patrones simples; `fancy-regex` con límite de
  retroceso sobre una ventana materializada; una tabla de diferencias de dialecto y de reemplazo.
- **Buscar y reemplazar en archivos** (hito 2): filtros, archivos ocultos, `.gitignore`, binarios y
  tope de tamaño — sugerencia: filtros de inclusión y exclusión al estilo de Notepad++;
  `.gitignore` y ocultos sin filtrar por defecto, configurable; saltar los binarios; tope de tamaño
  por archivo; reemplazar en archivos con vista previa y respaldo o deshacer por archivo.
- **Líneas muy largas**, como un archivo de 10 MB sin saltos: ajuste de línea y virtualización
  horizontal — sugerencia: maquetado por trozos y un modo seguro para líneas largas.
- **Modo degradado de [Q11].** Qué más se apaga arriba de los umbrales, además del resaltado y el
  ajuste de línea, y cómo se carga el archivo — sugerencia: umbrales configurables; arriba de
  ellos, plegado apagado, memoria de deshacer con tope y un aviso en la barra de estado; si ese
  modo usa `mmap`, que sea de solo lectura y recargue cuando otro proceso trunque el archivo.
- **Recuperación y sesión.** `diario` trae la recuperación de pestañas sin guardar ([Q03]); falta
  el intervalo, la ubicación y el formato de las instantáneas, y la versión del esquema de
  configuración y sesión — sugerencia: guardar al salir la sesión y los búferes sin guardar
  (desactivable), con instantáneas periódicas en el directorio de estado; un campo
  `schema_version` en cada archivo persistido.
- **Rutas y marcador portable.** Directorios de estado y de caché aparte de la configuración,
  nombre y formato del archivo marcador, y qué hace la variante portable en un medio de solo
  lectura — sugerencia: marcador propio en TOML, nunca `doLocalConf.xml` ([Q21]); estado y caché
  separados de la configuración; en solo lectura no se vuelve al perfil del usuario, porque la
  variante portable no escribe fuera de su carpeta (sección 8 de
  [04-distribucion-y-empaquetado.md](04-distribucion-y-empaquetado.md)).
- **Formato de los comandos.** Cómo se serializa la capa de comandos de [Q06] — sugerencia: un
  formato propio en TOML o JSON.
- **Pánicos y diagnóstico.** Qué pasa con el texto sin guardar ante un pánico y dónde quedan los
  logs — sugerencia: un panic hook que escribe instantáneas de recuperación, y logs locales
  rotativos en el directorio de estado; nada sale de la máquina ([Q13]).
- **Columna que ve el usuario** en la barra de estado, en «ir a» y en la selección en columna
  (tabuladores, ancho CJK, emojis) — sugerencia: columna desde 1 con los tabuladores expandidos
  según la configuración; contar caracteres o clústeres de grafemas se decide en el hito 0, tras
  revisar la convención de Notepad++ en `npp-usermanual`; desplazamientos en bytes por dentro.

### Interfaz, rendimiento y calidad

- **Shaping y vista del editor.** Crate de shaping, un widget de edición propio sea cual sea el
  toolkit, y rangos de texto accesibles para esa vista — sugerencia: decidirlo en el hito 0,
  reusando la pila de shaping del toolkit.
- **Lista de comprobación del toolkit** que ninguna fuente cubre — sugerencia: sumar a la compuerta
  del hito 0 el menú global de macOS, los diálogos de archivo nativos o por portal, varias ventanas
  y una lista de requisitos de compilación por sistema.
- **Aspecto nativo como criterio del toolkit** — sugerencia: no se exige; la disposición es
  `familiar` ([Q12] (supuesto)); sí se exigen diálogos de archivo nativos, el modo oscuro del
  sistema y DPI alto ([Q04]).
- **CPU en reposo.** [Q25] solo fija la RAM — sugerencia: cerca de 0 % sin animaciones, repintando
  solo ante eventos, medido en el hito 0.
- **Máquina de referencia** para las mediciones de reloj del hito 0 y para los tiempos que [Q25]
  reporta sin bloquear — sugerencia: una máquina de gama baja a media, documentada.
- **Modelo de temas.** Cómo se pasa de los scopes de `syntect` a los colores de un tema editable
  del hito 3 — sugerencia: un conjunto propio de tokens semánticos, con una tabla de mapeo desde
  los scopes.
- **Meta de accesibilidad.** Qué se entrega además del teclado y el lector de pantalla de [Q20] —
  sugerencia: para la primera versión pública, interfaz completa con teclado y etiquetada para
  lector de pantalla, un tema de alto contraste y un informe corto de conformidad.
- **Español y escritura de derecha a izquierda.** Qué variante de español usa la interfaz y si hay
  traducciones de derecha a izquierda — sugerencia: español neutro, cercano a Latinoamérica;
  traducciones de interfaz de derecha a izquierda fuera hasta que llegue quien las haga; el texto
  bidireccional del hito 2 sigue siendo requisito de renderizado.
- **Fuzzing.** Objetivos y dónde corre — sugerencia: `cargo-fuzz` sobre los decodificadores, el
  normalizador de finales de línea, el lector de configuración y la traducción de regex, en un job
  nocturno.

### Distribución

- **Identificadores estables**: bundle ID de macOS, ID de dominio inverso, nombre del `.deb` y
  UpgradeCode del MSI, que no pueden cambiar después del primer release sin romper las
  actualizaciones — sugerencia: derivarlos del dominio verificado ([Q01]) y fijarlos antes del
  primer artefacto público.
- **Nombres de archivo y vocabulario de arquitecturas** (x64, amd64 y x86_64; arm64 y aarch64) —
  sugerencia: `amate-<versión>-<sistema>-<arquitectura>.<ext>`, y el `.deb` con la convención de
  Debian: `amate_<versión>_amd64.deb`.
- **Alcance del MSI**: por usuario o por máquina, entrada en el `PATH` para `amate archivo:línea`,
  accesos del menú Inicio, y si desinstalar conserva la configuración — sugerencia: por usuario,
  `PATH` opcional, y la configuración se conserva al desinstalar.
- **Tipos de archivo** que cada paquete registra en «Abrir con» — sugerencia: tipos de texto y de
  código, solo en «Abrir con» ([Q14]).
- **Metadatos de escritorio y de plataforma**: metainfo de AppStream, tamaños de iconos, `Exec` con
  `%F` en el `.desktop`, manifiesto de Windows (DPI `PerMonitorV2`, `longPathAware`) y tipos de
  documento del `Info.plist` — sugerencia: metainfo e iconos en el `.deb`, y todo generado desde
  plantillas en `xtask`.
- **Un solo `.deb` para Debian 12 y Ubuntu 24.04**: nombres de dependencias y los renombres `t64`
  de bibliotecas en Ubuntu (a verificar) — sugerencia: dependencias automáticas de `cargo-deb`
  calculadas en bookworm, con pruebas de instalación en contenedores limpios de Debian 12,
  Debian 13, Ubuntu 24.04 y Linux Mint.
- **El AppImage y FUSE**: el runtime clásico necesita `libfuse2`, que Debian 12 y Ubuntu 24.04 no
  instalan por defecto (a verificar) — sugerencia: runtime estático (type2), probado en Debian 12 y
  Ubuntu 24.04 limpios.
- **macOS**: versión mínima de despliegue, método del binario universal, un atajo de línea de
  comandos en el `PATH`, y cómo se prueba la mitad x86_64 — sugerencia: versión mínima igual a la
  del toolkit; compilar en un runner arm64 con el target x86_64 cruzado y unir con `lipo`; probar
  la mitad x86_64 en un runner Intel mientras exista, o con Rosetta (a verificar), y no publicar
  sin esa prueba.
- **Guardar con permisos elevados**: UAC en Windows, `pkexec` en Linux y quizá una política de
  polkit en el `.deb` — sugerencia: decidirlo en el hito 0; el ayudante solo va en los paquetes
  instalados, y la función se apaga en las variantes portables.
- **Credenciales de firma y `SECURITY.md`** con un solo mantenedor: custodia, rotación y revocación
  (SignPath, y en el hito 4b Developer ID, certificado de Windows y llave GPG del repositorio apt),
  versiones con soporte y proceso de divulgación — sugerencia: reporte privado de vulnerabilidades
  de GitHub ([Q15]); secretos solo en un entorno protegido del flujo de release, con respaldo fuera
  de línea a cargo del mantenedor y rotación documentada en `SECURITY.md`.
- **Avisos de terceros y SBOM**: formato y ubicación en cada paquete — sugerencia: generarlos en
  CI, incluidos el `copyright` del `.deb` en formato DEP-5 y el diálogo «Acerca de»; un SBOM
  CycloneDX por release.
- **Política de releases**: cadencia, versiones con soporte, correcciones de seguridad, formato del
  changelog y qué cuenta como cambio incompatible en una aplicación (esquema de configuración,
  opciones de línea de comandos, atajos) — sugerencia: solo la última versión tiene soporte;
  changelog con el formato de Keep a Changelog; etiquetas `vX.Y.Z`; subir la MSRV es un cambio
  menor.
- **La 0.1.0 del hito 1**: si es un release de GitHub, y entonces ya lleva `SHA256SUMS.txt` y la
  atestación de [Q28], o solo un artefacto de CI — sugerencia: releases 0.x etiquetados y sin firma
  desde el hito 1.
- **Prerelease o nightly**: nombre, retención y si es público — sugerencia: solo artefactos de CI,
  no releases públicos.
- **Tamaño y antivirus**: presupuesto de tamaño del binario y de la descarga, y falsos positivos
  con los builds de Windows sin firma — sugerencia: medir el tamaño en CI y reportar los falsos
  positivos a los fabricantes.
- **Builds reproducibles** — sugerencia: mejor esfuerzo, no compuerta.
- **Compilar desde el código**: requisitos por sistema, y que `cargo run --release` funcione sin
  `-p` en el workspace — sugerencia: la lista de requisitos por sistema una vez elegido el toolkit;
  `default-members` apuntando al crate de la aplicación.
- **Pasos para abrir builds sin firma** en el macOS vigente y en Windows 11 con Smart App Control
  activo — sugerencia: probarlos antes de escribirlos en el README.
- **Alojamiento** del repositorio apt (hito 4b) y de una futura página de descargas — sugerencia:
  GitHub Releases más GitHub Pages, hasta que haya una razón para mudarse.
- **«Notepad++» en metadatos de paquetes, tiendas y palabras clave de búsqueda** — sugerencia: solo
  en texto descriptivo y con el aviso de [Q21], nunca como nombre ni como palabra clave.

### Alcance y proyecto

- **La lista de `diario` a nivel de comando**, y qué pasa si el día de prueba del hito 2 encuentra
  un hueco — sugerencia: una matriz por área del manual y familia de comandos, con columnas de
  nivel, hito y «nunca o se reemplaza», y los ID de menú de Notepad++ en las filas de `diario`; el
  dueño la firma antes del hito 2; un hueco que aparezca en el día de prueba entra por un ADR que
  firma el dueño o va a una lista publicada de huecos conocidos.
- **Glosario** que liga cada término de `diario` con el elemento de menú de Notepad++ que nombra
  (por ejemplo, «carpeta de trabajo») — sugerencia: incluirlo en el análisis.
- **Lista de lo que nunca se hace o se reemplaza** — sugerencia: publicarla en el análisis y en el
  README.
- **Panorama de otros editores** — sugerencia: una comparación de una página en el análisis, que
  repita la propuesta de valor solo donde Amate gana.
- **Capacidades de plugins que se vuelven funciones propias**, y cuándo — sugerencia: sacarlas del
  triaje de `nppPluginList`; lo más usado encabeza la cola posterior a `diario`, que se decide por
  peticiones reales ([Q03]).
- **Prioridad de las peticiones después de la 1.0** — sugerencia: una plantilla de petición de
  función con reacciones, revisada en cada hito contra la tabla de niveles y los antiobjetivos de
  «Calidad mínima».
- **ADR**: ubicación, numeración y formato — sugerencia: `docs/adr/NNNN-titulo.md`, uno por
  decisión bloqueante, con contexto, decisión, consecuencias y alternativas descartadas.
- **Aprobación del trabajo del agente** durante la implementación — sugerencia: el agente trabaja
  en ramas, el mantenedor revisa y fusiona, y el CI tiene que pasar.
- **Código de conducta**: cuál y quién lo aplica — sugerencia: Contributor Covenant 2.1, con el
  mantenedor como contacto.

## Forma de la entrega

Guarda la entrega como `docs/05-plan-ejecutable.md`, en UTF-8 sin BOM y con finales LF ([Q30]).
Ábrela con el título `# Plan ejecutable de Amate`, la fecha de la corrida y el SHA del commit de
este repositorio que leíste. Después usa exactamente estos encabezados de nivel 2, en este orden;
dentro de cada uno, los subtítulos son libres:

1. `## Análisis`
2. `## Arquitectura`
3. `## Hitos`
4. `## Primera tanda`
5. `## Pruebas y CI`
6. `## Empaquetado`
7. `## Riesgos`
8. `## Cuestionario`
9. `## Supuestos aplicados`
10. `## Fuentes consultadas`

En «Supuestos aplicados» lista cada decisión, de [Q01] a [Q31], con la respuesta que usaste de
DECISIONES.md, y marca como supuesto la que aplique una recomendación (hoy, [Q12]). Después lista
cada hueco que cerraste con su sugerencia sin confirmación del dueño, y cada pregunta abierta de
[02-cuestionario.md](02-cuestionario.md) cuya recomendación aplicaste, con su identificador
antiguo.

En «Cuestionario» deja solo [Q12], si sigue vacía, y las decisiones que surjan en esta corrida: un
empate entre dos opciones, un hueco cuya sugerencia mueva el alcance, la licencia, el empaquetado,
la interfaz o el primer hito, o una pregunta abierta de 02 que pase ese mismo filtro. Las preguntas
nuevas usan los identificadores `N01`, `N02`…, para no chocar con los de las decisiones; si una
viene de 02, pon su identificador antiguo entre paréntesis en el título. Formula como máximo 15.
Usa este formato por pregunta:

```text
Qxx. Título
Aplicado: …
Si respondes otra cosa: … (qué hito o qué parte de la caja de crates cambia)
```

Las preguntas nuevas usan el mismo formato, con `N01` en lugar de `Qxx`. No repitas una pregunta
ya contestada en DECISIONES.md: consta solo en «Supuestos aplicados». Si las respuestas del dueño a
esta ronda crean ambigüedad nueva, la siguiente corrida abre una segunda ronda breve, con las
preguntas `N` que hagan falta.

En «Fuentes consultadas» lista cada URL, archivo, capítulo, issue o release que abriste, con la
fecha y, para los repositorios, el SHA del commit. Lo que no pudiste abrir se marca
«(a verificar)».

## Calidad mínima de tu respuesta

- Cada hito que asignas a una función de Notepad++ cita el capítulo del manual o el archivo del
  repositorio. Lo que no pudiste abrir se marca «(a verificar)».
- Separa los hechos verificados de tus recomendaciones. Cada hecho lleva su evidencia como enlace
  (archivo, directorio, capítulo, issue o release) y su sello: «(verificado: AAAA-MM-DD)», con la
  fecha de la corrida, o «(a verificar)». Cada recomendación se presenta como tal, con la
  restricción del producto o el compromiso que la justifica; nada de recomendaciones genéricas.
- No hay tareas del tipo «mejorar el editor» o «añadir más lenguajes» sin una lista.
- Antiobjetivos:
  - No construyas un IDE. Si una función hace que Amate tarde más en arrancar o deje de entenderse
    de un vistazo, queda fuera, salvo que el dueño diga otra cosa.
  - No prometas replicar los 510 elementos del menú de Notepad++ (verificado: 2026-10-15). Mejor un
    alcance pequeño, honesto y terminado.
  - No prometas fechas ni des estimaciones de calendario: tamaño relativo, dependencias y criterios
    de salida.
  - Nada de optimismo. Si algo es un proyecto enorme, dilo con los números que lo demuestran.
- No presentas Amate como fork ni como port de Notepad++.
- El hito 1 cabe en la base de este prompt más [Q04]. La paridad `diario` vive en el hito 2, y lo
  de `completo` se decide por peticiones reales de usuarios ([Q03]).
- Ninguna tecnología excluida aparece salvo como alternativa descartada, con su motivo.
- Cada crate o herramienta que propones trae sus tres candidatos y su comparación de seguridad, y
  queda pendiente de la aprobación del dueño.
- Un lector puede implementar el hito 0 sin hacerte otra pregunta, siempre que acepte los supuestos
  marcados.

# Amate

[English](README.md) · Español

**Editor de texto 100 % Rust: arranca al instante, se usa entero con teclado o lector de pantalla y
no se conecta a nada.**

Amate es un editor de texto y de código de escritorio, de código abierto, para Windows, Linux (con
prioridad para Debian y sus derivados) y macOS. Está pensado para quien usa Notepad++ en Windows y
quiere lo mismo en Linux y macOS ([Q17]). El nombre viene del náhuatl *āmatl*, el papel de corteza
de los códices ([Q01]). Los documentos de planeación están en español ([Q08]).

## Estado

Hito 0, en curso: un workspace de Cargo con CI y una ventana vacía hecha con egui; todavía no hay
binarios que descargar. Junto al código viven los documentos que lo preparan: el prompt operativo,
las decisiones, el análisis de Notepad++, el cuestionario, la hoja de ruta y el plan de
distribución.
La carpeta `notepadrust` es solo un nombre de trabajo local ([Q01]).

El contexto es fijo: una persona con experiencia sólida en Rust trabaja con un agente de código.
Las decisiones de [DECISIONES.md](docs/DECISIONES.md) se tomaron entre el 2026-10-15 y el
2026-10-20, y todas tienen respuesta salvo el parecido visual con Notepad++ ([Q12]), que aplica su
recomendación como supuesto.

Lo que lleva un proyecto público llega por etapas ([Q15]): [LICENSE](LICENSE) y este README, en
inglés y en español, desde el primer commit; el CI y `SECURITY.md`, con el reporte privado de
vulnerabilidades de GitHub, en el hito 0, junto con el primer código; `CONTRIBUTING.md` con DCO
([Q22]), el código de conducta y las plantillas de issues, en la primera versión pública; y
`GOVERNANCE.md`, cuando haya tres o más contribuyentes recurrentes.

## Qué es Amate y qué no

Amate es:

- **El sucesor espiritual de Notepad++** ([Q18]): la misma filosofía (rápido, ligero, abre
  cualquier archivo de texto sin ceremonia), un flujo y unos atajos reconocibles, y libertad para
  corregir lo que está mal diseñado. En Windows y Linux usa los mismos atajos que Notepad++; en
  macOS, Cmd donde Notepad++ usa Ctrl, respetando los atajos que reserva el sistema ([Q23]).
- **Rust estricto** ([Q19]): nada de C ni C++ compilado dentro del binario distribuido; sí usa las
  API y bibliotecas del sistema operativo. La interfaz es nativa: corre dentro del proceso, está
  escrita en Rust y no usa Electron ni un webview como interfaz principal ([Q16]).
- **Rápido, con presupuestos medidos** ([Q25]): desde el hito 1, el CI mide el arranque en frío
  (menos de 300 ms), la latencia de tecla (menos de 16 ms al p99) y la RAM en reposo con un archivo
  de 1 MB (menos de 100 MB), y falla ante una regresión.
- **Accesible por requisito** ([Q20]): se usa entero con teclado desde el hito 1, y el lector de
  pantalla (NVDA, VoiceOver y Orca) se verifica antes de la primera versión pública. El hito 0 lo
  mide junto con el IME, y un toolkit que no cumpla queda descartado ([Q24]).
- **Sin red** ([Q13]): ni comprobación de versión ni telemetría. La versión nueva se descarga de
  GitHub Releases.
- **Un producto para usuarios finales**: se instala, arranca, abre archivos y se actualiza sin que
  nadie tenga que saber qué es Rust.

Amate no es:

- **Un fork ni un port de Notepad++** ([Q18]). Es una reimplementación limpia: no copia ni traduce
  código, comentarios, textos del manual, iconos ni recursos, y ningún archivo de Notepad++ viaja en
  sus paquetes ([Q21]).
- **Un IDE.** Una función que haga que Amate tarde más en arrancar, o que deje de entenderse de un
  vistazo, queda fuera de alcance, salvo que el mantenedor decida otra cosa.
- **Una promesa de replicar todos los comandos de Notepad++.** Lo que va más allá de `diario` se
  decide por peticiones reales de usuarios, nunca por la lista del menú ([Q03]).
- **Compatible con los plugins de Notepad++.** Los plugins DLL nunca son objetivo, y no hay sistema
  de plugins antes de la 1.0 ([Q06]).
- **Un importador de la configuración de Notepad++.** Amate tiene su propia configuración ([Q05]).
- **Un editor para archivos de varios GB, ni un editor hexadecimal.** Un archivo de 10 MB se abre,
  se desplaza y se edita con fluidez; arriba de eso entra un modo degradado en el que la interfaz
  nunca se congela, pero sin promesa de fluidez ([Q11], [Q17]).

## Lo que encontró el análisis

El análisis de [01-analisis-notepad-plus-plus.md](docs/01-analisis-notepad-plus-plus.md) midió el
repositorio de Notepad++ sobre un clon superficial de `main` el 2026-10-15. Estos son los hallazgos
que más cambiaron el proyecto, cada uno con la decisión que se tomó.

1. **No es un puerto: es una reimplementación.** Notepad++ no tiene soporte de Linux ni de macOS:
   `SUPPORTED_SYSTEM.md` solo enumera Windows 8.1, 10 y 11. Su interfaz son 30 subdirectorios de
   controles Win32 en `PowerEditor/src/WinControls/`, definidos en archivos de recursos `.rc`; su
   modo oscuro está hecho con *subclassing* de Win32 (`NppDarkMode.cpp`) y su manejo de DPI es
   propio de Windows (`dpiManagerV2`) (verificado: 2026-10-15). No hay nada que portar.

   **Decisión:** Amate es sucesor espiritual, ni fork ni port ([Q18]), y una reimplementación
   limpia en la que no entra nada de Notepad++ ([Q21]). Su interfaz es nativa y está escrita en
   Rust ([Q16]). En Windows promete 10 (22H2) y 11 ([Q09]); Windows 8.1 no está en la lista.

2. **Notepad++ no implementa la edición de texto: conduce a Scintilla.** De unas 311 000 líneas de
   código del repositorio (solo `.cpp`, `.h` y `.cxx`, sin Boost.Regex, recursos ni XML), 158 145
   son la aplicación (`PowerEditor/src`), 73 252 son Scintilla, el motor de edición, que existe
   desde 1999, y 79 583 son Lexilla, con 128 lexers de resaltado (verificado: 2026-10-15).
   Reescribir eso en Rust o enlazarlo por FFI era la decisión número uno del proyecto: definía si
   había que reescribir 158 000 líneas (solo la aplicación) o 311 000 (también el motor y los
   lexers). Scintilla y Lexilla llevan una licencia permisiva propia, estilo HPND
   (verificado: 2026-10-15), así que enlazarlas era legalmente viable.

   **Decisión:** [Q19] la cerró por el lado largo: nada de C ni C++ compilado dentro del binario,
   así que Scintilla, Lexilla y Tree-sitter quedan fuera. Amate escribe su propio motor en Rust, con
   el documento en un rope desde el hito 1 ([Q11]), y resalta con `syntect` y gramáticas
   `.sublime-syntax`, que son datos y no código ([Q19], [Q26]). La referencia de tamaño es el total,
   no solo la aplicación.

3. **El ecosistema de plugins no se hereda.** Los plugins de Notepad++ son DLL de Windows que se
   comunican con Notepad++ por `SendMessage` de Win32, con 326 mensajes (`NPPM_*` y `NPPN_*` en
   `Notepad_plus_msgs.h`), y manipulan directamente los handles de ventana de Scintilla. El
   catálogo oficial, `nppPluginList`, tiene 1 701 estrellas en GitHub, que miden su popularidad y no
   cuántos plugins tiene (verificado: 2026-10-15). Es un ecosistema que hay que sustituir, no
   migrar.

   **Decisión:** [Q06] es `despues`: no hay sistema de plugins antes de la 1.0, y los plugins DLL
   de Notepad++ nunca son objetivo de compatibilidad. Desde el hito 1, cada acción del usuario es
   un comando con nombre y serializable, lo que habilita macros, paleta de comandos, atajos
   configurables y pruebas automatizadas. Lua queda fuera porque `mlua` compila C ([Q19]); los
   candidatos futuros son Rhai, o WebAssembly con `wasmi`, ambos en Rust puro.

4. **La configuración de Notepad++ es declarativa, pero Amate no la importa.** Sus datos son XML
   declarativo: 95 lenguajes con resaltado integrado (`langs.model.xml`), 22 temas, 94 traducciones
   de la interfaz y 48 definiciones de *Function List*, más la colección comunitaria de lenguajes
   definidos por el usuario (UDL), `userDefinedLanguages`, con 823 estrellas en GitHub, que miden
   popularidad y no número de lenguajes (verificado: 2026-10-15). Por eso importar esa
   configuración parecía la palanca de adopción más barata.

   **Decisión:** [Q05] es `ninguna`: Amate tiene su propia configuración (formato recomendado:
   TOML) y no hay importadores en la hoja de ruta. El hito 3 deja escrito el costo estimado de
   importar temas, UDL, atajos y sesión, como trabajo no incluido. Ningún archivo de Notepad++
   (temas, UDL, traducciones) viaja en los paquetes, aunque la GPL lo permitiría ([Q21]). Por eso
   Amate no se presenta como «compatible con la configuración de Notepad++».

5. **«Paridad con Notepad++» no es un objetivo verificable.** El menú principal (`IDR_M30_MENU` en
   `Notepad_plus.rc`) tiene 510 elementos repartidos en 13 menús de nivel superior. Solo
   `Edit → Line Operations` tiene 26 comandos, 14 de ellos variantes de ordenar líneas, y
   `Edit → Convert Case to` tiene 8 variantes (verificado: 2026-10-15). Hay que definir niveles de
   paridad explícitos y aceptar que la larga cola llega tarde o nunca: perseguirla entera es la
   forma estándar en que mueren los clones de software maduro.

   **Decisión:** [Q03] fija tres niveles con nombre. `familiar` es la base del hito 1, pulida.
   `diario` suma sesión con recuperación de pestañas sin guardar, buscar en archivos, edición en
   columna y multicursor, operaciones de línea (duplicar, mover, borrar, comentar, mayúsculas y
   minúsculas, sangría), plegado, vista dividida con dos vistas del mismo búfer, carpeta de
   trabajo, los lenguajes de [Q26], y temas y atajos configurables; sin plugins. `completo` suma
   macros, lenguajes definidos por el usuario, mapa del documento, lista de funciones, impresión y
   autocompletado. La primera versión pública tiene paridad `diario`, que se entrega en el hito 2.
   Lo de `completo` y la larga cola se deciden por peticiones reales de usuarios, nunca por la lista
   del menú de Notepad++.

**Y una corrección sobre la referencia de distribución.** La referencia de distribución de Amate es
la sección *standalone* de la página de PhotoCraft, una aplicación nativa en Rust para los tres
sistemas. Para Linux, en la página 0.7.0 solo Flatpak; el release 0.9.0 publica más archivos de
Linux (a verificar). Sea cual sea su lista real, la matriz de Linux de Amate sale del requisito de
Debian y derivados, no de PhotoCraft: Flatpak no viene preinstalado en Debian ni en Ubuntu
(a verificar), y su sandbox hace pasar el acceso al disco por portales XDG (a verificar), una
fricción constante para un editor de texto.

**Decisión:** [Q09] pone el `.deb` amd64 como artefacto de primera clase para Debian 12+ y
Ubuntu 24.04+ (Linux Mint 22+ por herencia), con AppImage y `.tar.gz` para el resto de las
distribuciones; Flatpak queda para el hito 4b. Linux se compila en un contenedor `debian:bookworm`
y se prueba en contenedores limpios. Del modelo de PhotoCraft, Amate toma lo que sí encaja: la
aplicación se instala sola, sin lanzador; en Windows hay instalador MSI y ZIP portable
(«descomprimir y ejecutar»); los binarios viven en GitHub Releases, el canal único, con
`SHA256SUMS.txt` y la atestación de procedencia ([Q28]); y la versión nueva se obtiene descargando
el release nuevo ([Q13]).

## Decisiones

### Qué está cerrado

Esta tabla resume las decisiones que más pesan en la primera impresión de quien llega de fuera
([Q31]). Todas, de Q01 a Q31, con lo que implica cada una y qué parte del plan se mueve si cambia,
están en [DECISIONES.md](docs/DECISIONES.md), que es la fuente de verdad. Los documentos las citan
por su identificador, así: [Q09].

| Tema | Decisión | ID |
| --- | --- | --- |
| Licencia | `GPL-3.0-or-later` para todo el repositorio: código, documentos de planeación y el futuro manual. | [Q02] |
| Paridad de la primera versión pública | `diario`, que se entrega en el hito 2. | [Q03] |
| Interfaz | `nativa`: corre dentro del proceso y está escrita en Rust. Sin Electron y sin un webview como interfaz principal. | [Q16] |
| Código nativo en el binario | Rust estricto: nada de C ni C++ compilado dentro del binario distribuido; sí se usan las API y bibliotecas del sistema operativo. | [Q19] |
| Accesibilidad | Requisito que filtra el toolkit: teclado completo desde el hito 1, y lector de pantalla verificado antes de la primera versión pública. | [Q20] |
| Idiomas | Interfaz en español e inglés en la primera versión pública. README bilingüe. Manual en los mismos idiomas que la interfaz, en el hito 3. Planeación en español; issues en español o en inglés; commits, código y comentarios en inglés. | [Q07], [Q08] |
| Tamaño de archivo | `normal`: un archivo de 10 MB se abre, se desplaza y se edita con fluidez; arriba de eso, modo degradado. | [Q11] |
| Lenguajes con resaltado | 13 en el hito 1 y 32 en la primera versión pública. | [Q26] |
| Configuración de Notepad++ | `ninguna`: Amate tiene su propia configuración y no hay importadores. | [Q05] |
| Plugins propios | `despues`: ninguno antes de la 1.0. | [Q06] |
| Paquetes | Windows 10 (22H2) y 11, x64: MSI y ZIP portable. macOS, universal: DMG. Debian 12+ y Ubuntu 24.04+, amd64: `.deb`. Resto de Linux, x86_64: AppImage y `.tar.gz`. Canal único: GitHub Releases. Fuera salvo petición explícita: Windows x86, `.rpm`, web/WASM, FreeBSD, RISC-V, Snap y Mac App Store. | [Q09] |
| Firma | `despues` para los certificados de pago. En el hito 4 se solicita la firma gratuita para código abierto de SignPath Foundation, cuyos requisitos falta revisar (a verificar); si la aprueban, la primera versión pública ya va firmada en Windows. Sin firma, quien tenga activo Smart App Control de Windows 11 no puede abrir Amate (a verificar). | [Q10] |
| Actualizaciones | `release`: la versión nueva se descarga de GitHub Releases. Amate no hace ninguna conexión de red: ni comprobación de versión ni telemetría. | [Q13] |
| Versionado y procedencia | SemVer, con 0.x hasta firmar. Cada release publica `SHA256SUMS.txt` y la atestación de procedencia de GitHub. | [Q27], [Q28] |
| Criterio de éxito | Adopción externa: que gente de fuera lo descargue, abra issues y contribuya. Sin telemetría, se mide con datos públicos de GitHub. | [Q31] |

### Qué sigue abierto

- **El parecido visual con Notepad++** ([Q12]) no tiene respuesta, así que aplica su
  recomendación como supuesto: `familiar`, con menús, pestañas arriba, barra de estado y atajos de
  Notepad++ en las acciones que existan, e iconos y marca propios.
- **La marca y el dominio.** Antes de publicar cualquier artefacto, crear el repositorio público o
  reservar el crate, falta verificar la marca «Amate» (IMPI, USPTO, EUIPO) y el dominio ([Q01]).
- **Las elecciones técnicas que las decisiones no fijan:** el toolkit de interfaz, los crates
  concretos (el del rope, codificaciones, configuración) y las herramientas de empaquetado. El plan
  ejecutable compara las alternativas de cada una y recomienda una; el hito 0 confirma el toolkit
  con mediciones ([Q24]). Cada crate o herramienta nueva se propone con al menos tres candidatos y
  una comparación de seguridad (mantenimiento, avisos conocidos, adopción, dependencias transitivas
  y licencia), y entra solo cuando el mantenedor la aprueba.
- **Las preguntas que ninguna decisión cerró** siguen abiertas en
  [02-cuestionario.md](docs/02-cuestionario.md), por urgencia y cada una con su recomendación.
- **Los huecos que el plan tiene que cerrar**, cada uno con una sugerencia, están en la sección
  «Huecos que el plan debe cerrar» de [PROMPT.md](docs/PROMPT.md).
- **Los hechos fechados** (versiones, fechas de soporte, requisitos de terceros) se vuelven a
  comprobar en cada corrida del prompt; la lista está en la sección «Hechos por re-verificar» de
  [fusion.md](docs/fusion.md).

## Cómo seguir

1. **Revisa [DECISIONES.md](docs/DECISIONES.md).** Una línea `Respuesta:` con contenido cierra la
   decisión; una vacía aplica la recomendación, y esa consecuencia queda marcada como supuesto en
   el plan. Para cambiar una decisión, edita su respuesta: el apartado «Si cambias» de cada una dice
   qué parte del plan se mueve.
2. **Corre el prompt.** Abre un agente de código dentro de este repositorio y dale
   [PROMPT.md](docs/PROMPT.md) como primer mensaje. El agente lee primero DECISIONES.md y los
   documentos 01 a 04, verifica y amplía lo que ya está hecho en vez de rehacerlo, y vuelve a
   comprobar los hechos fechados el día en que corre. Entrega un solo plan con encabezados fijos
   (análisis, arquitectura, hitos, primera tanda, pruebas y CI, empaquetado, riesgos, cuestionario,
   supuestos aplicados y fuentes consultadas) y lo guarda como `docs/05-plan-ejecutable.md`. Esa
   corrida no escribe código de la aplicación, no abre pull requests ni crea otro repositorio: este
   es el repositorio del producto. Los ADR se escriben en el hito 0, no en esa corrida.
3. **Revisa el plan.** Su cuestionario trae solo las decisiones que siguen sin respuesta o que
   surgieron en la corrida; las nuevas usan los identificadores `N01`, `N02`…, para no chocar con
   los de las decisiones (Q01 a Q31). Si cambias una respuesta, vuelve a correr el prompt.
4. **Empieza el hito 0**, el del primer código: el workspace de Cargo, el CI en los tres sistemas y
   la validación medida del toolkit.

### Hoja de ruta

| Hito | Qué entrega |
| --- | --- |
| Hito 0 — Cimientos y validación medida | Un workspace de Cargo que compila en Windows, Linux y macOS en CI: `fmt`, `clippy -D warnings`, pruebas, `cargo-deny` y la regla de [Q19] (el CI falla si el árbol de dependencias del binario incluye `cc`, `cmake` o `bindgen`, salvo el caso de `wayland-backend` que registra [Q19]). Una ventana vacía empaquetada de forma basta en cada sistema, todavía sin ser el MSI, el DMG ni el `.deb` finales. Spikes del toolkit en ramas `spike/`, que se borran, para medir IME, lector de pantalla, consumo en reposo, latencia de tecla, costo de empaquetado y el build universal de macOS; lo que queda es un ADR en `docs/adr/` con la decisión GO/PIVOT ([Q20], [Q24]). Con el primer código llegan el CI y `SECURITY.md` ([Q15]), y se fija con mediciones el umbral para abrir un archivo de 10 MB ([Q25]). Salida: CI verde en los tres sistemas, ADR con números y un artefacto basto descargable desde el CI. |
| Hito 1 — Primera versión usable (0.1.0, sin anuncio, [Q27]) | La «Base del hito 1» de PROMPT.md más los añadidos de [Q04]: coincidencia de paréntesis, espacios en blanco visibles, tema claro u oscuro del sistema, DPI alto, rutas de configuración resueltas al ejecutar con modo portable (archivo marcador junto al ejecutable), y abrir y guardar en ANSI (Windows-1252 e ISO-8859-1). También: teclado completo ([Q20]), textos fuera del código ([Q07]), los 13 lenguajes de [Q26], el documento en un rope ([Q11]), la capa de comandos serializable ([Q06]), un modelo de documento que ya admite varias vistas del mismo búfer (la vista dividida llega en el hito 2), el mapeo de atajos de macOS ([Q23]) y los paquetes de [Q09] generados en CI, sin firma ([Q10]). Los presupuestos de [Q25] se miden en CI desde aquí. La salida incluye el uso diario: el mantenedor usa Amate como editor principal para las tareas básicas durante dos semanas. |
| Hito 2 — Flujo diario | Todo lo de `diario` que no entró en el hito 1, con los 32 lenguajes de [Q26], y buscar en archivos, plegado y texto bidireccional ([Q03], [Q04]). Salida: un usuario habitual de Notepad++ trabaja un día completo sin volver atrás, comprobado con usuarios reales. |
| Hito 3 — Manual y preferencias | Preferencias y temas editables, manual de usuario en español e inglés ([Q08]), la interfaz completa en los dos idiomas ([Q07]) y, por escrito, el costo estimado de importar la configuración de Notepad++, como trabajo no incluido ([Q05]). |
| Hito 4 — Distribución y primera versión pública (0.N) | Los artefactos finales de [Q09], `SHA256SUMS.txt` y la atestación de procedencia ([Q28]), la integración con el escritorio ([Q14]), la sección de descargas del README con los pasos para abrir builds sin firma, la solicitud a SignPath Foundation ([Q10]) y `CONTRIBUTING.md` con DCO, código de conducta y plantillas de issues ([Q15], [Q22]). Salida: un `git tag` produce todo sin pasos manuales, y los paquetes se instalan y se verifican en máquinas limpias. |
| Hito 4b — opcional, después del hito 4 | Flatpak, `.deb` arm64, Windows ARM64, repositorio apt propio, winget, cask de Homebrew, firma de pago y notarización de macOS ([Q09], [Q10]). Va aparte del cierre del hito 4. Si las descargas lo piden, winget, Homebrew y apt son los primeros candidatos a adelantarse ([Q31]). |
| 1.0 | No es un hito, sino una compuerta: `diario` completo, firma en Windows, notarización en macOS y el manual ([Q27]). |
| Hito 5 — Extensiones | Con [Q06] en `despues`, es solo un anexo de una página del plan, nunca trabajo programado. |

Cada hito termina en algo que se puede ejecutar o instalar, y se cierra por criterio de salida
cumplido, no por fecha: no hay estimaciones de calendario. El detalle de cada uno (trabajo, crates
que toca, criterio de salida, riesgos y mitigación) está en
[03-plan-de-trabajo.md](docs/03-plan-de-trabajo.md).

## Documentos

- **Para seguir el trabajo:** revisa DECISIONES.md y corre PROMPT.md con un agente de código, como
  dice «Cómo seguir».
- **Para entender el proyecto:** lee DECISIONES.md, luego el análisis (01) y la hoja de ruta (03).
  La distribución está en 04, y el razonamiento detrás de cada pregunta, en 02.

| Archivo | Qué es |
| --- | --- |
| [docs/PROMPT.md](docs/PROMPT.md) | **El prompt operativo.** Un agente de código lo corre dentro de este repositorio. Fija las reglas que no se discuten, aplica las decisiones, define la base del hito 1, lista los huecos que el plan debe cerrar y exige un formato de entrega con encabezados fijos. Su resultado es `docs/05-plan-ejecutable.md`. |
| [docs/DECISIONES.md](docs/DECISIONES.md) | **La hoja de respuestas y la fuente de verdad.** De Q01 a Q31, cada decisión con su respuesta, lo que implica y qué parte del plan se mueve si cambia. |
| [docs/01-analisis-notepad-plus-plus.md](docs/01-analisis-notepad-plus-plus.md) | Análisis con datos medidos sobre un clon del repositorio de Notepad++: la organización y sus nueve repositorios públicos (verificado: 2026-10-15), líneas de código por componente, superficie funcional, formatos de configuración, API de plugins, licencias y marca, qué enseña PhotoCraft sobre distribución (y dónde su modelo no encaja) y lo que falta por analizar. |
| [docs/02-cuestionario.md](docs/02-cuestionario.md) | **El cuestionario largo:** 56 preguntas en 10 bloques, cada una con su urgencia (bloqueante, importante o diferible), por qué importa, opciones y recomendación, y con su estado: qué decisión la cerró o, si sigue abierta, qué se recomienda. DECISIONES.md tiene las respuestas; este archivo, el razonamiento. |
| [docs/03-plan-de-trabajo.md](docs/03-plan-de-trabajo.md) | La hoja de ruta por hitos, sin estimaciones de calendario: tamaño relativo, objetivo, trabajo, crates que toca, criterio de salida, riesgos y mitigación de cada hito, la compuerta de la 1.0 y la tabla de riesgos del proyecto ordenada por probabilidad. |
| [docs/04-distribucion-y-empaquetado.md](docs/04-distribucion-y-empaquetado.md) | La distribución: qué copiar de PhotoCraft y qué no, la matriz de artefactos por sistema y arquitectura, el problema de glibc en Debian, el modo portable, la firma y la notarización, la integración con el escritorio, las herramientas y el pipeline de release desde una etiqueta, las actualizaciones y la sección de descargas. |
| [docs/fusion.md](docs/fusion.md) | Cómo se fusionaron en este repositorio los borradores anteriores de la planeación: el mapa fuente → destino, las decisiones editoriales, los conflictos que decidió el dueño, el contenido descartado y por qué, por qué el prompt es así, y los hechos por re-verificar. |
| [LICENSE](LICENSE) | El texto de la licencia GPL-3.0. |
| [.gitattributes](.gitattributes) | LF en todo el repositorio, con CRLF solo para `*.bat` y `*.cmd` ([Q30]). |

Todavía no existe `docs/05-plan-ejecutable.md`, que crea la corrida del prompt. Los ADR viven en
`docs/adr/`, empezando por el del toolkit de la interfaz ([0001](docs/adr/0001-toolkit.md)).

## Licencia y marca

Copyright © 2026 Carlos Ceballos y colaboradores.

El titular del copyright es Carlos Ceballos, como persona ([Q29]). Amate se distribuye bajo la
licencia `GPL-3.0-or-later` ([Q02]), que cubre todo el repositorio: el código, los documentos de
planeación y el futuro manual. El texto completo está en [LICENSE](LICENSE). Las dependencias
deben tener una licencia compatible con GPLv3; desde el hito 0 lo controla `cargo-deny` en el CI.

Las contribuciones usan DCO (`Signed-off-by`), sin CLA ([Q22]). Cada commit escrito con ayuda de
un agente lo revisa y lo firma el mantenedor, y `CONTRIBUTING.md` pedirá lo mismo a quien
contribuya código asistido. Cambiar de licencia exigiría el permiso de cada contribuyente
([Q02]). Las issues se escriben en español o en inglés, y los commits, en inglés ([Q08]).

Notepad++ es una marca de su titular. Amate no está afiliado a Notepad++ ni a sus autores.

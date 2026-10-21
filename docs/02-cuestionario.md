# Cuestionario

Este es el cuestionario largo de Amate: 56 preguntas en 10 bloques, de A1 a J5. Las respuestas
viven en [DECISIONES.md](DECISIONES.md), que es la fuente de verdad; aquí queda el razonamiento:
por qué importa cada pregunta, qué opciones había y qué se recomendaba. Los identificadores A1 a J5
solo existen en este archivo y para trazabilidad; los demás documentos citan las decisiones por su
identificador, así: [Q09].

El cuestionario cubre también temas que DECISIONES.md resume en una línea: gobernanza (H2 a H4),
internacionalización (H5), accesibilidad (I3), presupuestos de rendimiento (I2) y política de
dependencias (E6). La licencia de los datos de configuración de Notepad++ no tiene pregunta propia:
la cierra [Q21], y el razonamiento está en C2 y en H1.

## Cómo leerlo

Cada pregunta trae su urgencia, según esta escala:

| Urgencia | Significado |
| --- | --- |
| bloqueante | Condiciona la arquitectura o la licencia. Se responde antes de escribir el código del producto. La excepción son los spikes de validación del hito 0, que existen justo para medir estas decisiones ([Q24]). |
| importante | Define el alcance de la primera versión pública. |
| diferible | Se puede decidir más adelante sin reescribir nada. Se incluye para que no se olvide. |

Algunas etiquetas cambiaron respecto de la versión anterior de este cuestionario porque
contradecían esta escala; esas preguntas lo dicen con «(antes: …)». El origen de esa versión está
en [fusion.md](fusion.md).

La línea «Estado:» dice una de dos cosas:

- `Cerrada → [Qxx](DECISIONES.md)`, con la respuesta en pocas palabras. La recomendación de
  entonces se conserva como «Recomendación original», para que se vea el razonamiento, y el texto
  dice si la decisión la adoptó o la superó. Manda la decisión, no la recomendación original.
- `Abierta — recomendación: …`. Ninguna decisión la cerró. Su recomendación ya es coherente con las
  decisiones cerradas.

Las preguntas abiertas conservan su línea `Respuesta:`. Mientras esté vacía, la corrida de
[PROMPT.md](PROMPT.md) aplica la recomendación, la cita por su identificador en este archivo (por
ejemplo, «F3 de 02-cuestionario.md») y la anota como supuesto. Con contenido, la respuesta cierra la
pregunta, como en DECISIONES.md. Las preguntas nuevas que surjan en esa corrida usan los
identificadores `N01`, `N02`… y van en su entrega, `docs/05-plan-ejecutable.md`, no aquí.

Cada dato fechado lleva «(verificado: 2026-10-15)» o «(a verificar)». La lista de hechos por
re-verificar está en [fusion.md](fusion.md).

---

## Estado

Todas las preguntas bloqueantes están cerradas: A1 [Q18], A2 [Q17], B1 [Q03], B2 [Q03], B3 [Q11],
B6 [Q06], C1 [Q05], C3 [Q23], D1 [Q06], E1 [Q19], E2 [Q16], E3 [Q19], F1 [Q09], F2 [Q09], G1 [Q10],
H1 [Q02] y [Q21], I3 [Q20] y J1 [Q24]. J2 la cierra el contexto fijo de DECISIONES.md.

| Urgencia | Cerradas | Abiertas |
| --- | --- | --- |
| bloqueante | 18 | 0 |
| importante | 19 | 9 |
| diferible | 4 | 6 |
| **Total** | **41** | **15** |

Ninguna pregunta abierta frena el hito 0. Estas son, por urgencia, con la recomendación que aplica
mientras su `Respuesta:` siga vacía:

| ID | Urgencia | Tema | Recomendación |
| --- | --- | --- | --- |
| B5 | importante | Autocompletado | Por palabras y por listas de API cuando llegue `completo` ([Q03]); LSP solo como extensión, nunca activada por defecto, y no antes de la 1.0 ([Q06]). |
| B8 | importante | Control de versiones | Git fuera de alcance; el historial de cambios queda en la larga cola, que se decide por peticiones reales ([Q03]). |
| C4 | importante | Dialecto de las expresiones regulares | `fancy-regex` por defecto, con límite de retroceso y cancelación, y `regex` como ruta rápida ([Q19]); el plan documenta las diferencias con tres ejemplos. |
| E4 | importante | Estructura del workspace | `core` sin interfaz ni `std::fs`, un crate para el sistema de archivos, el crate de la aplicación `amate` y `xtask`; sin `plugins-host` ([Q06]). |
| E5 | importante | MSRV | La penúltima estable, fijada al elegir los crates, declarada en `Cargo.toml` y verificada en CI. |
| F3 | importante | Wayland, X11 y escritorio de referencia | Ambos, con Wayland de referencia; GNOME y KDE como escritorios de prueba; diálogos de archivo por portales XDG. |
| F7 | importante | Build web | Sin build web ([Q09]); `core` libre de `std::fs`. |
| I1 | importante | Definición de «terminado» | La lista propuesta, escrita en CONTRIBUTING, que llega en el hito 4 ([Q15]). |
| I5 | importante | Corpus hostil | Desde el hito 1, con ISO-8859-1 y archivos de más de 10 MB; cada fallo suma un archivo. |
| B7 | diferible | Edición remota | Fuera de alcance: Amate no se conecta a la red ([Q13]); quizá una extensión después de la 1.0. |
| B9 | diferible | CLI headless | Sí, después de la primera versión pública, sobre el mismo núcleo. |
| C5 | diferible | Formato de configuración | TOML propio, sin importador ([Q05]). |
| D3 | diferible | Temas de usuario | Archivos de datos que se copian a una carpeta. |
| I4 | diferible | Pruebas automáticas y CI | Unitarias y de propiedades en el núcleo, pruebas de interfaz y CI en Windows x64, Linux x64 y macOS arm64. |
| J4 | diferible | Fechas externas | Ninguna; el orden lo fijan los criterios de salida de cada hito. |

La correspondencia completa entre las decisiones y estas preguntas está al final, en
«Correspondencia con DECISIONES.md».

---

## Bloque A — Producto y posicionamiento

### A1. ¿Cuál es la relación exacta con Notepad++: clon, sucesor espiritual o producto propio?

- Urgencia: bloqueante.
- Estado: Cerrada → [Q18](DECISIONES.md) — sucesor espiritual, ni fork ni port.

Por qué importa: determina si el criterio de éxito es «hace lo mismo» o «resuelve el mismo problema
mejor», y con ello la mitad de las decisiones de diseño.

- (a) **Clon de comportamiento**: cada decisión discutible se resuelve mirando qué hace Notepad++.
- (b) **Sucesor espiritual**: misma filosofía (rápido, ligero, sin ceremonia, abre cualquier cosa),
  libertad para mejorar lo que está mal diseñado.
- (c) **Producto propio** que solo toma inspiración y compatibilidad de configuración.

*Recomendación original:* (b). (a) te ata a dos décadas de deuda de interfaz; (c) pierde la razón
por la que alguien migraría. [Q18] la adoptó: misma filosofía, flujo y atajos reconocibles, y
libertad para corregir lo que está mal diseñado. No es fork ni port.

### A2. ¿Quién es el usuario que tiene que migrar el primer día?

- Urgencia: bloqueante.
- Estado: Cerrada → [Q17](DECISIONES.md) — quien usa Notepad++ y lo quiere en Linux y macOS.

Por qué importa: es el filtro para priorizar los 510 elementos del menú principal de Notepad++
(verificado: 2026-10-15).

- (a) Usuario de Windows que vive en Notepad++ y quiere lo mismo en Linux y macOS.
- (b) Desarrollador que hoy usa VS Code y lo encuentra pesado para abrir un archivo suelto.
- (c) Administrador de sistemas o analista que abre logs enormes, CSV y archivos de configuración.
- (d) Usuario general que quiere un Bloc de notas mejor.

*Recomendación original:* (a) como público primario y (c) como secundario, porque (c) daba un
diferenciador medible: archivos de varios GB, que Notepad++ hace mal (a verificar). [Q17] adoptó
(a). El público secundario (c) se cayó con su diferenciador: los archivos de varios GB no son
objetivo ([Q11], [Q17]).

### A3. ¿Cuál es la propuesta de valor en una frase, aparte de «es multiplataforma»?

- Urgencia: importante.
- Estado: Cerrada → [Q17](DECISIONES.md) — 100 % Rust, arranca al instante, accesible y sin red.

Opciones no excluyentes: arranque instantáneo · archivos enormes sin bloquearse · seguridad de
memoria · consumo de RAM · un solo binario sin instalador · interfaz accesible de verdad ·
compatibilidad total de configuración con Notepad++.

*Recomendación original, superada por [Q17]:* «Abre cualquier archivo, de cualquier tamaño, al
instante, en los tres sistemas, con tu configuración de Notepad++ intacta.» Dos de sus promesas ya
no se pueden hacer: «de cualquier tamaño» choca con [Q11], y «con tu configuración de Notepad++
intacta», con [Q05]. La frase de [Q17] es: «Editor de texto 100 % Rust: arranca al instante, se usa
entero con teclado o lector de pantalla y no se conecta a nada.»

### A4. ¿Cómo se llama el proyecto y hay nombre, dominio u organización ya reservados?

- Urgencia: importante.
- Estado: Cerrada → [Q01](DECISIONES.md) — `amate`; falta verificar marca y dominio.

Restricción dura: no puede contener «Notepad++», ni ser confundible con él, ni usar su logotipo.

Sin recomendación original: la pregunta no traía valor por defecto. [Q01] eligió `amate`, del
náhuatl *āmatl*. En la prosa se escribe «Amate»; el binario, el comando y el crate son `amate`.
Antes de publicar cualquier artefacto, crear el repo público o reservar el crate, falta verificar la
marca (IMPI, USPTO, EUIPO) y el dominio.

### A5. ¿Habrá alguna vez una versión de pago, soporte comercial o patrocinio?

- Urgencia: importante (antes: diferible).
- Estado: Cerrada → [Q02](DECISIONES.md) — `GPL-3.0-or-later`; sin planes comerciales ([Q22]).

Por qué importa: afecta a la elección de licencia (bloque H) y a si se acepta un toolkit con
licencia dual como Slint.

Sin recomendación original: la pregunta no traía valor por defecto. Se resolvió junto con la
licencia: [Q02] fija `GPL-3.0-or-later`, con la que Slint deja de tener conflicto de licencia, y
[Q22] dice que no hay planes comerciales.

---

## Bloque B — Alcance funcional de la primera versión pública

### B1. ¿Qué nivel de paridad define la primera versión pública?

- Urgencia: bloqueante.
- Estado: Cerrada → [Q03](DECISIONES.md) — `diario` en la primera versión pública.

Por qué importa: es la pregunta de la que depende que el proyecto termine o no termine nunca.

La propuesta original tenía tres niveles, con letras que chocaban con los bloques de este
cuestionario. Aquí van con un nombre descriptivo:

- **Núcleo insustituible:** pestañas, sesiones, buscar y reemplazar con regex, buscar en archivos,
  multicursor y selección en columna, resaltado de los lenguajes más usados, codificaciones y
  conversión de fin de línea, deshacer y rehacer, autoguardado y recuperación, temas claro y oscuro,
  mapa del documento, lista de funciones y plegado de código.
- **Lo que hace que no vuelvas:** operaciones de línea completas, editor de columnas, macros
  grabables, ejecutar comandos externos, panel de proyectos, explorador de archivos, comparación de
  documentos, panel de caracteres, historial del portapapeles y lenguajes definidos por el usuario
  (UDL).
- **La larga cola:** el resto, como el pegado de RTF, HTML o binario, las 14 variantes de ordenar
  líneas (verificado: 2026-10-15), las familias de estilos de marcado de tokens y la integración con
  la shell del sistema.

*Recomendación original:* la primera versión pública es el núcleo insustituible completo y bien
pulido; lo que hace que no vuelvas se reparte después; la larga cola se guía por peticiones reales
de usuarios, nunca por la lista del menú original.

[Q03] cerró la pregunta con otra escala, de nombres y no de letras: `familiar`, `diario` y
`completo`; el contenido de cada nivel está en [DECISIONES.md](DECISIONES.md). La primera versión
pública tiene paridad `diario`, que se entrega en el hito 2, y la 1.0 exige `diario` completo
([Q27]). Del núcleo insustituible, `diario` deja el mapa del documento y la lista de funciones en
`completo`. De lo que hace que no vuelvas, `completo` toma las macros y los lenguajes definidos por
el usuario. Lo que [Q03] no nombra queda en la larga cola, que se decide por peticiones reales de
usuarios. La integración con la shell la ordena aparte [Q14].

### B2. ¿Amate es de un solo documento por ventana, con pestañas, o con paneles divididos?

- Urgencia: bloqueante.
- Estado: Cerrada → [Q03](DECISIONES.md) — pestañas y dos vistas del mismo búfer.

Notepad++ tiene pestañas más dos vistas (izquierda y derecha), con la posibilidad de clonar un
documento en ambas. Es una restricción de arquitectura: «varias vistas del mismo búfer» hay que
diseñarlo desde el principio, no agregarlo después.

*Recomendación original:* pestañas y dos vistas con búfer compartido desde el día uno, porque
agregarlo después es caro.

[Q03] pone en `diario` la vista dividida con dos vistas del mismo búfer. La columna «Origen» de
DECISIONES.md no lista B2, pero el contenido de [Q03] la responde. En la hoja de ruta, el modelo de
documento admite varias vistas del mismo búfer desde el hito 1, y la vista dividida llega en el
hito 2 ([03-plan-de-trabajo.md](03-plan-de-trabajo.md)).

### B3. ¿Cuál es el tamaño máximo de archivo que debe abrirse con fluidez, y qué significa «con fluidez»?

- Urgencia: bloqueante (antes: importante).
- Estado: Cerrada → [Q11](DECISIONES.md) — `normal`: 10 MB con fluidez; arriba, modo degradado.

Por ejemplo: «un log de 4 GB se abre en menos de 2 s, se puede desplazar sin saltos y se puede
buscar con regex sin bloquear la interfaz». Notepad++ sufre a partir de unos 2 GB y con líneas muy
largas (a verificar). Si esto es un diferenciador, es un requisito de arquitectura (carga diferida,
`mmap`, indexado incremental de líneas), no una optimización posterior.

*Recomendación original, superada por [Q11]:* objetivo explícito de 4 GB con desplazamiento fluido
y búsqueda sin bloqueo; los archivos binarios se abren en modo hexadecimal de solo lectura.

[Q11] eligió `normal`: un archivo de 10 MB se abre, se desplaza y se edita con fluidez. Arriba de
10 MB entra un modo degradado en el que la interfaz nunca se congela, sin promesa de fluidez. Los
archivos de varios GB y el modo hexadecimal para binarios quedan fuera. El documento vive en un rope
desde el hito 1; si [Q11] cambia a `grandes` o más, la carga diferida, `mmap` y el índice de líneas
en segundo plano pasan al hito 1.

### B4. ¿Cuántos lenguajes con resaltado tiene que soportar la primera versión pública, y cuáles?

- Urgencia: importante.
- Estado: Cerrada → [Q26](DECISIONES.md) — 13 en hito 1 y 32 en primera versión pública.

Notepad++ trae 95 y Lexilla tiene 128 lexers (verificado: 2026-10-15).

*Recomendación original, superada por [Q26]:* 25 lenguajes de primera clase (C, C++, C#, CSS, Go,
HTML, INI, Java, JS, JSON, Kotlin, Lua, Markdown, PHP, Python, Rust, Shell/Bash, SQL, TOML, TS, XML,
YAML, Dockerfile, diff/patch, texto plano) y una vía de ampliación declarativa para el resto.

[Q26] fija 13 lenguajes en el hito 1 y 32 en la primera versión pública; las dos listas están en
DECISIONES.md. Los 25 de la recomendación original están todos entre los 32, que suman Batch,
PowerShell, PL/SQL, T-SQL, Makefile, CMake y LaTeX. La vía declarativa es la de [Q19] y [Q26]:
definiciones `.sublime-syntax` para `syntect`, con licencia compatible con GPL-3.0 y sus avisos
conservados.

### B5. ¿Autocompletado basado en palabras del documento, o integración con LSP?

- Urgencia: importante.
- Estado: Abierta — recomendación: por palabras y listas de API, con `completo`; LSP, nunca por
  defecto.

Notepad++ solo hace autocompletado por palabras y por listas de API declaradas en XML
(a verificar). LSP acerca a un IDE, lo que puede traicionar el posicionamiento de «editor ligero» y
multiplica el alcance (gestión de procesos de servidor, diagnósticos, ir a definición).

Recomendación: por palabras y por listas de API, como Notepad++, con las listas en el formato propio
de Amate, porque las de Notepad++ no viajan en los paquetes ([Q21]). La recomendación original lo
ponía en la primera versión pública, pero [Q03] deja el autocompletado en `completo`: llega cuando
lo pidan usuarios reales. LSP, solo como extensión opcional, nunca activada por defecto; como no hay
sistema de plugins antes de la 1.0 ([Q06]), no llega antes.

Respuesta:

### B6. ¿Las macros grabables (grabar, reproducir, guardar, asignar atajo) entran en la primera versión pública?

- Urgencia: bloqueante (antes: importante).
- Estado: Cerrada → [Q06](DECISIONES.md) — capa de comandos serializable desde el hito 1.

Son muy usadas por el público objetivo y afectan al diseño del núcleo: exigen que todos los
comandos pasen por una capa de comandos serializable. Agregarlas después es caro si no se previó.

*Recomendación original:* diseñar la capa de comandos serializable desde el principio, aunque la
interfaz de macros llegue después. [Q06] la adoptó: desde el hito 1, cada acción del usuario es un
comando con nombre y serializable, lo que habilita macros, paleta de comandos, atajos configurables
y pruebas automatizadas. Las macros en sí son del nivel `completo` ([Q03]), así que no entran en la
primera versión pública.

### B7. ¿Soporte para edición remota (SSH, SFTP, FTP), que en Notepad++ es el plugin NppFTP?

- Urgencia: diferible.
- Estado: Abierta — recomendación: fuera de alcance; quizá una extensión después de la 1.0.

El plugin NppFTP es la vía de Notepad++ para esto (a verificar).

Recomendación: fuera de alcance. Amate no hace ninguna conexión de red ([Q13]) y no hay sistema de
plugins antes de la 1.0 ([Q06]). Es la candidata natural a ser la primera extensión de terceros que
valide el modelo de extensiones cuando exista, y entonces habría que revisar [Q13].

Respuesta:

### B8. ¿Integración con control de versiones (indicadores de línea modificada, diff con HEAD)?

- Urgencia: importante (antes: diferible).
- Estado: Abierta — recomendación: Git fuera; historial de cambios, por peticiones reales.

Notepad++ tiene «Change History» (cambios desde el último guardado), no integración con Git
(a verificar).

Recomendación: Git fuera de alcance. La recomendación original ponía el historial de cambios de la
sesión en la primera versión pública, pero [Q03] no lo incluye en `diario`: queda en la larga cola,
que se decide por peticiones reales de usuarios.

Respuesta:

### B9. ¿Hace falta una CLI headless, como la que publica PhotoCraft, para scripts?

- Urgencia: diferible.
- Estado: Abierta — recomendación: sí, después de la primera versión pública, sobre el núcleo.

Caso de uso: conversión de codificación por lotes, reemplazo con regex en muchos archivos, formateo
en un *hook* de CI. PhotoCraft publica una CLI universal en `.zip` junto a su DMG
(verificado: 2026-10-15).

Recomendación: sí, pero después de la primera versión pública, y reutilizando el mismo núcleo como
biblioteca (ver E4). No es la línea de comandos del hito 1 (`amate archivo` y `amate archivo:línea`),
que abre archivos en la interfaz.

Respuesta:

---

## Bloque C — Compatibilidad con Notepad++

### C1. ¿La compatibilidad de configuración es un objetivo de producto o un «estaría bien»?

- Urgencia: bloqueante.
- Estado: Cerrada → [Q05](DECISIONES.md) — `ninguna`: configuración propia, sin importadores.

Parecía la palanca de migración más barata: son archivos XML declarativos, no código. Si era un
objetivo, el importador tenía que estar en la primera versión pública y tener pruebas con archivos
reales.

*Recomendación original, superada por [Q05]:* objetivo de primer nivel, con un asistente de
migración al primer arranque que detectara una instalación de Notepad++ y ofreciera importar.

[Q05] es `ninguna`: Amate tiene su propia configuración y no hay importadores en la hoja de ruta. El
hito 3 deja escrito el costo estimado de importar temas, UDL, atajos y sesión, como trabajo no
incluido. Por eso Amate no se presenta como «compatible con la configuración de Notepad++».

### C2. ¿Qué se importa exactamente?

- Urgencia: importante.
- Estado: Cerrada → [Q05](DECISIONES.md) — no se importa nada; el hito 3 estima el costo.

Archivos que se consideraron:

- `config.xml`: preferencias generales.
- `stylers.xml` y los temas de `nppThemes`: colores y fuentes.
- `shortcuts.xml`: atajos de teclado, macros guardadas y comandos externos.
- `session.xml`: pestañas abiertas, posición del cursor y marcadores.
- `userDefineLang*.xml`: lenguajes definidos por el usuario. La colección comunitaria
  `userDefinedLanguages` tiene 823 estrellas en GitHub, que miden popularidad y no número de
  lenguajes (verificado: 2026-10-15).
- `functionList/*.xml`: definiciones de la lista de funciones, 48 archivos (verificado: 2026-10-15).
- `langs.xml`: palabras clave personalizadas por lenguaje.
- `contextMenu.xml`: menú contextual.
- Carpeta `backup/`: recuperación de archivos no guardados.

*Recomendación original, superada por [Q05]:* todo salvo `contextMenu.xml` y `functionList`, que
podían esperar.

Con [Q05] en `ninguna` no se importa ninguno; la lista sirve para la estimación de costo del hito 3.
Si [Q05] cambiara, los archivos se leerían siempre del disco del usuario ([Q05], [Q21]) y cada
importación llegaría con la función que la usa. Los comandos externos de `shortcuts.xml` son
invocaciones de programas: se mostrarían con su línea de comandos completa y nunca se ejecutarían
sin confirmación.

**Licencia de estos archivos.** Leer un archivo que el usuario ya tiene y redistribuirlo dentro de un
paquete son dos cosas distintas. La segunda la cierra [Q21]: ningún archivo de Notepad++ (temas, UDL,
traducciones) viaja en los paquetes, aunque la GPL lo permitiría. La licencia de cada pieza está en
la sección 3 de [01-analisis-notepad-plus-plus.md](01-analisis-notepad-plus-plus.md).

### C3. ¿Los atajos de teclado deben ser idénticos a los de Notepad++, incluso donde chocan con las convenciones de macOS y Linux?

- Urgencia: bloqueante.
- Estado: Cerrada → [Q23](DECISIONES.md) — Cmd donde Notepad++ usa Ctrl, respetando el sistema.

Ejemplo concreto: `Ctrl+D` duplica la línea en Notepad++, pero en macOS lo natural es `Cmd+D`, y
`Ctrl+D` es «borrar hacia adelante» a nivel de sistema. `Ctrl+W` cierra la pestaña en Notepad++ y
cierra la ventana en casi todo lo demás. Los atajos de Notepad++ se toman de su manual
(a verificar).

- (a) Idénticos en las tres plataformas, cueste lo que cueste.
- (b) Idénticos en Windows y Linux; mapa adaptado en macOS (`Cmd` donde Notepad++ usa `Ctrl`) con
  las excepciones que el sistema reserva.
- (c) Un mapa nativo por plataforma y un perfil «Notepad++» opcional.

*Recomendación original:* (b), con (c) disponible mediante perfiles de teclado intercambiables.
[Q23] la adoptó: Cmd donde Notepad++ usa Ctrl, respetando los atajos que reserva el sistema (Cmd+H,
Cmd+Q y similares); en Windows y Linux, los mismos atajos que Notepad++; los perfiles de teclado
intercambiables llegan después. El mapeo de macOS entra en el hito 1.

### C4. ¿La sintaxis de las expresiones regulares debe ser compatible con la de Notepad++?

- Urgencia: importante.
- Estado: Abierta — recomendación: `fancy-regex` por defecto, con `regex` como ruta rápida.

Trampa técnica real: Notepad++ usa Boost.Regex, con sintaxis de estilo Perl, retrorreferencias y
*lookaround*, según el capítulo de búsqueda de `npp-usermanual` (a verificar). El crate `regex` de
Rust, el estándar del ecosistema, **no soporta retrorreferencias ni lookaround** por diseño. Las
alternativas eran:

- `fancy-regex`: soporta retrorreferencias y lookaround, es Rust puro y no garantiza tiempo lineal.
- Enlazar PCRE2 u Oniguruma por FFI. Fuera por [Q19]: son C.

Recomendación: `fancy-regex` como motor por defecto, para que los patrones que la gente guardó en
Notepad++ sigan funcionando en lo posible, con límite de retroceso y cancelación porque no garantiza
tiempo lineal; y `regex` como ruta rápida cuando el patrón no usa construcciones avanzadas. Es el
par de motores que fija [Q19]. Lo que sigue abierto es cuánta compatibilidad se promete: el plan
documenta las diferencias de dialecto con tres ejemplos que un usuario de Notepad++ notaría
([PROMPT.md](PROMPT.md)).

Respuesta:

### C5. ¿Hay que replicar el formato de los archivos de configuración (XML) o se usa un formato propio (TOML, JSON)?

- Urgencia: diferible.
- Estado: Abierta — recomendación: formato propio en TOML, sin importador.

Recomendación: formato propio en TOML, porque es legible, es el estándar en Rust y soporta bien los
comentarios. Replicar el XML de Notepad++ ata a sus limitaciones para siempre. La recomendación
original sumaba un importador desde el XML de Notepad++; ya no aplica, porque [Q05] es `ninguna`, y
DECISIONES.md remite a esta pregunta para el formato.

Respuesta:

---

## Bloque D — Extensibilidad

### D1. ¿Qué modelo de extensibilidad tiene el producto?

- Urgencia: bloqueante.
- Estado: Cerrada → [Q06](DECISIONES.md) — `despues`: sin plugins antes de la 1.0.

Recordatorio del análisis (sección 2.5 de
[01-analisis-notepad-plus-plus.md](01-analisis-notepad-plus-plus.md)): los plugins de Notepad++ son
DLL de Windows que hablan por `SendMessage` de Win32 con 326 mensajes (verificado: 2026-10-15) y
manipulan `HWND` de Scintilla. **No se pueden reutilizar de ninguna forma.** Opciones:

- (a) **Ninguna en la primera versión pública.** Todo integrado. Menos superficie, menos
  compromisos de API.
- (b) **WebAssembly (WASI + Component Model)** con `wasmtime`. Sandbox real, multiplataforma por
  construcción, cualquier lenguaje que compile a Wasm. Costo: diseñar la interfaz del host.
- (c) **Scripting embebido (Lua con `mlua`, o Rhai).** Barrera de entrada bajísima, ideal para
  automatización del usuario, insuficiente para plugins que necesitan interfaz propia. Lua queda
  fuera por [Q19]: `mlua` compila C.
- (d) **Procesos externos con un protocolo JSON-RPC** (modelo LSP). Aislamiento por proceso,
  cualquier lenguaje, sin límite de dependencias.
- (e) **Carga dinámica nativa (`.dll`, `.so`, `.dylib`).** Máximo rendimiento, pero ABI frágil en
  Rust, sin sandbox, y multiplica el costo de empaquetado.

*Recomendación original, superada por [Q06] y [Q19]:* (c) desde la primera versión pública para la
automatización del usuario, con la capa de comandos diseñada para poder agregar (b) después sin
reescribir el núcleo; (e), descartada.

[Q06] es `despues`: no hay sistema de plugins antes de la 1.0, y el plan trae solo un capítulo corto
de cómo se agregaría, sin ABI ni diseño detallado; el hito 5 es ese anexo, nunca trabajo programado.
La capa de comandos serializable sí llega desde el hito 1 (ver B6). Los candidatos futuros que nombra
[Q06] son Rhai, o WebAssembly con `wasmi`, ambos en Rust puro. Los plugins DLL de Notepad++ nunca son
objetivo de compatibilidad.

### D2. Si hay plugins, ¿habrá un catálogo instalable dentro de la app, como el *Plugins Admin*?

- Urgencia: importante.
- Estado: Cerrada → [Q06](DECISIONES.md) — sin sistema de plugins antes de la 1.0.

Implica infraestructura: índice firmado, verificación de integridad, política de revisión, y alguien
que diga que sí o que no a cada envío.

*Recomendación original:* después de la 1.0, y solo cuando existan al menos 5 plugins reales que lo
justifiquen. Con [Q06] en `despues` no hay plugins antes de la 1.0, así que tampoco catálogo; la
condición de los 5 plugins reales queda como sugerencia para cuando los haya. Un catálogo que se
descarga de la red tendría además que revisar [Q13], porque Amate no hace ninguna conexión de red.

### D3. ¿Los temas de usuario son extensiones o solo archivos de datos que se copian a una carpeta?

- Urgencia: diferible.
- Estado: Abierta — recomendación: archivos de datos que se copian a una carpeta.

Recomendación: archivos de datos, en el formato propio de Amate ([Q05]). Simplifica todo y basta.
Los temas de Notepad++ no viajan en los paquetes ([Q21]).

Respuesta:

---

## Bloque E — Arquitectura y stack de Rust

### E1. ¿Qué motor de edición: implementación propia en Rust, o Scintilla vía FFI?

- Urgencia: bloqueante.
- Estado: Cerrada → [Q19](DECISIONES.md) — núcleo propio en Rust; Scintilla queda fuera.

La decisión más determinante del proyecto. La comparación se conserva como razonamiento; la columna
de Scintilla quedó fuera por [Q19]: nada de C ni C++ compilado dentro del binario. Scintilla existe
desde 1999, tiene backends Win32, GTK, Cocoa y Qt, Lexilla trae 128 lexers, y las dos llevan una
licencia permisiva propia, estilo HPND (verificado: 2026-10-15).

| | Scintilla + Lexilla vía FFI (fuera por [Q19]) | Núcleo propio en Rust |
| --- | --- | --- |
| Madurez | Más de dos décadas de casos límite resueltos | Hay que volver a descubrirlos uno a uno |
| Lexers | 128 sin costo | Hay que construir la estrategia desde cero |
| Licencia | Permisiva propia, estilo HPND; compatible con `GPL-3.0-or-later` | Sin restricciones |
| Multiplataforma | Backends Win32, GTK, Cocoa y Qt: te ata al toolkit | Libertad total |
| «Escrito en Rust» | Falso: el corazón es C++ | Cierto |
| Rendimiento con archivos enormes | Limitado por su diseño actual | Donde se puede ganar de verdad |
| Riesgo | Bajo al principio, techo de cristal después | Alto al principio, sin techo |

*Recomendación original:* núcleo propio en Rust. Si la premisa del producto es «basado en Rust» y el
diferenciador son los archivos grandes, enlazar C++ anula las dos cosas. Pero la decisión era tuya,
y era legítimo optar por el camino de menor riesgo.

[Q19] eligió el núcleo propio, por una razón distinta de los archivos grandes, que [Q11] dejó fuera:
nada de C ni C++ compilado dentro del binario. La referencia de tamaño es entonces el total del
repositorio de Notepad++, no solo la aplicación
([01-analisis-notepad-plus-plus.md](01-analisis-notepad-plus-plus.md)).

### E2. ¿Qué toolkit de interfaz?

- Urgencia: bloqueante.
- Estado: Cerrada → [Q16](DECISIONES.md) — `nativa`; el toolkit se confirma midiendo en el hito 0.

[Q16] cerró la pregunta de fondo: la interfaz es nativa, corre dentro del proceso y está escrita en
Rust, sin Electron y sin un webview como interfaz principal. El toolkit concreto no lo elige el
dueño: el plan lo recomienda y el hito 0 lo confirma con mediciones. Un toolkit que no cumpla con el
lector de pantalla o el IME queda descartado ([Q20], [Q24]).

Resumen del estado del ecosistema que se hizo para esta pregunta (a verificar). La corrida del
prompt revisa cada fila el día en que se ejecuta:

| Toolkit | Modelo | Accesibilidad | Licencia | Observación para Amate |
| --- | --- | --- | --- | --- |
| **egui** (+`eframe`) | Inmediato | AccessKit activado por defecto | MIT/Apache | El más rápido para llegar a algo usable; renderiza cada fotograma, hay que cuidar el consumo en reposo; aspecto poco nativo. |
| **Slint** | Retenido, DSL propio | AccessKit (cableado manual) | GPL o comercial | Muy pulido, con vista previa en vivo. Con [Q02] ya no hay conflicto de licencia. |
| **Iced** | Retenido, estilo Elm | Borrador, incompleta | MIT | El más «producto» en sensación y widgets; su accesibilidad tiene que pasar la medición del hito 0 ([Q20]). |
| **GPUI** | Híbrido, el de Zed | Ninguna | Apache-2.0 | Rendimiento excelente, pero sin campo de texto integrado, sin AccessKit, y antes de su 1.0, con cambios que rompen compatibilidad. Sin accesibilidad no pasa [Q20]. |
| **Xilem / Masonry** | Retenido, Vello | AccessKit desde el inicio | MIT/Apache | La apuesta más limpia en arquitectura; sigue siendo experimental. |
| **gtk4-rs** (fuera por [Q19]) | Nativo GTK | Nativa, excelente | LGPL | Perfecto en Debian, incómodo en Windows y macOS. |
| **Tauri** (fuera por [Q16]) | Web | La del navegador | MIT/Apache | Contradice «nativo en Rust» y arrastra un webview. |

Faltan en la tabla floem y los bindings de Qt, que también se propusieron: Qt queda fuera por
[Q19], y floem entra en la comparación del plan ([PROMPT.md](PROMPT.md)).

Nota de contexto: `VPAT-for-Npp` existe porque la accesibilidad es un requisito real en entornos
corporativos y públicos. Elegir un toolkit sin AccessKit cierra esa puerta.

*Recomendación original:* **egui** para el prototipo y la primera versión pública (pragmatismo y
accesibilidad incluida), con una capa de abstracción fina entre el núcleo y la interfaz para no
quedar atrapado. Si el aspecto nativo pesaba más que la velocidad de desarrollo, **Iced** era la
alternativa, asumiendo el hueco de accesibilidad. Esa salida ya no existe: con [Q20], un toolkit con
ese hueco queda descartado.

### E3. ¿Resaltado de sintaxis: Tree-sitter, gramáticas TextMate (`syntect`), Lexilla vía FFI, o motor propio?

- Urgencia: bloqueante.
- Estado: Cerrada → [Q19](DECISIONES.md) — `syntect` con `fancy-regex`; Tree-sitter y Lexilla fuera.

| | Tree-sitter (fuera por [Q19]) | `syntect` (TextMate) | Lexilla vía FFI (fuera por [Q19]) |
| --- | --- | --- | --- |
| Calidad | Árbol sintáctico real: plegado, lista de funciones y selección semántica salen gratis | Expresiones regulares línea a línea | Equivalente a Notepad++ por definición |
| Gramáticas disponibles | Cientos, mantenidas por su comunidad | Cientos (ecosistema de Sublime y VS Code) | 128 |
| Empaquetado | Cada gramática es C que hay que compilar o cargar dinámicamente | Rust puro, datos declarativos | C++ |
| Incremental | Sí, reparse incremental | No | Parcial |
| Archivos enormes | Hay que limitar al viewport | Barato, por línea | Diseñado para ello |

*Recomendación original, superada por [Q19]:* Tree-sitter para los lenguajes de primera clase
(habilita plegado y lista de funciones con un solo mecanismo) y `syntect` como respaldo de la cola
larga, comprobando antes con un prototipo el costo de empaquetar gramáticas en los tres sistemas,
que era el riesgo real de esa opción.

[Q19] eligió `syntect` con su backend `fancy-regex`, sin Oniguruma: las gramáticas de Tree-sitter
son C, y Lexilla es C++. El juego de sintaxis que trae `syntect` no cubre todos los lenguajes de
[Q26], y falta saber cuáles (a verificar); se suman definiciones `.sublime-syntax` con licencia
compatible con GPL-3.0, conservando sus avisos. El plegado y la lista de funciones, que Tree-sitter daba
gratis, necesitan otro mecanismo: el plegado entra en el hito 2 y la lista de funciones es del nivel
`completo` ([Q03]).

### E4. ¿Estructura del repositorio: un solo crate, o workspace con núcleo reutilizable?

- Urgencia: importante.
- Estado: Abierta — recomendación: workspace con un `core` sin interfaz ni acceso a disco.

Recomendación: workspace con un `core` (búfer, codificaciones, búsqueda, comandos) sin dependencias
de interfaz ni de `std::fs` (ver F7); un crate aparte para el sistema de archivos (abrir, guardar,
recorrer carpetas); el crate de la aplicación, `amate` ([Q01]); y `xtask` para las tareas de
empaquetado y release. Es lo que permite que la CLI de B9 y las pruebas del núcleo existan sin
arrastrar el toolkit; el nombre `cli` queda reservado para B9. La recomendación original incluía un
crate `plugins-host`, que no entra mientras [Q06] sea `despues`.

Respuesta:

### E5. ¿Versión mínima de Rust soportada (MSRV) y política de actualización?

- Urgencia: importante.
- Estado: Abierta — recomendación: la penúltima estable, declarada en `Cargo.toml` y verificada en
  CI.

PhotoCraft declara 1.90+ (verificado: 2026-10-15). Declarar una MSRV condiciona qué crates se pueden
usar y qué distribuciones pueden compilar Amate desde el código.

Recomendación: «la penúltima estable», fijada después de elegir los crates, revisada cada seis meses,
declarada en `Cargo.toml` y verificada en CI. La sección de descargas del README la declara junto a
la vía desde el código ([04-distribucion-y-empaquetado.md](04-distribucion-y-empaquetado.md)).

Respuesta:

### E6. ¿Hay restricciones sobre dependencias?

- Urgencia: diferible.
- Estado: Cerrada → [Q19](DECISIONES.md) — Rust estricto; el CI rechaza `cc`, `cmake` y `bindgen`.

Por ejemplo: prohibido `unsafe` fuera de módulos marcados, auditoría con `cargo-deny` o
`cargo-audit`, límite al número de dependencias transitivas, o veto a crates sin mantenimiento
activo.

*Recomendación original:* `cargo-deny` en CI (licencias y avisos de seguridad) y
`#![forbid(unsafe_code)]` en todos los crates salvo los que hacen FFI o `mmap`.

Lo decidido: nada de C ni C++ compilado dentro del binario, y el CI falla si el árbol de
dependencias del binario incluye `cc`, `cmake` o `bindgen` ([Q19]). Las licencias tienen que ser
compatibles con GPLv3, y lo controla `cargo-deny` ([Q02]). Además, cada crate o herramienta nueva se
propone con al menos tres candidatos y una comparación de seguridad (mantenimiento, avisos conocidos,
adopción, dependencias transitivas y licencia), y entra solo cuando el mantenedor la aprueba
([PROMPT.md](PROMPT.md)). `#![forbid(unsafe_code)]` no lo fija ninguna decisión: queda como
sugerencia.

---

## Bloque F — Plataformas, empaquetado y distribución

### F1. Confirma la matriz exacta de sistemas y arquitecturas.

- Urgencia: bloqueante.
- Estado: Cerrada → [Q09](DECISIONES.md) — MSI, ZIP, DMG, `.deb`, AppImage y `.tar.gz`.

Propuesta inicial, con lo que decidió [Q09] para la primera versión pública:

| Sistema | Arquitectura | Formato | Propuesta original | Con [Q09] |
| --- | --- | --- | --- | --- |
| Windows 10 y 11 | x86_64 | Instalador MSI o NSIS | Sí | MSI, en Windows 10 (22H2) y 11; NSIS no entra |
| Windows 10 y 11 | x86_64 | Portable `.zip` (sin instalación, sin permisos de administrador) | Sí: requisito cultural del público de Notepad++ | Sí, «descomprimir y ejecutar» |
| Windows 11 | ARM64 | MSI + portable | Por decidir | Hito 4b |
| Windows | x86 (32 bits) | MSI + portable | Por decidir | Fuera salvo petición explícita |
| Debian 12+ y Ubuntu 22.04+ | x86_64 | `.deb` | Sí | Sí, con Debian 12+ y Ubuntu 24.04+ (Linux Mint 22+ por herencia) |
| Debian y Ubuntu | ARM64 | `.deb` | Por decidir | Hito 4b |
| Linux (cualquiera) | x86_64 | AppImage y/o Flatpak | ¿Cuál de los dos? | AppImage y `.tar.gz` de binarios; Flatpak en el hito 4b |
| macOS 12+ | Universal (arm64 + x86_64) | `.dmg` | Sí | Sí, desde la versión de macOS que soporte el toolkit |

*Recomendación original:* primera versión pública con Windows x64 (instalador y portable), `.deb`
amd64, AppImage x86_64 y `.dmg` universal; Flatpak, ARM64 y x86 en la versión siguiente (lo que hoy
es el hito 4b) según demanda; x86 de 32 bits, probablemente nunca. [Q09] la adoptó casi entera: sumó
el `.tar.gz` de binarios, subió Ubuntu a 24.04, dejó Flatpak y ARM64 para el hito 4b, y Windows x86
fuera salvo petición explícita. También quedan fuera salvo petición explícita `.rpm`, web/WASM,
FreeBSD, RISC-V, Snap y Mac App Store.

Windows 10 ya no recibe parches de consumo (a verificar): se sigue prometiendo, pero las pruebas
principales se hacen en Windows 11 ([Q09]). El detalle por artefacto está en
[04-distribucion-y-empaquetado.md](04-distribucion-y-empaquetado.md).

### F2. ¿Qué versión mínima de Debian y Ubuntu hay que soportar, y qué glibc implica?

- Urgencia: bloqueante.
- Estado: Cerrada → [Q09](DECISIONES.md) — Debian 12+, Ubuntu 24.04+; compila en `debian:bookworm`.

Es la decisión que determina si se puede compilar en un runner moderno de CI o hace falta un
contenedor antiguo. Debian 12 (bookworm) trae glibc 2.36 y Debian 13 (trixie), la 2.41
(a verificar). Un binario compilado contra glibc 2.41 **no arranca** en bookworm.

*Recomendación original:* compilar en un contenedor con la glibc más antigua que se quiera soportar
(bookworm), o enlazar estáticamente contra musl para el AppImage, y declarar la política en el
README desde el principio. [Q09] eligió el contenedor `debian:bookworm` (glibc 2.36) y las pruebas en
contenedores limpios, no en el runner de CI. El `.deb`, el AppImage y el `.tar.gz` salen del mismo
contenedor, y musl solo entra si el hito 0 demuestra que funciona
([04-distribucion-y-empaquetado.md](04-distribucion-y-empaquetado.md)).

### F3. Para Linux, ¿Wayland, X11 o ambos? ¿Y qué entorno de escritorio es el de referencia?

- Urgencia: importante.
- Estado: Abierta — recomendación: ambos, Wayland de referencia, GNOME y KDE, portales XDG.

Afecta al portapapeles, los menús globales, el escalado fraccionario, las decoraciones de ventana
del lado del cliente y los diálogos de archivo (portales XDG). Amate usa las API del sistema,
Wayland y X11 incluidas ([Q19]).

Recomendación: ambos, con Wayland como referencia principal y GNOME y KDE como escritorios de
prueba; diálogos de archivo por portales XDG, para que funcionen también si Flatpak llega en el
hito 4b.

Respuesta:

### F4. ¿Se intenta entrar en los repositorios oficiales de Debian y Ubuntu, o se distribuye un repositorio apt propio?

- Urgencia: importante.
- Estado: Cerrada → [Q09](DECISIONES.md) — `.deb` en GitHub Releases; apt propio en hito 4b.

Entrar en Debian oficial exige empaquetar cada dependencia de Rust por separado, lo que es muy
costoso para un proyecto con un árbol de dependencias típico de Rust.

*Recomendación original:* repositorio apt propio (firmado) más `.deb` descargable directo, e
intentar Debian oficial solo mucho después, si alguien de la comunidad lo adopta. [Q09] deja la
primera versión pública con un solo canal, GitHub Releases, y el repositorio apt propio en el
hito 4b. Debian oficial no se busca
([04-distribucion-y-empaquetado.md](04-distribucion-y-empaquetado.md)).

### F5. ¿Qué gestores de paquetes de terceros entran, y quién los mantiene?

- Urgencia: importante.
- Estado: Cerrada → [Q09](DECISIONES.md) — winget y cask de Homebrew en el hito 4b.

Candidatos: `winget`, Chocolatey y Scoop (Windows) · cask de Homebrew y MacPorts (macOS) · Flathub,
Snap, AUR y Nixpkgs (Linux). Cada uno es un canal con su propio proceso de revisión y su
mantenimiento continuo.

*Recomendación original, superada por [Q09]:* en la primera versión pública, solo `winget` y el cask
de Homebrew, los dos más baratos de automatizar; Flathub después; el resto, bienvenidos si la
comunidad los adopta, pero sin que el proyecto los mantenga.

[Q09] deja winget, el cask de Homebrew y Flatpak para el hito 4b, y Snap fuera salvo petición
explícita. Si las descargas lo piden, winget, Homebrew y apt son los primeros candidatos a
adelantarse ([Q31]). Chocolatey, Scoop, MacPorts, AUR y Nixpkgs siguen siendo bienvenidos si la
comunidad los adopta, sin que el proyecto los mantenga.

### F6. ¿Habrá un lanzador o instalador unificado como el de ArtCraft?

- Urgencia: importante.
- Estado: Cerrada → [Q13](DECISIONES.md) — `release`: sin lanzador ni actualizador.

Tiene sentido cuando hay una familia de aplicaciones. Para un editor suelto es sobreingeniería.

*Recomendación original:* no, y en su lugar actualización automática integrada en la propia app (ver
G3). La primera mitad sigue en pie; la segunda la descartó [Q13]: la versión nueva se descarga de
GitHub Releases y Amate no hace ninguna conexión de red.

### F7. ¿Compilación a WebAssembly para una demo en el navegador, como hace PhotoCraft?

- Urgencia: importante (antes: diferible).
- Estado: Abierta — recomendación: sin build web ([Q09]); `core` sin `std::fs`.

Es una herramienta de marketing excelente («pruébalo sin instalar nada») y una restricción de
arquitectura real: el núcleo no puede asumir acceso al sistema de archivos. PhotoCraft publica un
build web en `.zip` (verificado: 2026-10-15).

Recomendación: no hay build web, porque [Q09] deja web/WASM fuera salvo petición explícita. Lo que
sigue abierto es la mitad de diseño: mantener el crate `core` libre de `std::fs` y llevar el acceso a
disco a un crate aparte (ver E4). Es barato ahora, deja el núcleo fácil de probar en memoria y no
cierra la puerta.

Respuesta:

### F8. ¿Integración con el sistema operativo?

- Urgencia: importante (antes: diferible).
- Estado: Cerrada → [Q14](DECISIONES.md) — `con-el-paquete`; nunca se apropia de las asociaciones.

Asociación de extensiones de archivo, entrada «Abrir con…» en el menú contextual (lo que en
Notepad++ es el repositorio `nppShell`), icono en la bandeja, listas de salto en la barra de tareas y
apertura de archivos por arrastre.

*Recomendación original:* asociación de archivos y apertura por arrastre en la primera versión
pública; menú contextual del explorador por plataforma después (en Windows 11 exige un paquete MSIX
y es un proyecto aparte).

[Q14] es `con-el-paquete`: el `.deb` instala `.desktop`, iconos y tipos MIME; en Windows y macOS, el
registro en «Abrir con» entra en el hito 4; Amate nunca se apropia de las asociaciones por defecto;
y el verbo «Editar con Amate» del menú contextual de Windows 11 exige un paquete MSIX firmado y queda
para después de la 1.0. El icono en la bandeja, las listas de salto y la apertura por arrastre son
comportamiento de la aplicación, no empaquetado, y [Q14] no los decide
([04-distribucion-y-empaquetado.md](04-distribucion-y-empaquetado.md)).

---

## Bloque G — Firma, actualizaciones y cadena de suministro

### G1. ¿Hay presupuesto y entidad legal para los certificados de firma de código?

- Urgencia: bloqueante.
- Estado: Cerrada → [Q10](DECISIONES.md) — certificados de pago después; SignPath en el hito 4.

Sin firma, SmartScreen de Windows muestra advertencias de software desconocido y Gatekeeper de macOS
bloquea la aplicación. Es la primera impresión del producto.

- macOS: Apple Developer Program, 99 USD al año (a verificar), necesario para firmar y notarizar.
- Windows: certificado OV o EV, o Azure Trusted Signing (a verificar), con validación de identidad
  de la organización.

*Recomendación original:* presupuestarlo desde el inicio; si no hay presupuesto, documentar en el
README los pasos de desbloqueo de cada sistema y asumir la pérdida de instalaciones.

[Q10] es `despues` para los certificados de pago. En el hito 4 se solicita la firma gratuita para
código abierto de SignPath Foundation, cuyos requisitos falta revisar (a verificar); si la aprueban,
la primera versión pública ya va firmada en Windows. Mientras tanto, el README explica cómo abrir la
app sin firma. Smart App Control de Windows 11 bloquea los binarios sin firmar y no admite
excepciones (a verificar). La 1.0 exige firma en Windows y notarización en macOS ([Q27]). El titular
es una persona, no una empresa ([Q29]). El detalle está en la sección 3 de
[04-distribucion-y-empaquetado.md](04-distribucion-y-empaquetado.md).

### G2. ¿Se publican sumas de verificación y firmas de los artefactos?

- Urgencia: importante.
- Estado: Cerrada → [Q28](DECISIONES.md) — atestación de GitHub y `SHA256SUMS.txt`.

PhotoCraft publica un `SHA256SUMS.txt` (verificado: 2026-10-15). La práctica actual agrega firmas
con `minisign` o `cosign` y atestaciones de procedencia (SLSA) generadas por el propio CI.

*Recomendación original:* `SHA256SUMS.txt`, firma del archivo de sumas y atestación de procedencia
desde GitHub Actions; es casi gratis si se configura al principio. [Q28] tomó las sumas y la
atestación: cada release publica `SHA256SUMS.txt`, en el formato que acepta `sha256sum -c`, y la
atestación de `actions/attest-build-provenance`, que se verifica con `gh attestation verify`. La
firma del archivo de sumas no entra, y la atestación no sustituye la firma de código.

### G3. ¿Actualización automática dentro de la aplicación?

- Urgencia: importante.
- Estado: Cerrada → [Q13](DECISIONES.md) — `release`: sin actualizador ni conexión de red.

Notepad++ usa un binario aparte, WinGup (`wingup`) (verificado: 2026-10-15). En Rust existen
`self_update` o el actualizador de Tauri. Pero hay un matiz importante: una app instalada por
`.deb`, Flatpak o Homebrew **no debe** actualizarse sola, porque rompe el gestor de paquetes.

*Recomendación original, superada por [Q13]:* que el binario sepa cómo fue instalado, con
autoactualización completa en las variantes portable y de instalador directo, y solo un aviso de
versión nueva en el resto.

[Q13] es `release`: la versión nueva se descarga de GitHub Releases y Amate no hace ninguna conexión
de red. Si algún día se agrega un aviso de versión, será opt-in en el primer arranque, solo enviará
la versión y usará el TLS del sistema. Para ese caso, el diseño por canal está en la sección 6 de
[04-distribucion-y-empaquetado.md](04-distribucion-y-empaquetado.md): en el ZIP, el MSI, el DMG, el
`.deb` suelto y el `.tar.gz`, avisa y enlaza a la descarga; con el cask de Homebrew o el repositorio
apt, avisa y la actualización la hace el gestor de paquetes; en el AppImage avisa por defecto, con
`AppImageUpdate` como opción; en Flatpak no hace nada. El binario sabría su canal por un
identificador que se incrusta al compilar cada paquete.

### G4. ¿La aplicación tiene telemetría, comprobación de versión en línea o informes de fallos?

- Urgencia: diferible.
- Estado: Cerrada → [Q13](DECISIONES.md) — sin telemetría ni comprobación de versión.

El público de Notepad++ es especialmente sensible a esto.

*Recomendación original:* nada de telemetría; comprobación de versión desactivable y bien visible en
el primer arranque; informes de fallos solo manuales y con el contenido a la vista antes de
enviarse.

[Q13] fue más lejos: Amate no hace ninguna conexión de red, ni comprobación de versión ni
telemetría, y un aviso futuro sería opt-in, no desactivable. Los informes de fallos siguen la
recomendación original: son manuales, el usuario ve su contenido y lo envía él mismo
([04-distribucion-y-empaquetado.md](04-distribucion-y-empaquetado.md)). Sin telemetría, la adopción
se mide con datos públicos de GitHub ([Q31]).

---

## Bloque H — Código abierto: licencia, gobernanza y comunidad

### H1. ¿Qué licencia?

- Urgencia: bloqueante.
- Estado: Cerrada → [Q02](DECISIONES.md), [Q21](DECISIONES.md) — `GPL-3.0-or-later` y
  reimplementación limpia.

- **MIT o Apache-2.0 (o el doble `MIT OR Apache-2.0`)**: la convención del ecosistema Rust, máxima
  adopción, permite uso comercial cerrado por terceros.
- **GPL-3.0**: igual que Notepad++, obliga a que los derivados sigan siendo libres.
- **MPL-2.0**: punto medio, *copyleft* por archivo.

Había dos restricciones cruzadas: si se elegía Slint como toolkit (E2), su licencia GPL se propagaba
salvo que se comprara la licencia comercial; y si se pensaba en soporte comercial (A5), el copyleft
condicionaba el modelo.

*Recomendación original, superada por [Q02]:* `MIT OR Apache-2.0`, porque es lo que espera quien
contribuye en Rust y evita bloqueos de adopción en empresas.

[Q02] eligió `GPL-3.0-or-later` para todo el repositorio: código, documentos de planeación y el
futuro manual. Las dependencias tienen que tener licencia compatible con GPLv3, y lo controla
`cargo-deny`. Slint deja de tener conflicto de licencia; sus niveles de licencia vigentes se revisan
antes de compararlo (a verificar). No hay planes comerciales ([Q22]). [Q21] fija aparte la
reimplementación limpia: nada de Notepad++ dentro del proyecto, aunque con esta licencia copiar sería
compatible en principio. Ese mismo [Q21] cierra la licencia de los datos de configuración: ningún
archivo de Notepad++ viaja en los paquetes (ver C2).

### H2. ¿Quién decide? ¿Dictador benévolo, equipo central o decisión por consenso?

- Urgencia: importante.
- Estado: Cerrada → [Q15](DECISIONES.md) — `publico`, por etapas; GOVERNANCE.md con tres
  contribuyentes recurrentes.

Notepad++ es un ejemplo extremo de proyecto gobernado por una sola persona, con todo lo bueno
(coherencia, velocidad) y lo malo (factor autobús, decisiones polémicas) que eso implica.

*Recomendación original:* mantenedor único al principio, con un `GOVERNANCE.md` que describa el
camino a un equipo central cuando haya tres o más contribuyentes recurrentes. [Q15] la adoptó en su
tabla de etapas: `GOVERNANCE.md` llega con tres o más contribuyentes recurrentes. Hasta entonces
decide el mantenedor, una persona que trabaja con un agente de código (contexto fijo de
[DECISIONES.md](DECISIONES.md)).

### H3. ¿Se exige CLA o DCO a quien contribuye?

- Urgencia: importante.
- Estado: Cerrada → [Q22](DECISIONES.md) — DCO, sin CLA.

Un CLA permite cambiar la licencia en el futuro; un DCO (`Signed-off-by`) es más ligero y suficiente
para la mayoría.

*Recomendación original:* DCO, porque el CLA ahuyenta las contribuciones ocasionales. [Q22] la
adoptó: DCO, sin CLA. El mantenedor revisa y firma con `Signed-off-by` cada commit escrito con ayuda
de un agente, y CONTRIBUTING pide lo mismo a quien contribuya código asistido. Como no hay CLA,
cambiar de licencia exigiría el permiso de cada contribuyente ([Q02]).

### H4. ¿Dónde vive el proyecto y dónde ocurre la conversación?

- Urgencia: importante.
- Estado: Cerrada → [Q15](DECISIONES.md) — proyecto `publico` por etapas, en una org de GitHub.

GitHub, GitLab o Codeberg; Discord, Matrix o solo GitHub Discussions.

*Recomendación original:* GitHub, donde está el público que se quiere captar, más GitHub
Discussions; un Discord solo cuando haya gente suficiente para que no parezca vacío. Las decisiones
ya fijan GitHub: la org va a nombre del titular ([Q29]), los binarios viven en GitHub Releases
([Q09]) y las vulnerabilidades llegan por el reporte privado de GitHub ([Q15]). Las issues se
escriben en español o en inglés ([Q08]). Ninguna decisión fija GitHub Discussions ni Discord: quedan
como sugerencia.

### H5. ¿Hay compromiso de internacionalización, y con qué infraestructura?

- Urgencia: importante (antes: diferible).
- Estado: Cerrada → [Q07](DECISIONES.md) — español e inglés; textos fuera del código desde hito 1.

Notepad++ tiene 94 traducciones (verificado: 2026-10-15). Es un activo enorme y una obligación
continua.

*Recomendación original:* `fluent-rs` (el sistema de Mozilla, que maneja bien plurales y género)
desde la primera versión pública con español e inglés, y Weblate autoalojado o Crowdin para la
comunidad cuando llegue. [Q07] la adoptó en lo esencial: español e inglés en la primera versión
pública, con los textos fuera del código desde el hito 1 (recomendado: Fluent con `fluent-rs`), y la
prueba desde el inicio de que caben los textos en español, que son más largos. La interfaz completa
en los dos idiomas llega en el hito 3. Ninguna decisión fija Weblate ni Crowdin.

### H6. ¿Qué documentación se compromete el proyecto a mantener?

- Urgencia: diferible.
- Estado: Cerrada → [Q15](DECISIONES.md) — documentos por etapas; manual bilingüe en el hito 3
  ([Q08]).

Notepad++ tiene un repositorio entero, `npp-usermanual`, solo para eso.

*Recomendación original:* README, guía de contribución, guía de compilación por plataforma y un
manual de usuario mínimo como sitio estático; el manual exhaustivo, trabajo de comunidad.

[Q15] fija qué llega y cuándo: LICENSE y README (inglés y español) desde el primer commit; CI y
`SECURITY.md` en el hito 0, con el primer código; CONTRIBUTING con DCO, código de conducta y
plantillas de issues en la primera versión pública; `GOVERNANCE.md` con tres o más contribuyentes
recurrentes. [Q08] fija el manual: en español e inglés, los mismos idiomas que la interfaz, en el
hito 3. Si el manual se publica como sitio, lleva metadatos Open Graph con URLs absolutas, que
dependen del dominio de [Q01] ([PROMPT.md](PROMPT.md)).

---

## Bloque I — Calidad: pruebas, accesibilidad y rendimiento

### I1. ¿Cuál es la definición de «terminado» para cada función?

- Urgencia: importante.
- Estado: Abierta — recomendación: la lista propuesta, escrita en CONTRIBUTING en el hito 4.

Propuesta: compila sin avisos · tiene pruebas unitarias en el núcleo · tiene una prueba de interfaz
si toca la interfaz · funciona en los tres sistemas en CI · es navegable solo con teclado · está
documentada.

Recomendación: exactamente esa lista, escrita en `CONTRIBUTING.md` para que no se negocie caso por
caso. CONTRIBUTING llega con la primera versión pública, en el hito 4 ([Q15]). «Navegable solo con
teclado» es además requisito desde el hito 1 ([Q20]). Es la definición por función; los criterios de
salida de cada hito están en [03-plan-de-trabajo.md](03-plan-de-trabajo.md).

Respuesta:

### I2. ¿Qué objetivos de rendimiento son requisitos y no deseos?

- Urgencia: importante.
- Estado: Cerrada → [Q25](DECISIONES.md) — presupuestos medidos en CI que fallan ante regresión.

Propuesta de presupuesto: arranque en frío < 300 ms · abrir un archivo de 100 MB < 1 s · latencia de
tecla < 16 ms al percentil 99 · memoria en reposo con un archivo de 1 MB < 100 MB.

*Recomendación original:* medirlos en CI con un banco de pruebas y fallar la compilación si hay
regresión; es la única forma de que «rápido» siga siendo cierto dentro de dos años.

[Q25] adoptó la recomendación y tres de los cuatro números. El cuarto cambió con [Q11]: en vez de
abrir 100 MB en menos de 1 s, el presupuesto es abrir un archivo de 10 MB, con un umbral que fija el
hito 0 con mediciones. En los runners compartidos se bloquea con conteo de instrucciones (Linux); los
tiempos de reloj se reportan sin bloquear. Los presupuestos se miden en CI desde el hito 1.

### I3. ¿Qué nivel de accesibilidad se compromete?

- Urgencia: bloqueante (antes: importante).
- Estado: Cerrada → [Q20](DECISIONES.md) — requisito que filtra el toolkit.

Que exista un VPAT en la organización de Notepad++ (verificado: 2026-10-15) indica que hay clientes
institucionales que lo exigen por contrato. Las piezas son: lector de pantalla (AccessKit en los
tres sistemas), navegación completa por teclado, contraste y temas de alto contraste, respeto del
tamaño de fuente del sistema y soporte de IME para idiomas asiáticos.

*Recomendación original:* navegación por teclado y AccessKit como requisitos de la primera versión
pública; IME verificado con pruebas manuales en los tres sistemas, porque es un punto donde fallan
muchos toolkits de Rust.

[Q20] lo volvió un requisito que filtra el toolkit: teclado completo desde el hito 1; lector de
pantalla (NVDA, VoiceOver y Orca) verificado antes de la primera versión pública; el hito 0 lo mide
junto con el IME, y un toolkit que no cumpla queda descartado ([Q24]). Ninguna decisión nombra el
contraste, los temas de alto contraste ni el tamaño de fuente del sistema: quedan como parte de la
recomendación.

### I4. ¿Qué se prueba automáticamente y en qué sistemas corre el CI?

- Urgencia: diferible.
- Estado: Abierta — recomendación: unitarias y de propiedades en el núcleo; CI en los tres sistemas.

Recomendación: pruebas unitarias y de propiedades sobre el núcleo (el búfer de texto y las
codificaciones son candidatos perfectos para `proptest`), pruebas de interfaz con `egui_kittest` o
el equivalente del toolkit que confirme el hito 0, y CI en cada pull request sobre Windows x64,
Linux x64 y macOS arm64 ([04-distribucion-y-empaquetado.md](04-distribucion-y-empaquetado.md)).
Desde el hito 0, el CI ya corre `fmt`, `clippy -D warnings`, pruebas, `cargo-deny` y la regla de
[Q19]; desde el hito 1, mide los presupuestos de [Q25].

Respuesta:

### I5. ¿Hay un corpus de archivos de prueba, incluidos casos hostiles?

- Urgencia: importante (antes: diferible).
- Estado: Abierta — recomendación: sí, desde el hito 1; cada fallo suma un archivo.

Archivos con BOM, UTF-16 LE y BE, CP1252, Shift-JIS, finales de línea mezclados, líneas de 10 MB sin
salto, archivos sin salto final, binarios, enlaces simbólicos, archivos de solo lectura, rutas con
emojis y rutas de más de 260 caracteres en Windows.

Recomendación: construir el corpus con el primer código del núcleo, en el hito 1, y que pasarlo
entero sea parte de la salida de ese hito ([03-plan-de-trabajo.md](03-plan-de-trabajo.md)). Las
decisiones suman casos: archivos en ISO-8859-1 ([Q04]) y archivos de más de 10 MB para el modo
degradado ([Q11]). Cada fallo reportado se convierte en un archivo nuevo del corpus.

Respuesta:

---

## Bloque J — Forma de trabajar

### J1. ¿Qué se espera exactamente del agente en la siguiente iteración?

- Urgencia: bloqueante.
- Estado: Cerrada → [Q24](DECISIONES.md) — validación medida dentro del hito 0.

- (a) Solo el plan y los documentos; tú programas.
- (b) Plan más un prototipo funcional mínimo que valide el stack (abrir, editar, guardar, resaltar,
  en los tres sistemas) antes de decidir nada más.
- (c) Construcción completa del primer nivel de paridad, por hitos, con entregas revisables.
- (d) Construcción completa y además la infraestructura (CI, empaquetado, firma, web de descargas).

*Recomendación original:* (b). Un prototipo desechable que valide toolkit, resaltado y empaquetado
es más útil que cualquier documento, porque las decisiones E1, E2 y E3 son irreversibles en la
práctica.

[Q24] metió esa validación dentro del hito 0, sin un hito de investigación aparte: el hito 0 entrega
una ventana vacía empaquetada en los tres sistemas y no se cierra sin medir IME, lector de pantalla,
consumo en reposo, latencia de tecla, costo de empaquetado y build universal de macOS. Los spikes
viven en ramas `spike/` que se borran; lo que queda es el ADR en `docs/adr/`. La corrida de
[PROMPT.md](PROMPT.md) no programa: solo entrega el plan. Como [Q19] ya cerró E1 y E3, la validación
del hito 0 se concentra en el toolkit y el empaquetado.

### J2. ¿Quién más trabaja en esto y con qué experiencia en Rust?

- Urgencia: importante.
- Estado: Cerrada → contexto fijo de [DECISIONES.md](DECISIONES.md) — una persona con Rust sólido y
  un agente.

Determina cuánta complejidad de arquitectura es asumible y cuánto hay que documentar.

Sin recomendación original: la pregunta no traía valor por defecto. La cierra el contexto fijo de
DECISIONES.md: una persona con experiencia sólida en Rust trabaja con un agente de código. Por eso
los criterios humanos de salida de la hoja de ruta los cumple el mantenedor (y probadores
tempranos), no un equipo.

### J3. ¿Prefieres el repositorio en español, en inglés, o código en inglés y documentación bilingüe?

- Urgencia: importante.
- Estado: Cerrada → [Q08](DECISIONES.md) — código y commits en inglés; README y manual bilingües.

Afecta a nombres de identificadores, mensajes de commit, issues y documentación.

*Recomendación original:* código, identificadores y commits en inglés, porque es lo que permite
contribuciones externas; README y manual de usuario bilingües. [Q08] la adoptó y la precisó: README
en inglés y español; manual en los mismos idiomas que la interfaz, español e inglés, en el hito 3;
documentos de planeación en español; issues en español o en inglés; commits, código, identificadores
y comentarios en inglés.

### J4. ¿Hay alguna fecha, evento o hito externo que condicione el orden del trabajo?

- Urgencia: diferible.
- Estado: Abierta — recomendación: ninguna; el orden lo fijan los criterios de salida.

Recomendación: la pregunta no traía valor por defecto. El que aplica es que no hay fecha externa: el
orden lo fijan los criterios de salida de cada hito, sin estimaciones de calendario
([03-plan-de-trabajo.md](03-plan-de-trabajo.md)).

Respuesta:

### J5. ¿Qué es lo que, si pasara, haría que consideraras el proyecto un fracaso?

- Urgencia: diferible.
- Estado: Cerrada → [Q31](DECISIONES.md) — adopción externa; fracaso: que nadie de fuera lo use.

Pregunta deliberadamente incómoda. Las respuestas típicas («que nadie lo use», «que acabe siendo
otro editor a medias», «que tarde tanto que pierda el interés») son la mejor guía para priorizar.

Sin recomendación original: la pregunta no traía valor por defecto. [Q31] fija el criterio de éxito:
adopción externa. Amate salió bien si gente de fuera lo descarga, abre issues y contribuye; el
fracaso es que nadie de fuera lo use. Cuando haya que recortar, gana lo que decide la primera
impresión de quien llega de fuera: distribución (hito 4), firma (SignPath, [Q10]), README y manual
bilingües ([Q08]) y CONTRIBUTING.

---

## Correspondencia con DECISIONES.md

Cada decisión, con las preguntas de este cuestionario que cubre. Sale de la columna «Origen» del
índice de [DECISIONES.md](DECISIONES.md), con una sola adición por contenido, marcada en la tabla.

| Decisión | Tema | Preguntas |
| --- | --- | --- |
| Q01 | Nombre | A4 |
| Q02 | Licencia | H1, A5 |
| Q03 | Paridad de la primera versión pública | B1; también B2, por contenido (la columna «Origen» solo lista B1) |
| Q04 | Añadidos al hito 1 | — |
| Q05 | Configuración de Notepad++ | C1, C2 |
| Q06 | Plugins propios | D1, D2, B6 |
| Q07 | Idioma de la interfaz | H5 |
| Q08 | Idioma del repositorio y del manual | J3 |
| Q09 | Paquetes y plataformas | F1, F2, F4, F5 |
| Q10 | Firma y notarización | G1 |
| Q11 | Tamaño de archivo | B3 |
| Q12 | Parecido visual | — (sin respuesta: aplica `familiar` como supuesto) |
| Q13 | Actualizaciones | G3, G4, F6 |
| Q14 | Integración con el escritorio | F8 |
| Q15 | Tipo de proyecto | H2, H4, H6 |
| Q16 | Tecnología de la interfaz | E2 |
| Q17 | Usuario principal y propuesta de valor | A2, A3 |
| Q18 | Relación con Notepad++ | A1 |
| Q19 | Código nativo en el binario | E1, E3, E6 |
| Q20 | Accesibilidad | I3 |
| Q21 | Material de Notepad++ | H1 |
| Q22 | Contribuciones | H3 |
| Q23 | Atajos en macOS | C3 |
| Q24 | Validación en el hito 0 | J1 |
| Q25 | Presupuestos de rendimiento | I2 |
| Q26 | Lenguajes con resaltado | B4 |
| Q27 | Versionado | — |
| Q28 | Procedencia de los releases | G2 |
| Q29 | Titular | — |
| Q30 | Finales de línea del repo | — |
| Q31 | Criterio de éxito | J5 |

J2 la cierra el contexto fijo de DECISIONES.md, no una decisión numerada. Ninguna decisión cierra
B5, B7, B8, B9, C4, C5, D3, E4, E5, F3, F7, I1, I4, I5 ni J4: son las abiertas de la sección
«Estado».

# Plan de trabajo

Hoja de ruta de Amate por hitos, con compuertas. Deliberadamente **sin estimaciones de
calendario**: un hito se cierra cuando cumple su criterio de salida, no en una fecha. Lo que sí
dice este plan es el tamaño relativo de cada hito, de qué depende, qué partes del workspace toca,
qué entrega y qué riesgos concretos arrastra, con su mitigación.

El plan sale de [DECISIONES.md](DECISIONES.md), que es la fuente de verdad; cada decisión se cita
por su identificador, así: [Q09]. Todas tienen respuesta salvo [Q12], que aplica su recomendación,
`familiar`, como supuesto. Si cambia una respuesta, el plan se rehace desde el hito afectado: el
apartado «Si cambias» de cada decisión dice qué se mueve. Este documento es insumo para la corrida
de [PROMPT.md](PROMPT.md): el agente lo verifica y lo amplía, no lo rehace, y su entrega,
`docs/05-plan-ejecutable.md`, baja los hitos 0 y 1 a tareas del tamaño de una sesión.

El contexto es fijo: una persona con experiencia sólida en Rust trabaja con un agente de código.
Los criterios humanos de salida los cumplen el mantenedor (y probadores tempranos), no un equipo.

El criterio de éxito es la adopción externa ([Q31]): Amate salió bien si gente de fuera lo
descarga, abre issues y contribuye. Cuando haya que priorizar o recortar, gana lo que decide la
primera impresión de quien llega de fuera: la distribución del hito 4, la firma con SignPath
([Q10]), el README y el manual bilingües ([Q08]) y CONTRIBUTING.

Cada dato fechado lleva «(verificado: 2026-10-15)» o «(a verificar)»; la lista de hechos por
re-verificar está en [fusion.md](fusion.md). Lo que este plan llama «hueco» es algo que ninguna
decisión cierra; está, con su sugerencia, en la sección «Huecos que el plan debe cerrar» de
[PROMPT.md](PROMPT.md), y lo cierra la corrida del prompt.

Los crates y las herramientas que este plan nombra son candidatos, no elecciones: cada uno entra
con al menos tres candidatos y una comparación de seguridad (estado de mantenimiento, avisos
conocidos, adopción, número de dependencias transitivas y licencia), y solo cuando el mantenedor lo
aprueba. Además tiene que cumplir [Q02] (licencia compatible con GPLv3) y [Q19] (nada de C ni C++
compilado dentro del binario).

La numeración por fases de la versión anterior de este plan y su equivalencia con los hitos están
en [fusion.md](fusion.md).

---

## Mapa general

| Hito | Versión | Tamaño | Compuerta (resumen del criterio de salida) |
| --- | --- | --- | --- |
| Hito 0 — Cimientos y validación medida | — | Mediano | CI verde en los tres sistemas, ADR con números y decisión GO/PIVOT, y un artefacto basto descargable desde el CI. |
| Hito 1 — Primera versión usable | 0.1.0 | Grande, el mayor del plan | El corpus hostil pasa entero, los presupuestos de [Q25] corren en CI, la etiqueta produce los paquetes de [Q09] sin firma, y el mantenedor usa Amate dos semanas como editor principal. |
| Hito 2 — Flujo diario | 0.x | Grande | La lista firmada de `diario` está entregada, y un usuario habitual de Notepad++ trabaja un día completo sin volver atrás, comprobado con usuarios reales. |
| Hito 3 — Manual y preferencias | 0.x | Mediano | Preferencias y temas editables, interfaz y manual completos en español e inglés, y el costo de importar la configuración de Notepad++ escrito. |
| Hito 4 — Distribución y primera versión pública | 0.N | Mediano en código, grande en fricción externa | Un `git tag` produce todo sin pasos manuales, y los paquetes se instalan y se verifican en máquinas limpias. |
| Hito 4b — Opcional, después del hito 4 | — | Pequeño a mediano por canal | Cada canal instala Amate en una máquina limpia desde el mismo flujo de release. No bloquea el cierre del hito 4. |
| 1.0 | 1.0 | — | Compuerta, no hito: `diario` completo, firma en Windows, notarización en macOS y manual. |
| Hito 5 — Extensiones | — | Anexo | Con [Q06] en `despues`, un anexo de una página; nunca trabajo programado. |

Los hitos se cierran en este orden, y cada uno termina en algo que se ejecuta o se instala: un
hito que solo prepara la arquitectura no cuenta. Un hito se puede partir en dos si cada mitad sigue
terminando en algo demostrable. El trabajo sí se solapa: dentro del hito 1, el núcleo va primero y
la aplicación se construye encima; el hito 3 avanza en paralelo al 2 en lo que no depende de
`diario`; y la distribución no espera al hito 4: arranca en el hito 0 con paquetes bastos, sigue en
el hito 1 con los paquetes de [Q09] generados en CI, y el hito 4 la termina. Qué se distribuye en
cada momento está en [04-distribucion-y-empaquetado.md](04-distribucion-y-empaquetado.md).

### Versiones

SemVer, con 0.x hasta firmar ([Q27]). Las etiquetas siguen el formato `vX.Y.Z`.

| Momento | Versión | Qué se publica |
| --- | --- | --- |
| Hito 1 | 0.1.0 | La etiqueta `v0.1.0` y los paquetes que produce esa corrida del CI, sin firma. No es un release de GitHub y no se anuncia ([Q27]). |
| Hitos 2 y 3 | 0.x | Etiquetas intermedias, si hacen falta, con la misma regla que la 0.1.0. |
| Hito 4 | 0.N | El primer release de GitHub: la primera versión pública, la que se anuncia ([Q27]), con `SHA256SUMS.txt` y la atestación de procedencia ([Q28]). |
| 1.0 | 1.0 | No es un hito sino una compuerta (sección «La 1.0»). |

La 0.1.0 no es un release de GitHub porque [Q09] hace de GitHub Releases el canal único, [Q28] pide
`SHA256SUMS.txt` y atestación en cada release, y [Q27] llama primera versión pública a la del
hito 4. Antes del hito 4 no hay releases: quien prueba antes descarga los artefactos de la corrida
del CI de la etiqueta. Eso también cuida la primera impresión ([Q31]): lo primero que alguien
encuentra en los releases del repositorio es una versión con descargas, sumas e instrucciones.

### Cómo se leen los criterios de salida

- Cada hito tiene criterios objetivos que se comprueban con un comando, un archivo de prueba o un
  paquete instalable.
- Los criterios humanos (dos semanas de uso diario, un día de trabajo con usuarios reales) son
  listas de comprobación que se suman a los objetivos. Nunca son la única salida.
- La definición de «terminado» por función es la pregunta I1 de
  [02-cuestionario.md](02-cuestionario.md), todavía abierta. Su recomendación (compila sin avisos,
  tiene pruebas en el núcleo, tiene una prueba de interfaz si toca la interfaz, funciona en los
  tres sistemas en CI, se usa solo con teclado y está documentada) se escribe en CONTRIBUTING en el
  hito 4.
- Si después del GO del hito 0 el toolkit falla en una plataforma (por ejemplo, el IME en Wayland o
  el lector de pantalla en macOS), el hito en curso no cierra con esa plataforma rota: [Q09]
  promete las tres. Primero se intenta corregir en el proyecto del toolkit; si no se puede, se
  reabre su ADR con las mediciones del segundo candidato, que el hito 0 deja escritas. La capa fina
  entre el núcleo y la interfaz acota el costo del cambio.

---

## Hito 0 — Cimientos y validación medida

**Tamaño:** mediano. **Depende de:** [DECISIONES.md](DECISIONES.md), ya cerrado. **Toca:** el
esqueleto del workspace, el CI, `xtask` y las ramas `spike/`; ningún código de producto más allá
de una ventana vacía.

**Objetivo:** un workspace de Cargo que compila y se prueba en Windows, Linux y macOS, con una
ventana vacía empaquetada, y las elecciones que después no se revierten decididas con mediciones,
no con documentos ([Q24]).

El toolkit, la estructura del documento y el resaltado son elecciones que en la práctica no se
revierten, y ningún documento las resuelve con la confiabilidad de un binario que se ejecuta. Por
eso [Q24] mete la validación dentro de este hito: no hay un hito de investigación aparte.

### Antes de hacer público el repositorio

1. **Despejar el nombre** ([Q01]). Antes de hacer público el repositorio, publicar cualquier
   artefacto o reservar el crate, se verifica la marca «Amate» en IMPI, USPTO y EUIPO, y el
   dominio. El 2026-10-15, crates.io, Debian, Homebrew, PyPI, npm, AUR y la web estaban libres, y
   el usuario `amate` de GitHub lo ocupaba una cuenta ajena activa (verificado: 2026-10-15); la org
   propuesta es `amate-editor` (a verificar). La org y el dominio van a nombre del titular ([Q29]).
2. **Permiso del empleador.** Si se usaron recursos u horario de Gometrix, se consigue permiso por
   escrito antes de publicar ([Q29]).
3. **Lo que lleva el primer commit público:** LICENSE y los README en inglés y en español ([Q15]),
   que ya existen. Desde el primer commit de código, cada commit lleva `Signed-off-by`, y el
   mantenedor revisa y firma cada commit escrito con ayuda de un agente ([Q22]).

### Trabajo

1. **Esqueleto del workspace.** Propuesta de partida; la corrida del prompt fija los límites y los
   nombres:

   ```text
   crates/
     core/     documento (rope), selecciones, deshacer, búsqueda, codificaciones en memoria
               y comandos. Sin interfaz y sin std::fs: se prueba entero en memoria.
     io/       abrir y guardar, escritura atómica, detección de cambios externos y recorrido
               de carpetas.
     syntax/   resaltado con syntect y plegado.
     config/   esquema propio en TOML, rutas de cada sistema y modo portable.
     app/      la aplicación de escritorio; produce el binario `amate`.
   xtask/      empaquetado, metadatos de plataforma y tareas de release.
   ```

   El acceso al disco vive en `io` y no en `core` para que el núcleo se pruebe sin disco. No hay
   crate de CLI reservado: una CLI sin interfaz es la pregunta B9, abierta en
   [02-cuestionario.md](02-cuestionario.md). Tampoco hay crate de plugins ([Q06]).
2. **CI desde el primer código**, en cada pull request, con Windows x64, Linux x64 y macOS arm64:
   `cargo fmt --check`, `clippy -D warnings`, las pruebas, `cargo-deny` (licencias compatibles con
   GPLv3 y avisos de seguridad, [Q02]) y la regla de [Q19]: el CI falla si el árbol de dependencias
   del binario incluye `cc`, `cmake` o `bindgen`. Las herramientas que no viajan en el binario
   (valgrind, libFuzzer) quedan fuera de esa regla. El CI de los pull requests nunca ve secretos de
   firma.
3. **`SECURITY.md`**, con el reporte privado de vulnerabilidades de GitHub ([Q15]).
4. **Una ventana vacía empaquetada de forma basta** en cada sistema y descargable desde el CI;
   todavía no son el MSI, el DMG ni el `.deb` finales ([Q24]). Linux compila desde aquí en un
   contenedor `debian:bookworm` ([Q09]).
5. **Spikes del toolkit**, en ramas `spike/`. El candidato que recomienda la corrida del prompt,
   entre los que pueden cumplir [Q16], [Q19] y [Q20] (la lista de candidatos y de descartados está
   en [PROMPT.md](PROMPT.md)), construye un prototipo desechable que hace esto y nada más; si
   falla, el siguiente candidato construye el suyo:
   - abrir un archivo, editarlo y guardarlo;
   - varias pestañas;
   - buscar y reemplazar con expresiones regulares (`regex` y `fancy-regex`, [Q19]);
   - resaltar con `syntect` un lenguaje sencillo y uno complejo, por ejemplo JSON y C++;
   - abrir un archivo de más de 10 MB y comprobar el modo degradado de [Q11]: barra de progreso,
     sin resaltado ni ajuste de línea arriba del umbral, y la interfaz nunca se congela;
   - compilar, ejecutarse y empaquetarse desde el CI en Windows, Debian y macOS.

   Y responde con mediciones, no con opiniones. [Q24] exige las seis primeras filas; las demás
   vienen de [Q19], [Q25] y [Q26] y de los riesgos de este plan:

   | Pregunta | Cómo se mide |
   | --- | --- |
   | ¿El toolkit aguanta un campo de texto serio? | Latencia de tecla medida, no percibida, contra el presupuesto de [Q25]: menos de 16 ms al p99. |
   | ¿Funciona el IME (japonés, chino y coreano)? | Prueba manual en los tres sistemas. Aquí fallan varios toolkits de Rust. |
   | ¿El lector de pantalla ve la aplicación? | NVDA en Windows, VoiceOver en macOS y Orca en Linux ([Q20]). |
   | ¿Cuánto consume en reposo? | CPU y RAM con la ventana quieta, contra el presupuesto de [Q25]. Importa mucho en los toolkits de modo inmediato. |
   | ¿Cuánto cuesta empaquetar? | Tamaño del binario y trabajo para producir el paquete basto en cada sistema. |
   | ¿Sale el binario universal de macOS? | La mitad x86_64 compilada de forma cruzada en un runner arm64 y unida con `lipo` ([04-distribucion-y-empaquetado.md](04-distribucion-y-empaquetado.md)). |
   | ¿El texto se dibuja bien? | Respaldo de fuentes, ligaduras, texto bidireccional, ancho de caracteres CJK y emojis que no rompen la rejilla. Es donde los toolkits de Rust más decepcionan en casos reales. |
   | ¿Pasa la regla de [Q19]? | Ni `cc`, ni `cmake`, ni `bindgen` en el árbol de dependencias del candidato. |
   | ¿Qué cubre `syntect`? | Cuáles de los 13 lenguajes del hito 1 y de los 32 de la primera versión pública trae su juego de sintaxis (a verificar), y cuánto tarda en resaltar un archivo de 10 MB ([Q26]). |
   | ¿Cuánto tarda abrir 10 MB? | Fija el umbral de [Q25] para abrir un archivo de 10 MB. |
   | ¿Cuánto tarda una compilación limpia? | Determina la velocidad de iteración durante años. |

6. **Un ADR por cada elección técnica que DECISIONES.md deja al plan:** toolkit; estructura del
   documento (rope o piece table; `ropey`, `crop` o una propia); cómo se conectan `regex` y
   `fancy-regex` con el documento; detección de codificación; formato y rutas de configuración; y
   herramienta de empaquetado. Formato corto: contexto, decisión, alternativas descartadas con su
   motivo, consecuencias y qué pasa si la elección resulta equivocada. Viven en `docs/adr/`. El ADR
   del toolkit nace con estado «propuesta» y pasa a «aceptada» con el GO. Las ramas `spike/` se
   borran al terminar; lo que queda es el ADR con sus números, no el código ([Q24]).
7. **Regla contra la parálisis.** Si una elección técnica no se resuelve con argumentos en una
   sesión, pasa a un spike y la resuelve una medición. Si dos candidatos quedan empatados, se miden
   los dos; si el empate sigue, se vuelve una sola pregunta para el mantenedor.

### Criterio de salida

- CI verde en Windows, Linux y macOS: formato, `clippy`, pruebas, `cargo-deny` y la regla de [Q19].
- `SECURITY.md` publicado, con el reporte privado de vulnerabilidades activado ([Q15]).
- Un artefacto basto por sistema, descargable desde el CI.
- ADR en `docs/adr/` con las mediciones delante y una decisión GO/PIVOT sobre el toolkit. Si el
  toolkit falla el IME o la accesibilidad, se repite con el siguiente candidato ([Q24]). **Eso es
  un éxito del hito, no un fracaso del proyecto.**
- El umbral para abrir un archivo de 10 MB, fijado con mediciones ([Q25]).

### Riesgos y mitigación

- **Que el spike se vuelva el producto.** Nace en una rama `spike/` y se borra al terminar. Lo que
  sobrevive es el ADR, no el código.
- **Parálisis por análisis.** La regla del punto 7.
- **Ningún candidato pasa.** Sin candidato no hay GO: el hito no cierra y la decisión vuelve al
  mantenedor, el único que puede cambiar [Q16], [Q19] o [Q20].
- **El nombre no se despeja.** Si la marca o el dominio chocan, cambian el comando, los
  identificadores de paquete y la org ([Q01]). Por eso se verifica antes de cualquier artefacto
  público, y los identificadores estables (bundle ID de macOS, UpgradeCode del MSI) se fijan
  después.
- **El binario universal de macOS.** Los runners hospedados de macOS con Intel están en retiro (a
  verificar), así que la mitad x86_64 se compila de forma cruzada. Cómo se prueba esa mitad sigue
  abierto ([04-distribucion-y-empaquetado.md](04-distribucion-y-empaquetado.md)).

---

## Hito 1 — Primera versión usable (0.1.0)

**Tamaño:** grande, el mayor del plan: junta el núcleo y la aplicación mínima. **Depende de:** el GO
del hito 0 y sus ADR. **Toca:** `core`, `io`, `syntax`, `config`, `app`, `xtask` y el CI.

**Objetivo:** la primera versión con la que una persona edita código de verdad, y llegar cuanto
antes al punto en que el mantenedor la usa todos los días. El uso diario encuentra pronto lo que
una lista de requisitos no encuentra nunca.

El núcleo va primero. Es el crate que tiene que ser excelente, porque todo lo demás descansa encima
y porque es el único que se puede probar a fondo sin abrir una ventana. La aplicación se construye
encima, con trabajo solapado.

### Alcance

La lista canónica es la «Base del hito 1» de [PROMPT.md](PROMPT.md), más los añadidos de [Q04]. En
resumen:

- **Base:** crear, abrir, guardar, guardar como y cerrar pestañas, con indicador de modificado;
  varias pestañas en una ventana; deshacer y rehacer; cortar, copiar, pegar y seleccionar todo;
  buscar y reemplazar en el documento actual, en literal y con expresiones regulares; números de
  línea, resalte de la línea actual y ajuste de línea opcional; barra de estado con línea, columna,
  codificación, fin de línea y lenguaje; UTF-8 con y sin BOM y UTF-16 LE y BE al abrir, conservando
  la codificación al guardar; conversión explícita entre LF, CRLF y CR; tema claro y tema oscuro;
  los atajos de Notepad++ de esas acciones; y `amate archivo` y `amate archivo:línea`.
- **[Q04]:** coincidencia de paréntesis; espacios en blanco visibles; seguir el tema claro u oscuro
  del sistema; DPI alto; rutas de configuración resueltas al ejecutar, con modo portable (archivo
  marcador junto al ejecutable); y abrir y guardar en ANSI (Windows-1252 e ISO-8859-1).
- **Además:** teclado completo ([Q20]), textos fuera del código ([Q07]), los 13 lenguajes de
  [Q26], el documento en un rope ([Q11]), la capa de comandos serializable ([Q06]), un modelo de
  documento que ya admite varias vistas del mismo búfer, el mapeo de atajos de macOS ([Q23]) y los
  paquetes de [Q09] generados en CI, sin firma ([Q10]). Los presupuestos de [Q25] se miden en CI
  desde aquí.

Queda fuera, explícitamente: buscar en archivos, plegado y texto bidireccional, que siguen en el
hito 2 ([Q04]); la vista dividida, cuya interfaz llega en el hito 2 aunque el modelo la admita desde
aquí; y el resto de `diario`. Nada entra al hito 1 solo por estar en Notepad++. Un añadido nuevo
sigue el criterio con que [Q04] eligió los suyos (básicos, baratos y que evitan retrabajo), es una
función sin la cual Amate no se usaría, y lo aprueba el mantenedor.

### Núcleo: `core` e `io`

1. **Documento en un rope** ([Q11]), nunca en un solo `String`. Requisitos: inserción y borrado en
   O(log n), índice de líneas, conversión entre desplazamiento en bytes, desplazamiento en
   caracteres y columna visual, y soporte de clústeres de grafemas (emojis, texto combinado). Se
   empieza con `ropey`, que ya es muy bueno, y se cambia (por `crop`, una piece table o una
   estructura propia) solo si los bancos de prueba demuestran que es el cuello de botella.
2. **Archivos grandes** ([Q11]). Un archivo de 10 MB se abre, se desplaza y se edita con fluidez.
   Arriba de 10 MB entra el modo degradado: abre con barra de progreso y, arriba de un umbral
   configurable, sin resaltado ni ajuste de línea; la interfaz nunca se congela, pero no hay
   promesa de fluidez. Quedan fuera los archivos de varios GB y el modo hexadecimal. Si [Q11]
   cambiara a `grandes`, la carga diferida, `mmap` y el índice de líneas en segundo plano pasarían
   a este hito.
3. **Carga.** Detección de codificación (candidato: `chardetng`), decodificación (candidato:
   `encoding_rs`), BOM, UTF-16 LE y BE, Windows-1252 e ISO-8859-1 ([Q04]) y finales de línea
   mezclados. Nunca se guarda con pérdida sin confirmación: si el texto tiene caracteres que la
   codificación del archivo no puede representar, Amate lo dice antes de escribir. Si `encoding_rs`
   no codifica UTF-16 al guardar (a verificar), hace falta un codificador propio pequeño; eso y la
   codificación de respaldo en Linux y macOS, que no tienen página ANSI, son huecos que cierra el
   plan ([PROMPT.md](PROMPT.md)).
4. **Guardado.** Escritura atómica (archivo temporal más `rename`), conservando permisos y
   propietario; copia de seguridad opcional; detección de una modificación externa, al recuperar el
   foco y antes de guardar, con una pregunta antes de sobrescribirlo; y el caso incómodo de
   guardar en un archivo sin permisos: elevación en Windows y `pkexec` en Linux. El ayudante de
   elevación sigue abierto; la sugerencia es decidirlo en el hito 0 y llevarlo solo en los paquetes
   instalados, nunca en las variantes portables
   ([04-distribucion-y-empaquetado.md](04-distribucion-y-empaquetado.md)). Lo que el renombrado
   atómico rompe (enlaces simbólicos y duros, ACL de Windows, unidades de red) también está entre
   los huecos de [PROMPT.md](PROMPT.md).
5. **Deshacer y rehacer.** Agrupación por lotes, puntos de guardado y un modelo que funcione con
   selecciones múltiples. Aquí se decide si el historial es lineal o en árbol.
6. **Selecciones múltiples y modo columna** como concepto de primera clase del núcleo, no como un
   añadido de la interfaz: agregar el multicursor después es una de las reescrituras más dolorosas
   que existen. La edición en columna y el multicursor se entregan en el hito 2; el núcleo los
   admite desde aquí.
7. **Varias vistas del mismo búfer:** búfer y deshacer compartidos; cursor, desplazamiento y
   selección propios de cada vista. La vista dividida llega en el hito 2, pero diseñar esto tarde
   es caro.
8. **Búsqueda en el documento.** Literal (con Aho-Corasick) y con expresiones regulares: `regex`
   como ruta rápida y `fancy-regex` para retrorreferencias y *lookaround* ([Q19]). Insensible a
   mayúsculas con plegado Unicode correcto, hacia atrás y dentro de la selección. El motor es
   cancelable y no bloquea la interfaz, porque buscar en archivos lo reutiliza en el hito 2.
   Notepad++ usa Boost.Regex, con sintaxis de estilo Perl (a verificar): el plan escribe tres
   ejemplos de diferencias de dialecto que un usuario notaría. Cómo correr `fancy-regex`, que
   necesita un `&str` contiguo, sobre un rope es un hueco de [PROMPT.md](PROMPT.md).
9. **Capa de comandos** ([Q06]). Cada acción del usuario es un comando con nombre y serializable.
   Es lo que habilita macros, paleta de comandos, atajos configurables y pruebas automatizadas, y
   diseñarla tarde obliga a rehacer cada acción. Los atajos y las pruebas del hito 1 ya pasan por
   ella.
10. **Modelo de documento:** documentos sucios, archivos sin título y archivos bloqueados por otro
    programa. Se diseña para que la sesión del hito 2 pueda guardar y recuperar pestañas sin
    guardar.

### Aplicación: `app`, `syntax` y `config`

1. **Ventana, menú, barra de estado y pestañas**, con la disposición `familiar`: menús, pestañas
   arriba, barra de estado y atajos de Notepad++ en las acciones que existan, con iconos y marca
   propios ([Q12], supuesto).
2. **Vista del editor:** números de línea, márgenes, resalte de la línea actual, coincidencia de
   paréntesis y espacios en blanco visibles ([Q04]), y ajuste de línea opcional.
3. **Un dibujo del texto que funcione de verdad:** fuentes monoespaciadas con respaldo correcto,
   ancho de caracteres CJK y emojis que no rompen la rejilla. El texto bidireccional llega en el
   hito 2 ([Q04]). Este riesgo se midió en el hito 0; aquí solo se implementa.
4. **Buscar y reemplazar** en un diálogo, sobre el documento actual.
5. **Resaltado con `syntect`** y su backend `fancy-regex`, sin Oniguruma ([Q19]), para los 13
   lenguajes de [Q26], con repintado incremental limitado al área visible. Las definiciones
   `.sublime-syntax` que `syntect` no trae se suman con licencia compatible con GPL-3.0 y
   conservando sus avisos ([Q26]).
6. **Temas claro y oscuro**, y seguimiento del tema del sistema en los tres sistemas operativos
   ([Q04]).
7. **Configuración en TOML**, el formato recomendado en [Q05], en la ruta de cada sistema
   (`%APPDATA%`, `~/.config`, `~/Library/Application Support`) o, en modo portable, junto al
   ejecutable, cuando existe el archivo marcador ([Q04]). Ninguna ruta se codifica: todas se
   resuelven al ejecutar según el modo detectado. El nombre y el formato del marcador, y las rutas
   de estado y caché, son huecos. La pantalla de preferencias llega en el hito 3.
8. **DPI alto** y escalado fraccionario, verificados en monitores mixtos ([Q04]). El manifiesto de
   Windows y el resto de los metadatos de plataforma salen de plantillas en `xtask` (sugerencia de
   [04-distribucion-y-empaquetado.md](04-distribucion-y-empaquetado.md)).
9. **Teclado completo** ([Q20]): abrir, guardar, cerrar, cambiar de pestaña, buscar y moverse por
   el texto solo con teclado. Los atajos de Notepad++ de las acciones de este hito van en una tabla
   atajo → acción sacada del manual de Notepad++, no de memoria, y citada por enlace y capítulo
   ([Q21]), con su columna de macOS: Cmd donde Notepad++ usa Ctrl, respetando los atajos que
   reserva el sistema ([Q23]). El informe `VPAT-for-Npp` sirve para fijar ese mínimo de teclado.
   El lector de pantalla se verifica antes de la primera versión pública ([Q20]), pero cada widget
   lleva sus etiquetas de accesibilidad desde aquí.
10. **Textos fuera del código** ([Q07]), con Fluent (`fluent-rs`, recomendado). Desde el inicio se
    prueba que caben los textos en español, que son más largos.
11. **Línea de comandos:** `amate archivo` y `amate archivo:línea`. Qué pasa al abrir un archivo
    desde la terminal con Amate ya abierto (el modelo de proceso) y cómo se lee `archivo:línea` en
    Windows, donde la letra de unidad también lleva dos puntos, son huecos de
    [PROMPT.md](PROMPT.md).
12. **Paquetes de [Q09] en CI**, sin firma ([Q10]): MSI, ZIP portable, DMG universal, `.deb`,
    AppImage y `.tar.gz`, con Linux compilado en `debian:bookworm` ([Q09]). El `.deb` instala desde
    ya `.desktop`, iconos y tipos MIME ([Q14]). La etiqueta `v0.1.0` los produce como artefactos
    del CI.

### Pruebas y medición

- **Corpus hostil** desde el primer código (I5 de [02-cuestionario.md](02-cuestionario.md)): BOM,
  UTF-16 LE y BE, CP1252, Shift-JIS, finales de línea mezclados, líneas de 10 MB sin salto,
  archivos sin salto final, binarios, enlaces simbólicos, archivos de solo lectura, rutas con
  emojis y rutas de más de 260 caracteres en Windows. Cada fallo reportado se vuelve un archivo
  nuevo del corpus.
- **Pruebas de propiedades** con `proptest` sobre el búfer y las codificaciones.
- **Presupuestos de [Q25]** en CI: arranque en frío en menos de 300 ms, latencia de tecla en menos
  de 16 ms al p99, RAM en reposo con un archivo de 1 MB por debajo de 100 MB, y abrir un archivo de
  10 MB dentro del umbral que fijó el hito 0. En los runners compartidos se bloquea con conteo de
  instrucciones (Linux); los tiempos de reloj se reportan sin bloquear.
- **Pruebas de interfaz** con el arnés del toolkit elegido, en los tres sistemas.
- **Cero `unsafe`** en los crates propios (`#![forbid(unsafe_code)]`): sin `mmap` en este hito
  ([Q11]), ningún módulo lo necesita.
- **Fuzzing** de los decodificadores, del normalizador de finales de línea, de la configuración y
  de la búsqueda. Dónde y cuándo corre es un hueco de [PROMPT.md](PROMPT.md); libFuzzer queda
  fuera de la regla de [Q19] porque no viaja en el binario.

### Criterio de salida

Objetivo:

- El corpus hostil pasa entero: ningún archivo se corrompe al abrirlo y guardarlo sin cambios, y lo
  que Amate no sabe decodificar sin pérdida no se guarda en silencio.
- Las pruebas de propiedades del búfer pasan, y los presupuestos de [Q25] corren en CI y fallan
  ante una regresión.
- Cada acción de la base se hace solo con teclado en los tres sistemas, y la tabla de atajos tiene
  su columna de macOS.
- Los 13 lenguajes de [Q26] se resaltan, y los tres ejemplos de dialecto de las expresiones
  regulares están escritos.
- La etiqueta `v0.1.0` produce en el CI los paquetes de [Q09], sin firma.

Humano: el mantenedor usa Amate como editor principal durante dos semanas para las tareas básicas,
sin volver a Notepad++ ni a VS Code para ellas.

### Riesgos y mitigación

- **Sobreingeniería del búfer.** `ropey` primero; se cambia solo con bancos de prueba delante.
- **El dibujo del texto.** CJK, texto bidireccional y respaldo de fuentes son, con diferencia, donde
  los toolkits de Rust más decepcionan en casos reales. Mitigación: se midió en el hito 0, no aquí.
- **Que el hito crezca.** Cada añadido pasa el criterio de [Q04] y la aprobación del mantenedor;
  lo demás espera al hito 2 o a peticiones reales.
- **El dialecto de las expresiones regulares.** Los patrones que un usuario guardó en Notepad++ se
  escribieron para Boost.Regex. Los tres ejemplos y el manual dejan la diferencia a la vista.
  `fancy-regex` no garantiza tiempo lineal: lleva un límite de retroceso y se puede cancelar.
- **El toolkit falla en una plataforma después del GO.** La regla de «Cómo se leen los criterios
  de salida».

---

## Hito 2 — Flujo diario

**Tamaño:** grande. **Depende de:** hito 1. **Toca:** `core`, `io`, `syntax`, `config` y `app`.

**Objetivo:** entregar todo lo de `diario` ([Q03]) que no entró en el hito 1, para que quien viene
de Notepad++ trabaje un día completo en Amate.

Antes de empezar, la lista enumerada de `diario` (por familia de comandos, con su equivalente en el
menú de Notepad++) queda escrita y el mantenedor la firma. Contarla contra los 510 elementos del
menú principal de Notepad++ (verificado: 2026-10-15) es un hueco de [PROMPT.md](PROMPT.md).

### Trabajo

Lo que `diario` suma a `familiar` ([Q03]):

1. **Sesión con recuperación de pestañas sin guardar**, también tras un cierre inesperado, con
   instantáneas periódicas de los documentos sucios. Qué sobrevive a un cierre normal y dónde se
   guarda la sesión son huecos.
2. **Buscar en archivos**, con recorrido paralelo (candidatos: `ignore` y `rayon`), cancelable y
   con panel de resultados. `ignore` trae la semántica de `.gitignore` y de archivos ocultos, que
   Notepad++ no tiene. Los filtros, los binarios, el tamaño máximo y reemplazar en archivos, que
   puede perder datos, son huecos.
3. **Edición en columna y multicursor**, sobre el concepto del núcleo del hito 1.
4. **Operaciones de línea:** duplicar, mover, borrar, comentar, cambiar mayúsculas y minúsculas, y
   sangrar.
5. **Plegado.** Con `syntect` no sale gratis: su resaltado son expresiones regulares línea por
   línea, no un árbol sintáctico. Tree-sitter lo daba, pero [Q19] lo descarta porque cada gramática
   es C compilado dentro del binario. El plegado necesita un mecanismo propio, por sangría o por
   delimitadores, que elige el plan.
6. **Vista dividida con dos vistas del mismo búfer**, sobre el modelo del hito 1.
7. **Carpeta de trabajo.**
8. **Los 32 lenguajes de [Q26]:** los 13 del hito 1 más C#, Go, INI, Java, Kotlin, Lua, PHP, TOML,
   XML, YAML, Dockerfile, diff/patch, Batch, PowerShell, PL/SQL, T-SQL, Makefile, CMake y LaTeX.
   Las definiciones que `syntect` no trae se suman como `.sublime-syntax` con licencia compatible
   con GPL-3.0 y con sus avisos ([Q26]).
9. **Temas y atajos configurables.** Los perfiles de teclado intercambiables llegan después
   ([Q23]).
10. **Texto bidireccional** en el dibujo del texto ([Q04]).
11. **La base, pulida:** `familiar` es la base del hito 1 pulida, por ejemplo reordenar pestañas
    arrastrándolas y cerrarlas con el clic central.

Lo que la versión anterior de este plan sumaba al flujo diario y `diario` no nombra (lista de
recientes, ir a línea, marcadores, búsqueda incremental, marcar todas las coincidencias, zoom,
pantalla completa, convertir entre páginas de códigos, detectar el lenguaje por el contenido y la
paleta de comandos que habilita [Q06]) no es trabajo programado. Entra por petición real de
usuarios o como añadido que el mantenedor aprueba con un ADR. El mapa del documento y la lista de
funciones son de `completo`.

### Criterio de salida

Objetivo:

- Cada elemento de la lista firmada de `diario` tiene pruebas de aceptación que pasan en los tres
  sistemas.
- Los 32 lenguajes se resaltan, y los avisos de cada `.sublime-syntax` sumada viajan con los avisos
  de terceros ([Q02]).
- Buscar en archivos sobre una carpeta grande se cancela y nunca congela la interfaz.
- Los presupuestos de [Q25] siguen en verde.

Humano: un usuario habitual de Notepad++ trabaja un día completo sin encontrar un hueco que lo
obligue a volver atrás. Se comprueba con usuarios reales, no con la lista de arriba. Cómo se
consiguen esos usuarios sin telemetría es un hueco de [PROMPT.md](PROMPT.md).

**Control de cambios.** Si el día real encuentra un hueco, hay dos salidas y solo dos: entra al hito
con un ADR que firma el mantenedor, o va a una lista pública de huecos conocidos. El criterio se
cumple cuando el día no encuentra huecos fuera de esa lista. Sin esta regla, el criterio humano y
el alcance escrito se contradicen: o el hito no cierra nunca, o el alcance crece sin control.

### Riesgos y mitigación

- **Deriva de alcance hacia `completo`.** `diario` está escrito y firmado; todo lo demás va a una
  lista visible que se decide después de la 1.0, con peticiones reales ([Q03]).
- **Plegado sin árbol sintáctico.** El mecanismo se elige y se mide antes de empezar, sobre los 13
  lenguajes del hito 1.
- **Lenguajes sin definición.** Si alguno de los 32 no tiene una `.sublime-syntax` con licencia
  compatible, hay que escribirla. El hito 0 ya dejó la lista de los que faltan.
- **El toolkit falla en una plataforma.** La regla de «Cómo se leen los criterios de salida».

---

## Hito 3 — Manual y preferencias

**Tamaño:** mediano. **Depende de:** el hito 2 para el manual; las preferencias, la interfaz en dos
idiomas y el estimado de importación avanzan en paralelo al hito 2. **Toca:** `app`, `config`, los
archivos de Fluent y el manual.

**Objetivo:** que Amate se configure sin editar archivos a mano y que quien llega de fuera lo
entienda en su idioma.

### Trabajo

1. **Preferencias editables** desde la interfaz, sobre la configuración en TOML del hito 1 ([Q05]).
2. **Temas editables.** Un tema es un archivo de datos que se copia a una carpeta, no una extensión
   (recomendación de D3, abierta en [02-cuestionario.md](02-cuestionario.md)).
3. **Interfaz completa en español y en inglés** ([Q07]). Qué variante del español usa la interfaz
   es un hueco; la sugerencia es un español neutro.
4. **Manual de usuario en español y en inglés** ([Q08]), centrado en «vengo de Notepad++, ¿dónde
   está X?», que nombra los menús en el idioma de quien lee. Cuando habla de Notepad++ lo hace con
   sus propias palabras y cita `npp-usermanual` por enlace y capítulo; nunca copia su texto
   ([Q21]). El formato y el alojamiento los fija el plan. Si se publica como sitio web, lleva
   metadatos Open Graph con URL absolutas, que dependen del dominio de [Q01].
5. **El costo estimado de importar la configuración de Notepad++**, escrito como trabajo no
   incluido ([Q05]). Cubre temas, UDL, atajos y sesión, y parte de lo que la versión anterior de
   este plan proponía construir:
   - lectores para `config.xml`, `stylers.xml`, `shortcuts.xml`, `session.xml`,
     `userDefineLang*.xml` y `langs.xml`;
   - la traducción de cada ajuste al esquema propio, con un informe honesto de qué no se pudo
     importar y por qué: nada de fallos silenciosos;
   - un asistente al primer arranque que detecte una instalación de Notepad++ (ruta estándar, modo
     portable o carpeta indicada a mano) y que se pueda saltar;
   - un corpus de instalaciones reales con muchos años de uso, como pruebas de regresión;
   - el acoplamiento de cada importación con la función que la usa: los UDL necesitan un motor de
     lenguajes definidos por el usuario, que es de `completo`; las listas de palabras clave de
     `langs.xml`, una capa de palabras del usuario que `syntect` no trae; y los comandos externos
     de `shortcuts.xml`, la función de ejecutar comandos, que muestra cada línea de comando
     completa y nunca la ejecuta sin confirmación;
   - la regla legal: los archivos se leerían siempre del disco del usuario y nunca se
     redistribuyen. `nppThemes` es GPL-3.0 y `userDefinedLanguages` no declara licencia
     (verificado: 2026-10-15), y ningún archivo de Notepad++ viaja en los paquetes ([Q21]).

### Criterio de salida

- Cada preferencia se cambia desde la interfaz, se guarda en TOML y sobrevive a un reinicio,
  también en modo portable.
- Ningún texto de la interfaz vive en el código, y las traducciones al español y al inglés están
  completas.
- El manual existe en los dos idiomas y cubre cada comando de `diario`.
- El estimado de importación está escrito en el repositorio.
- El build del hito corre en los tres sistemas con todo lo anterior.

### Riesgos y mitigación

- **[Q08] duplica la documentación.** Cada capítulo se escribe en los dos idiomas en el mismo
  cambio, para que ninguno se quede atrás: nadie que use la interfaz en inglés se queda sin manual
  ([Q31]).
- **Los textos en español, más largos, rompen la disposición.** Se prueban desde el hito 1 ([Q07]).
- **Copiar el manual de Notepad++ por comodidad.** La regla de [Q21]: se parafrasea y se cita.

---

## Hito 4 — Distribución y primera versión pública (0.N)

**Tamaño:** mediano en código, grande en fricción externa. **Depende de:** los hitos 1 a 3; el
pipeline viene creciendo desde el hito 0. **Toca:** `xtask`, los flujos de CI y de release, el
README y los archivos de comunidad.

**Objetivo:** que una persona de fuera descargue, verifique, instale y abra Amate en su sistema, y
que publicar una versión sea empujar una etiqueta. Es el hito que más pesa en la primera impresión
([Q31]).

### Trabajo

1. **Pipeline de release desde una etiqueta**, con `dist` como candidato (su versión vigente y su
   mantenimiento, a verificar). El diseño, la herramienta de cada artefacto y el diagrama del
   pipeline están en [04-distribucion-y-empaquetado.md](04-distribucion-y-empaquetado.md): imágenes
   fijas, nunca `-latest`; Linux en `debian:bookworm`; y pruebas de instalación en contenedores
   limpios de Debian 12, Debian 13 y Ubuntu 24.04, nunca en el runner del CI. Cualquier paso manual
   se vuelve, antes o después, un release roto o un release que no se hace.
2. **Artefactos finales de [Q09]:** MSI y ZIP portable de Windows x64, DMG universal, `.deb` amd64,
   AppImage y `.tar.gz` x86_64. Los identificadores estables (bundle ID de macOS, UpgradeCode del
   MSI) se derivan del dominio verificado ([Q01]) y se fijan antes del primer artefacto público.
3. **`SHA256SUMS.txt`**, en el formato que acepta `sha256sum -c`, y la **atestación de
   procedencia** de `actions/attest-build-provenance`, que se verifica con `gh attestation verify`
   ([Q28]). No sustituye la firma de código.
4. **Sin actualizador** ([Q13]). La versión nueva se obtiene descargando el release nuevo de
   GitHub Releases. Amate no hace ninguna conexión de red: ni comprobación de versión ni
   telemetría. WinGup, el actualizador de Notepad++, no se porta.
5. **Modo portable real.** El ZIP trae el archivo marcador, es la única variante que se anuncia
   como portable y no escribe nada fuera de su carpeta: ni registro de Windows, ni asociaciones de
   archivo, ni menú Inicio, ni perfil del usuario.
6. **Integración con el escritorio** ([Q14]): registro en «Abrir con» del MSI y del DMG; el `.deb`
   la trae desde el hito 1. Amate nunca se apropia de las asociaciones por defecto. El verbo
   «Editar con Amate» del menú contextual de Windows 11 exige un paquete MSIX firmado y queda para
   después de la 1.0.
7. **Firma** ([Q10]). Al abrir el hito, no al cerrarlo, se solicita la firma gratuita para código
   abierto de SignPath Foundation, cuyos requisitos hay que revisar (a verificar). Si la aprueban,
   el MSI y el ZIP salen firmados. Construir y firmar son pasos distintos, y la firma usa secretos
   que no viven en el repositorio ni en el CI público. macOS sale sin firma ni notarización.
8. **Sección de descargas del README**, en inglés y en español ([Q08]): una descarga recomendada
   por sistema y las variantes plegadas, cada una con una descripción de una línea en lenguaje
   llano; la versión actual con sus notas; `SHA256SUMS.txt` con su instrucción de verificación; la
   vía «ejecútalo hoy desde el código», con la MSRV declarada y lo que hay que instalar antes en
   cada sistema; la base `debian:bookworm` de los binarios de Linux; y los pasos para abrir builds
   sin firma: Gatekeeper en macOS, SmartScreen en Windows, y el aviso sin rodeos de que, con Smart
   App Control activo, Windows 11 no deja abrir un binario sin firma (a verificar). El README lleva
   capturas reales y la lista pública de huecos conocidos del hito 2. El detalle está en la sección
   7 de [04-distribucion-y-empaquetado.md](04-distribucion-y-empaquetado.md).
9. **Lector de pantalla verificado** con NVDA, VoiceOver y Orca antes de publicar ([Q20]).
10. **Comunidad** ([Q15], [Q22]): `CONTRIBUTING.md` con DCO (`Signed-off-by`, también para el
    código asistido por un agente), la definición de «terminado» por función y la guía de
    compilación por sistema; código de conducta; plantillas de issues; etiquetas de buen primer
    issue; y una respuesta rápida a las primeras contribuciones, que es de lo que más depende que un
    proyecto de código abierto arranque. `GOVERNANCE.md` llega cuando haya tres o más contribuyentes
    recurrentes ([Q15]).
11. **Avisos de terceros** generados por el CI dentro de cada artefacto ([Q02]).
12. **Anuncio.** La 0.N es la primera versión que se anuncia ([Q27]): r/rust, r/notepadplusplus,
    Hacker News, Lobsters y Mastodon. El mensaje es honesto sobre lo que Amate todavía no hace (sin
    plugins, sin importar la configuración de Notepad++, macOS sin notarizar y, si SignPath no
    aprobó, Windows sin firma): eso es lo que distingue un lanzamiento creíble de uno que
    decepciona. La marca Notepad++ solo aparece en texto descriptivo y con el aviso de [Q21]. Desde
    aquí, la adopción se mide con datos públicos de GitHub: descargas por release, issues, pull
    requests y estrellas ([Q31]).

### Criterio de salida

- Un `git tag` produce todo sin pasos manuales: compila, firma si hay firma, empaqueta, genera
  `SHA256SUMS.txt` y la atestación, y publica el release de GitHub con sus notas.
- Los paquetes se instalan y se verifican en máquinas limpias: `sha256sum -c` y
  `gh attestation verify` pasan, y cada paquete abre un archivo en Windows 11, en macOS y en los
  contenedores limpios de Linux.
- El lector de pantalla está verificado en los tres sistemas ([Q20]).
- El README en los dos idiomas, CONTRIBUTING, el código de conducta y las plantillas de issues
  están publicados.

**La aprobación de SignPath.** Si su política exige aprobar a mano cada solicitud de firma (a
verificar), esa aprobación choca con «sin pasos manuales». Se resuelve así: primero se busca que
SignPath apruebe sola la firma que pide el flujo de release de una etiqueta (a verificar). Si no se
puede, esa aprobación es la única intervención humana admitida, porque pertenece a la firma, que
[Q10] separa de la construcción. El flujo espera la firma en un entorno protegido y después sigue
solo: sumas, atestación y publicación. Las sumas se calculan después de firmar, porque la firma
cambia el binario.

### Riesgos y mitigación

- **SignPath no aprueba a tiempo, o no aprueba.** La solicitud sale al abrir el hito. Si al cerrar
  no hay aprobación, la 0.N sale sin firma, el README lo explica y la firma de pago pasa al hito 4b.
- **Firma y notarización dan fallos opacos.** Son procesos burocráticos; se ensayan con un binario
  trivial en cuanto existe la cuenta, antes de que bloqueen un release.
- **glibc.** Un binario compilado contra una glibc más nueva no arranca en una más vieja, y el fallo
  aparece en la máquina del usuario, no en la del mantenedor. Mitigación: contenedor
  `debian:bookworm` y pruebas en contenedores limpios. Enlazar con musl solo entra si el hito 0
  demostró que funciona.
- **`dist` no cubre todo**, por ejemplo si sus sumas salen con otro nombre o formato (a verificar).
  Lo que falte va en `xtask`.
- **Falsos positivos de antivirus** con los builds de Windows sin firma. Es un hueco; la sugerencia
  es reportarlos a los fabricantes.

---

## Hito 4b — Opcional, después del hito 4

**Tamaño:** pequeño a mediano por canal; la notarización es lo que más fallos opacos da.
**Depende de:** hito 4. **Toca:** la etapa de publicación del flujo de release y `xtask`.

**Objetivo:** sumar canales y arquitecturas sin mezclarlos con el cierre del hito 4 ([Q09],
[Q10]).

| Trabajo | Notas |
| --- | --- |
| Flatpak en Flathub | Con `flatpak-builder`. Exige resolver bien los portales XDG; abrir los diálogos de archivo por portales, como recomienda F3 en [02-cuestionario.md](02-cuestionario.md), lo abarata. |
| `.deb` arm64 | Raspberry Pi y servidores ARM. |
| Windows ARM64 | MSI y ZIP portable: el portable existe en cada arquitectura de Windows que se publica. |
| Repositorio apt propio | `aptly` o `reprepro`, firmado con GPG; la custodia de la llave es un hueco. Con él entran al README las instrucciones de `apt`. |
| winget | `wingetcreate`, en un paso posterior al release. |
| Cask de Homebrew | Va después de la notarización: Homebrew ha ido desactivando los casks que no pasan Gatekeeper (a verificar). |
| Firma de pago en Windows | Solo si hace falta, por ejemplo si SignPath no aprueba: certificado OV, certificado EV o Azure Trusted Signing (a verificar). No todas las opciones aceptan necesariamente a una persona física, y el titular lo es ([Q29]; a verificar). |
| Notarización de macOS | Apple Developer Program, 99 USD al año (a verificar); firma con Developer ID, envío con `notarytool` y *staple* del ticket con `stapler`. |

**Criterio de salida, por canal:** Amate se instala desde ese canal en una máquina limpia, y el
canal se alimenta del mismo flujo de release, sin pasos manuales.

**Prioridad** ([Q31]): winget, Homebrew y apt son los primeros candidatos a adelantarse si las
descargas lo piden. La firma de pago y la notarización son opcionales para cerrar el hito 4, pero no
para llegar a la 1.0.

---

## La 1.0: compuerta, no hito

La 1.0 no tiene trabajo propio. Es la versión que se etiqueta cuando se cumplen tres condiciones
([Q27]):

1. `diario` completo: cada elemento de la lista firmada del hito 2 está entregado.
2. Firma en Windows y notarización en macOS ([Q10]): SignPath o la firma de pago, y la
   notarización del hito 4b.
3. El manual del hito 3.

Hasta entonces, cada versión es una 0.x. Quedan fuera de la 1.0 el sistema de plugins ([Q06]) y el
verbo del menú contextual de Windows 11 ([Q14]); los importadores de Notepad++ no están en la hoja
de ruta ([Q05]).

---

## Hito 5 — Extensiones (anexo)

Con [Q06] en `despues`, el hito 5 no es trabajo programado: es este anexo de una página. La regla
de que cada hito termina en algo que se ejecuta no aplica, porque no se programa nada. Si [Q06]
cambiara para tener plugins antes de la 1.0, el hito 5 se volvería un hito de verdad, antes de la
1.0, que termina en un anfitrión de extensiones que se ejecuta.

Cómo se agregarían las extensiones:

- **Lo que ya existe.** La capa de comandos con nombre y serializable, desde el hito 1 ([Q06]):
  deja la puerta abierta sin comprometer todavía una interfaz pública.
- **Candidatos:** Rhai, o WebAssembly con `wasmi`, ambos en Rust puro ([Q06]). Primero la
  automatización del usuario con un lenguaje de scripts; un anfitrión de WebAssembly, solo si el
  scripting resulta insuficiente.
- **Descartados:** Lua, porque `mlua` compila C ([Q19]); la carga dinámica nativa (`.dll`, `.so`,
  `.dylib`), porque en Rust su ABI es frágil, no tiene sandbox y multiplica el costo de empaquetado
  (D1 en [02-cuestionario.md](02-cuestionario.md)); y los plugins DLL de Notepad++, que nunca son
  objetivo de compatibilidad ([Q06]).
- **Después, y solo con demanda:** un catálogo instalable dentro de la aplicación, cuando haya al
  menos cinco plugins reales que lo justifiquen (recomendación de D2 en
  [02-cuestionario.md](02-cuestionario.md)); y la integración con LSP, como extensión desactivada
  por defecto (B5, abierta).
- **Condición:** después de la 1.0 y por peticiones reales.

---

## Después de la 1.0

Iterativo y guiado por peticiones reales de usuarios, nunca por la lista de 510 comandos ([Q03]).
Candidatos, en orden tentativo de valor:

1. Macros grabables y reproducibles (`completo`). La capa de comandos ya lo permite.
2. Editor de columnas y las operaciones de línea que falten, como las variantes de ordenar líneas.
3. Extensiones, según el anexo del hito 5.
4. Explorador de archivos y panel de proyectos, más allá de la carpeta de trabajo.
5. Comparar documentos.
6. Ejecutar comandos externos sobre el documento actual.
7. Lenguajes definidos por el usuario (`completo`), sin importar la colección comunitaria ([Q05],
   [Q21]).
8. El resto de `completo`: mapa del documento, lista de funciones, impresión y autocompletado.
9. Una CLI sin interfaz que reutilice el núcleo (B9, abierta).
10. El verbo «Editar con Amate» del menú contextual de Windows 11, con un paquete MSIX firmado
    ([Q14]), y los perfiles de teclado intercambiables ([Q23]).

---

## Riesgos del proyecto, por orden de probabilidad

| Riesgo | Probabilidad | Impacto | Mitigación |
| --- | --- | --- | --- |
| **Deriva de alcance** hacia los 510 elementos del menú de Notepad++ (verificado: 2026-10-15) | Alta | Mortal: es la forma estándar en que mueren los clones de software maduro. | Niveles de [Q03] escritos; lista de `diario` firmada antes del hito 2; control de cambios con ADR o lista pública de huecos; `completo` y la larga cola, solo por peticiones reales. |
| **El toolkit no da la talla** en accesibilidad, IME o dibujo del texto | Alta | Reescritura de la capa de interfaz. | Mediciones del hito 0 ([Q20], [Q24]); PIVOT al siguiente candidato; capa fina entre el núcleo y la interfaz; las mediciones del segundo candidato quedan en el ADR. |
| **Cobertura y rendimiento de `syntect`** | Alta | Lenguajes de [Q26] sin resaltar, resaltado lento en archivos grandes, y plegado que no sale de un árbol sintáctico. | El hito 0 mide cobertura y tiempos; definiciones `.sublime-syntax` con licencia compatible y sus avisos ([Q26]); resaltado limitado al área visible y apagado arriba del umbral de [Q11]. |
| **Smart App Control bloquea los builds sin firma** (a verificar) | Alta mientras no haya firma | Quien lo tenga activo no puede abrir Amate: la peor primera impresión ([Q31]). | Solicitud a SignPath al abrir el hito 4 ([Q10]); el README lo dice sin rodeos; firma de pago en el hito 4b si hace falta; la 1.0 exige firma. |
| **Fin del soporte de Windows 10**, que ya no recibe parches de consumo (a verificar) | Alta | Una promesa sobre un sistema sin parches; el toolkit o Rust pueden dejar de soportarlo. | Se sigue prometiendo, con las pruebas principales en Windows 11 ([Q09]); la fecha se vuelve a comprobar en cada corrida; si el toolkit exige algo más nuevo, el mínimo sube con justificación. |
| **Factor autobús**: un solo mantenedor | Media | El proyecto se detiene. | Decisiones en DECISIONES.md y en ADR, no en la cabeza de nadie; release sin pasos manuales; CONTRIBUTING desde el hito 4; `GOVERNANCE.md` con tres o más contribuyentes recurrentes ([Q15]); la custodia de las credenciales de firma, documentada (hueco). |
| **Fragmentación de Linux**: glibc, Wayland y X11, escritorios y portales | Media | Fallos imposibles de reproducir. | `debian:bookworm` y contenedores limpios ([Q09]); referencia de Wayland y X11 según F3, abierta. |
| **Regresiones de rendimiento** silenciosas | Media | Se pierde lo que promete la propuesta de valor: arrancar al instante ([Q17]). | Presupuestos de [Q25] en CI desde el hito 1, con fallo ante una regresión. |
| **Sin ecosistema de plugins** frente a Notepad++ | Media | Quien depende de un plugin concreto no migra. | Se acepta ([Q06]) y el README lo dice; lo que más se pida se integra por peticiones reales. |
| **El nombre no se despeja** | Media | Cambian el comando, los identificadores de paquete y la org ([Q01]). | Verificar marca y dominio antes del repositorio público, de cualquier artefacto y del crate; los identificadores estables se fijan después. |
| **Contaminación de licencia** al leer el código de Notepad++ | Baja | Reescritura forzada; cambiar de licencia exigiría el permiso de cada contribuyente ([Q02]). | [Q21]: se leen el comportamiento y la documentación, se escriben especificaciones y nunca se copia; `cargo-deny` y la regla de [Q19]; DCO ([Q22]). |

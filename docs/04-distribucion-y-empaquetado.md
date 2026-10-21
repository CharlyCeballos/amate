# Distribución y empaquetado

Cómo se construye, se firma, se publica y se instala Amate en Windows, Linux y macOS. Toma como
referencia la sección *standalone* de PhotoCraft y la corrige donde no encaja con los requisitos de
Amate. Las decisiones viven en [DECISIONES.md](DECISIONES.md) y aquí se citan por su
identificador, así: [Q09]. Este documento es insumo para la corrida de [PROMPT.md](PROMPT.md): la
sección `## Empaquetado` del plan parte de aquí y fija, para cada artefacto de [Q09], la
herramienta concreta, el nombre del archivo, las arquitecturas y el sistema mínimo.

La distribución es trabajo de primera clase, no un apéndice. Amate es un producto para usuarios
finales: tiene que instalarse, arrancar, abrir archivos y actualizarse sin que nadie tenga que
saber qué es Rust. El cuello de botella del proyecto no es saber Rust, sino distribuir software
firmado en tres sistemas operativos y conocer los límites de los toolkits de interfaz de Rust. Y
pasar de una plataforma a tres multiplica el costo de empaquetado, firma, pruebas y soporte; no lo
suma.

Cada dato fechado lleva «(verificado: 2026-10-15)» o «(a verificar)». Son el punto de partida, no
una cita permanente: la corrida del prompt vuelve a comprobar fechas, versiones de herramientas y
sistemas operativos el día en que se ejecuta. La lista de hechos por re-verificar está en
[fusion.md](fusion.md), igual que el origen de lo que aquí se llama «la versión anterior de este
documento».

Cuatro términos tienen un sentido fijo en este documento:

- **Standalone**, como lo usa PhotoCraft: la aplicación se instala sola, sin un lanzador que la
  administre. Todos los artefactos de Amate lo son.
- **Portable**: se ejecuta sin instalar y guarda su configuración en su propia carpeta. Solo el
  ZIP de Windows se anuncia como portable.
- **Registro en «Abrir con»**: lo que hace un paquete para que Amate aparezca entre los programas
  que pueden abrir un tipo de archivo, sin quedarse con la asociación por defecto ([Q14]).
- **Verbo del menú contextual**: una entrada propia como «Editar con Amate», al estilo de
  `nppShell`. Queda para después de la 1.0 ([Q14]).

---

## 1. Lo que hace PhotoCraft

La sección «Just PhotoCraft? Download it standalone.» de la
[página de PhotoCraft](https://getartcraft.com/apps/photocraft) es un buen modelo porque resuelve
el mismo problema que Amate: una aplicación nativa en Rust, de código abierto, para tres sistemas
operativos. Es referencia de distribución, no de alcance.

Matriz de artefactos de la versión 0.7.0, tal como la publicaba esa sección
(verificado: 2026-10-15):

| Plataforma | Descarga destacada | Variantes plegadas debajo |
| --- | --- | --- |
| macOS | `.dmg` universal (Apple Silicon + Intel) | CLI universal en `.zip` |
| Windows | Instalador `.msi` de 64 bits | portable `.zip` x64 · `.msi` x86 · portable x86 · `.msi` ARM64 · portable ARM64 |
| Linux | Flatpak x86_64 (`flatpak install --user`) | — |
| Otros | — | FreeBSD 14 en `.tar.gz` · build web (WASM) en `.zip` · `SHA256SUMS.txt` |

Ese día el release de GitHub iba por delante de la página: ya existía la 0.9.0
(verificado: 2026-10-15). Para Linux, en la página 0.7.0 solo Flatpak; el release 0.9.0 publica
más archivos de Linux (a verificar). La página sirve como modelo de presentación y el release
vigente como lista real de artefactos; la corrida del prompt vuelve a abrir los dos.

Y dos elementos que no son artefactos pero sí producto (verificado: 2026-10-15):

- **Un lanzador propio** que instala y mantiene actualizadas todas las aplicaciones de la familia,
  elige el *build* correcto para la máquina y adopta las instalaciones standalone previas.
- **Una vía desde el código fuente** de tres líneas: versión mínima de Rust declarada (1.90+),
  `git clone`, `cargo run --release -p photocraft`. Sin dependencias de sistema que explicar.

### Qué copiar

1. **Una descarga recomendada por sistema, evidente, y el resto plegado.** La mayoría de la gente
   quiere el botón grande. Las cinco variantes plegadas de Windows existen para quien las
   necesita, pero no compiten visualmente con la recomendada.
2. **Descripciones de una línea escritas para humanos**, no para ingenieros: «Unzip and run, no
   install», «Older 32-bit Windows», «Windows 11 on ARM laptops». Amate las escribe en los dos
   idiomas del README ([Q08]).
3. **`SHA256SUMS.txt` siempre presente** y verificable sin leer documentación. Amate lo publica en
   el formato que acepta `sha256sum -c`, junto con la atestación de procedencia ([Q28]).
4. **La variante portable existe en todas las arquitecturas de Windows que se publican**, no solo
   en x64. Con [Q09], la primera versión pública solo publica x64; cuando Windows ARM64 llegue en
   el hito 4b, llega también su ZIP portable.
5. **La vía desde el código fuente es parte de la sección de descargas**, no una nota escondida.
   Para un proyecto de código abierto es señal de que el código realmente compila.
6. **La página es el canal de actualización**: «vuelve aquí para nuevas versiones». Es justo lo
   que dice [Q13]: la versión nueva se obtiene descargando el release nuevo. La otra mitad del
   mensaje de PhotoCraft, «o instala el lanzador y él se encarga», no aplica.

### Qué no copiar

**La estrategia de Linux.** Para Linux, en la página 0.7.0 solo Flatpak; el release 0.9.0 publica
más archivos de Linux (a verificar). Sea cual sea la lista real de PhotoCraft, la matriz de Linux
de Amate no sale de ahí, sino del requisito de Debian y derivados, y para ese requisito un Flatpak
solo no basta:

- Flatpak no viene preinstalado en Debian ni en Ubuntu, que promueve Snap (a verificar).
- La cultura Debian prefiere `.deb` y `apt`.
- Una aplicación en Flatpak vive en un sandbox: el acceso al sistema de archivos pasa por portales
  XDG, y para un editor de texto, cuyo trabajo entero es abrir archivos arbitrarios del disco, eso
  es fricción constante y nada trivial de resolver bien.

Por eso [Q09] pone el `.deb` amd64 como artefacto de primera clase y deja Flatpak para el hito 4b.

**El lanzador.** Tiene sentido para una familia de aplicaciones. Para un editor suelto es una
segunda aplicación que mantener, y no resuelve nada que no resuelva ya el modelo de [Q13]: cada
paquete de Amate se instala solo y la versión nueva se descarga del release.

**El alcance.** PhotoCraft es referencia de distribución, no de producto: nada de su lienzo GPU ni
de PSD. De sus formatos, Windows x86, el build web (WASM) y FreeBSD quedan fuera salvo petición
explícita ([Q09]). La CLI universal que publica en `.zip` tampoco está en [Q09]: una CLI headless
es una pregunta abierta del cuestionario (B9 en [02-cuestionario.md](02-cuestionario.md)).

---

## 2. Matriz de Amate

La matriz sale de [Q09]. La columna «Cuándo» tiene tres valores: **primera versión pública**
(hito 4, una 0.N según [Q27]), **hito 4b** (opcional, sin mezclarse con el cierre del hito 4) y
**fuera salvo petición explícita**. Además de las filas de cada tabla, quedan fuera salvo petición
explícita web/WASM, FreeBSD y RISC-V ([Q09]). En la primera versión pública hay un solo canal:
GitHub Releases ([Q09]).

Sistema mínimo que se promete: Windows 10 (22H2) y 11, Debian 12, Ubuntu 24.04 y la versión de
macOS que el toolkit soporte de verdad ([Q09]). Nada por debajo. Si el toolkit exige algo más
nuevo, el plan sube el mínimo y lo justifica.

Cuándo se distribuye cada cosa (el detalle de cada hito está en
[03-plan-de-trabajo.md](03-plan-de-trabajo.md)):

| Momento | Qué se distribuye |
| --- | --- |
| Hito 0 | Una ventana vacía empaquetada de forma basta en cada sistema, descargable desde el CI; todavía no son el MSI, el DMG ni el `.deb` finales. El hito mide el costo real de empaquetar en los tres sistemas y el build universal de macOS ([Q24]). |
| Hito 1 (0.1.0, sin anuncio) | Los paquetes de [Q09], generados en CI y sin firma ([Q10], [Q27]). |
| Hito 4: primera versión pública (0.N) | Los artefactos finales de [Q09], `SHA256SUMS.txt` y la atestación de procedencia ([Q28]), la integración con el escritorio ([Q14]), la sección de descargas del README con los pasos para abrir builds sin firma, y la solicitud a SignPath Foundation ([Q10]). Salida: un `git tag` produce todo sin pasos manuales, y los paquetes se instalan y verifican en máquinas limpias. |
| Hito 4b, opcional | Flatpak, `.deb` arm64, Windows ARM64, repositorio apt propio, winget, cask de Homebrew, firma de pago y notarización de macOS ([Q09], [Q10]). |
| 1.0 | No es un hito sino una compuerta: `diario` completo, firma en Windows, notarización en macOS y manual ([Q27]). Lo que el hito 4b aporta de firma y notarización es opcional para cerrar el hito 4, pero no para llegar a la 1.0. |

### Windows

| Artefacto | Formato | Cuándo | Notas |
| --- | --- | --- | --- |
| **Instalador x64** | `.msi` (WiX) | **Primera versión pública** | Registro en «Abrir con», accesos directos y desinstalador limpio. Nunca se queda con las asociaciones por defecto ([Q14]). |
| **Portable x64** | `.zip` | **Primera versión pública** | «Descomprimir y ejecutar». Requisito cultural del público de Notepad++: memoria USB y sin permisos de administrador. |
| Instalador ARM64 | `.msi` | Hito 4b | Computadoras portátiles con Windows en ARM; barato una vez que existe el pipeline. Mientras tanto, el MSI x64 corre emulado en Windows 11 sobre ARM (a verificar). |
| Portable ARM64 | `.zip` | Hito 4b | Llega con el instalador ARM64. |
| x86 (32 bits) | `.msi` + `.zip` | Fuera salvo petición explícita | Solo si aparece demanda real. |

Windows 10 ya no recibe parches de consumo (a verificar): se sigue prometiendo, pero las pruebas
principales se hacen en Windows 11 ([Q09]). Notepad++ todavía soporta Windows 8.1
(verificado: 2026-10-15); Amate no baja de Windows 10.

**Instalador.** [Q09] fija MSI. Notepad++ se empaqueta con NSIS (`nppSetup.nsi`) y con WiX/MSI
(verificado: 2026-10-15); el `.exe` de NSIS no entra en Amate. Cómo compila, prueba y publica
Notepad++ (`appveyor.yml` incluido) lo analiza la corrida del prompt; ver la sección 6 de
[01-analisis-notepad-plus-plus.md](01-analisis-notepad-plus-plus.md).

**Modo portable.** Notepad++ lo resuelve con un archivo marcador, `doLocalConf.xml`, junto al
ejecutable: si existe, la configuración vive en la carpeta de la aplicación y no en el perfil del
usuario (a verificar). Amate replica el mecanismo, porque es lo que su público ya sabe usar, pero
con un marcador propio: Amate tiene su propia configuración ([Q05]) y nada de Notepad++ entra en el
proyecto ([Q21]). Las reglas:

- Ninguna ruta de configuración se codifica: todas se resuelven en tiempo de ejecución según el
  modo detectado. Es una regla de arquitectura, no de empaquetado, y rige desde el primer código
  que lee o escribe configuración.
- El ZIP de Windows trae el marcador y es la única variante que se anuncia como portable. Como las
  rutas se resuelven en tiempo de ejecución, el mecanismo no depende del sistema, pero ni el
  AppImage ni el `.tar.gz` ni el DMG se anuncian como portables.
- Una variante portable no escribe nada fuera de su carpeta: ni registro de Windows, ni
  asociaciones de archivo, ni entradas del menú Inicio, ni archivos en el perfil del usuario.
- El nombre y el formato del marcador, y qué hace la variante portable en un medio de solo
  lectura, siguen abiertos (sección 8).

La firma de Windows está en la sección 3.

### Linux

| Artefacto | Formato | Cuándo | Notas |
| --- | --- | --- | --- |
| **Debian y Ubuntu, amd64** | `.deb` | **Primera versión pública** | El requisito principal: Debian 12+ y Ubuntu 24.04+, con Linux Mint 22+ por herencia ([Q09]). Instala `.desktop`, iconos y tipos MIME, y Amate aparece en «Abrir con» ([Q14]). |
| **Resto de distribuciones, x86_64** | AppImage | **Primera versión pública** | Red de seguridad para Fedora, Arch, openSUSE y compañía. No exige instalación. |
| **Resto de distribuciones, x86_64** | `.tar.gz` de binarios | **Primera versión pública** | Los binarios sueltos, sin instalador ([Q09]). |
| Repositorio apt propio | `apt` firmado con GPG | Hito 4b | Es lo que hace que las actualizaciones funcionen como espera quien usa Debian. Hasta entonces, el `.deb` se descarga de GitHub Releases. |
| Debian y Ubuntu, arm64 | `.deb` | Hito 4b | Raspberry Pi y servidores ARM. |
| Flathub | Flatpak | Hito 4b | Alcance amplio; exige resolver bien los portales XDG. |
| Snap | — | Fuera salvo petición explícita | Salvo que aparezca una razón concreta. |
| `.rpm` | — | Fuera salvo petición explícita | Las distribuciones que usan `.rpm` tienen el AppImage y el `.tar.gz`. |
| Debian oficial | — | No se busca | Exige empaquetar por separado cada dependencia de Rust; inviable salvo que alguien de Debian lo adopte. |

**El problema de glibc.** Un binario compilado contra una glibc más nueva puede no arrancar en una
más vieja, y el fallo aparece en la máquina del usuario, no en la del mantenedor. Debian 13
(trixie) trae glibc 2.41 y Debian 12 (bookworm), la 2.36 (a verificar): el runner de CI más
moderno produce binarios que pueden fallar justo en las distribuciones que hay que soportar.

La salida que fija [Q09]: Linux se compila en un contenedor `debian:bookworm`, la glibc más antigua
que se promete, y esa base se declara en público en el README. El `.deb`, el AppImage y el
`.tar.gz` salen del mismo contenedor.

La alternativa era enlazar estáticamente contra musl para el AppImage: un binario que funciona en
cualquier distribución, a cambio de algo de rendimiento y de complicaciones con el DNS y las
fuentes del sistema. Además, un binario estático con musl probablemente no puede cargar en tiempo
de ejecución las bibliotecas de GL, Vulkan, Wayland y X11 del sistema que usan los toolkits
gráficos (a verificar). El enlace con musl solo entra si el hito 0 demuestra que funciona.

Ubuntu 22.04 no está en la promesa: trae glibc 2.35 (a verificar), debajo de la base de bookworm,
y soportar esa versión exigiría bajar el contenedor de compilación.

**Pruebas en contenedores limpios.** Los paquetes se prueban en contenedores limpios, nunca en el
runner de CI, que tiene instaladas cosas que el usuario no tendrá ([Q09]). Como mínimo: Debian 12,
Debian 13 y Ubuntu 24.04. Debian 12 ya está en soporte LTS (a verificar) y sigue dentro de la
promesa de [Q09].

**Wayland y X11.** Afectan al portapapeles, los menús globales, el escalado fraccionario, las
decoraciones de ventana del lado del cliente y los diálogos de archivo. Amate usa las API del
sistema, Wayland y X11 incluidas ([Q19]). Cuál es la referencia sigue abierto: es la pregunta F3
de [02-cuestionario.md](02-cuestionario.md). Su recomendación: soportar ambos con Wayland como
referencia, probar en GNOME y KDE, y abrir los diálogos de archivo por portales XDG, que además es
lo que permite empaquetar en Flatpak en el hito 4b sin reescribir nada.

### macOS

| Artefacto | Formato | Cuándo | Notas |
| --- | --- | --- | --- |
| **Universal** | `.dmg` (arm64 + x86_64) | **Primera versión pública** | Un solo descargable para toda la base instalada. Versión mínima: la que el toolkit soporte de verdad ([Q09]). |
| Homebrew | cask | Hito 4b | Barato de automatizar, y es como instala el público técnico. Va después de la notarización: Homebrew ha ido desactivando los casks que no pasan Gatekeeper (a verificar). |
| Mac App Store | `.pkg` | Fuera salvo petición explícita | El sandbox es incompatible con un editor que abre archivos arbitrarios. |

**El binario universal.** El hito 0 mide el build universal ([Q24]). El plan consulta los runners
hospedados vigentes el día de la corrida y explica cómo se produce la mitad x86_64. Los runners de
macOS con Intel están en retiro y Rust bajó de nivel el target `x86_64-apple-darwin`
(a verificar), así que la vía prevista es compilar esa mitad de forma cruzada en un runner arm64 y
unir las dos con `lipo`. Cómo se prueba la mitad x86_64 sigue abierto (sección 8).

La firma y la notarización de macOS están en la sección 3.

### Común a todos los sistemas

| Artefacto | Propósito |
| --- | --- |
| `SHA256SUMS.txt` | Integridad. Uno por release, con las sumas de todos los archivos, en el formato que acepta `sha256sum -c` ([Q28]). |
| Atestación de procedencia | Autenticidad: prueba que el binario salió de ese commit y de ese flujo. La genera `actions/attest-build-provenance` en GitHub Actions y se verifica con `gh attestation verify`. No sustituye la firma de código ([Q28]). |
| Avisos de terceros | El CI los genera y cada artefacto los lleva ([Q02]), incluidos los de las definiciones `.sublime-syntax` que se sumen a `syntect` ([Q26]). |
| Notas de versión | Enlazadas desde la sección de descargas del README. |
| `git clone` + `cargo run --release` | La vía desde el código, con la MSRV declarada. La MSRV se fija después de elegir crates, con la versión estable reciente como suelo. |

La firma del archivo de sumas con `minisign` o `cosign`, que proponía la versión anterior de este
documento, no entra: [Q28] eligió la atestación de GitHub para probar la autenticidad.

Reglas para el contenido de todos los paquetes:

- Ningún archivo de Notepad++ viaja en los paquetes: ni temas, ni UDL, ni traducciones, aunque la
  GPL lo permitiría ([Q21]).
- Nada de C ni C++ compilado dentro del binario distribuido. El CI falla si el árbol de
  dependencias del binario incluye `cc`, `cmake` o `bindgen` ([Q19]).
- Las dependencias tienen licencia compatible con GPLv3; lo controla `cargo-deny` ([Q02]).
- Si la descripción de un paquete (el `.deb`, los metadatos de AppStream, el manifiesto de winget o
  el cask) menciona Notepad++, lo hace solo en texto descriptivo y con el aviso de [Q21]. Ningún
  nombre ni identificador de paquete usa la marca ([Q01]).

**Canales de terceros.** El proyecto mantiene solo los del hito 4b: repositorio apt, Flathub,
winget y el cask de Homebrew. Chocolatey, Scoop, MacPorts, AUR y Nixpkgs son bienvenidos si la
comunidad los adopta, pero el proyecto no los mantiene. Snap queda fuera salvo petición explícita
([Q09]).

---

## 3. Firma y notarización

[Q10]: los certificados de pago (Apple Developer y certificado de Windows) quedan para después. En
el hito 4 se solicita la firma gratuita para código abierto de SignPath Foundation, cuyos
requisitos falta revisar (a verificar); si la aprueban, la primera versión pública ya va firmada en
Windows.

Construir y firmar son pasos distintos. Los paquetes se construyen igual sin firma, y la firma usa
secretos que no viven en el repo ni en el CI público ([Q10]).

| Momento | Windows | macOS |
| --- | --- | --- |
| Hitos 0 a 3 | Paquetes sin firma. | Paquetes sin firma. |
| Hito 4 | Solicitud a SignPath Foundation. Si la aprueban, el MSI y el ZIP salen firmados; si no, salen sin firma y el README explica cómo abrirlos. | Sin firma ni notarización. El README explica cómo abrir la app pese a Gatekeeper. |
| Hito 4b | Firma de pago, si hace falta (por ejemplo, si SignPath no aprueba la solicitud). | Apple Developer Program, firma con Developer ID y notarización. |
| 1.0 | Firma obligatoria ([Q27]). | Notarización obligatoria ([Q27]). |

### Windows

Sin firma Authenticode, SmartScreen muestra la advertencia de «editor desconocido» y se pierde una
fracción importante de las instalaciones (a verificar). Peor aún: Smart App Control de Windows 11
bloquea los binarios sin firma y no admite excepciones (a verificar), así que quien lo tenga activo
no puede abrir Amate sin firma ([Q10]). Por eso la solicitud a SignPath importa: es la vía sin
costo para que la primera versión pública ya salga firmada.

Con SignPath, la llave privada vive en el servicio de SignPath y el CI solo le envía el artefacto
(a verificar). Si su política exige aprobar a mano cada solicitud de firma (a verificar), ese paso
choca con la salida del hito 4, que pide un `git tag` sin pasos manuales; el plan lo resuelve antes
de cerrar el hito.

Las opciones de pago para el hito 4b, tal como las comparaba la versión anterior de este documento
(a verificar):

- **Certificado OV**: más barato; la reputación en SmartScreen se acumula despacio.
- **Certificado EV**: reputación inmediata; requiere un token físico o un HSM.
- **Azure Trusted Signing**: pago por uso y sin hardware; la vía más cómoda para un proyecto
  pequeño con entidad legal.

El titular de Amate es una persona física, no una empresa ([Q29]), y no todas estas opciones
aceptan necesariamente a una persona física (a verificar). Se revisa antes de elegir.

Hasta que haya firma, el README explica cómo pasar la advertencia de SmartScreen, y dice sin
rodeos que con Smart App Control activo no hay paso de desbloqueo ([Q10]).

### macOS

Sin firma ni notarización, Gatekeeper bloquea la aplicación con un mensaje que la mayoría de la
gente interpreta como «esto es malware». En la primera versión pública, el README explica cómo
abrirla ([Q10]). Esos pasos tienen que respetar dos hechos (a verificar):

- Desde macOS 15, el atajo de Control-clic y «Abrir» ya no salta Gatekeeper: hay que pasar por
  Configuración del Sistema, en Privacidad y seguridad.
- Un binario para Apple Silicon necesita al menos una firma ad hoc para ejecutarse.

El proceso completo, para el hito 4b:

1. Inscribirse en el Apple Developer Program, que cuesta 99 USD al año (a verificar).
2. Firmar con Developer ID.
3. Enviar a notarizar con `notarytool`.
4. Hacer *staple* del ticket al `.dmg` con `stapler`.

Es el paso del pipeline que más fallos opacos produce, así que conviene ensayarlo con un binario
trivial en cuanto exista la cuenta, antes de que bloquee un release. El hito 0 no lo ensaya: no
está entre sus mediciones ([Q24]) y sin cuenta no hay con qué.

---

## 4. Integración con el escritorio

[Q14] es `con-el-paquete`: la integración llega con cada paquete instalable, y Amate nunca se
apropia de las asociaciones por defecto.

| Paquete | Qué integra | Cuándo |
| --- | --- | --- |
| `.deb` | `.desktop`, iconos y tipos MIME; Amate aparece en «Abrir con». | Con el `.deb` ([Q14]) |
| MSI | Registro en «Abrir con» y accesos directos. | Hito 4 |
| DMG | Registro en «Abrir con», con los tipos de documento en el `Info.plist`. | Hito 4 |
| Paquete MSIX firmado | Verbo «Editar con Amate» del menú contextual de Windows 11. | Después de la 1.0 |

`nppShell`, la extensión del Explorador de Notepad++ («Edit with Notepad++», puro Win32/COM)
(verificado: 2026-10-15), no se porta ([Q21]). Su equivalente en Amate es el registro en «Abrir
con» y, después de la 1.0, el verbo del menú contextual.

Las variantes portables no registran nada (sección 2, modo portable). El AppImage tampoco instala
un `.desktop`; los metadatos de escritorio de Linux siguen abiertos (sección 8).

Lo demás que se listaba junto a la integración con el sistema (icono en la bandeja, listas de
salto de la barra de tareas, abrir archivos por arrastre) es comportamiento de la aplicación, no
empaquetado, y [Q14] no lo decide.

---

## 5. Cómo se construye

**Herramienta recomendada: `dist`** (antes `cargo-dist`). Según la versión anterior de este
documento, a septiembre de 2026 era un proyecto activo, en la versión 0.33 (a verificar). Genera el
flujo de CI completo a partir de la configuración del workspace: planificar el release, levantar
una máquina por plataforma, compilar, generar los instaladores, subir los artefactos, crear el
release de GitHub y anunciarlo. El disparador es un `git tag`.

Lo que `dist` cubre de forma nativa (a verificar): archivos comprimidos, instaladores de shell y de
PowerShell, MSI, paquetes de Homebrew y de npm, y las sumas de verificación. De eso, [Q09] usa los
archivos comprimidos (el ZIP portable y el `.tar.gz`), el MSI y las sumas; los instaladores de
shell y de PowerShell y los paquetes de npm no hacen falta. Su soporte de Homebrew genera fórmulas
para binarios de línea de comandos, no casks para una `.app` (a verificar), así que el cask del
hito 4b necesita su propio paso. Si sus sumas salen con otro nombre o formato (a verificar), se
configura o se agrega un paso que produzca `SHA256SUMS.txt`.

Herramienta por necesidad, para la primera versión pública:

| Necesidad | Herramienta |
| --- | --- |
| ZIP portable y `.tar.gz` | `dist` (archivos comprimidos) |
| MSI | `dist`, o `cargo-wix` (WiX) |
| `.deb` | `cargo-deb` |
| AppImage | `appimagetool`, o `cargo-packager` |
| `.dmg` con fondo y disposición | `create-dmg`, o `cargo-packager` |
| `SHA256SUMS.txt` | `dist`, o un paso propio con `sha256sum` |
| Atestación de procedencia | `actions/attest-build-provenance` ([Q28]) |
| Firma de Windows | SignPath, si aprueban la solicitud ([Q10]); en el hito 4b, `signtool` con un certificado de pago |

Y para el hito 4b:

| Necesidad | Herramienta |
| --- | --- |
| Firma y notarización de macOS | `codesign`, `notarytool` y `stapler` de Xcode, invocados desde el CI |
| Repositorio apt | `aptly` o `reprepro`, publicado en almacenamiento estático |
| Manifiesto de winget | `wingetcreate`, en un paso posterior al release |
| Flatpak | `flatpak-builder` |
| Cask de Homebrew | Un paso propio, porque `dist` genera fórmulas (a verificar) |

Alternativa a considerar si la matriz crece: **`cargo-packager`**, que cubre `.deb`, AppImage,
`.dmg`, MSI y NSIS desde una sola configuración (a verificar), a cambio de escribir el CI por tu
cuenta; NSIS no hace falta ([Q09]). Lo que ninguna herramienta cubra, como generar los metadatos de
cada sistema desde plantillas, vive en código propio, en un crate `xtask` del workspace.

El hito 0 mide el costo real de empaquetar en los tres sistemas ([Q24]); esa medición confirma
`dist` o cambia la elección, y el resultado queda en un ADR de `docs/adr/`. La corrida del prompt
verifica la versión vigente y el mantenimiento de `dist`, `cargo-packager` y `cargo-bundle` el día
en que se ejecuta.

### Estructura del pipeline

```text
git tag vX.Y.Z
  └─ plan       determina qué se publica y con qué versión
  └─ build      imágenes fijas, nunca -latest: Windows x64 · macOS arm64 · contenedor debian:bookworm
       ├─ compila el binario; en macOS, también la mitad x86_64 cruzada, unida con lipo
       ├─ firma, solo si hay firma disponible
       │    hito 4:  Windows con SignPath, si aprueban la solicitud
       │    hito 4b: signtool · codesign + notarytool + stapler
       └─ empaqueta: msi · zip portable · dmg · deb · AppImage · tar.gz
  └─ test       instala y arranca los paquetes de Linux en contenedores limpios
                (Debian 12, Debian 13 y Ubuntu 24.04)
  └─ host       genera la atestación de procedencia y sube los artefactos con SHA256SUMS.txt
  └─ publish    solo en el hito 4b: repositorio apt, Flathub, manifiesto de winget y cask de Homebrew
  └─ announce   crea el release de GitHub con las notas de versión
```

El objetivo de diseño es que **publicar una versión sea empujar una etiqueta**. Cualquier paso
manual en este pipeline se convierte, antes o después, en un release roto o en un release que no se
hace. La salida del hito 4 lo comprueba: un `git tag` produce todo sin pasos manuales, y los
paquetes se instalan y verifican en máquinas limpias.

**CI y release son flujos distintos.** El CI de cada pull request corre en runners hospedados de
Windows x64, Linux x64 y macOS arm64, y nunca ve secretos de firma. El release lo dispara la
etiqueta, corre en imágenes fijas y es el único flujo que firma ([Q10]).

**Versiones.** SemVer, con 0.x hasta firmar ([Q27]): el hito 1 es la 0.1.0, sin anuncio; la
primera versión pública es una 0.N; la 1.0 llega con `diario` completo, firma y manual. Las
etiquetas siguen el formato `vX.Y.Z`.

---

## 6. Actualizaciones

[Q13] es `release`: la versión nueva se descarga de GitHub Releases. Amate no hace ninguna
conexión de red, ni comprobación de versión ni telemetría. Es parte de la propuesta de valor: un
editor de texto que «no se conecta a nada» ([Q17]).

Notepad++ se actualiza con un binario aparte, WinGup (`wingup`), un actualizador genérico solo
para Windows (verificado: 2026-10-15). Amate no lo porta ni tiene equivalente. Las alternativas
para un actualizador propio en Rust (el crate `self_update`, el actualizador de Tauri o una
implementación propia sobre un manifiesto firmado) quedan descartadas por [Q13]; la de Tauri,
además, por [Q16].

El matiz que se suele pasar por alto sigue valiendo: una aplicación instalada por `.deb`, Flatpak o
Homebrew no debe actualizarse sola, porque rompe la coherencia del gestor de paquetes y deja al
sistema en un estado que el administrador no puede razonar. Con [Q13] hoy no aplica, pero
condiciona cualquier aviso futuro.

### Diseño si algún día hay aviso de versión

[Q13] admite agregar algún día un aviso de versión con tres condiciones: opt-in en el primer
arranque, envía solo la versión y usa el TLS del sistema, porque TLS con proveedores en C (`ring`,
`aws-lc-rs`) queda fuera ([Q19]). Si se agrega, este es su comportamiento por canal:

| Canal de instalación | Qué hace el aviso | Diseño anterior, descartado por [Q13] |
| --- | --- | --- |
| ZIP portable de Windows | Avisa y enlaza a la descarga. | Autoactualización completa, con consentimiento. |
| MSI de Windows | Avisa y enlaza a la descarga. | Autoactualización completa, con consentimiento. |
| DMG de macOS | Avisa y enlaza a la descarga. | Autoactualización completa (modelo Sparkle), con consentimiento. |
| Cask de Homebrew (hito 4b) | Avisa; actualiza `brew upgrade`. | Lo mismo. |
| `.deb` con repositorio apt (hito 4b) | Avisa; actualiza `apt upgrade`. | Lo mismo. |
| `.deb` suelto | Avisa y enlaza a la descarga. | Lo mismo. |
| AppImage | Avisa por defecto; `AppImageUpdate` como opción. | Autoactualización con `AppImageUpdate`, o avisar. |
| `.tar.gz` | Avisa y enlaza a la descarga. | No tenía fila. |
| Flatpak (hito 4b) | Nada: lo gestiona Flatpak. | Lo mismo. |

Para eso el binario tiene que saber cómo fue instalado: se incrusta un identificador de canal al
compilar cada paquete.

El público de Notepad++ es especialmente sensible a la telemetría, con razón. Por eso el aviso, si
existe, es opt-in y no envía nada más que la versión actual. Los informes de fallos son manuales:
el usuario ve su contenido y lo envía él mismo; Amate no envía nada ([Q13]).

---

## 7. Descargas: GitHub Releases y el README

En la primera versión pública, los binarios viven como assets de GitHub Releases y la sección de
descargas está en el README, en inglés y en español ([Q08]). Una página propia, al estilo de
PhotoCraft, necesita un dominio, que [Q01] manda verificar y que va a nombre del titular ([Q29]);
no está asignada a ningún hito.

La sección de descargas del README copia la estructura de PhotoCraft, que está bien resuelta:

1. **Un bloque por sistema** (Windows, macOS y Linux), con la descarga recomendada primero: el MSI
   x64, el DMG universal y, en Linux, el `.deb` amd64 para Debian, Ubuntu y derivadas, y el
   AppImage para el resto.
2. **Debajo de cada recomendada, un bloque plegable** «Otros archivos para …» con las variantes,
   cada una con su descripción de una línea en lenguaje llano.
3. **La versión actual a la vista**, con enlace a sus notas de versión y a todos los releases.
4. **`SHA256SUMS.txt` junto a las descargas**, con una instrucción de verificación de una línea por
   sistema (en Linux, por ejemplo, `sha256sum -c SHA256SUMS.txt --ignore-missing`) y el comando
   `gh attestation verify` para comprobar la atestación ([Q28]).
5. **Un bloque «Ejecútalo hoy desde el código»**, con la MSRV declarada y las líneas de `git clone`
   y `cargo run --release`.
6. **Mientras falte la firma, los pasos para abrir Amate sin ella**: Gatekeeper en macOS y
   SmartScreen en Windows ([Q10]). El bloque dice también que, con Smart App Control activo,
   Windows 11 no deja abrir un binario sin firma.
7. **La base de los binarios de Linux**, declarada en público: se compilan en `debian:bookworm`
   ([Q09]).

Lo que un README no puede hacer y una página sí: detectar el sistema del visitante, mostrar un
único botón grande con la descarga correcta y separar los sistemas en pestañas. Eso queda para
cuando exista la página.

Las instrucciones completas de `apt` (agregar la llave GPG, agregar el repositorio, `apt install`)
entran solo cuando exista el repositorio apt, en el hito 4b.

---

## 8. Lo que queda abierto

Sigue abierta en el cuestionario la pregunta F3 (Wayland, X11 y escritorio de referencia); su
recomendación está en la sección 2. Lo demás de esta tabla no lo decide
[DECISIONES.md](DECISIONES.md) ni este documento: lo cierra el plan, que recibe la lista completa
de huecos en [PROMPT.md](PROMPT.md). Cada fila trae la sugerencia de la auditoría de la fusión.

| Hueco | Sugerencia |
| --- | --- |
| Identificadores estables: bundle ID de macOS, ID de dominio inverso de la aplicación y UpgradeCode del MSI. No pueden cambiar después del primer release sin romper las actualizaciones. | Derivarlos del dominio verificado y fijarlos antes del primer artefacto público. [Q01] exige verificar la marca y el dominio antes de publicar cualquier artefacto o crear el repo público. |
| Esquema de nombres de archivo y vocabulario de arquitecturas (x64, amd64 y x86_64; arm64 y aarch64). | `amate-<versión>-<sistema>-<arquitectura>.<ext>`, y el `.deb` con la convención de Debian: `amate_<versión>_amd64.deb`. |
| Alcance del MSI: por usuario o por máquina, entrada en el `PATH` para `amate archivo:línea`, accesos del menú Inicio, y si desinstalar o actualizar conserva la configuración. | Por usuario por defecto, `PATH` opcional y la configuración se conserva al desinstalar. |
| Qué tipos de archivo se registran en «Abrir con» en cada sistema. | Tipos de texto y de código, solo en «Abrir con» ([Q14]). |
| Metadatos de escritorio en Linux: metainfo de AppStream, tamaños de iconos y `Exec` con `%F` en el `.desktop`. Se cruza con el modelo de proceso del plan: qué pasa al abrir un archivo con Amate ya abierto. | Metainfo e iconos en el `.deb`; una sola instancia que recibe los archivos por un socket local. |
| Metadatos de plataforma: manifiesto de Windows (DPI `PerMonitorV2`, `longPathAware`), tipos de documento en el `Info.plist` de macOS, `.desktop` y tipos MIME. | Generarlos desde plantillas en `xtask`. |
| Un solo `.deb` válido en Debian 12 y en Ubuntu 24.04: nombres de dependencias y los renombres `t64` de bibliotecas en Ubuntu (a verificar), con pruebas de instalación también en Linux Mint. | Dependencias automáticas de `cargo-deb`, calculadas en bookworm, y pruebas de instalación en contenedores limpios. |
| El AppImage y FUSE: el runtime clásico necesita `libfuse2`, que Debian 12 y Ubuntu 24.04 no instalan por defecto (a verificar). | Runtime estático (type2), probado en Debian 12 y Ubuntu 24.04 limpios. |
| macOS: versión mínima de despliegue, método del binario universal (target x86_64 cruzado, `lipo` y nueva firma ad hoc), un atajo de línea de comandos en el `PATH`, y cómo se prueba la mitad x86_64. | Versión mínima igual a la del toolkit; compilar en un runner arm64 con `lipo`; probar la mitad x86_64 en un runner Intel mientras exista, o con Rosetta (a verificar), y no publicar sin esa prueba. |
| Custodia, rotación y revocación de las credenciales de firma (SignPath, Developer ID, certificado de Windows y la llave GPG del repositorio apt) con un solo mantenedor. | Secretos solo en un entorno protegido del flujo de release, con respaldo fuera de línea a cargo del mantenedor y rotación documentada en `SECURITY.md`. |
| Ayudante para guardar con permisos elevados, por tipo de paquete: UAC en Windows, `pkexec` en Linux y quizá una política de polkit en el `.deb`. | Decidirlo en el hito 0; el ayudante solo va en los paquetes instalados y la función se desactiva en las variantes portables. |
| Nombre y formato del archivo marcador del modo portable, y qué hace la variante portable en un medio de solo lectura. | Marcador propio en TOML. En un medio de solo lectura, la regla de no escribir fuera de la carpeta descarta volver al perfil del usuario, que era la sugerencia original de la auditoría. |
| Formato y ubicación de los avisos de terceros en cada paquete (por ejemplo, el `copyright` del `.deb` en formato DEP-5) y un SBOM por release. | Generarlos en CI; un SBOM CycloneDX por release. |
| Política de releases: cadencia, versiones con soporte, correcciones de seguridad y formato del changelog. | Solo la última versión tiene soporte; changelog con el formato de Keep a Changelog. |
| Canal de prerelease o nightly: nombre, retención y si es público. | Solo artefactos de CI, no releases públicos. |
| Presupuesto de tamaño del binario y de la descarga, y falsos positivos de antivirus con los builds de Windows sin firma. | Medir el tamaño en CI; reportar los falsos positivos a los fabricantes. |
| Builds reproducibles. | Mejor esfuerzo, no compuerta. |
| Requisitos para compilar desde el código en cada sistema, y que `cargo run --release` funcione sin `-p` en el workspace. | Lista de requisitos por sistema una vez elegido el toolkit; `default-members` apuntando al crate de la aplicación. |
| Contenido de los pasos para abrir builds sin firma. | Probarlos en el macOS vigente y en Windows 11 con Smart App Control activo antes de escribirlos. |
| «Notepad++» en las palabras clave de búsqueda de paquetes y tiendas. | Solo en texto descriptivo, nunca como nombre ni como palabra clave. |
| Alojamiento del repositorio apt (hito 4b) y de una futura página de descargas. | GitHub Releases más GitHub Pages, hasta que haya una razón para mudarse. |

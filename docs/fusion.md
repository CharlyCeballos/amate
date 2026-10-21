# Registro de fusión

Cómo se unieron las cuatro fuentes originales (carpetas `01` a `04`, ya fuera del repo) en este
proyecto, entre el 2026-10-15 y el 2026-10-20.

## 1. Qué se fusionó

- `01`: README, PROMPT, DECISIONES y `docs/00` a `docs/04` (el juego más completo).
- `02`: solo su README; su PROMPT y su DECISIONES eran idénticos byte a byte a los de `01`.
- `03/prompt.md` y `04/prompt.md`: borradores de prompt independientes.

El dueño contestó todas las decisiones de producto ([DECISIONES.md](DECISIONES.md), Q01–Q31). Lo
editorial se resolvió en la fusión y se registra aquí.

## 2. Mapa fuente → destino

| Fuente | Destino |
| --- | --- |
| `01/PROMPT.md` + aportes de `03` y `04` | [PROMPT.md](PROMPT.md) |
| `01/DECISIONES.md` + preguntas de `01/docs/02` contestadas | [DECISIONES.md](DECISIONES.md) |
| `01/docs/01` | [01-analisis-notepad-plus-plus.md](01-analisis-notepad-plus-plus.md) |
| `01/docs/02` | [02-cuestionario.md](02-cuestionario.md) |
| `01/docs/03` | [03-plan-de-trabajo.md](03-plan-de-trabajo.md) |
| `01/docs/04` | [04-distribucion-y-empaquetado.md](04-distribucion-y-empaquetado.md) |
| `01/README.md`, `02/README.md` | `README.md` y `README.es.md` en la raíz |
| `01/docs/00` (razonamiento) | sección 6 de este archivo |

## 3. Decisiones editoriales

- Un solo esquema de IDs: Q01–Q31 en DECISIONES; A1–J5 solo como referencia en el cuestionario.
- Paridad con nombres (`familiar`, `diario`, `completo`), no letras: chocaban con los bloques A–J.
- Hitos 0–5 y 4b como único esquema. Equivalencia con las fases viejas de `01/docs/03`: Fase 0 y
  1 → Hito 0; Fase 2 y 3 → Hito 1; Fase 4 → Hito 2; Fase 5 → Hito 3 (sin importadores); Fase 6
  → Hito 4 y 4b; Fase 7 → Hito 4 y la compuerta 1.0; Fase 8+ → después de la 1.0.
- `PROMPT.md` y `DECISIONES.md` pasan a `docs/`; `02-preguntas-abiertas` se renombra
  `02-cuestionario`; `docs/00` no se conserva.
- La entrega del prompt pasa de 8 a 10 encabezados y se guarda en `docs/05-plan-ejecutable.md`.
- El cuestionario tiene 56 preguntas, no 58.
- [Q15]: CI y SECURITY.md entran en el hito 0, porque el primer commit no tiene código.
- `diario` detalla «edición» con las operaciones de línea de `01/docs/03`.
- Las preguntas que genere una corrida del prompt usan el prefijo `N01`, `N02`…
- Los enlaces externos a las fuentes analizadas se permiten como evidencia; entre archivos del
  repo, solo enlaces relativos.

## 4. Conflictos que decidió el dueño

Licencia, Rust estricto, paridad, tamaño de archivo, importación de configuración, plugins,
idiomas, paquetes, firma, actualizaciones, accesibilidad, versionado y criterio de éxito: ver
[DECISIONES.md](DECISIONES.md).

## 5. Contenido descartado

- Las versiones larga y corta del prompt de `01/docs/00`, y `03`/`04` como prompts vivos: el
  prompt único las sustituye.
- Recomendaciones que las decisiones superaron: Lua, Tree-sitter, autoactualización, repo apt y
  canales de terceros en la primera versión pública, archivos de 4 GB, importadores de Notepad++.
- Afirmaciones corregidas: «solo Flatpak» de PhotoCraft (sin calificar), «25 años» de Scintilla,
  estrellas leídas como tamaño, traducciones «sin licencia».

## 6. Por qué el prompt es así

Resumen de los cambios razonados de `01/docs/00`, vigentes: rol con competencias explícitas;
PhotoCraft solo como referencia de distribución; análisis medido, no recordado; sin calendario,
con criterios de salida verificables; niveles de paridad explícitos; validación medida antes de
construir (ahora dentro del hito 0); recomendar, no solo enumerar; preguntas con ID y valor por
defecto; restricciones explícitas; antiobjetivos; orden de trabajo fijo; corregir premisas
equivocadas.

## 7. Hechos por re-verificar

| Hecho | Estado |
| --- | --- |
| Versiones de PhotoCraft: página 0.7.0, release 0.9.0 | a verificar |
| Texto adicional del `LICENSE` de Notepad++ y su condición de marca registrada | a verificar |
| Versión vigente de `dist` y runners de GitHub Actions | a verificar |
| Requisitos de SignPath Foundation y comportamiento de Smart App Control | a verificar |
| Fin de soporte de Windows 10; Debian 12 en LTS | a verificar |
| Sintaxis que faltan en el juego de `syntect` para [Q26] | a verificar |
| Niveles de licencia de Slint; accesibilidad y dependencias en C de cada toolkit | a verificar |
| Marca de «Amate» (IMPI, USPTO, EUIPO) y dominio libre; dominios existentes revisados el 2026-10-20 (Amate Audio, Amate Tecnología) | a verificar |

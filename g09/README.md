# Checklist de Corrección y QA – Primera Entrega TP Final: Marketplace Descentralizado

**Materia:** Seminario de Lenguajes – Opción Rust
**Entrega:** Primera entrega obligatoria (18 de julio)
**Cobertura de tests requerida:** ≥ 85%

---
**Referencias:**
  🟢 Cumple
  🟡 Suficiente
  🔴 Insuficiente
  ⚫ No está implementado

---

## 1. Requisitos funcionales obligatorios

### Registro y gestión de usuarios
- [ 🟢 ] Permite registrar usuarios con rol `Comprador`, `Vendedor` o ambos.
- [ ⚫ ] Permite modificar roles de usuario luego del registro.

### Publicación de productos
- [ 🟢 ] Solo los usuarios con rol `Vendedor` pueden publicar productos.
- [ 🟡 [1] ] El producto incluye nombre, descripción, precio, cantidad y categoría.
- [ 🟢 ] El usuario puede visualizar sus propios productos publicados.

### Compra y gestión de órdenes
- [ 🟢 ] Solo usuarios con rol `Comprador` pueden crear órdenes de compra.
- [ 🟡 [2] ] Al comprar, se crea la orden y se descuenta el stock correctamente.
- [ 🔴 [3] ] La orden puede tener estado: `pendiente`, `enviado`, `recibido`.
- [ ⚫ ] Solo el `Vendedor` puede marcar la orden como `enviado`.
- [ ⚫ ] Solo el `Comprador` puede marcar la orden como `recibido`.
- [ 🟡 [4] ] Las validaciones de permisos y estados se aplican correctamente.

---

## 2. Contrato desplegado en testnet

- [ 🟢 ] Se incluye la dirección (`address`) del contrato desplegado en **Shibuya Testnet**.
- [ 🟡 [5] ] El contrato desplegado en testnet es **funcional** y permite interactuar con todas las funcionalidades requeridas.

---

## 3. Testing y calidad del código

- [ 🟢 [6] ] Existe una suite de tests automatizados que cubre **≥ 85%** del código del contrato.
- [ 🟢 [7] ] El código está bien estructurado y comentado según lo visto en clase.
- [ 🔴 [8] ] Incluye documentación técnica clara para las funcionalidades implementadas.

#### Set mínimo de pruebas obligatorio:
- [ 🟡 [9] ] Test de registro de usuario con cada rol posible.
- [ 🟢 ] Test de publicación de producto.
- [ 🟢 ] Test de compra de producto y generación de orden.
- [ ⚫ ] Test de cambio de estado de la orden (`pendiente` → `enviado` → `recibido`).
- [ 🟢 ] Test de validación de permisos (solo quien corresponde puede ejecutar cada acción).
- [ 🔴 [10] ] Test de errores esperados (ej: intentar comprar sin stock, cambiar estado sin permisos, etc.).

---

## 4. Checklist QA – Verificación manual en Testnet (Shibuya)

**Antes de aprobar la entrega, probar en la testnet lo siguiente:**

### Registro y roles
- [ 🟢 ] Registrar un usuario nuevo como `Comprador` y verificar que se guarde correctamente.
- [ 🟢 ] Registrar un usuario nuevo como `Vendedor` y verificar que se guarde correctamente.
- [ 🟢 ] Registrar un usuario con ambos roles.
- [ ⚫ ] Cambiar el rol de un usuario y chequear que el cambio se refleje en el contrato.

### Publicación de productos
- [ 🟢 ] Como `Vendedor`, publicar al menos un producto con todos los datos requeridos.
- [ 🟢 [11] ] Verificar que el producto figure en la lista de productos del vendedor.

### Compra y órdenes
- [ 🟢 ] Como `Comprador`, realizar la compra de un producto disponible.
- [ 🟢 ] Chequear que se descuente correctamente el stock tras la compra.
- [ 🟢 ] Verificar que la orden queda en estado `pendiente`.
- [ ⚫ ] Como `Vendedor`, cambiar el estado de la orden a `enviado`.
- [ ⚫ ] Como `Comprador`, cambiar el estado de la orden a `recibido`.

### Validaciones y errores esperados
- [ 🟢 ] Intentar comprar un producto sin stock y verificar que el contrato rechaza la operación.
- [ ⚫ ] Intentar que alguien que no sea el vendedor cambie el estado a `enviado` (debe fallar).
- [ ⚫ ] Intentar que alguien que no sea el comprador cambie el estado a `recibido` (debe fallar).
- [ 🟢 ] Intentar publicar un producto sin ser `Vendedor` (debe fallar).

### General
- [ 🟢 [12] ] Confirmar que los cambios de estado y acciones se reflejan correctamente en el almacenamiento y pueden ser consultados vía RPC.

**En caso de detectar algún fallo o comportamiento incorrecto, dejar evidencia** (logs, capturas o referencias a transacciones) que ilustre el problema.

---

## 5. Observaciones y comentarios

> Anotar aquí cualquier observación relevante (errores encontrados, código confuso, validaciones ausentes, recomendaciones, etc.)

> Muy incompleto.

[1]  No hay “Producto”. Es directamente la “Publicación”, la cual sí contiene nombre, descripción, precio, categoría, stock.

[2]  Se descuenta siempre un stock fijo con .checked_sub(1). El comprador no decide cuánto comprar. Pero, sí. Cumple.

[3] OrdenCompra tiene un campo estado: Estado, y Estado es un enum que puede ser Pendiente, Enviada, Recibida, Cancelada. Pero no está implementada la lógica necesaria para realizar estos cambios de estado.

[4] En cuanto a los Estados de la Orden, no cumple. En cuanto a los Roles del Usuario, cumple.

[5] No está implementada la funcionalidad para modificar roles de usuario luego del registro ni cambios de estado de las órdenes. Faltan implementar bastantes funcionalidades mínimas. Pero está desplegado correctamente.

[6]  Cobertura de casi el 100%.
```
test result: ok. 29 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.07s
 Jul 27 16:42:44.578  INFO cargo_tarpaulin::report: Coverage Results:
 || Tested/Total Lines:
 || lib.rs: 81/82
 ||
 98.78% coverage, 81/82 lines covered, -1.22% change in coverage
 ```
 Pero… Cuando corremos `cargo test --features e2e-tests` se corren los tests de ejemplo que vinieron con el proyecto de Ink.
 ```
 Some errors have detailed explanations: E0061, E0599.
 For more information about an error, try `rustc --explain E0061`.
 warning: `marketplace` (lib test) generated 17 warnings (8 duplicates)
 error: could not compile `marketplace` (lib test) due to 6 previous errors; 17 warnings emitted
 Uno de ellos es que como los constructores cambiaron,
 al hacer el llamado a los mismos pero con parámetros que no van se rompe todo. No compila.
 error[E0061]: this function takes 0 arguments but 1 argument was supplied
    --> lib.rs:1159:35
      |
 1159 |             let mut constructor = MarketplaceRef::new(false);
      |                                   ^^^^^^^^^^^^^^^^^^^ ----- unexpected argument of type `bool`
```

[7] Está bien estructurado. Me gustó también que los tests estén bien separados en distintos mods para cada funcionalidad. Legible y ordenado.

[8] No hay documentación con “///”, son todos comentarios normales. Por supuesto tampoco tienen ejemplos. No es poco clara… Pero no es técnica ni completa.

[9] No se testean registros exitosos de usuarios explícitamente, sólo dos casos de error. `tests_registrar_usuario_no_registrado()` y `tests_registrar_usuario_ya_registrado_error()`. Sin embargo, en los mods `mod tests_es_comprador { … }` y `mod tests_es_vendedor { … }` se testea la creación de todos los roles dentro de otros tests que no tienen que ver con la creación de usuarios en sí.

[10] Al no tener los procedimientos de cambio de estado de la Orden, ni cambios de rol de usuario no puedo decir que está todo según los requerimientos. Falta implementar mucho y no tenemos tests de esas funcionalidades sin implementar. Hay una lógica de permisos según roles y control de stock (por más que sea un decremento fijo) mínima. Pero faltan demasiadas cosas.

[11] Aparece en la lista de “PublicacionesVendedor”. No parece tener “Productos” separados de “Publicaciones”. Está bueno que tenga un listado de Publicaciones en general y las Publicaciones sólo del caller.

[12] Ver: https://shibuya.subscan.io/wasm_contract/YMYok9yfG81ujdbyUJmrhuqDdqHxReuvAgxgs6rchhLW7UX


---

**Resultado general:**
- [ ] APROBADO
- [x] DESAPROBADO

---


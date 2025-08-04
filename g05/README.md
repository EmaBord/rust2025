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
- [ 🟢 ] Permite modificar roles de usuario luego del registro.

### Publicación de productos
- [ 🟢 ] Solo los usuarios con rol `Vendedor` pueden publicar productos.
- [ 🟡 [1] ] El producto incluye nombre, descripción, precio, cantidad y categoría.
- [ 🟡 [2] ] El usuario puede visualizar sus propios productos publicados.

### Compra y gestión de órdenes
- [ 🟢] Solo usuarios con rol `Comprador` pueden crear órdenes de compra.
- [ 🟢 [3] ] Al comprar, se crea la orden y se descuenta el stock correctamente.
- [ 🟢 [4] ] La orden puede tener estado: `pendiente`, `enviado`, `recibido`.
- [ 🟢 ] Solo el `Vendedor` puede marcar la orden como `enviado`.
- [ 🟢 ] Solo el `Comprador` puede marcar la orden como `recibido`.
- [ 🟢 ] Las validaciones de permisos y estados se aplican correctamente.

---

## 2. Contrato desplegado en testnet

- [ 🟢 ] Se incluye la dirección (`address`) del contrato desplegado en **Shibuya Testnet**.
- [ 🟢 ] El contrato desplegado en testnet es **funcional** y permite interactuar con todas las funcionalidades requeridas.

---

## 3. Testing y calidad del código

- [ 🟡[5] ] Existe una suite de tests automatizados que cubre **≥ 85%** del código del contrato.
- [ 🟡[6] ] El código está bien estructurado y comentado según lo visto en clase.
- [ 🟢[7] ] Incluye documentación técnica clara para las funcionalidades implementadas.

#### Set mínimo de pruebas obligatorio:
- [ 🟡[8] ] Test de registro de usuario con cada rol posible.
- [ 🟢 ] Test de publicación de producto.
- [ 🟢[9] ] Test de compra de producto y generación de orden.
- [ 🟢[10] ] Test de cambio de estado de la orden (`pendiente` → `enviado` → `recibido`).
- [ 🟢 ] Test de validación de permisos (solo quien corresponde puede ejecutar cada acción).
- [ 🟢[11] ] Test de errores esperados (ej: intentar comprar sin stock, cambiar estado sin permisos, etc.).

---

## 4. Checklist QA – Verificación manual en Testnet (Shibuya)

**Antes de aprobar la entrega, probar en la testnet lo siguiente:**

### Registro y roles
- [ 🟢 ] Registrar un usuario nuevo como `Comprador` y verificar que se guarde correctamente.
- [ 🟢 ] Registrar un usuario nuevo como `Vendedor` y verificar que se guarde correctamente.
- [ 🟢 ] Registrar un usuario con ambos roles.
- [ 🟢 ] Cambiar el rol de un usuario y chequear que el cambio se refleje en el contrato.

### Publicación de productos
- [ 🟢 ] Como `Vendedor`, publicar al menos un producto con todos los datos requeridos.
- [ 🟡[12] ] Verificar que el producto figure en la lista de productos del vendedor.

### Compra y órdenes
- [ 🟢 ] Como `Comprador`, realizar la compra de un producto disponible.
- [ 🟢 ] Chequear que se descuente correctamente el stock tras la compra.
- [ 🟢 ] Verificar que la orden queda en estado `pendiente`.
- [ 🟢 ] Como `Vendedor`, cambiar el estado de la orden a `enviado`.
- [ 🟢 ] Como `Comprador`, cambiar el estado de la orden a `recibido`.

### Validaciones y errores esperados
- [ 🟢 ] Intentar comprar un producto sin stock y verificar que el contrato rechaza la operación.
- [ 🟡[13] ] Intentar que alguien que no sea el vendedor cambie el estado a `enviado` (debe fallar).
- [ 🟡[14] ] Intentar que alguien que no sea el comprador cambie el estado a `recibido` (debe fallar).
- [ 🟢 ] Intentar publicar un producto sin ser `Vendedor` (debe fallar).

### General
- [ 🟢[15] ] Confirmar que los cambios de estado y acciones se reflejan correctamente en el almacenamiento y pueden ser consultados vía RPC.

**En caso de detectar algún fallo o comportamiento incorrecto, dejar evidencia** (logs, capturas o referencias a transacciones) que ilustre el problema.

---

## 5. Observaciones y comentarios

> Anotar aquí cualquier observación relevante (errores encontrados, código confuso, validaciones ausentes, recomendaciones, etc.)

 > No tengo mucho para decir. Muy completo, muy bien el manejo de errores, funciona bastante bien.

[1] No cumple estrictamente. Producto sólo incluye nombre, descripción y categoría. El precio unitario de cada producto está en el struct Publicacion (publicacion.rs), y el stock de cada producto que tiene un usuario está en el struct StockProductos dentro de usuario.rs

[2] Cumple. Ver en producto.rs `pub(crate) fn _ver_stock_propio(..)` y `obtener_stock_productos(…)` en usuario.rs permite ver los IDs de los productos propios y su stock, aunque no el resto de su información.

[3] Aclaración: en el código “Pedido” es “Orden de compra”.

[4] Aclaración: “Enviado” es “Despachado”.

[5]   No llega al 85%, pero creo que se debe a la forma de organizar el código (structs: %89.62, lib.rs: 44.34%).
`test result: ok. 177 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.42s`
 ```
 Jul 25 18:09:47.651  INFO cargo_tarpaulin::report: Coverage Results:
 || Tested/Total Lines:
 || lib.rs: 47/106 +0.00%
 || structs/pedido.rs: 197/219 +0.00%
 || structs/producto.rs: 50/55 +0.00%
 || structs/publicacion.rs: 66/75 +0.00%
 || structs/usuario.rs: 136/152 +0.00%
 ||
 81.71% coverage, 496/607 lines covered, +0.00% change in coverage
 ```

[6]  Estructura dentro del lib.rs por un lado los ink!messages y por otro lado los “imports propios”, utiliza estos imports para hacer llamados a usuario.rs, publicacion.rs, etc. donde se implementan en sí los métodos y variables de cada estructura. Tiene Storage y el impl ink! más abajo. Muy limpio y modularizado. Documental frugal pero concisa. No es tal cual como lo dado en clase, pero está bien realizado. Sin embargo, a mi parecer cambiar de nombre a las cosas, como “Compras” en vez de “Órdenes” o “Despachado” en vez de “Enviado” genera bastante confusión a la hora de leer el código. Es algo a lo que te tenés que acostumbrar…

[7] Las funciones especifican los posibles errores y valores de retorno, pero no tienen ejemplos. A pesar de eso, es muy completa y clara en su simpleza, con muchos comentarios que ayudan muchísimo a la legibilidad.

[8] Sólo prueba explícitamente con comprador en usuario.rs `registrar_usuario_funciona_correctamente(..)`. Sin embargo, en `es_comprador_y_es_vendedor_funcionan_correctamente(..)` se testea la creación de todos los roles. Además, en otros tests que no tienen que ver con la creación de usuarios en sí, se revisa que los usuarios de distintos roles sean creados correctamente mediante `is_ok()`.

[9] Aclaración: “Orden” es “Pedido”.

[10] Aclaración: “Enviado” es “Despachado”. `fn compra_despachada_exitoso()` testea Pendiente → Despachado. `fn reclamar_fondos_exitoso()` testea Despachado → Recibido.

[11] Y el repertorio es muy completo.

[12] Al publicar Producto podemos publicar nombre, descripción, categoría y stock inicial, pero no precio. Y al realizar Publicación le podemos establecer un precio y decidir cuanta cantidad vender del stock que tiene el producto.

[13] Al cambiar el estado a enviado (“Despachado”) con un Comprador sale "TransaccionInexistente" en vez de “SoloVendedorPuede”.

[14] Cambiando el estado a Recibido con un Vendedor sale "PedidoInexistente" pero no “SoloCompradorPuede”.

[15] Ver aquí: https://shibuya.subscan.io/wasm_contract/aaUgbgCYnjHr6MVU2rNNnrp37zLM5jyZmpkUeXr48Zrvccx?tab=timeline

---

**Resultado general:**
- [x] APROBADO
- [ ] DESAPROBADO

---


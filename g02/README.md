# Checklist de Corrección y QA – Primera Entrega TP Final: Marketplace Descentralizado

**Materia:** Seminario de Lenguajes – Opción Rust
**Entrega:** Primera entrega obligatoria (18 de julio)
**Cobertura de tests requerida:** ≥ 85%

---

## 1. Requisitos funcionales obligatorios

### Registro y gestión de usuarios
- [ Cumple ] Permite registrar usuarios con rol `Comprador`, `Vendedor` o ambos.
- [ No está implementado ] Permite modificar roles de usuario luego del registro.

### Publicación de productos
- [ Cumple ] Solo los usuarios con rol `Vendedor` pueden publicar productos.
- [ Suficiente [1] ] El producto incluye nombre, descripción, precio, cantidad y categoría.
- [ No está implementado ] El usuario puede visualizar sus propios productos publicados.

### Compra y gestión de órdenes
- [ Cumple ] Solo usuarios con rol `Comprador` pueden crear órdenes de compra.
- [ Cumple ] Al comprar, se crea la orden y se descuenta el stock correctamente.
- [ Cumple ] La orden puede tener estado: `pendiente`, `enviado`, `recibido`.
- [ Cumple ] Solo el `Vendedor` puede marcar la orden como `enviado`.
- [ Cumple ] Solo el `Comprador` puede marcar la orden como `recibido`.
- [ Cumple ] Las validaciones de permisos y estados se aplican correctamente.

---

## 2. Contrato desplegado en testnet

- [ Cumple ] Se incluye la dirección (`address`) del contrato desplegado en **Shibuya Testnet**.
- [ Suficiente [2] ] El contrato desplegado en testnet es **funcional** y permite interactuar con todas las funcionalidades requeridas.

---

## 3. Testing y calidad del código

- [ Cumple [3] ] Existe una suite de tests automatizados que cubre **≥ 85%** del código del contrato.
- [ Cumple [4] ] El código está bien estructurado y comentado según lo visto en clase.
- [ Suficiente [5] ] Incluye documentación técnica clara para las funcionalidades implementadas.

#### Set mínimo de pruebas obligatorio:
- [ Cumple ] Test de registro de usuario con cada rol posible.
- [ Cumple ] Test de publicación de producto.
- [ Cumple ] Test de compra de producto y generación de orden.
- [ Cumple ] Test de cambio de estado de la orden (`pendiente` → `enviado` → `recibido`).
- [ Cumple ] Test de validación de permisos (solo quien corresponde puede ejecutar cada acción).
- [ Cumple ] Test de errores esperados (ej: intentar comprar sin stock, cambiar estado sin permisos, etc.).

---

## 4. Checklist QA – Verificación manual en Testnet (Shibuya)

**Antes de aprobar la entrega, probar en la testnet lo siguiente:**

### Registro y roles
- [ Cumple ] Registrar un usuario nuevo como `Comprador` y verificar que se guarde correctamente.
- [ Cumple ] Registrar un usuario nuevo como `Vendedor` y verificar que se guarde correctamente.
- [ Cumple ] Registrar un usuario con ambos roles.
- [ No está implementado ] Cambiar el rol de un usuario y chequear que el cambio se refleje en el contrato.

### Publicación de productos
- [ Cumple [6] ] Como `Vendedor`, publicar al menos un producto con todos los datos requeridos.
- [ No está implementado ] Verificar que el producto figure en la lista de productos del vendedor.

### Compra y órdenes
- [ Cumple ] Como `Comprador`, realizar la compra de un producto disponible.
- [ Cumple [7] ] Chequear que se descuente correctamente el stock tras la compra.
- [ Cumple [8] ] Verificar que la orden queda en estado `pendiente`.
- [ Cumple ] Como `Vendedor`, cambiar el estado de la orden a `enviado`.
- [ Cumple ] Como `Comprador`, cambiar el estado de la orden a `recibido`.

### Validaciones y errores esperados
- [ Cumple ] Intentar comprar un producto sin stock y verificar que el contrato rechaza la operación.
- [ Cumple ] Intentar que alguien que no sea el vendedor cambie el estado a `enviado` (debe fallar).
- [ Cumple ] Intentar que alguien que no sea el comprador cambie el estado a `recibido` (debe fallar).
- [ Cumple ] Intentar publicar un producto sin ser `Vendedor` (debe fallar).

### General
- [ Cumple [9] ] Confirmar que los cambios de estado y acciones se reflejan correctamente en el almacenamiento y pueden ser consultados vía RPC.

**En caso de detectar algún fallo o comportamiento incorrecto, dejar evidencia** (logs, capturas o referencias a transacciones) que ilustre el problema.

---

## 5. Observaciones y comentarios

> Anotar aquí cualquier observación relevante (errores encontrados, código confuso, validaciones ausentes, recomendaciones, etc.)

 Este es el trabajo del grupo que tenía problemas de gente que tiraba del carro y gente que era llevada, según contaban en las consultas. No se si desaprobarlos o no porque por un lado, no cumplen con los requerimientos mínimos (por dos puntos), pero por otro, el resto del trabajo está bastante bien en términos generales. Yo lo que haría en aprobarlos pero apartarlos y seriamente discutir si tienen la intención de continuar con la segunda entrega, y considerar dividir el grupo.

[1]  Cumple. Aunque Publicación y Producto son una sola estructura.
[2]  El contrato compila y funcional. Aunque no hay “Publicaciones” y faltan algunas funcionalidades como poder visualizar los propios productos publicados y modificar roles de usuario luego del registro. Pero las que sí están implementadas funcionan.
[3]  Tarpaulin arroja un **%89.08** de coverage.
 test result: ok. 26 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.07s
 Jul 25 17:01:16.853  INFO cargo_tarpaulin::report: Coverage Results:
 || Tested/Total Lines:
 || marketplace_principal/lib.rs: 106/119 +0.00%
 ||
 89.08% coverage, 106/119 lines covered, +0.00% change in coverage
 No tiene tests E2E.
[4]  En cuanto a estructura y comentarios auxiliares a la documentación, está bien. Se ve que distintas personas escribieron distintas partes del código por estilos ligeramente distintos, pero no es súper notorio ni dificulta el entendimiento del mismo.
[5]  Las funciones muestran los posibles errores y valores de retorno, pero no tienen ejemplos. registrar_usuario_interno, crear_producto_seguro, crear_nueva_orden, actualizar_estado_orden, no tienen documentación clara (sólo dicen “Lógica interna de metodo_publico() tal” y no explicitan cuál es esa lógica). Mismo la sección de funciones auxiliares y ENUMS, no se explayaron mucho.
[6] Aunque no hay Publicaciones, Producto es la única estructura que hay.
[7] Aunque no vea la lista de productos, si intento hacer otra compra del mismo producto (id = 0, originalmente con stock = 1),provoca un error "StockInsuficiente".
[8] Aunque no vea la lista de órdenes, si intento hacer que el vendedor le cambie el estado a “enviada” (orden_id = 0), muestra en pantalla que "Contract call will be successful!".
[9] Ver aquí: https://shibuya.subscan.io/wasm_contract/WJ8nHZ8n4P9k8193yWK2oLgoFiWvNVJg2WJhZ8oom59NikM?tab=transaction

---

**Resultado general:**
- [ O ] APROBADO
- [ ] DESAPROBADO

---


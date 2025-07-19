# Checklist de Corrección y QA – Primera Entrega TP Final: Marketplace Descentralizado

**Materia:** Seminario de Lenguajes – Opción Rust  
**Entrega:** Primera entrega obligatoria (18 de julio)  
**Cobertura de tests requerida:** ≥ 85%

---

## 1. Requisitos funcionales obligatorios

### Registro y gestión de usuarios
- [ ] Permite registrar usuarios con rol `Comprador`, `Vendedor` o ambos.
- [ ] Permite modificar roles de usuario luego del registro.

### Publicación de productos
- [ ] Solo los usuarios con rol `Vendedor` pueden publicar productos.
- [ ] El producto incluye nombre, descripción, precio, cantidad y categoría.
- [ ] El usuario puede visualizar sus propios productos publicados.

### Compra y gestión de órdenes
- [ ] Solo usuarios con rol `Comprador` pueden crear órdenes de compra.
- [ ] Al comprar, se crea la orden y se descuenta el stock correctamente.
- [ ] La orden puede tener estado: `pendiente`, `enviado`, `recibido`.
- [ ] Solo el `Vendedor` puede marcar la orden como `enviado`.
- [ ] Solo el `Comprador` puede marcar la orden como `recibido`.
- [ ] Las validaciones de permisos y estados se aplican correctamente.

---

## 2. Contrato desplegado en testnet

- [ ] Se incluye la dirección (`address`) del contrato desplegado en **Shibuya Testnet**.
- [ ] El contrato desplegado en testnet es **funcional** y permite interactuar con todas las funcionalidades requeridas.

---

## 3. Testing y calidad del código

- [ ] Existe una suite de tests automatizados que cubre **≥ 85%** del código del contrato.
- [ ] El código está bien estructurado y comentado según lo visto en clase.
- [ ] Incluye documentación técnica clara para las funcionalidades implementadas.

#### Set mínimo de pruebas obligatorio:
- [ ] Test de registro de usuario con cada rol posible.
- [ ] Test de publicación de producto.
- [ ] Test de compra de producto y generación de orden.
- [ ] Test de cambio de estado de la orden (`pendiente` → `enviado` → `recibido`).
- [ ] Test de validación de permisos (solo quien corresponde puede ejecutar cada acción).
- [ ] Test de errores esperados (ej: intentar comprar sin stock, cambiar estado sin permisos, etc.).

---

## 4. Checklist QA – Verificación manual en Testnet (Shibuya)

**Antes de aprobar la entrega, probar en la testnet lo siguiente:**

### Registro y roles
- [ ] Registrar un usuario nuevo como `Comprador` y verificar que se guarde correctamente.
- [ ] Registrar un usuario nuevo como `Vendedor` y verificar que se guarde correctamente.
- [ ] Registrar un usuario con ambos roles.
- [ ] Cambiar el rol de un usuario y chequear que el cambio se refleje en el contrato.

### Publicación de productos
- [ ] Como `Vendedor`, publicar al menos un producto con todos los datos requeridos.
- [ ] Verificar que el producto figure en la lista de productos del vendedor.

### Compra y órdenes
- [ ] Como `Comprador`, realizar la compra de un producto disponible.
- [ ] Chequear que se descuente correctamente el stock tras la compra.
- [ ] Verificar que la orden queda en estado `pendiente`.
- [ ] Como `Vendedor`, cambiar el estado de la orden a `enviado`.
- [ ] Como `Comprador`, cambiar el estado de la orden a `recibido`.

### Validaciones y errores esperados
- [ ] Intentar comprar un producto sin stock y verificar que el contrato rechaza la operación.
- [ ] Intentar que alguien que no sea el vendedor cambie el estado a `enviado` (debe fallar).
- [ ] Intentar que alguien que no sea el comprador cambie el estado a `recibido` (debe fallar).
- [ ] Intentar publicar un producto sin ser `Vendedor` (debe fallar).

### General
- [ ] Confirmar que los cambios de estado y acciones se reflejan correctamente en el almacenamiento y pueden ser consultados vía RPC.

**En caso de detectar algún fallo o comportamiento incorrecto, dejar evidencia** (logs, capturas o referencias a transacciones) que ilustre el problema.

---

## 5. Observaciones y comentarios

> Anotar aquí cualquier observación relevante (errores encontrados, código confuso, validaciones ausentes, recomendaciones, etc.)

---

**Resultado general:**  
- [ ] APROBADO  
- [ ] DESAPROBADO

---


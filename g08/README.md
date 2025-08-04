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
- [ 🟡 [1] ] Permite modificar roles de usuario luego del registro.

### Publicación de productos
- [ 🟢 ] Solo los usuarios con rol `Vendedor` pueden publicar productos.
- [ 🟢 [2] ] El producto incluye nombre, descripción, precio, cantidad y categoría.
- [ 🔴 [3] ] El usuario puede visualizar sus propios productos publicados.

### Compra y gestión de órdenes
- [ 🟢 ] Solo usuarios con rol `Comprador` pueden crear órdenes de compra.
- [ 🔴 [4] ] Al comprar, se crea la orden y se descuenta el stock correctamente.
- [ 🟢 ] La orden puede tener estado: `pendiente`, `enviado`, `recibido`.
- [ 🟢 ] Solo el `Vendedor` puede marcar la orden como `enviado`.
- [ 🟢 ] Solo el `Comprador` puede marcar la orden como `recibido`.
- [ 🟡 [5] ] Las validaciones de permisos y estados se aplican correctamente.

---

## 2. Contrato desplegado en testnet

- [ 🟢 ] Se incluye la dirección (`address`) del contrato desplegado en **Shibuya Testnet**.
- [ 🟡 [6] ] El contrato desplegado en testnet es **funcional** y permite interactuar con todas las funcionalidades requeridas.

---

## 3. Testing y calidad del código

- [ 🟢 [7] ] Existe una suite de tests automatizados que cubre **≥ 85%** del código del contrato.
- [ 🔴 [8] ] El código está bien estructurado y comentado según lo visto en clase.
- [ 🟡 [9] ] Incluye documentación técnica clara para las funcionalidades implementadas.

#### Set mínimo de pruebas obligatorio:
- [ 🟢 ] Test de registro de usuario con cada rol posible.
- [ 🟢 ] Test de publicación de producto.
- [ 🟢 ] Test de compra de producto y generación de orden.
- [ 🟢 [10] ] Test de cambio de estado de la orden (`pendiente` → `enviado` → `recibido`).
- [ 🟢 ] Test de validación de permisos (solo quien corresponde puede ejecutar cada acción).
- [ 🟢 ] Test de errores esperados (ej: intentar comprar sin stock, cambiar estado sin permisos, etc.).

---

## 4. Checklist QA – Verificación manual en Testnet (Shibuya)

**Antes de aprobar la entrega, probar en la testnet lo siguiente:**

### Registro y roles
- [ 🟢 ] Registrar un usuario nuevo como `Comprador` y verificar que se guarde correctamente.
- [ 🟢 ] Registrar un usuario nuevo como `Vendedor` y verificar que se guarde correctamente.
- [ 🟢 ] Registrar un usuario con ambos roles.
- [ 🟡 [11] ] Cambiar el rol de un usuario y chequear que el cambio se refleje en el contrato.

### Publicación de productos
- [ 🔴 [12] ] Como `Vendedor`, publicar al menos un producto con todos los datos requeridos.
- [ 🔴 [13] ] Verificar que el producto figure en la lista de productos del vendedor.

### Compra y órdenes
- [ 🟡 [14] ] Como `Comprador`, realizar la compra de un producto disponible.
- [ 🔴 [15] ] Chequear que se descuente correctamente el stock tras la compra.
- [ 🟡 [16] ] Verificar que la orden queda en estado `pendiente`.
- [ 🟡 [17] ] Como `Vendedor`, cambiar el estado de la orden a `enviado`.
- [ 🟡 [18] ] Como `Comprador`, cambiar el estado de la orden a `recibido`.

### Validaciones y errores esperados
- [ 🔴 [19] ] Intentar comprar un producto sin stock y verificar que el contrato rechaza la operación.
- [ 🟢 [20] ] Intentar que alguien que no sea el vendedor cambie el estado a `enviado` (debe fallar).
- [ 🟡 [21] ] Intentar que alguien que no sea el comprador cambie el estado a `recibido` (debe fallar).
- [ 🟢 ] Intentar publicar un producto sin ser `Vendedor` (debe fallar).

### General
- [ 🟢 [22] ] Confirmar que los cambios de estado y acciones se reflejan correctamente en el almacenamiento y pueden ser consultados vía RPC.

**En caso de detectar algún fallo o comportamiento incorrecto, dejar evidencia** (logs, capturas o referencias a transacciones) que ilustre el problema.

---

## 5. Observaciones y comentarios

> Anotar aquí cualquier observación relevante (errores encontrados, código confuso, validaciones ausentes, recomendaciones, etc.)

 > Este código se me hizo muy dificil de leer y seguir. De no ser por la documentación creo que me hubiera rendido. Las órdenes de compra están demasiado rotas para aprobar… El stock no se maneja bien, permitiendo que se realicen sobre-ventas. Tampoco se respetan los cambios de roles requeridos, pero me hace falta testear más a fondo para determinar si la lógica que diseñaron e implementaron funciona 100% bien. Me da pena porque se nota el esmero. Se me ocurre que no llegaron a hacer testeos manuales al contrato ya desplegado y al ver que los tests de unidad corrían, se durmieron en los laureles.


[1] Cumple. Pero no debería poder cambiar rol de Vendedor a Comprador, no cumple con los requerimientos discutidos en las consultas. Sin embargo, construyeron cierta lógica para manejar eso.

[2] Cumple, aunque es un poco confuso porque el stock está en el Mapping dentro del Storage, en vez de en el Producto directamente.
```rust
/// Los productos se almacenan en un Mapping. Donde la clave es el id del producto. y su contenido es una tupla que contiene los datos del producto y el stock de éste. <id, (producto, stock)>.
historial_productos: Mapping<u32, (Producto, u32)>,
```
[3] Se pueden ver las publicaciones de cualquier usuario. Pero no los Productos propios.

[4] Es en la Publicación donde se determina el stock del producto, no en Producto. En `impl OrdenCompra { .. }` está el stock que se compra que se obtiene directamente de la publicación al instanciarse la Orden. **Pareciera que cada orden de compra tiene que estrictamente comprar todo el stock que marque una publicación**.  Como no se puede crear una publicación con mayor stock que la que marca el producto, no se testea un intento de creación de orden de compra con más cantidad pedida que el stock que figura en la publicación (Ver: `test_crear_orden_con_stock_insuficiente_falla(..)`)

[5] `fn modificar_rol(...)` sólo limita el cambio de rol si el viejo rol y el nuevo rol son iguales. Pero cualquier otro pase es válido, cambia el rol inmediatamente sin limitar el cambio de Comprador a Vendedor y viceversa.  Si es Comprador y pasa a Vendedor o viceversa crea las estructuras propias de cada rol si es que no están ya instanciadas.

[6] Está desplegada correctamente en Aleph Zero Testnet, sin embargo, tiene funcionalidades que no están implementadas y funcionan a medias.

[7] Me llama la atención que testearan tantos casos (que en general están bastante bien) y lograran semejante cobertura, pero a la hora de testear manualmente el contrato ya desplegado en la Testnet fallaran tantas cosas. Quizás no pensaron suficientes casos límite.
```
 test result: ok. 37 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.11s
 Jul 30 16:43:59.538  INFO cargo_tarpaulin::report: Coverage Results:
 || Tested/Total Lines:
 || lib.rs: 203/208
 ||
 97.60% coverage, 203/208 lines covered
 ```
 No hay tests E2E
```test result: ok. 37 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
   Doc-tests primer_contrato
 running 0 tests
 test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

[8]  Es bastante difícil de seguir, tiene nombres de funciones repetidos en las distintas implementaciones de los structs, funciones muy largas y se fragmenta la lógica de negocio de forma muy dispersa a lo largo de todo el código. Los comentarios ayudan pero en general me resulta difícil de seguir.

[9] No es documentación “técnica”, ni en vocabulario ni en estructura, explica redactado como un cuentito lo que hace. No incluye ejemplos, ni errores posibles.

[10] Muy completos y prolija la separación de casos de prueba.

[11] Comprador → Ambos → Vendedor → Comprador … Puede cambiar de cualquier rol a cualquir rol nuevo siempre y cuando no sea el mismo.

[12] No es muy user-friendly la forma de publicar… `crearPublicacion(..)` recibe como parámetro un `Vec<(u32,u32)>` pero no informa qué representa cada uno de esos números u32. Un Comprador no puede crear una Publicación de cierto Producto particular, pero cualquier Vendedor o C-V puede, incluso aquellos vendedores que NO cargaron ese Producto. Parece que el Vendedor no es dueño de un Producto sino de un Producto Publicado, nada evita que cualquier Vendedor se apropie y publique cualquier Producto que ya haya sido cargado al sistema. También puedo cargar un producto con stock 0 y precio 0, pero no puedo crear publicación con este producto por falta de stock.

> ![[Grupo_8_stockcero_preciocero.png]]
> > Captura de pantalla: Vendedor publicando un producto con stock creo y precio cero.

[13] Figura en la lista de Publicaciones, no hay listado de Productos. Y son las publicaciones en general, no propias del Vendedor. Cualquier Comprador puede visualizar esta lista también.

[14] Cumple… Pero pude hacer 3 ordenes de compra iguales de un Producto con stock 1.  <br>Y permite que haga tantas órdenes como desee.

[15]  No cambia nada al visualizar las publicacioens. Stock sigue siendo “1” a pesar de haber hecho 3 órdenes de compra.
```rust
 {
  Ok: {
    id: '0',
    productos: [
      [
        '1',
        '1',
      ],
    ],
    precioFinal: '1',
    idVendedor: 'a7LiSboqVgo8TUyB17GX4sZ8xjrspJUzz3mZ9kD1w7ue64W',
  },
 }
 ```
 Los controles de permisos y roles están y funcionan, pero **el control de stock no funciona correctamente**.

[16]  No se puede visualizar las órdenes de compra propias, pero se puede inferir a partir de `enviar_compra(…)` que están pendientes porque se permite enviar la orden de compra de ID 0, 1 y 2 (Las tres órdenes de compra que realicé).

[17] Cumple. Si el Vendedor no es el correspondiente, sale un error "La publicacion buscada no pertenece a este vendedor." Pero pude enviar la orden tres veces, a pesar de que el stock era “1”.|

[18] Cumple. Aunque si trato de de cambiar el estado con una cuenta de Comprador distinta al que creó la orden de compra, el error que figura es “No se encontro la orden de compra” en vez de un error que se ajuste mejor a la situación. Pero pude recibir la orden tres veces, a pesar de que el stock era “1”.

[19] Puedo crear la orden de compra de la Publicación ID 0 (con una cuenta Compradora distinta a la de recién),  que ya fue ordenada, enviada y recibida tres veces a pesar de tener stock 1. En este punto ya debería tener stock -2, pero aún así logré hacer la nueva orden de compra.

[20] Cumple, y el error es acorde "El usuario no posee el rol de vendedor." si es un Comprador,  y “La publicacion buscada no pertenece a este vendedor.” si el usuario es Vendedor o C-V.

[21] Cumple. Pero el error que marca si el usuario es Vendedor es "El producto todavía no fue enviado." o "No se encontró la orden de compra." si el usuario es otro Comprador.

[22] Cumple. No encontré el contrato en Subscan https://alephzero.subscan.io/account/5CQdp3P2RT4eHSxRU8o1RBWTs4dmTwjycub92HyGdjpTnexP, pero confío.

---

**Resultado general:**
- [ ] APROBADO
- [x] DESAPROBADO

---


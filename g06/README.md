# Checklist de Corrección y QA – Primera Entrega TP Final: Marketplace Descentralizado

**Materia:** Seminario de Lenguajes – Opción Rust
**Entrega:** Primera entrega obligatoria (18 de julio)
**Cobertura de tests requerida:** ≥ 85%

---

## 1. Requisitos funcionales obligatorios

### Registro y gestión de usuarios
- [ Cumple ] Permite registrar usuarios con rol `Comprador`, `Vendedor` o ambos.
- [ Suficiente [1] ] Permite modificar roles de usuario luego del registro.

### Publicación de productos
- [ Cumple ] Solo los usuarios con rol `Vendedor` pueden publicar productos.
- [ Suficiente [2] ] El producto incluye nombre, descripción, precio, cantidad y categoría.
- [ No está implementado ] El usuario puede visualizar sus propios productos publicados.

### Compra y gestión de órdenes
- [ No cumple ] Solo usuarios con rol `Comprador` pueden crear órdenes de compra.
- [ Cumple ] Al comprar, se crea la orden y se descuenta el stock correctamente.
- [ Suficiente [3] ] La orden puede tener estado: `pendiente`, `enviado`, `recibido`.
- [ Insuficiente [4] ] Solo el `Vendedor` puede marcar la orden como `enviado`.
- [ Insuficiente [5] ] Solo el `Comprador` puede marcar la orden como `recibido`.
- [ Suficiente [6] ] Las validaciones de permisos y estados se aplican correctamente.

---

## 2. Contrato desplegado en testnet

- [ Cumple ] Se incluye la dirección (`address`) del contrato desplegado en **Shibuya Testnet**.
- [ Suficiente [7] ] El contrato desplegado en testnet es **funcional** y permite interactuar con todas las funcionalidades requeridas.

---

## 3. Testing y calidad del código

- [ Suficiente [8] ] Existe una suite de tests automatizados que cubre **≥ 85%** del código del contrato.
- [ Suficiente [9] ] El código está bien estructurado y comentado según lo visto en clase.
- [ Insuficiente [10] ] Incluye documentación técnica clara para las funcionalidades implementadas.

#### Set mínimo de pruebas obligatorio:
- [ Suficiente [11] ] Test de registro de usuario con cada rol posible.
- [ Cumple ] Test de publicación de producto.
- [ Insuficiente [12] ] Test de compra de producto y generación de orden.
- [ Insuficiente [13] ] Test de cambio de estado de la orden (`pendiente` → `enviado` → `recibido`).
- [ Suficiente [14] ] Test de validación de permisos (solo quien corresponde puede ejecutar cada acción).
- [ Suficiente [15] ] Test de errores esperados (ej: intentar comprar sin stock, cambiar estado sin permisos, etc.).

---

## 4. Checklist QA – Verificación manual en Testnet (Shibuya)

**Antes de aprobar la entrega, probar en la testnet lo siguiente:**

### Registro y roles
- [ Cumple ] Registrar un usuario nuevo como `Comprador` y verificar que se guarde correctamente.
- [ Suficiente [16] ] Registrar un usuario nuevo como `Vendedor` y verificar que se guarde correctamente.
- [ Suficiente [17] ] Registrar un usuario con ambos roles.
- [ Suficiente [18] ] Cambiar el rol de un usuario y chequear que el cambio se refleje en el contrato.

### Publicación de productos
- [ Suficiente [19] ] Como `Vendedor`, publicar al menos un producto con todos los datos requeridos.
- [ No está implementado ] Verificar que el producto figure en la lista de productos del vendedor.

### Compra y órdenes
- [ Insuficiente [20] ] Como `Comprador`, realizar la compra de un producto disponible.
- [ Suficiente [21] ] Chequear que se descuente correctamente el stock tras la compra.
- [ Suficiente [22] ] Verificar que la orden queda en estado `pendiente`.
- [ No está implementado ] Como `Vendedor`, cambiar el estado de la orden a `enviado`.
- [ No está implementado ] Como `Comprador`, cambiar el estado de la orden a `recibido`.

### Validaciones y errores esperados
- [ Cumple ] Intentar comprar un producto sin stock y verificar que el contrato rechaza la operación.
- [ No está implementado ] Intentar que alguien que no sea el vendedor cambie el estado a `enviado` (debe fallar).
- [ No está implementado ] Intentar que alguien que no sea el comprador cambie el estado a `recibido` (debe fallar).
- [ Cumple ] Intentar publicar un producto sin ser `Vendedor` (debe fallar).

### General
- [ Cumple [23] ] Confirmar que los cambios de estado y acciones se reflejan correctamente en el almacenamiento y pueden ser consultados vía RPC.

**En caso de detectar algún fallo o comportamiento incorrecto, dejar evidencia** (logs, capturas o referencias a transacciones) que ilustre el problema.

---

## 5. Observaciones y comentarios

> Anotar aquí cualquier observación relevante (errores encontrados, código confuso, validaciones ausentes, recomendaciones, etc.)

[1] Puede pasar de Vendedor a V-C, de Comprador a Vendedor o a V-C, y de V-C a Vendedor nuevamente. Esto no cumple estrictamente con los requerimientos discutidos en las consultas, que dictaban que sólo se pudiera “aumentar” un rol de Vendedor o Comprador a Ambos. Parece que pensaron la lógica de esta forma para no tener que borrar un Inventario después de crearlo para un V o V-C, pero podría generar problemas si por ejemplo se cambia de rol Comprador a Vendedor y éste tiene una orden pendiente o sin recibir.

[2] No cumple estrictamente. Producto incluye los campos: id, nombre, categoria y cantidad (stock). Pero no precio. El precio está contenido dentro de la estructura de Publicación.

[3] Cumple de cierta forma, aunque la función cambiar_estado(…) dice no estar en su versión final.
```rust
//Caso de cambiar de estado validando roles de usuarios
   /// Cambia el estado interno de la orden
   pub fn cambiar_estado(&mut self, usuario: Usuario, estado_nuevo: EstadoOrden) -> Result<bool, Errores> {
      //Se necesita un get_rol() que nos permita verificar el rol del usuario
      match usuario.get_rol() {
      //Primera implementacion - no final - de como desarrollar el metodo
```

[4] No cumple. Lógica incorrecta. Según cambiar_estado(…) el Comprador es quien hace pasar el estado a Enviado.
```rust
   Roles::Comprador => {
      match estado_nuevo {
         EstadoOrden::Enviado => {
            self.estado = estado_nuevo;
            Ok(true)
         },
         _ => Err(Errores::PermisoDenegado)

      }
   },
```

[5]  No cumple. Lógica incorrecta. Según cambiar_estado(…) el Vendedor es quien hace pasar el estado a Enviado.
```rust
   Roles::Vendedor => {
      match estado_nuevo {
         EstadoOrden::Recibido => {
            self.estado = estado_nuevo;
            Ok(true)
         },
```

[6] En los estados de la Orden, no. En cuanto a los roles del usuario, en parte.

[7] Son pocas las funcionalidades mínimas que funcionan correctamente. Pero el contrato está correctamente desplegado.

[8] `test result: ok. 16 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s`
```
 Jul 28 16:55:37.955  INFO cargo_tarpaulin::report: Coverage Results:
 || Tested/Total Lines:
 || lib.rs: 144/168
 ||
 85.71% coverage, 144/168 lines covered
```
Cuando corro los tests E2E no compila el contrato.
```
For more information about this error, try `rustc --explain E0107`.
error: could not compile `marketplace` (lib test) due to 4 previous errors; 9 warnings emitted
warning: build failed, waiting for other jobs to finish...

error[E0107]: function takes 0 generic arguments but 1 generic argument was supplied
   --> lib.rs:803:28
error[E0107]: function takes 0 generic arguments but 1 generic argument was supplied
   --> lib.rs:806:13
error[E0107]: function takes 0 generic arguments but 1 generic argument was supplied
   --> lib.rs:811:13
note: function defined here, with 0 generic parameters
   --> lib.rs:782:20
error[E0107]: function takes 0 generic arguments but 1 generic argument was supplied
   --> lib.rs:823:13
```

[9] Bien estructurado. Me gustó la separación por módulos de los tests de unidad. Tiene comentarios que ayudan con la legibilidad del código en algunas partes, pero en general no las tiene.

[10] No es documentación técnica, no incluye ejemplos y en algunos casos ni siquiera explica los diferentes outputs o errores que pueden surgir al correr una función. Tampoco describe los distintos inputs posibles. A veces ni siquiera explica qué hace la función sino que sólo repite el nombre de la misma.

[11]  No hay test explícitamente dedicado a la creación de usuarios con distintos roles. En test_usuario_puede_vender() prueba la creación de un Vendedor y un Comprador, pero no de un VendedroComprador.

[12] En el test el ID que se genera de Comprador y Vendedor son el mismo, por lo que se considera válida una auto-compra.
```rust
test_crear_orden_valida() {
   let orden = OrdenCompra::crear_orden(1, **crear_id(1)**, **crear_id(1)**, 10, 5);
   assert!(orden.is_ok());
```

[13] Hay dos tests, uno prueba que un usuario distinto al que hizo la orden de compra pueda marcarla como Enviada. Funciona, sí. En una auto-compra donde Vendedor y Comprador son la misma AccountID, un tercer usuario puede marcarla como “Enviada”. Pero no es un test que pruebe una funcionalidad pedida.
```rust
fn test_cambiar_estado_por_comprador() {
   match OrdenCompra::crear_orden(1, crear_id(1), crear_id(1), 10, 2) {
      Ok(mut orden) => {
         let usuario = Usuario::new("comprador".to_string(), 100, crear_id(2), Roles::Comprador);
         let resultado = orden.cambiar_estado(usuario, EstadoOrden::Enviado);
         assert_eq!(resultado, Ok(true));
         assert_eq!(orden.estado, EstadoOrden::Enviado);
         },
      Err(e) => panic!("fallo orden {:?}", e),
   }
}
```
Luego, está este test. Donde también se realiza una auto-compra  y donde se comprueba que un Vendedor no puede marcar una orden como “Enviada”, cuando es en realidad al revés, un Vendedor debería poder hacer eso. Lo que me llama la atención también es que se pueda crear el nuevo usuario considerando que los AccountID están repetidos…
```rust
fn test_cambiar_estado_permiso_denegado() {
   let mut orden=OrdenCompra::crear_orden(1, crear_id(1), crear_id(1), 10, 2).unwrap();
   let usuario=Usuario::new("otro".to_string(), 101, crear_id(1), Roles::Vendedor);
   let resultado=orden.cambiar_estado(usuario, EstadoOrden::Enviado);
   assert_eq!(resultado,Err(Errores::PermisoDenegado));
}
```
Creo… que como `cambiar_estado()` no chequea que el usuario caller esté registrado, `Usuario::new` seguramente falla y `cambiar_estado()` no encuentra nada en `usuario.rol`, por lo que escupe directamente un `Err(Errores::PermisoDenegado)` pero no por las razones que se asumen en el test.

[14] En `test_alta_orden_todos_los_casos()` se testea bastante, pero no hay suficientes tests que explícitamente chequeen el correcto funcionamiento de los permisos distintos de los Vendedores y Compradores.

[15] En `test_alta_orden_todos_los_casos()` se testea bastante, compra sin stock, comprador o producto inexistente, etc. Creo que `fn cambiar_de_rol(&mut self, rol_nuevo: Roles) -> Result<bool, Errores>` permite cambiarse de Vendedor a Comprador aunque devuelva error inmediatamente después. Ver:
```rust
 _ => {
   usuario.rol = rol_nuevo.clone(); // ¡ACÁ CAMBIA EL ROL! (usuario.rol ES VENDEDOR, rol_nuevo ES COMPRADOR)
   let _ =match rol_nuevo {
     Roles::Vendedor => {
       self.inventarios.insert(&caller,&Inventario::new());
     },
     Roles::CompradorVendedor => {
       self.inventarios.insert(&caller, &Inventario::new());
     },
     _ => {
       return Err(Errores::ConflictoRol); /Y RECIÉN ACÁ FALLA, PERO EL ROL YA FUE MODIFICADO
     }
```

[16] Cumple. Pero, pude hacer que Vendedor y Comprador tengan el mismo número de DNI, que según su propio código no debía pasar.

[17] Cumple. Pude hacer que tanto Vendedor como Comprador y VendedorComprador tengan todos el mismo DNI número 12345678. No era requerimiento pero en la propia documentación/comentarios del código decía que: `“(dni e id deben ser unicos dentro del sistema)”` así que bajo un poco de puntos.

[18] Cumple. Puede pasar de Vendedor a V-C, de Comprador a Vendedor o a V-C, y de V-C a Vendedor nuevamente. Esto no cumple estrictamente con los requerimientos discutidos en las consultas, que dictaban que sólo se pudiera “aumentar” un rol de Vendedor o Comprador a Ambos. Parece que pensaron la lógica de esta forma para no tener que borrar un Inventario después de crearlo para un V o V-C, pero podría generar problemas si se cambia de rol Comprador a Vendedor si éste tiene una orden pendiente o sin recibir.

[19] `agregarProducto(stockTotal, nombre, categoria)` sólo recibe la cantidad de stock, nombre del producto y categoría. El precio se agrega directamente en la publicación.

[20] Cumple. Pero no necesito ingresar dinero para comprar. Además, el mismo Vendedor puede auto-comprarse y cualquier otro usuario con rol Vendedor puede hacer una orden.

[21] Cumple. Aunque no vea la lista de productos, si intento hacer otra compra del mismo producto (id = 0, originalmente con stock = 1), provoca un error "ProductoSinStock".

[22]  No hay método que muestre un listado de órdenes propias. Tampoco se muestra al dar de alta una orden de compra los detalles de la misma, como su estado. Tampoco puedo deducirlo a través de algún método que le cambie el estado porque no hay ninguno implementado.

[23] Cumple. Ver https://shibuya.subscan.io/wasm_contract/Y6tvgmeT3HrUYXzCp4jWXjsbeGvnTG7N2yidMFr75jUmenu?tab=transaction

---

**Resultado general:**
- [ ] APROBADO
- [x] DESAPROBADO

---


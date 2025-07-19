#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod marketplace {

    use ink::prelude::string::String;
    use scale_info::prelude::format;
    use ink::prelude::vec::Vec;
    use ink::storage::{Mapping, StorageVec};

    #[derive(Debug, PartialEq)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    #[ink::scale_derive(Decode, Encode, TypeInfo)]
    /// Errores del sistema
    pub enum Errores{
        NoEsComprador,
        NombreNulo,
        UsuarioExistente,
        ConflictoDni,
        UsuarioInexistente,
        ConflictoRol,
        ConsentimientoFaltante, // no llegaron a un acuerdo para cancelar
        PermisoDenegado, // si un comprador quiere hacer algo que le correponde a un vendedor y viceversa 
        OrdenNoExistente,
        ProductoNoEncontrado,
        CompradorNoEncontrado,
        ProductoSinStock,
        ProductoSinDescripcion,
        PrecioNulo,
        NoEsVendedor,
        VendedorNoEncontrado,
        SinInventario,
    }

    #[derive(Clone, Debug,PartialEq)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    #[ink::scale_derive(Decode, Encode, TypeInfo)]
    /// Roles de los usuarios
    pub enum Roles {
        Comprador,
        Vendedor,
        CompradorVendedor,
    }

    #[derive(Clone, Debug)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    #[ink::scale_derive(Decode, Encode, TypeInfo)]
    /// Usuario del sistema
    pub struct Usuario {
        nombre: String,
        dni: u64,
        id: AccountId,
        rol: Roles,
        calificaciones: Vec<u8>,
    }

    impl Usuario{
        /// Creacion de un nuevo usuario
        pub fn new(nom: String, dni_in:u64, id_in:AccountId,r: Roles)->Usuario{
            return Usuario{
                nombre : nom,
                dni : dni_in,
                id : id_in,
                rol : r,
                calificaciones : Vec::new() //Futura implementacion
            }
        }
        /// Se obtiene un rol nuevo
        pub fn obtener_nuevo_rol(&mut self,r: Roles){
            self.rol = r.clone();
        }
        /// Devuelve si puede vender o no
        pub fn puede_vender(&self)->bool{
            matches!(self.rol, Roles::Vendedor | Roles::CompradorVendedor)
        }
        /// Devuelve el rol actual
        pub fn get_rol(&self)->Roles{
            self.rol.clone()
        }
        /// Devuelve el ID
        pub fn get_id(&self)->AccountId{
            self.id
        }
        /// Devuelve el nombre
        pub fn get_nom(&self)->String{
            self.nombre.clone()
        }
        /// Devuelve el dni
        pub fn get_dni(&self)->u64{
            self.dni
        }
    }    

    #[derive(Clone, Debug, PartialEq)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    #[ink::scale_derive(Decode, Encode, TypeInfo)]
    /// Categoria de un producto
    pub enum Categoria {
        Limpieza,
        Hogar,
        Deporte,
        Electronica,
    }

    #[derive(Clone, Debug, PartialEq)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    #[ink::scale_derive(Decode, Encode, TypeInfo)]
    /// Producto
    pub struct Producto {
        id:u64,
        nombre:String,
        categoria:Categoria,
        cantidad: u64,
    }
    impl Producto{
        /// Creacion de un nuevo producto
        fn new(id:u64,nombre:String,categoria:Categoria, cantidad: u64)->Producto{
            Producto{
                id,
                nombre,
                categoria, 
                cantidad
            }
        } 
        /// Se devuelve el producto
        fn get_producto(&self)->Producto{
            self.clone()
        }
    }

    #[derive(Clone, Debug, PartialEq)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    #[ink::scale_derive(Decode, Encode, TypeInfo)]
    /// Estado de la orden de compra
    pub enum EstadoOrden {
        Pendiente,
        Enviado,
        Recibido,
        Cancelada,
    }


    #[derive(Clone, Debug, PartialEq)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    #[ink::scale_derive(Decode, Encode, TypeInfo)]    
    /// Orden de compra
    pub struct OrdenCompra{
        id_orden:u64,
        id_comprador:AccountId,
        id_vendedor:AccountId,
        id_producto:u64,
        cantidad:u64,
        estado:EstadoOrden,
    }
    impl OrdenCompra {
        /// Se crea una nueva orden
        fn crear_orden(id_ord:u64, comprador : AccountId,  id_vend:AccountId, id_prod:u64, cant:u64) -> Result<OrdenCompra, Errores> {
            
            if cant == 0 {
                return Err(Errores::ProductoSinStock);
            }

            Ok(OrdenCompra{
                id_orden: id_ord, 
                id_comprador: comprador, 
                id_vendedor: id_vend, 
                id_producto: id_prod, 
                cantidad: cant,
                estado: EstadoOrden::Pendiente
            })

        }

        //Caso de cambiar de estado validando roles de usuarios
        /// Cambia el estado interno de la orden
        pub fn cambiar_estado(&mut self, usuario: Usuario, estado_nuevo: EstadoOrden) -> Result<bool, Errores> {
            //Se necesita un get_rol() que nos permita verificar el rol del usuario
            match usuario.get_rol() {
                //Primera implementacion - no final - de como desarrollar el metodo
                Roles::Comprador => {
                    match estado_nuevo {
                        EstadoOrden::Enviado => {
                            self.estado = estado_nuevo;
                            Ok(true)
                        },
                        _ => Err(Errores::PermisoDenegado)
                        
                    }
                },

                Roles::Vendedor => {
                    match estado_nuevo {
                        EstadoOrden::Recibido => {
                            self.estado = estado_nuevo;
                            Ok(true)
                        },
                        EstadoOrden::Cancelada => {
                            // Se necesita consentimiento mutuo
                            Err(Errores::ConsentimientoFaltante)
                            
                        },
                        _ => {
                            Err(Errores::PermisoDenegado)
                        }
                    }
                }
                _ => Err(Errores::PermisoDenegado)
            }
        }

    }

    #[derive(Clone, Debug, PartialEq)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    #[ink::scale_derive(Decode, Encode, TypeInfo)]        
    /// Publicacion de un producto
    pub struct Publicacion {
        id_vendedor: AccountId,
        producto: Producto,
        descripcion: String,
        fecha: Timestamp, // !TODO implementar fecha de alta de la publicacion
        precio: Balance,
        disponibles: u64,
    }    
    impl Publicacion {
        /// Se crea una nueva publicacion
        pub fn alta_publicacion( 
            id_vendedor: AccountId, 
            producto: Producto, 
            descripcion: String, 
            fecha: Timestamp /* implementar fecha */, 
            precio: Balance, 
            stock_inicial: u64
        ) -> Self {
            
            Publicacion {
                id_vendedor,
                producto,
                descripcion,
                fecha,
                precio,
                disponibles: stock_inicial,
            }

        }
        /// Se cambia la descripcion de la publicacion
        pub fn cambiar_descripcion(&mut self, nueva_descripcion: String) -> Result<bool, Errores> {

            if nueva_descripcion.is_empty() {
                return Err(Errores::ProductoSinDescripcion)
            }

            self.descripcion = nueva_descripcion;
            Ok(true)
        }
        pub fn descontar_stock(&mut self, cantidad: u64) -> Result<bool, Errores> {
            match self.disponibles.checked_sub(cantidad) {
                Some(_) => {
                    self.disponibles =cantidad;//actualizo el stock YA SE DESCONTO CANTIDAD PERO NO SE ACTUALIZABA 
                    return Ok(true)},
                None => return Err(Errores::ProductoSinStock)
            }
        }

        /// Se cambia el precio de la publicacion
        pub fn cambiar_precio(&mut self, nuevo_precio: Balance) -> Result<bool, Errores> {
            if nuevo_precio == 0 {
                return Err(Errores::PrecioNulo)
            }
            self.precio = nuevo_precio;
            Ok(true)
        }
        
    }

    #[derive(Clone, Debug, PartialEq)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    #[ink::scale_derive(Decode, Encode, TypeInfo)]
    /// Inventario de productos de un usuario
    pub struct Inventario {
        listado_productos: Vec<Producto>   
    }
    impl Inventario {
        fn new() -> Self {
            Inventario {
                listado_productos: Vec::new(),
            }
        }
        fn descontar_stock_inventario(&mut self, cantidad: u64, id_producto: u64) -> Result<bool, Errores>{
            for p in self.listado_productos.iter_mut() {
                if p.id == id_producto {
                    match p.cantidad.checked_sub(cantidad) {
                        Some(nuevo_stock) => {
                            p.cantidad=nuevo_stock;
                            return Ok(true)},
                        None => return Err(Errores::ProductoSinStock)
                    }
                }
            }
            Err(Errores::ProductoNoEncontrado)
        }
    }
    
    /// # Marketplace Contract
    #[ink(storage)]
    pub struct Marketplace {
        inventarios: Mapping<AccountId, Inventario>,
        usuarios: Mapping<AccountId, Usuario>,
        publicaciones: StorageVec<Publicacion>,
        ordenes_de_compra: StorageVec<OrdenCompra>,
    }

    impl Marketplace {        
        #[ink(constructor)] 
        /// # Creacion default del contrato
        pub fn new() -> Self {
            Self {
                usuarios: Mapping::default(),
                publicaciones: StorageVec::default(),
                ordenes_de_compra: StorageVec::default(),
                inventarios: Mapping::default(),
            }
        }  

        //La estructura de usuarios debe ser un Mapping
        fn alta_usuario(&mut self, nombre: String, dni: u64, id: AccountId, rol: Roles)-> Result<bool, Errores> {
            // Validación de nombre
            if nombre.is_empty() {
                return Err(Errores::NombreNulo);
            }
            if dni == 0 {
                return Err(Errores::ConflictoDni);
            }

            // Validación de ID y DNI
            if let Some(usuario_existente) = self.usuarios.get(&id) {
                if usuario_existente.get_dni() != dni {
                    // Caso 1: Id coincide con el ingresado pero el dni no (dni e id deben ser unicos dentro del sistema)
                    return Err(Errores::ConflictoDni);
                }else{
                    // Caso 2: ID y DNI coinciden con los ingresados(usuario ya registrado)
                    return Err(Errores::UsuarioExistente);
                }
            }

            // Se procede a ingresar al usuario sino hubo interrupciones anteriores
            let user = Usuario::new(nombre,dni,id,rol);

            // Asigna un inventario de productos para el nuevo usuario con roles : Vendedor,Vendedor/Comprador
            // Se comprende que en base a las condiciones impuestas arriba , no pueden crearse inventarios de usuarios "repetidos"

            if user.puede_vender(){
                self.inventarios.insert(id,&Inventario::new());
            }

            self.usuarios.insert(id,&user);
            Ok(true)
        }
        /// Crea un usuario
        #[ink(message)]
        pub fn crear_usuario(&mut self, nombre: String, dni: u64, rol: Roles) -> Result<bool, Errores> {
            let caller = self.env().caller();
            self.alta_usuario(nombre, dni, caller, rol)
        }


        /// Se da de alta en una orden
        fn alta_orden(&mut self, id_usuario: AccountId, id_producto:u64, cantidad:u64) -> Result<bool, Errores> {

            //Se busca en la estructura de usuarios el id con el caller
            if let Some(comprador) = self.usuarios.get(&id_usuario){

                for i in 0..self.publicaciones.len() {
                    if let Some(mut publi) = self.publicaciones.get(i) {
                        if publi.producto.id == id_producto {

                            //fn crear_orden(id_ord:u64, comprador : AccountId,  id_vend:AccountId, id_prod:u64, cant:u64, puede_vender: bool)

                            if cantidad > publi.disponibles {
                                return Err(Errores::ProductoSinStock);
                            }
                            publi.descontar_stock(cantidad)?;
                            self.publicaciones.set(i, &publi);
                            
                            if let Some(  mut inventario ) = self.inventarios.get(&publi.id_vendedor){
                                inventario.descontar_stock_inventario(cantidad, id_producto)?;
                                self.inventarios.insert(&publi.id_vendedor,&inventario);
                            }
                            let orden = OrdenCompra::crear_orden(
                                self.ordenes_de_compra.len() as u64,
                                id_usuario,
                                publi.id_vendedor.clone(),
                                0,
                                cantidad,
                            )?;

                            self.ordenes_de_compra.push(&orden);

                            return Ok(true);
                        }
                    }
                }
                return Err(Errores::ProductoNoEncontrado);
            }
            Err(Errores::CompradorNoEncontrado)
        
        }
        /// Crea una orden de compra
        #[ink(message)]
        /// se crea efectivamente la orden publica
        pub fn hacer_orden(&mut self, id_producto:u64, cantidad:u64) -> Result<bool, Errores>{
            let caller = self.env().caller();
            self.alta_orden(caller, id_producto, cantidad)?;
            Ok(true)
        }

        ///se da de alta la publicacion
        fn alta_publicacion(
            &mut self,  
            id_usuario: AccountId,
            nombre_producto: String,
            fecha: Timestamp, 
            descripcion: String, 
            precio: Balance,
            stock_inicial: u64
        ) -> Result<bool, Errores> 
        {
            let usuario = self.usuarios.get(&id_usuario);
            let inventarios = self.inventarios.get(&id_usuario);
            

            match usuario {
                Some(usuario) => {
                    match inventarios {
                        Some(inventario) => {
                            let producto = inventario.listado_productos.iter().find(|p| p.nombre == nombre_producto);
                            match producto {
                                Some(p) => {
                                    let publi = Publicacion::alta_publicacion(
                                        id_usuario,
                                        p.clone(),
                                        descripcion,
                                        fecha,
                                        precio,
                                        stock_inicial
                                    );
                                    self.publicaciones.push(&publi);
                                    Ok(true)
                                },
                                None => Err(Errores::ProductoNoEncontrado)
                            }
                        },
                        None => Err(Errores::SinInventario)
                    }


                },
                None => Err(Errores::UsuarioInexistente)
            }

        }
        /// Crea una publicacion de un producto
        #[ink(message)]
        ///Se crea efectivamente la publicacion
        pub fn crear_publicacion(
            &mut self,  
            nombre_producto: String,
            descripcion: String, 
            precio: Balance,
            stock_inicial: u64
        ) -> Result<bool, Errores> {
            let caller = self.env().caller();
            let fecha = self.env().block_timestamp();
            self.alta_publicacion(caller, nombre_producto, fecha, descripcion, precio, stock_inicial)
        }

        fn cambiar_de_rol(&mut self, rol_nuevo: Roles) -> Result<bool, Errores> {
            // ErrorUsuario::UsuarioInexistente
            let caller = self.env().caller();

            let user = self.usuarios.get(&caller);
            if let Some( mut usuario ) = user {
                let rol_usuario = usuario.rol.clone();
                match (rol_usuario, rol_nuevo.clone()) {
                    (Roles::Comprador, Roles::Comprador ) => return Err(Errores::ConflictoRol),
                    (Roles::Vendedor, Roles::Vendedor) => return Err(Errores::ConflictoRol),
                    (Roles::CompradorVendedor, Roles::CompradorVendedor) => return Err(Errores::ConflictoRol),
                    _ => {
                        usuario.rol = rol_nuevo.clone();
                        let _ =match rol_nuevo {
                            Roles::Vendedor => {
                                self.inventarios.insert(&caller,&Inventario::new());
                            },
                            Roles::CompradorVendedor => {
                                self.inventarios.insert(&caller, &Inventario::new());
                            },
                            _ => {
                                return Err(Errores::ConflictoRol);
                            }
                        };
                        self.usuarios.insert(&caller, &usuario);
                        Ok(true)
                    }
                }
            } else {
                Err(Errores::UsuarioInexistente)
            }
        }
        /// Cambia el rol del usuario
        #[ink(message)]
        pub fn cambiar_rol(&mut self, rol_nuevo: Roles) -> Result<bool, Errores> {
            self.cambiar_de_rol(rol_nuevo)
        }

        fn agregar_producto_inventario(&mut self, id_producto: u64, stock_total: u64, nombre: String, categoria: Categoria) -> Result<bool, Errores> {
            let caller = self.env().caller();

            let user = self.usuarios.get(&caller);

            if let Some(usuario) = user {
                match usuario.rol {
                    Roles::Comprador => Err(Errores::PermisoDenegado),
                    _ => {

                        let inventarios = self.inventarios.get(&caller);

                        if let Some(mut inventario) = inventarios {
                            //   fn new(id:u64,nombre:String,categoria:Categoria, cantidad: u64)->Producto{

                            let producto = Producto::new(
                                id_producto,
                                nombre,
                                categoria,
                                stock_total,
                            );

                            inventario.listado_productos.push(producto);
                            self.inventarios.insert(&caller, &inventario);

                        } else {
                            return Err(Errores::SinInventario);
                        }

                        return Ok(true);
                    }
                }
            } else {
                Err(Errores::UsuarioInexistente)
            }
        }
        /// Agregar Producto
        #[ink(message)]
        pub fn agregar_producto(&mut self, stock_total: u64, nombre: String, categoria: Categoria) -> Result<bool, Errores> {
            let caller = self.env().caller();
            let productos = self.inventarios.get(&caller);
            let id_producto = match productos {
                Some(lista) => lista.listado_productos.len() as u64,
                None => 0 as u64,
            };

            self.agregar_producto_inventario(id_producto, stock_total, nombre, categoria)
        }
    }
    #[cfg(test)]
    mod tests {
// TEST PARA PRODUCTO//
        use super::*;
        use crate::marketplace::helpers::{set_caller, default_accounts, iniciar_contrato};
        #[test]
        fn test_para_new_prod(){
            let p:Producto=Producto::new(1,"mesa".to_string(),Categoria::Hogar,10);
            assert_eq!(p.id,1);
            assert_ne!(p.id,4);
            assert_eq!(p.nombre,"mesa".to_string());
            assert_ne!(p.nombre,"silla".to_string());
        }
        #[test]
        fn test_para_get_producto(){
            let p:Producto=Producto::new(1,"lavandina".to_string(),Categoria::Limpieza,10);
            let p2=p.get_producto();
            assert_eq!(p.id,p2.id);
            assert_eq!(p.nombre,p2.nombre)
        }
// FIN TEST PRODUCTO //
//---------------------------------------
// TEST PUBLICACION //

        
        fn crear_fecha()->u64{
            1752796800000
        }
        fn crear_precio_balance()->Balance{
            1_000_000_000_000//equivale a tokens
        }
        fn crear_precio_balance_b()->Balance{
            5_000_000_000_000//equivale a token
        }
        fn crear_id(n: u8) -> AccountId {
            AccountId::from([n; 32])
        }

        #[test]
        fn test_usuario_puede_vender() {
            let comprador = Usuario::new("A".to_string(), 1, crear_id(1), Roles::Comprador);
            let vendedor = Usuario::new("B".to_string(), 2, crear_id(1), Roles::Vendedor);
            assert_eq!(comprador.puede_vender(), false);
            assert_eq!(vendedor.puede_vender(), true);
        }
        #[test]
        fn test_crear_orden_valida() {
            let orden = OrdenCompra::crear_orden(1, crear_id(1), crear_id(1), 10, 5);
            assert!(orden.is_ok());
            let orden = orden.unwrap();
            assert_eq!(orden.id_comprador, crear_id(1));           
            assert_eq!(orden.cantidad, 5);
            assert_eq!(orden.estado, EstadoOrden::Pendiente);            
        }
        #[test]
        fn test_crear_orden_cantidad_cero() {
            let orden = OrdenCompra::crear_orden(1, crear_id(1), crear_id(1), 10, 0);
            assert_eq!(orden, Err(Errores::ProductoSinStock));
        }
        #[ink::test]
        fn test_alta_orden_todos_los_casos() {
            let mut contrato = iniciar_contrato();
            let accounts = default_accounts();
            //comprador no existe
            let result = contrato.alta_orden(accounts.alice, 1, 1);
            assert_eq!(result, Err(Errores::CompradorNoEncontrado));
            // crear comprador
            set_caller(accounts.alice);
            let creado = contrato.crear_usuario("Comprador".to_string(), 1111, Roles::Comprador);
            assert_eq!(creado, Ok(true));
            // producto no encontrado (id no existe)
            let result2 = contrato.alta_orden(accounts.alice, 99, 1);
            assert_eq!(result2, Err(Errores::ProductoNoEncontrado));
            // Crear vendedor y producto
            set_caller(accounts.bob);
            let creado_vend = contrato.crear_usuario("Vendedor".to_string(), 2222, Roles::Vendedor);
            assert_eq!(creado_vend, Ok(true));
            let agregar = contrato.agregar_producto(10, "mouse".to_string(), Categoria::Electronica);
            assert_eq!(agregar, Ok(true));
            // crear publicacion para ese produ
            let publica = contrato.crear_publicacion("mouse".to_string(), "rapido".to_string(), 100, 10);
            assert_eq!(publica, Ok(true));
            // tengo el id del producto "mouse" 
            let inventario = contrato.inventarios.get(&accounts.bob).expect("deberia tener inventario");
            let producto_mouse = inventario.listado_productos.iter().find(|p| p.nombre == "mouse").expect("deberia existir");
            let id_producto_mouse = producto_mouse.id;
            // stock insuficiente
            set_caller(accounts.alice);
            let stock_insuf = contrato.alta_orden(accounts.alice, id_producto_mouse, 20); // pedir de mas
            assert_eq!(stock_insuf, Err(Errores::ProductoSinStock));
            // compra exitosa
            let ok = contrato.alta_orden(accounts.alice, id_producto_mouse, 2);
            assert_eq!(ok, Ok(true));
        }

        #[test]
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
        #[test]
        fn test_cambiar_estado_permiso_denegado() {
            let mut orden=OrdenCompra::crear_orden(1, crear_id(1), crear_id(1), 10, 2).unwrap();
            let usuario=Usuario::new("otro".to_string(), 101, crear_id(1), Roles::Vendedor);
            let resultado=orden.cambiar_estado(usuario, EstadoOrden::Enviado);
            assert_eq!(resultado,Err(Errores::PermisoDenegado));
        }
        #[test]
        fn test_alta_publicacion_y_cambiar_descripcion() {
            let prod = Producto::new(1,"mouse".to_string(), Categoria::Electronica, 20);
            let mut pub1 = Publicacion::alta_publicacion(crear_id(1),prod,String::from("bueno"),123,1000,10,);
            let mut resultado = pub1.cambiar_descripcion(String::from("nuevo modelo"));
            assert_eq!(resultado, Ok(true));
            assert_eq!(pub1.descripcion, "nuevo modelo");
            let resultado = pub1.cambiar_descripcion("".to_string());
            assert_eq!(resultado, Err(Errores::ProductoSinDescripcion));        
            assert!(pub1.descontar_stock(2).is_ok());
            assert!(pub1.descontar_stock(25).is_err());
        }
        #[test]
        fn test_cambiar_precio_valido() {
            let prod = Producto::new(1,"monitor".to_string(), Categoria::Electronica, 20);
            let mut pub1 = Publicacion::alta_publicacion(crear_id(1),prod,String::from("HD"),123,500,10,);
            let mut resultado = pub1.cambiar_precio(800);
            assert_eq!(resultado, Ok(true));
            assert_eq!(pub1.precio, 800);
            resultado = pub1.cambiar_precio(0);
            assert_eq!(resultado, Err(Errores::PrecioNulo));            

        }
        #[test]
        fn test_inventario_descontar_stock_ok() {
            let mut inventario = Inventario::new();
            let prod = Producto::new(1,"camiseta".to_string(), Categoria::Deporte, 10);
            inventario = Inventario {listado_productos: vec![prod]};
            let mut resultado = inventario.descontar_stock_inventario(5, 1);
            assert_eq!(resultado, Ok(true));
            assert_eq!(inventario.listado_productos[0].cantidad, 5);
            resultado = inventario.descontar_stock_inventario(5, 1);
           //assert_eq!(resultado, Err(Errores::ProductoSinStock));            
            resultado = inventario.descontar_stock_inventario(1, 999);
           assert_eq!(resultado, Err(Errores::ProductoNoEncontrado));            
           resultado = inventario.descontar_stock_inventario(20, 1);
           assert_eq!(resultado, Err(Errores::ProductoSinStock));
        }
    
// FIN TEST USUARIO //
//---------------------------------------

// TEST SISTEMA // //los pongo aca porque en e2e se simula un flujo del sistema no los test unitarios//preguntar
        #[ink::test]
        fn test_crear_usuario() {
            let mut contrato=iniciar_contrato();
            let accounts=default_accounts();
            set_caller(accounts.alice);
            let mut resultado=contrato.crear_usuario("juancito".to_string(), 12345678, Roles::Comprador);
            assert!(resultado.is_ok());
            resultado = contrato.crear_usuario("".to_string(), 12345678, Roles::Comprador);
            assert_eq!(resultado, Err(Errores::NombreNulo));
            resultado = contrato.crear_usuario("juancarlo".to_string(),0, Roles::Comprador);
            assert_eq!(resultado, Err(Errores::ConflictoDni));            
            resultado = contrato.crear_usuario("juancito".to_string(), 12345678, Roles::Comprador);
            assert_eq!(resultado, Err(Errores::UsuarioExistente));            
        }
        #[ink::test]
        fn test_cambiar_rol() {
            let mut contrato =iniciar_contrato();
            let accounts = default_accounts();
            set_caller(accounts.alice);
            contrato.crear_usuario("anita".to_string(), 12345678, Roles::Comprador);
            let mut resultado = contrato.cambiar_rol(Roles::Vendedor);
            assert_eq!(resultado, Ok(true));
            resultado = contrato.cambiar_rol(Roles::Vendedor);
            assert_eq!(resultado, Err(Errores::ConflictoRol));            
        }

        #[ink::test]
        fn test_agregar_producto_ok() {
            let mut contrato = iniciar_contrato();
            let accounts = default_accounts();
            set_caller(accounts.alice);
            contrato.crear_usuario("bob".to_string(), 12345678, Roles::Vendedor);
            let mut resultado = contrato.agregar_producto(10, "teclado".to_string(), Categoria::Electronica);
            assert_eq!(resultado, Ok(true));
            contrato.crear_usuario("bob".to_string(), 12345678, Roles::Comprador);
            resultado = contrato.agregar_producto(10, "teclado".to_string(), Categoria::Electronica);
            //assert_eq!(resultado, Err(Errores::PermisoDenegado));            
        }
        #[ink::test]
        fn test_crear_publicacion_sin_producto() {
            let mut contrato = iniciar_contrato();
            let accounts = default_accounts();
            set_caller(accounts.alice);
            contrato.crear_usuario("carlos".to_string(), 12345678, Roles::Vendedor);
            let resultado = contrato.crear_publicacion("mouse".to_string(), "buen mouse".to_string(), 1000, 10);
            assert_eq!(resultado, Err(Errores::ProductoNoEncontrado));
        }
        #[ink::test]
        fn test_hacer_orden_sin_usuario() {
            let mut contrato = iniciar_contrato();
            let accounts = default_accounts();
            set_caller(accounts.alice);
            let resultado = contrato.hacer_orden(0, 1);
            assert_eq!(resultado, Err(Errores::CompradorNoEncontrado));
        }    
    }
        #[cfg(test)]
        mod helpers {
            use super::*;
            use ink::env::{self, test, DefaultEnvironment};

            pub fn set_caller(account: AccountId) {
                test::set_caller::<DefaultEnvironment>(account);
            }

            pub fn default_accounts() -> test::DefaultAccounts<DefaultEnvironment> {
                test::default_accounts::<DefaultEnvironment>()
            }

            pub fn iniciar_contrato() -> Marketplace {
                Marketplace::new()
            }
        }
// FIN TEST SISTEMA //

    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        use super::*;
        use crate::marketplace::helpers::{set_caller, default_accounts, iniciar_contrato};
        #[ink::test]
        fn test_flujo_completo_e2e() {
            let mut contrato = Marketplace::new();
            let accounts = default_accounts::<ink::env::DefaultEnvironment>();

            // 1  creo usuario comprador alice
            set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
            let res_comprador = contrato.crear_usuario("Alice".to_string(), 1111, Roles::Comprador);
            assert_eq!(res_comprador, Ok(true));

            // 2 creo usuario vendedor bob
            set_caller::<ink::env::DefaultEnvironment>(accounts.bob);
            let res_vendedor = contrato.crear_usuario("Bob".to_string(), 2222, Roles::Vendedor);
            assert_eq!(res_vendedor, Ok(true));

            // 3agrego producto al inventario de vendedor
            let agregar_producto = contrato.agregar_producto(10, "teclado".to_string(), Categoria::Electronica);
            assert_eq!(agregar_producto, Ok(true));

            // 4  creo publicacion para el producto
            let crear_publicacion = contrato.crear_publicacion("teclado".to_string(),"teclado mecánico de alta calidad".to_string(),1000, 10);
            assert_eq!(crear_publicacion, Ok(true));
            // 5cambio caller a comprador para hacer orden
            set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
            // 6 hago una orden de compra
            let hacer_orden = contrato.hacer_orden(0, 5);  // producto id=0 porque es el primero agregado
            assert_eq!(hacer_orden, Ok(true));
            // 7 verifico que bajo
            let publi = contrato.publicaciones.get(0).expect("la publicacion debe existir");
            assert_eq!(publi.disponibles, 5);
            // 8 verifico que se registro la orden correctamente
            let orden = contrato.ordenes_de_compra.get(0).expect("la orden debe existir");
            assert_eq!(orden.id_comprador, accounts.alice);
            assert_eq!(orden.id_producto, 0);
            assert_eq!(orden.cantidad, 5);
            assert_eq!(orden.estado, EstadoOrden::Pendiente);
        }
    }

}

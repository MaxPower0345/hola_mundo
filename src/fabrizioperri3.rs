/*3 -La plataforma de streaming "StreamingRust" ofrece distintos tipos de suscripciones
(Basic, Clasic, Super) a sus usuarios. Cada suscripción tiene un costo mensual y una
duración de meses y una fecha de inicio, además los usuarios pueden pagar por sus
suscripciones con distintos medios de pago que son Efectivo, MercadoPago, Tarjeta de
Crédito, Transferencia Bancaria, Cripto. Cada medio de pago tiene sus datos
correspondientes a excepción de Efectivo.
Los usuarios solo pueden tener una suscripción activa a la vez.
Implemente las estructuras, funciones asociadas y traits necesarios para resolver las
siguientes acciones:
➢ Crear un usuario con una determinada suscripción y medio de pago.
➢ Dado un usuario hacer un upgrade sobre la suscripción. Es decir si está a Basic
pasa a Clasic y si está en Clasic pasa a Super.
➢ Dado un determinado usuario, hacer un downgrade sobre una suscripción, si la
suscripción es del tipo Basic al hacerlo se cancelará la suscripción.
➢ Dado un usuario cancelar la suscripción.
➢ Saber el medio de pago que es más utilizado por los usuarios sobre las
suscripciones activas
➢ Saber cual es la suscripción más contratada por los usuarios sobre las suscripciones
activas.
➢ Saber cuál fue el medio de pago más utilizado.
➢ Saber cuál fue la suscripción más contratada.

5- En base al ejercicio 3 del tp#4 implemente lo siguiente:
a- Realice todos los tests de la funcionalidad implementada obteniendo un coverage
de por lo menos 90%
b- Todas las suscripciones deben almacenarse en un archivo en formato JSON,
implemente lo necesario para que toda la funcionalidad de las suscripciones se realice
guardando, leyendo o modificando archivos..No debe modificar los tests hechos en el punto
a. Si puede agregar más en caso de que haga métodos nuevos para cumplir con este
punto. Recuerde también que se debe seguir manteniendo un coverage de al menos 90%. */

use std::collections::HashMap;
#[derive(Clone)]
pub struct Fecha {
    dia: u32,
    mes: u32,
    anio: u32,
}

impl Fecha{
    pub fn fecha_de_hoy()->Fecha{
        Fecha{dia:26,mes:5,anio:2003}
    }
}
#[derive(Eq)]
#[derive(PartialEq)]
#[derive(Hash)]
#[derive(Clone)]
#[derive(Debug)]
pub enum TipoDeSuscripcion{
    Basic,
    Clasic,
    Super
}
impl TipoDeSuscripcion{
    pub fn es_basic(&self)->bool{
        match self{
            TipoDeSuscripcion::Basic=>true,
            _=>false
        }
    }
}
#[derive(Clone)]
#[derive(Eq)]
#[derive(PartialEq)]
#[derive(Hash)]
#[derive(Debug)]
pub enum MedioDePago<'a>{
    Efectivo,
    MercadoPago{dato_mercado:&'a str},
    TarjetaDeCredito{dato_tarjeta:&'a str},
    TransferenciaBancaria{dato_transferencia:&'a str},
    Cripto{dato_cripto:&'a str}
}
impl <'a>MedioDePago<'a>{
    pub fn a_clave(&self)->u32{
        match self{
        MedioDePago::Efectivo=>1,
        MedioDePago::MercadoPago{..}=>2,
        MedioDePago::TarjetaDeCredito{..}=>3,
        MedioDePago::TransferenciaBancaria {..}=>4,
        MedioDePago::Cripto {..}=>5
        }
    }
}
pub fn a_medio_de_pago<'a>(clave:u32)->MedioDePago<'a>{
    match clave{
        1=>MedioDePago::Efectivo,
        2=>MedioDePago::MercadoPago { dato_mercado:"nada" },
        3=>MedioDePago::TarjetaDeCredito { dato_tarjeta:"nada" },
        4=>MedioDePago::TransferenciaBancaria { dato_transferencia:"nada" },
        _=>MedioDePago::Cripto { dato_cripto:"nada" }
    }
} 
#[derive(PartialEq, Eq, Hash)]

pub struct Usuario{
    id:u32
}
impl Usuario{
    pub fn new(id:u32)->Usuario{
        Usuario { id }
    }
}
#[derive(Clone)]
pub struct Suscripcion<'a>{
    inicio_fecha_suscripcion:Fecha,
    tipo_de_suscripcion:TipoDeSuscripcion,
    usuario:&'a Usuario,
    medio_de_pago:MedioDePago<'a>,
}
impl <'a>Suscripcion<'a>{
    //➢ Crear un usuario con una determinada suscripción y medio de pago. ¿? se contradice con lo dicho por el profe(?
    pub fn new(tipo_de_suscripcion:TipoDeSuscripcion,usuario:&'a Usuario,medio_de_pago:MedioDePago<'a>)->Suscripcion<'a>{
        Suscripcion { inicio_fecha_suscripcion:Fecha::fecha_de_hoy(), tipo_de_suscripcion, usuario, medio_de_pago }
    }
    fn upgrade(&mut self){
        match self.tipo_de_suscripcion{
            TipoDeSuscripcion::Basic=>self.tipo_de_suscripcion=TipoDeSuscripcion::Clasic,
            TipoDeSuscripcion::Clasic=>self.tipo_de_suscripcion=TipoDeSuscripcion::Super,
            _=>print!("ya posee tier super, por lo que no es posible mejorar su suscripcion")
        }
    }
    fn downgrade(&mut self){
        match self.tipo_de_suscripcion{
            TipoDeSuscripcion::Super=>self.tipo_de_suscripcion=TipoDeSuscripcion::Clasic,
            _=>self.tipo_de_suscripcion=TipoDeSuscripcion::Basic,
        }
    }
}
pub struct InfoTipoSuscripcion{
    duracion:u32,
    costo:f64
}
impl InfoTipoSuscripcion{
    pub fn new(duracion:u32,costo:f64)->InfoTipoSuscripcion{
        InfoTipoSuscripcion { duracion, costo }
    }
}
pub struct Plataforma<'a>{
    usuarios_activos:HashMap<&'a Usuario,Suscripcion<'a>>,
    datos_suscripciones:HashMap<TipoDeSuscripcion,InfoTipoSuscripcion>,
    historial:Vec<Suscripcion<'a>>
}
impl <'a>Plataforma<'a>{
    pub fn new(info_basic:InfoTipoSuscripcion,info_clasic:InfoTipoSuscripcion,info_super:InfoTipoSuscripcion)->Plataforma<'a>{
        let d_sucripciones: HashMap<TipoDeSuscripcion, InfoTipoSuscripcion>=HashMap::from([(TipoDeSuscripcion::Basic,info_basic),(TipoDeSuscripcion::Clasic,info_clasic),(TipoDeSuscripcion::Super,info_super)]);
        Plataforma {usuarios_activos:HashMap::new(),datos_suscripciones:d_sucripciones,historial:Vec::new()}
    }
    pub fn agregar_suscripcion(&mut self,una_suscripcion:Suscripcion<'a>){
        if self.usuarios_activos.contains_key(una_suscripcion.usuario){
            self.cancelar_suscripcion(&una_suscripcion.usuario);
        }
        self.usuarios_activos.insert(una_suscripcion.usuario, una_suscripcion);
    }
    pub fn upgrade_usuario(&mut self, usuario: &'a Usuario) {
        let s = self.usuarios_activos.get_mut(usuario);
        if let Some(suscripcion) =  s{
            suscripcion.upgrade();
        }
    }
    pub fn cancelar_suscripcion(&mut self, usuario: &Usuario) {
        let s=self.usuarios_activos.get(usuario);
        if let Some(suscripcion) = s {
            self.historial.push(suscripcion.clone());
            self.usuarios_activos.remove(usuario);
        }
    }
    pub fn downgrade_usuario(&mut self, usuario: &'a Usuario) {
        let s=self.usuarios_activos.get_mut(usuario);
        if let Some(suscripcion) = s{
            if suscripcion.tipo_de_suscripcion.es_basic(){
                self.cancelar_suscripcion(usuario)
            }
            else{
                suscripcion.downgrade();
            }
        }
    }
    pub fn hash_contador_de_medios_de_pago_activos(&self)->HashMap<u32, u8>{
        let mut hash_contadores: HashMap<u32, u8>=HashMap::from([
            (1,0),
            (2,0),
            (3,0),
            (4,0),
            (5,0),
            ]);
        for suscripciones in &self.usuarios_activos{
            if let Some(x)=hash_contadores.get_mut(&suscripciones.1.medio_de_pago.a_clave()){
                *x=*x+1;
            }
        }
    hash_contadores
    }
    pub fn medio_de_pago_mas_utilizado_activo(&self)->MedioDePago{
        let hash_contadores=self.hash_contador_de_medios_de_pago_activos();
        a_medio_de_pago(hash_contadores.iter().max_by(|x,y|x.1.cmp(y.1)).unwrap().0.clone())
    }
    pub fn hash_contador_de_tipos_de_suscripcion_activos(&self)->HashMap<TipoDeSuscripcion, u8>{
        let mut hash_contadores: HashMap<TipoDeSuscripcion, u8>=HashMap::from([
            (TipoDeSuscripcion::Basic,0),
            (TipoDeSuscripcion::Clasic,0),
            (TipoDeSuscripcion::Super,0)
        ]);
        for suscripciones in &self.usuarios_activos{
            if let Some(x)=hash_contadores.get_mut(&suscripciones.1.tipo_de_suscripcion){
                *x=*x+1;
            }
        }
        hash_contadores
    }
    pub fn tipo_de_suscripcion_mas_utilizado_activos(&self)->TipoDeSuscripcion{
        let hash_contadores=self.hash_contador_de_tipos_de_suscripcion_activos();
        hash_contadores.iter().max_by(|x,y|x.1.cmp(y.1)).unwrap().0.clone()
    }
    pub fn medio_de_pago_mas_utilizado(&self)->MedioDePago{
        let mut hash_contadores=self.hash_contador_de_medios_de_pago_activos();
        for suscripcion in &self.historial{
            if let Some(x)=hash_contadores.get_mut(&suscripcion.medio_de_pago.a_clave()){
                *x=*x+1;
            }
        }
        a_medio_de_pago(hash_contadores.iter().max_by(|x,y|x.1.cmp(y.1)).unwrap().0.clone())
    }
    pub fn tipo_de_suscripcion_mas_utilizada(&self)->TipoDeSuscripcion{
        let mut hash_contadores=self.hash_contador_de_tipos_de_suscripcion_activos();
        for suscripcion in &self.historial{
            if let Some(x)=hash_contadores.get_mut(&suscripcion.tipo_de_suscripcion){
                *x=*x+1;
            }
        }
        hash_contadores.iter().max_by(|x,y|x.1.cmp(y.1)).unwrap().0.clone()
    }
}

#[test]
fn test_upgrade_usuario_basic(){
    let info_basic = InfoTipoSuscripcion::new(30, 9.99);
    let info_clasic = InfoTipoSuscripcion::new(60, 19.99);
    let info_super = InfoTipoSuscripcion::new(90, 29.99);

    let usuario1 = Usuario::new(1);

    let mut plataforma = Plataforma::new(info_basic, info_clasic, info_super);

    let suscripcion1 = Suscripcion::new(TipoDeSuscripcion::Basic, &usuario1, MedioDePago::Efectivo);
    plataforma.agregar_suscripcion(suscripcion1);
    plataforma.upgrade_usuario(&usuario1);
    assert_eq!(plataforma.usuarios_activos.get(&usuario1).unwrap().tipo_de_suscripcion,TipoDeSuscripcion::Clasic);
}
#[test]
fn test_upgrade_usuario_clasic(){
    let info_basic = InfoTipoSuscripcion::new(30, 9.99);
    let info_clasic = InfoTipoSuscripcion::new(60, 19.99);
    let info_super = InfoTipoSuscripcion::new(90, 29.99);

    let usuario1 = Usuario::new(1);

    let mut plataforma = Plataforma::new(info_basic, info_clasic, info_super);

    let suscripcion1 = Suscripcion::new(TipoDeSuscripcion::Clasic, &usuario1, MedioDePago::Efectivo);
    plataforma.agregar_suscripcion(suscripcion1);
    plataforma.upgrade_usuario(&usuario1);
    assert_eq!(plataforma.usuarios_activos.get(&usuario1).unwrap().tipo_de_suscripcion,TipoDeSuscripcion::Super);
}
#[test]
fn test_upgrade_usuario_super(){
    let info_basic = InfoTipoSuscripcion::new(30, 9.99);
    let info_clasic = InfoTipoSuscripcion::new(60, 19.99);
    let info_super = InfoTipoSuscripcion::new(90, 29.99);

    let usuario1 = Usuario::new(1);

    let mut plataforma = Plataforma::new(info_basic, info_clasic, info_super);

    let suscripcion1 = Suscripcion::new(TipoDeSuscripcion::Super, &usuario1, MedioDePago::Efectivo);
    plataforma.agregar_suscripcion(suscripcion1);
    plataforma.upgrade_usuario(&usuario1);
    assert_eq!(plataforma.usuarios_activos.get(&usuario1).unwrap().tipo_de_suscripcion,TipoDeSuscripcion::Super);
}
#[test]
fn test_downgrade_usuario_super(){
    let info_basic = InfoTipoSuscripcion::new(30, 9.99);
    let info_clasic = InfoTipoSuscripcion::new(60, 19.99);
    let info_super = InfoTipoSuscripcion::new(90, 29.99);

    let usuario1 = Usuario::new(1);

    let mut plataforma = Plataforma::new(info_basic, info_clasic, info_super);

    let suscripcion1 = Suscripcion::new(TipoDeSuscripcion::Super, &usuario1, MedioDePago::Efectivo);
    plataforma.agregar_suscripcion(suscripcion1);
    plataforma.downgrade_usuario(&usuario1);
    assert_eq!(plataforma.usuarios_activos.get(&usuario1).unwrap().tipo_de_suscripcion,TipoDeSuscripcion::Clasic);
}
#[test]
fn test_downgrade_usuario_clasic(){
    let info_basic = InfoTipoSuscripcion::new(30, 9.99);
    let info_clasic = InfoTipoSuscripcion::new(60, 19.99);
    let info_super = InfoTipoSuscripcion::new(90, 29.99);

    let usuario1 = Usuario::new(1);

    let mut plataforma = Plataforma::new(info_basic, info_clasic, info_super);

    let suscripcion1 = Suscripcion::new(TipoDeSuscripcion::Clasic, &usuario1, MedioDePago::Efectivo);
    plataforma.agregar_suscripcion(suscripcion1);
    plataforma.downgrade_usuario(&usuario1);
    assert_eq!(plataforma.usuarios_activos.get(&usuario1).unwrap().tipo_de_suscripcion,TipoDeSuscripcion::Basic);
}
#[test]
fn test_downgrade_usuario_basic(){
    let info_basic = InfoTipoSuscripcion::new(30, 9.99);
    let info_clasic = InfoTipoSuscripcion::new(60, 19.99);
    let info_super = InfoTipoSuscripcion::new(90, 29.99);

    let usuario1 = Usuario::new(1);

    let mut plataforma = Plataforma::new(info_basic, info_clasic, info_super);

    let suscripcion1 = Suscripcion::new(TipoDeSuscripcion::Basic, &usuario1, MedioDePago::Efectivo);
    plataforma.agregar_suscripcion(suscripcion1);
    plataforma.downgrade_usuario(&usuario1);
    assert_eq!(plataforma.historial.len(),1);
    assert_eq!(plataforma.usuarios_activos.len(),0);
}
#[test]
fn test_cancelar_usuario(){
    let info_basic = InfoTipoSuscripcion::new(30, 9.99);
    let info_clasic = InfoTipoSuscripcion::new(60, 19.99);
    let info_super = InfoTipoSuscripcion::new(90, 29.99);

    let usuario1 = Usuario::new(1);

    let mut plataforma = Plataforma::new(info_basic, info_clasic, info_super);

    let suscripcion1 = Suscripcion::new(TipoDeSuscripcion::Basic, &usuario1, MedioDePago::Efectivo);
    plataforma.agregar_suscripcion(suscripcion1);
    plataforma.cancelar_suscripcion(&usuario1);
    assert_eq!(plataforma.historial.len(),1);
    assert_eq!(plataforma.usuarios_activos.len(),0);
}
#[test]
fn test_medio_de_pago_activo_mas_utilizado(){
        // Crear información de suscripciones
        let info_basic = InfoTipoSuscripcion::new(30, 9.99);
        let info_clasic = InfoTipoSuscripcion::new(60, 19.99);
        let info_super = InfoTipoSuscripcion::new(90, 29.99);
    
        // Crear usuarios
        let usuario1 = Usuario::new(1);
        let usuario2 = Usuario::new(2);
        let usuario3 = Usuario::new(3);
        let usuario4 = Usuario::new(4);
        let usuario5 = Usuario::new(5);
        let usuario6 = Usuario::new(6);
    
        // Crear plataforma
        let mut plataforma = Plataforma::new(info_basic, info_clasic, info_super);
    
        // Agregar suscripciones
        let suscripcion1 = Suscripcion::new(TipoDeSuscripcion::Basic, &usuario1, MedioDePago::Efectivo);//se anula
        let suscripcion2 = Suscripcion::new(TipoDeSuscripcion::Super, &usuario2, MedioDePago::TransferenciaBancaria{ dato_transferencia: "banco456" });//Clasic
        let suscripcion3 = Suscripcion::new(TipoDeSuscripcion::Basic, &usuario3, MedioDePago::MercadoPago{ dato_mercado: "mercado123" });//clasic
        let suscripcion4 = Suscripcion::new(TipoDeSuscripcion::Super, &usuario4, MedioDePago::TransferenciaBancaria{ dato_transferencia: "banco456" });//super
        let suscripcion5 = Suscripcion::new(TipoDeSuscripcion::Basic, &usuario5, MedioDePago::Efectivo);
        let suscripcion6 = Suscripcion::new(TipoDeSuscripcion::Clasic, &usuario6, MedioDePago::Efectivo);
    
        plataforma.agregar_suscripcion(suscripcion1);
        plataforma.agregar_suscripcion(suscripcion2);
        plataforma.agregar_suscripcion(suscripcion3);
        plataforma.agregar_suscripcion(suscripcion4);
        plataforma.agregar_suscripcion(suscripcion5);
        plataforma.agregar_suscripcion(suscripcion6);

        plataforma.cancelar_suscripcion(&usuario1);
        plataforma.cancelar_suscripcion(&usuario5);

        assert_eq!(plataforma.medio_de_pago_mas_utilizado_activo(),MedioDePago::TransferenciaBancaria { dato_transferencia:"nada" });
}

#[test]
fn test_tipo_de_suscripcion_activa_mas_utilizada(){
    // Crear información de suscripciones
    let info_basic = InfoTipoSuscripcion::new(30, 9.99);
    let info_clasic = InfoTipoSuscripcion::new(60, 19.99);
    let info_super = InfoTipoSuscripcion::new(90, 29.99);

    // Crear usuarios
    let usuario1 = Usuario::new(1);
    let usuario2 = Usuario::new(2);
    let usuario3 = Usuario::new(3);
    let usuario4 = Usuario::new(4);
    let usuario5 = Usuario::new(5);
    let usuario6 = Usuario::new(6);

    // Crear plataforma
    let mut plataforma = Plataforma::new(info_basic, info_clasic, info_super);

    // Agregar suscripciones
    let suscripcion1 = Suscripcion::new(TipoDeSuscripcion::Basic, &usuario1, MedioDePago::Efectivo);//se anula
    let suscripcion2 = Suscripcion::new(TipoDeSuscripcion::Super, &usuario2, MedioDePago::TransferenciaBancaria{ dato_transferencia: "banco456" });//Clasic
    let suscripcion3 = Suscripcion::new(TipoDeSuscripcion::Basic, &usuario3, MedioDePago::MercadoPago{ dato_mercado: "mercado123" });//clasic
    let suscripcion4 = Suscripcion::new(TipoDeSuscripcion::Super, &usuario4, MedioDePago::TransferenciaBancaria{ dato_transferencia: "banco456" });//super
    let suscripcion5 = Suscripcion::new(TipoDeSuscripcion::Basic, &usuario5, MedioDePago::Efectivo);
    let suscripcion6 = Suscripcion::new(TipoDeSuscripcion::Clasic, &usuario6, MedioDePago::Efectivo);

    plataforma.agregar_suscripcion(suscripcion1);
    plataforma.agregar_suscripcion(suscripcion2);
    plataforma.agregar_suscripcion(suscripcion3);
    plataforma.agregar_suscripcion(suscripcion4);
    plataforma.agregar_suscripcion(suscripcion5);
    plataforma.agregar_suscripcion(suscripcion6);

    plataforma.cancelar_suscripcion(&usuario3);
    plataforma.cancelar_suscripcion(&usuario5);

    assert_eq!(plataforma.tipo_de_suscripcion_mas_utilizado_activos(),TipoDeSuscripcion::Super);
}

#[test]
fn test_medio_de_pago_mas_utilizado(){
        // Crear información de suscripciones
        let info_basic = InfoTipoSuscripcion::new(30, 9.99);
        let info_clasic = InfoTipoSuscripcion::new(60, 19.99);
        let info_super = InfoTipoSuscripcion::new(90, 29.99);
    
        // Crear usuarios
        let usuario1 = Usuario::new(1);
        let usuario2 = Usuario::new(2);
        let usuario3 = Usuario::new(3);
        let usuario4 = Usuario::new(4);
        let usuario5 = Usuario::new(5);
        let usuario6 = Usuario::new(6);
    
        // Crear plataforma
        let mut plataforma = Plataforma::new(info_basic, info_clasic, info_super);
    
        // Agregar suscripciones
        let suscripcion1 = Suscripcion::new(TipoDeSuscripcion::Basic, &usuario1, MedioDePago::Efectivo);//se anula
        let suscripcion2 = Suscripcion::new(TipoDeSuscripcion::Super, &usuario2, MedioDePago::TransferenciaBancaria{ dato_transferencia: "banco456" });//Clasic
        let suscripcion3 = Suscripcion::new(TipoDeSuscripcion::Basic, &usuario3, MedioDePago::MercadoPago{ dato_mercado: "mercado123" });//clasic
        let suscripcion4 = Suscripcion::new(TipoDeSuscripcion::Super, &usuario4, MedioDePago::TransferenciaBancaria{ dato_transferencia: "banco456" });//super
        let suscripcion5 = Suscripcion::new(TipoDeSuscripcion::Basic, &usuario5, MedioDePago::Efectivo);
        let suscripcion6 = Suscripcion::new(TipoDeSuscripcion::Clasic, &usuario6, MedioDePago::Efectivo);
    
        plataforma.agregar_suscripcion(suscripcion1);
        plataforma.agregar_suscripcion(suscripcion2);
        plataforma.agregar_suscripcion(suscripcion3);
        plataforma.agregar_suscripcion(suscripcion4);
        plataforma.agregar_suscripcion(suscripcion5);
        plataforma.agregar_suscripcion(suscripcion6);

        plataforma.cancelar_suscripcion(&usuario1);
        plataforma.cancelar_suscripcion(&usuario5);

        assert_eq!(plataforma.medio_de_pago_mas_utilizado(),MedioDePago::Efectivo);
}

#[test]
fn test_tipo_de_suscripcion_mas_utilizada(){
    // Crear información de suscripciones
    let info_basic = InfoTipoSuscripcion::new(30, 9.99);
    let info_clasic = InfoTipoSuscripcion::new(60, 19.99);
    let info_super = InfoTipoSuscripcion::new(90, 29.99);

    // Crear usuarios
    let usuario1 = Usuario::new(1);
    let usuario2 = Usuario::new(2);
    let usuario3 = Usuario::new(3);
    let usuario4 = Usuario::new(4);
    let usuario5 = Usuario::new(5);
    let usuario6 = Usuario::new(6);

    // Crear plataforma
    let mut plataforma = Plataforma::new(info_basic, info_clasic, info_super);

    // Agregar suscripciones
    let suscripcion1 = Suscripcion::new(TipoDeSuscripcion::Basic, &usuario1, MedioDePago::Efectivo);//se anula
    let suscripcion2 = Suscripcion::new(TipoDeSuscripcion::Super, &usuario2, MedioDePago::TransferenciaBancaria{ dato_transferencia: "banco456" });//Clasic
    let suscripcion3 = Suscripcion::new(TipoDeSuscripcion::Basic, &usuario3, MedioDePago::MercadoPago{ dato_mercado: "mercado123" });//clasic
    let suscripcion4 = Suscripcion::new(TipoDeSuscripcion::Super, &usuario4, MedioDePago::TransferenciaBancaria{ dato_transferencia: "banco456" });//super
    let suscripcion5 = Suscripcion::new(TipoDeSuscripcion::Basic, &usuario5, MedioDePago::Efectivo);
    let suscripcion6 = Suscripcion::new(TipoDeSuscripcion::Clasic, &usuario6, MedioDePago::Efectivo);

    plataforma.agregar_suscripcion(suscripcion1);
    plataforma.agregar_suscripcion(suscripcion2);
    plataforma.agregar_suscripcion(suscripcion3);
    plataforma.agregar_suscripcion(suscripcion4);
    plataforma.agregar_suscripcion(suscripcion5);
    plataforma.agregar_suscripcion(suscripcion6);

    plataforma.cancelar_suscripcion(&usuario3);
    plataforma.cancelar_suscripcion(&usuario5);

    assert_eq!(plataforma.tipo_de_suscripcion_mas_utilizada(),TipoDeSuscripcion::Basic);
}

//cargo tarpaulin --target-dir src/coverage --skip-clean --exclude-files=target/debug/* --out html
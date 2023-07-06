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
use std::io::Seek;
use serde::Serialize;
use serde::Deserialize;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::fs;
use std::path::Path;

#[derive(Clone)]
#[derive(Serialize,Deserialize)]
pub struct Fecha {
    dia: u32,
    mes: u32,
    anio: u32,
}

impl Fecha{
    pub fn fecha_de_hoy(dia:u32,mes:u32,anio:u32)->Fecha{
        Fecha{dia,mes,anio}
    }
}
#[derive(Eq)]
#[derive(PartialEq)]
#[derive(Hash)]
#[derive(Clone)]
#[derive(Debug)]
#[derive(Serialize,Deserialize)]
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
#[derive(Serialize,Deserialize)]
pub enum MedioDePago{
    Efectivo,
    MercadoPago{numero_de_mercado:u8},
    TarjetaDeCredito{numero_de_tarjeta:u8},
    TransferenciaBancaria{cbu:u8},
    Cripto{blockchain:u8}
}
impl MedioDePago{
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
pub fn a_medio_de_pago<'a>(clave:u32)->MedioDePago{
    match clave{
        1=>MedioDePago::Efectivo,
        2=>MedioDePago::MercadoPago { numero_de_mercado:0 },
        3=>MedioDePago::TarjetaDeCredito { numero_de_tarjeta:0 },
        4=>MedioDePago::TransferenciaBancaria { cbu:0 },
        _=>MedioDePago::Cripto { blockchain:0 }
    }
} 
#[derive(PartialEq, Eq, Hash)]
#[derive(Serialize,Deserialize)]
#[derive(Clone)]
pub struct Usuario{
    id:u8
}
impl Usuario{
    pub fn new(id:u8)->Usuario{
        Usuario { id }
    }
}
#[derive(Clone)]
#[derive(Serialize,Deserialize)]
pub struct Suscripcion{
    inicio_fecha_suscripcion:Fecha,
    tipo_de_suscripcion:TipoDeSuscripcion,
    id_usuario:u8,
    medio_de_pago:MedioDePago
}
impl Suscripcion{
    pub fn new(fecha_de_hoy:Fecha,tipo_de_suscripcion:TipoDeSuscripcion,id_usuario:u8,medio_de_pago:MedioDePago)->Suscripcion{
        Suscripcion { inicio_fecha_suscripcion:fecha_de_hoy, tipo_de_suscripcion,id_usuario, medio_de_pago }
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
#[derive(Serialize,Deserialize)]
pub struct InfoTipoSuscripcion{
    duracion:u32,
    costo:f64
}
impl InfoTipoSuscripcion{
    pub fn new(duracion:u32,costo:f64)->InfoTipoSuscripcion{
        InfoTipoSuscripcion { duracion, costo }
    }
}

pub struct Plataforma{
    registro_usuarios:HashMap<u8,Usuario>,
    usuarios_activos:HashMap<u8,Suscripcion>,
    datos_suscripciones:HashMap<TipoDeSuscripcion,InfoTipoSuscripcion>,
    historial:Vec<Suscripcion>
}
impl Plataforma{
    pub fn new(info_basic:InfoTipoSuscripcion,info_clasic:InfoTipoSuscripcion,info_super:InfoTipoSuscripcion)->Plataforma{
        let d_sucripciones: HashMap<TipoDeSuscripcion, InfoTipoSuscripcion>=HashMap::from([(TipoDeSuscripcion::Basic,info_basic),(TipoDeSuscripcion::Clasic,info_clasic),(TipoDeSuscripcion::Super,info_super)]);
        let plataforma=Plataforma {registro_usuarios:HashMap::new(),usuarios_activos:HashMap::new(),datos_suscripciones:d_sucripciones,historial:Vec::new()};
        plataforma
    }
    pub fn crear_y_agregar_usuario_a_la_plataforma(&mut self,un_id_del_usuario:u8,un_medio_de_pago:MedioDePago,una_fecha:Fecha,un_tipo_de_suscripcion:TipoDeSuscripcion){
        if !self.registro_usuarios.contains_key(&un_id_del_usuario){
            let nuevo_usuario=Usuario::new(un_id_del_usuario.clone());
            self.registro_usuarios.insert(nuevo_usuario.id, nuevo_usuario);
            let suscripcion=Suscripcion::new(una_fecha,un_tipo_de_suscripcion,un_id_del_usuario.clone(),un_medio_de_pago);
            self.usuarios_activos.insert(un_id_del_usuario.clone(),suscripcion);
        }
        else{
            println!("dicho id de usuario ya existe, seleccione otro");
        }
    }
    pub fn upgrade_usuario(&mut self, id_del_usuario:&u8) {
        let s = self.usuarios_activos.get_mut(id_del_usuario);
        if let Some(suscripcion) =  s{
            suscripcion.upgrade();
        }
    }
    pub fn cancelar_suscripcion(&mut self, id_del_usuario: &u8) {
        let s=self.usuarios_activos.get(id_del_usuario);
        if let Some(suscripcion) = s {
            self.historial.push(suscripcion.clone());
            self.usuarios_activos.remove(id_del_usuario);
        }
    }
    pub fn downgrade_usuario(&mut self, id_del_usuario: &u8) {
        let s=self.usuarios_activos.get_mut(id_del_usuario);
        if let Some(suscripcion) = s{
            if suscripcion.tipo_de_suscripcion.es_basic(){
                self.cancelar_suscripcion(id_del_usuario);
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
pub struct PlataformaEnArchivo{
    plataforma:Plataforma,
    path_usuarios_activos:String,
    path_registro_usuarios:String,
    path_historial_suscripciones:String
}
impl PlataformaEnArchivo{ 
pub fn new (info_basic:InfoTipoSuscripcion,info_clasic:InfoTipoSuscripcion,info_super:InfoTipoSuscripcion,path_usuarios_activos:String,path_registro_usuarios:String,path_historial_suscripciones:String)->PlataformaEnArchivo{
    let plataforma=Plataforma::new(info_basic, info_clasic, info_super);
    PlataformaEnArchivo { plataforma, path_usuarios_activos, path_registro_usuarios, path_historial_suscripciones }
}
pub fn crear_o_cargar_archivo_plataforma(&mut self){
    if fs::metadata(self.path_usuarios_activos.clone()).is_ok(){
        self.cargar_archivo_de_usuarios_activos();
    }
    else {
        self.crear_archivo_de_usuarios_activos();
    }
    if fs::metadata(self.path_historial_suscripciones.clone()).is_ok(){
        self.cargar_archivo_historial();
    }
    else {
        self.crear_archivo_historial();
    }
    if fs::metadata(self.path_registro_usuarios.clone()).is_ok(){
        self.cargar_archivo_registros_usuarios();
    }
    else {
        self.crear_archivo_registros_usuarios()
    }
}

pub fn cargar_archivo_registros_usuarios(&mut self){
    let mut f = File::open(self.path_registro_usuarios.as_str()).expect("no se pudo abrir el archivo para lectura/escritura");
    let mut buf=String::new();
    f.read_to_string(&mut buf).expect("No se pudo leer el archivo");
    let mut hash_registro_usuarios:HashMap<u8,Usuario>=serde_json::from_str(&buf).expect("no se pudo inyectar el string porque no tiene nada el archivo");
    self.plataforma.registro_usuarios=hash_registro_usuarios;
}
pub fn cargar_archivo_de_usuarios_activos(&mut self){
    let mut f = File::open(self.path_usuarios_activos.as_str()).expect("no se pudo abrir el archivo para lectura/escritura");
    let mut buf=String::new();
    f.read_to_string(&mut buf).expect("No se pudo leer el archivo");
    let mut hash_usuarios_activos:HashMap<u8,Suscripcion>=serde_json::from_str(&buf).expect("no se pudo inyectar el string porque no tiene nada el archivo");
    self.plataforma.usuarios_activos=hash_usuarios_activos;
}
pub fn cargar_archivo_historial(&mut self){
    let mut f = File::open(self.path_historial_suscripciones.as_str()).expect("no se pudo abrir el archivo para lectura/escritura");
    let mut buf=String::new();
    f.read_to_string(&mut buf).expect("No se pudo leer el archivo");
    let mut historial:Vec<Suscripcion>=serde_json::from_str(&buf).expect("no se pudo inyectar el string porque no tiene nada el archivo");
    self.plataforma.historial=historial;
}

pub fn actualizar_info_del_archivo_registro_usuarios(&self){
    let registro_usuarios_serializados = serde_json::to_string(&self.plataforma.registro_usuarios).expect("no se pudo serializar");
    let mut f=File::create(self.path_registro_usuarios.as_str()).expect("no se sobreescribio correctamente el archivo del registro de usuarios");
    f.write_all(&registro_usuarios_serializados.as_bytes()).expect("no se pudo escribir en el archivo");
}
pub fn actualizar_info_del_archivo_usuarios_activos(&self){
    let usuarios_activos_serializados = serde_json::to_string(&self.plataforma.usuarios_activos).expect("no se pudo serializar");
    let mut f=File::create(self.path_usuarios_activos.as_str()).expect("no se sobreescribio correctamente el archivo de los usuarios activos");
    f.write_all(&usuarios_activos_serializados.as_bytes()).expect("no se pudo escribir en el archivo");
}
pub fn actualizar_info_del_archivo_historial(&self){
    let historial_serializados = serde_json::to_string(&self.plataforma.historial).expect("no se pudo serializar");
    let mut f=File::create(self.path_historial_suscripciones.as_str()).expect("no se sobreescribio correctamente el archivo de los usuarios activos");
    f.write_all(&historial_serializados.as_bytes()).expect("no se pudo escribir en el archivo");
}    

pub fn crear_archivo_plataforma(&self){
    self.crear_archivo_de_usuarios_activos();
    self.crear_archivo_historial();
    self.crear_archivo_registros_usuarios();
}

pub fn crear_archivo_registros_usuarios(&self){
    let hash_vacio:HashMap<u8,Usuario>=HashMap::new();
    let registro_usuarios_serealizados=serde_json::to_string(&hash_vacio).expect("no se pudo serealizar el archivo");
    let mut f=File::create(self.path_registro_usuarios.as_str()).expect("no se pudo crear el archivo");
    f.write_all(&registro_usuarios_serealizados.as_bytes()).expect("error al escribir en el archivo");
}
pub fn crear_archivo_de_usuarios_activos(&self){
    let hash_vacio:HashMap<u8,Suscripcion>=HashMap::new();
    let registros_activos_serealizados=serde_json::to_string(&hash_vacio).expect("no se pudo serealizar el archivo");
    let mut f=File::create(self.path_usuarios_activos.as_str()).expect("no se pudo crear el archivo");
    f.write_all(& registros_activos_serealizados.as_bytes()).expect("no se pudo serealizar el archivo");
}
pub fn crear_archivo_historial(&self){
    let vec_vacio:Vec<Suscripcion>=Vec::new();
    let registro_usuarios_serealizados=serde_json::to_string(&vec_vacio).expect("no se pudo serealizar el archivo");
    let mut f=File::create(self.path_historial_suscripciones.as_str()).expect("no se pudo crear el archivo");
    f.write_all(&registro_usuarios_serealizados.as_bytes()).expect("error al escribir en el archivo");
}

pub fn agregar_usuario_a_la_plataforma_y_al_archivo(&mut self,un_id_del_usuario:u8,un_medio_de_pago:MedioDePago,una_fecha:Fecha,un_tipo_de_suscripcion:TipoDeSuscripcion){
        self.crear_o_cargar_archivo_plataforma();
        self.plataforma.crear_y_agregar_usuario_a_la_plataforma(un_id_del_usuario, un_medio_de_pago, una_fecha, un_tipo_de_suscripcion);
        self.actualizar_info_del_archivo_registro_usuarios();
        self.actualizar_info_del_archivo_usuarios_activos();
    }
    pub fn upgrade_usuario_en_archivo(&mut self, id_del_usuario:&u8) {
        self.crear_o_cargar_archivo_plataforma();
        self.plataforma.upgrade_usuario(id_del_usuario);
        self.actualizar_info_del_archivo_usuarios_activos();
    }
    pub fn downgrade_usuario_en_archivo(&mut self, id_del_usuario:&u8) {
        self.crear_o_cargar_archivo_plataforma();
        self.plataforma.downgrade_usuario(id_del_usuario);
        self.actualizar_info_del_archivo_usuarios_activos();
        self.actualizar_info_del_archivo_historial();
    }
    pub fn cancelar_suscripcion_en_archivos(&mut self,id_del_usuario:&u8){
        self.crear_o_cargar_archivo_plataforma();
        self.plataforma.cancelar_suscripcion(id_del_usuario);
        self.actualizar_info_del_archivo_usuarios_activos();
        self.actualizar_info_del_archivo_historial();
    }
}
#[test]
fn test_agregar_usuario_a_la_plataforma_y_al_archivo() {
    let mut plataforma = PlataformaEnArchivo::new(
        InfoTipoSuscripcion::new(30, 9.99),
        InfoTipoSuscripcion::new(30, 19.99),
        InfoTipoSuscripcion::new(30, 29.99),
        "src/archivo_usuarios_activos_test1.json".to_string(),
        "src/archivo_registro_usuarios_test1.json".to_string(),
        "src/archivo_historial_test1.json".to_string()
    
    );
    let fecha_hoy = Fecha::fecha_de_hoy(11, 6, 2023);
    let medio_de_pago = MedioDePago::Efectivo;
    let tipo_de_suscripcion = TipoDeSuscripcion::Basic;
    plataforma.crear_archivo_plataforma();
    plataforma.agregar_usuario_a_la_plataforma_y_al_archivo(1, medio_de_pago.clone(), fecha_hoy.clone(),tipo_de_suscripcion.clone());
    plataforma.agregar_usuario_a_la_plataforma_y_al_archivo( 2, medio_de_pago.clone(), fecha_hoy.clone(),tipo_de_suscripcion.clone());
}
#[test]
fn test_agregar_y_mejorar_suscripcion_de_usuario() {
    let mut plataforma = PlataformaEnArchivo::new(
        InfoTipoSuscripcion::new(30, 9.99),
        InfoTipoSuscripcion::new(30, 19.99),
        InfoTipoSuscripcion::new(30, 29.99),
        "src/archivo_usuarios_activos_test2.json".to_string(),
        "src/archivo_registro_usuarios_test2.json".to_string(),
        "src/archivo_historial_test2.json".to_string()
    
    );
    let fecha_hoy = Fecha::fecha_de_hoy(11, 6, 2023);
    let medio_de_pago = MedioDePago::Efectivo;
    let tipo_de_suscripcion = TipoDeSuscripcion::Basic;
    plataforma.crear_archivo_plataforma();
    plataforma.agregar_usuario_a_la_plataforma_y_al_archivo(3, medio_de_pago.clone(), fecha_hoy.clone(),tipo_de_suscripcion.clone());
    plataforma.upgrade_usuario_en_archivo(&3);
}
#[test]
fn test_agregar_y_bajar_la_calidad_de_la_suscripcion_de_usuario() {
    let mut plataforma = PlataformaEnArchivo::new(
        InfoTipoSuscripcion::new(30, 9.99),
        InfoTipoSuscripcion::new(30, 19.99),
        InfoTipoSuscripcion::new(30, 29.99),
        "src/archivo_usuarios_activos_test3.json".to_string(),
        "src/archivo_registro_usuarios_test3.json".to_string(),
        "src/archivo_historial_test3.json".to_string()
    
    );
    let fecha_hoy = Fecha::fecha_de_hoy(11, 6, 2023);
    let medio_de_pago = MedioDePago::Efectivo;
    let tipo_de_suscripcion = TipoDeSuscripcion::Basic;
    plataforma.crear_archivo_plataforma();
    plataforma.agregar_usuario_a_la_plataforma_y_al_archivo(4, medio_de_pago.clone(), fecha_hoy.clone(),tipo_de_suscripcion.clone());
    plataforma.downgrade_usuario_en_archivo(&4);
}
#[test]
fn test_anular_una_suscripcion_de_usuario() {
    let mut plataforma = PlataformaEnArchivo::new(
        InfoTipoSuscripcion::new(30, 9.99),
        InfoTipoSuscripcion::new(30, 19.99),
        InfoTipoSuscripcion::new(30, 29.99),
        "src/archivo_usuarios_activos_test4.json".to_string(),
        "src/archivo_registro_usuarios_test4.json".to_string(),
        "src/archivo_historial_test4.json".to_string()
    
    );
    let fecha_hoy = Fecha::fecha_de_hoy(11, 6, 2023);
    let medio_de_pago = MedioDePago::Efectivo;
    let tipo_de_suscripcion = TipoDeSuscripcion::Basic;
    plataforma.crear_archivo_plataforma();
    plataforma.agregar_usuario_a_la_plataforma_y_al_archivo(5, medio_de_pago.clone(), fecha_hoy.clone(),tipo_de_suscripcion.clone());
    plataforma.downgrade_usuario_en_archivo(&5);
}
#[test]
    fn test_crear_y_agregar_usuario_a_la_plataforma() {
        let mut plataforma = Plataforma::new(
            InfoTipoSuscripcion::new(30, 9.99),
            InfoTipoSuscripcion::new(30, 19.99),
            InfoTipoSuscripcion::new(30, 29.99),
        );
        let fecha_hoy = Fecha::fecha_de_hoy(11, 6, 2023);
        let medio_de_pago = MedioDePago::Efectivo;
        let tipo_de_suscripcion = TipoDeSuscripcion::Basic;

        plataforma.crear_y_agregar_usuario_a_la_plataforma(1, medio_de_pago, fecha_hoy, tipo_de_suscripcion);

        assert_eq!(plataforma.registro_usuarios.len(), 1);
        assert_eq!(plataforma.usuarios_activos.len(), 1);
    }

    #[test]
    fn test_upgrade_usuario() {
        let mut plataforma = Plataforma::new(
            InfoTipoSuscripcion::new(30, 9.99),
            InfoTipoSuscripcion::new(30, 19.99),
            InfoTipoSuscripcion::new(30, 29.99),
        );
        let fecha_hoy = Fecha::fecha_de_hoy(11, 6, 2023);
        let medio_de_pago = MedioDePago::Efectivo;
        let tipo_de_suscripcion = TipoDeSuscripcion::Basic;

        plataforma.crear_y_agregar_usuario_a_la_plataforma(1, medio_de_pago, fecha_hoy, tipo_de_suscripcion);
        plataforma.upgrade_usuario(&1);

        let suscripcion = plataforma.usuarios_activos.get(&1).unwrap();
        assert_eq!(suscripcion.tipo_de_suscripcion, TipoDeSuscripcion::Clasic);
    }

    #[test]
    fn test_cancelar_suscripcion() {
        let mut plataforma = Plataforma::new(
            InfoTipoSuscripcion::new(30, 9.99),
            InfoTipoSuscripcion::new(30, 19.99),
            InfoTipoSuscripcion::new(30, 29.99),
        );
        let fecha_hoy = Fecha::fecha_de_hoy(11, 6, 2023);
        let medio_de_pago = MedioDePago::Efectivo;
        let tipo_de_suscripcion = TipoDeSuscripcion::Basic;

        plataforma.crear_y_agregar_usuario_a_la_plataforma(1, medio_de_pago, fecha_hoy, tipo_de_suscripcion);
        plataforma.cancelar_suscripcion(&1);

        assert_eq!(plataforma.usuarios_activos.len(), 0);
        assert_eq!(plataforma.historial.len(), 1);
    }

    #[test]
    fn test_downgrade_usuario() {
        let mut plataforma = Plataforma::new(
            InfoTipoSuscripcion::new(30, 9.99),
            InfoTipoSuscripcion::new(30, 19.99),
            InfoTipoSuscripcion::new(30, 29.99),
        );
        let fecha_hoy = Fecha::fecha_de_hoy(11, 6, 2023);
        let medio_de_pago = MedioDePago::Efectivo;
        let tipo_de_suscripcion = TipoDeSuscripcion::Clasic;

        plataforma.crear_y_agregar_usuario_a_la_plataforma(1, medio_de_pago, fecha_hoy, tipo_de_suscripcion);
        plataforma.downgrade_usuario(&1);

        let suscripcion = plataforma.usuarios_activos.get(&1).unwrap();
        assert_eq!(suscripcion.tipo_de_suscripcion, TipoDeSuscripcion::Basic);
    }

    #[test]
    fn test_medio_de_pago_mas_utilizado_activo() {
        let mut plataforma = Plataforma::new(
            InfoTipoSuscripcion::new(30, 9.99),
            InfoTipoSuscripcion::new(30, 19.99),
            InfoTipoSuscripcion::new(30, 29.99),
        );
        let fecha_hoy = Fecha::fecha_de_hoy(11, 6, 2023);
        let medio_de_pago_1 = MedioDePago::Efectivo;
        let medio_de_pago_2 = MedioDePago::MercadoPago { numero_de_mercado: 1 };
        let medio_de_pago_3 = MedioDePago::Efectivo;

        plataforma.crear_y_agregar_usuario_a_la_plataforma(1, medio_de_pago_1, fecha_hoy.clone(), TipoDeSuscripcion::Basic);
        plataforma.crear_y_agregar_usuario_a_la_plataforma(2, medio_de_pago_2, fecha_hoy.clone(), TipoDeSuscripcion::Basic);
        plataforma.crear_y_agregar_usuario_a_la_plataforma(3, medio_de_pago_3, fecha_hoy.clone(), TipoDeSuscripcion::Basic);

        let medio_de_pago_mas_utilizado = plataforma.medio_de_pago_mas_utilizado_activo();
        assert_eq!(medio_de_pago_mas_utilizado, MedioDePago::Efectivo);
    }

    #[test]
    fn test_tipo_de_suscripcion_mas_utilizado_activos() {
        let mut plataforma = Plataforma::new(
            InfoTipoSuscripcion::new(30, 9.99),
            InfoTipoSuscripcion::new(30, 19.99),
            InfoTipoSuscripcion::new(30, 29.99),
        );
        let fecha_hoy = Fecha::fecha_de_hoy(11, 6, 2023);

        plataforma.crear_y_agregar_usuario_a_la_plataforma(1, MedioDePago::Efectivo, fecha_hoy.clone(), TipoDeSuscripcion::Basic);
        plataforma.crear_y_agregar_usuario_a_la_plataforma(2, MedioDePago::Efectivo, fecha_hoy.clone(), TipoDeSuscripcion::Clasic);
        plataforma.crear_y_agregar_usuario_a_la_plataforma(3, MedioDePago::Efectivo, fecha_hoy.clone(), TipoDeSuscripcion::Basic);

        let tipo_de_suscripcion_mas_utilizado = plataforma.tipo_de_suscripcion_mas_utilizado_activos();
        assert_eq!(tipo_de_suscripcion_mas_utilizado, TipoDeSuscripcion::Basic);
    }

    #[test]
    fn test_medio_de_pago_mas_utilizado() {
        let mut plataforma = Plataforma::new(
            InfoTipoSuscripcion::new(30, 9.99),
            InfoTipoSuscripcion::new(30, 19.99),
            InfoTipoSuscripcion::new(30, 29.99),
        );
        let fecha_hoy = Fecha::fecha_de_hoy(11, 6, 2023);
        let medio_de_pago_1 = MedioDePago::TarjetaDeCredito { numero_de_tarjeta: 1 };
        let medio_de_pago_2 = MedioDePago::MercadoPago { numero_de_mercado: 1 };
        let medio_de_pago_3 = MedioDePago::TarjetaDeCredito { numero_de_tarjeta: 1 };

        plataforma.crear_y_agregar_usuario_a_la_plataforma(1, medio_de_pago_1, fecha_hoy.clone(), TipoDeSuscripcion::Basic);
        plataforma.crear_y_agregar_usuario_a_la_plataforma(2, medio_de_pago_2, fecha_hoy.clone(), TipoDeSuscripcion::Basic);
        plataforma.crear_y_agregar_usuario_a_la_plataforma(3, medio_de_pago_3, fecha_hoy.clone(), TipoDeSuscripcion::Basic);

        plataforma.cancelar_suscripcion(&1);
        plataforma.cancelar_suscripcion(&2);
        plataforma.cancelar_suscripcion(&3);

        let medio_de_pago_mas_utilizado = plataforma.medio_de_pago_mas_utilizado();
        assert_eq!(medio_de_pago_mas_utilizado,MedioDePago::TarjetaDeCredito { numero_de_tarjeta: 0 });
    }

    #[test]
    fn test_tipo_de_suscripcion_mas_utilizada() {
        let mut plataforma = Plataforma::new(
            InfoTipoSuscripcion::new(30, 9.99),
            InfoTipoSuscripcion::new(30, 19.99),
            InfoTipoSuscripcion::new(30, 29.99),
        );
        let fecha_hoy = Fecha::fecha_de_hoy(11, 6, 2023);

        plataforma.crear_y_agregar_usuario_a_la_plataforma(1, MedioDePago::Efectivo, fecha_hoy.clone(), TipoDeSuscripcion::Basic);
        plataforma.crear_y_agregar_usuario_a_la_plataforma(2, MedioDePago::Efectivo, fecha_hoy.clone(), TipoDeSuscripcion::Clasic);
        plataforma.crear_y_agregar_usuario_a_la_plataforma(3, MedioDePago::Efectivo, fecha_hoy.clone(), TipoDeSuscripcion::Basic);

        plataforma.cancelar_suscripcion(&1);
        plataforma.cancelar_suscripcion(&2);
        plataforma.cancelar_suscripcion(&3);

        let tipo_de_suscripcion_mas_utilizada = plataforma.tipo_de_suscripcion_mas_utilizada();
        assert_eq!(tipo_de_suscripcion_mas_utilizada, TipoDeSuscripcion::Basic);
    }

//cargo tarpaulin --target-dir src/coverage --skip-clean --exclude-files=target/debug/* --out html
//cargo test  -- --test-threads 1
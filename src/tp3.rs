use std::collections::{HashMap, VecDeque};

/*1- Escribir un programa que defina una estructura Persona que tenga campos para el
nombre, la edad y la dirección(que puede ser nulo al momento de la creación de una
persona). Para dicha estructura implemente los siguientes métodos:
➢ new: que pasando los parámetros correspondientes, crea una Persona y la retorna.
➢ imprimir: que imprime los datos de la persona sobre el mensaje ejecutado por ej:
person.imprimir() , donde person es una variable del tipo Persona.
➢ obtener_edad: retorna la edad de la persona.
➢ actualizar_direccion(nueva_direccion) */
pub struct Persona{
    nombre:String,
    edad:u32,
    direccion:Option<String>
}
impl Persona{
    pub fn new(nombre:String,edad:u32,direccion:Option<String>)->Persona{
        Persona{nombre,edad,direccion}
    }
    pub fn imprimir(&self){
        print!("nombre {}, edad{},",self.nombre,self.edad);
        if let Some(direccion)=&self.direccion{
            println!("direccion {}",direccion);
        }
    }
    pub fn obtener_edad(&self)->u32{
        self.edad
    }
    pub fn actualizar_direccion(&mut self,direccion:String){
        self.direccion=Some(direccion);
    }
}
/*2- Escribir un programa que defina la estructura Rectángulo que tenga campos para la
longitud y el ancho. Para dicha estructura implemente los siguientes métodos:
➢ new: que pasando los parámetros correspondientes, crea un Rectángulo y lo
retorna.
➢ calcular_area: calcular el área y la retorna.
➢ calcular_perimetro: calcula el perímetro y lo retorna.
➢ es_cuadrado: retorna true si es cuadrado, false caso contrario */
pub struct Rectángulo{
    longitud:f32,
    ancho:f32
}
impl Rectángulo{
    pub fn new(longitud:f32,ancho:f32)->Self{
        Self { longitud, ancho }
    }
    pub fn calcular_area(&self)->f32{
        self.longitud*self.ancho
    }
    pub fn calcular_perimetro(&self)->f32{
        2.0*(self.ancho+self.longitud)
    }
    pub fn es_cuadrado(&self)->bool{
        self.ancho==self.longitud
    }
}
/*3- Escribir un programa que defina una estructura Fecha que tenga campos para el día, el
mes y el año. Para dicha estructura implemente los siguientes métodos:
➢ new: que pasando los parámetros correspondientes, crea una Fecha y la retorna.
➢ es_fecha_valida: retorna true si es una fecha valida, false caso contrario.//tenga en
cuenta los años bisiestos también.
➢ es_bisiesto: retorna true si el año de la fecha pertenece a un año bisiesto.
➢ sumar_dias(dias): suma la cantidad de días a la fecha, modificándose
➢ restar_dias(dias): resta la cantidad de días a la fecha, modificándose
➢ es_mayor(una_fecha): que retorna true si la fecha que recibe el mensaje es mayor a
la fecha pasada por parámetro.. */
#[derive(Clone,Debug,PartialEq)]
pub struct Fecha{
    dia:u8,
    mes:u8,
    anio:u128
}
impl Fecha {
    pub fn new(dia: u8, mes: u8, anio: u128) -> Fecha {
        Fecha { dia, mes, anio }
    }
    pub fn es_fecha_valida(&self) -> bool {
        if self.mes < 1 || self.mes > 12 {
            return false;
        }
        let dias_en_mes = match self.mes {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => {
                if self.es_bisiesto() {
                    29
                } else {
                    28
                }
            }
            _ => return false,
        };
        self.dia >= 1 && self.dia <= dias_en_mes
    }
    pub fn es_bisiesto(&self) -> bool {
        if self.anio % 4 == 0{
            if self.anio % 100 ==0{
                if self.anio % 400 ==0{
                    true
                }
                else{
                    false
                }
            }
            else{
                true
            }
        }
        else {
            false
        }
    }
    pub fn sumar_dias(&mut self, dias: u32)->bool {
        if self.es_fecha_valida(){
            for i in 0..dias {
                self.dia += 1;
                if !self.es_fecha_valida() {
                    self.dia = 1;
                    self.mes += 1;
                    if self.mes > 12 {
                        self.mes = 1;
                        self.anio += 1;
                    }
                }
            }
            true
        }else {
            false
        }
    }
    pub fn restar_dias(&mut self, dias: u32)->bool {
        if self.es_fecha_valida(){
            for i in 0..dias {
                if self.dia == 1 {
                    self.mes -= 1;
                    if self.mes == 0 {
                        self.mes = 12;
                        self.anio -= 1;
                    }
                    let dias_en_mes = match self.mes {
                        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
                        6 | 9 | 11 => 30,
                        _=> {               // el otro unico mes posible es que sea el 2 pero necesito poner el _ para finalizar el match
                            if self.es_bisiesto() {
                                29
                            } else {
                                28
                            }
                        }
                    };
                    self.dia = dias_en_mes;
                } else {
                    self.dia -= 1;
                }
            }
            true
        }
        else{
            false
        }
    }
    pub fn es_mayor(&self, otra_fecha: &Fecha) -> bool {
        if self.anio > otra_fecha.anio {
            true
        } else if self.anio == otra_fecha.anio && self.mes > otra_fecha.mes {
            true
        } else if self.anio == otra_fecha.anio && self.mes == otra_fecha.mes && self.dia > otra_fecha.dia {
            true
        } else {
            false
        }
    }
    pub fn igual_fecha(&self,otro:&Self)->bool{
        self.anio==otro.anio&&self.mes==otro.mes&&self.dia==otro.dia
    }
}
/*4- Escribir un programa que defina la estructura Triángulo que tenga campos para las
longitudes de sus tres lados. Para dicha estructura implemente los siguientes métodos:
➢ new: que pasando los parámetros correspondientes, crea un Triángulo y lo retorna.
➢ determinar_tipo: retorna el tipo del triángulo, los tipos pueden ser equilátero,
isósceles o escaleno.
➢ calcular_area: calcular el área y la retorna.
➢ calcular_perimetro: calcula el perímetro y lo retorna */
pub struct Triangulo{
    lado1:f32,
    lado2:f32,
    lado3:f32
}
impl Triangulo{
    pub fn new(lado1:f32,lado2:f32,lado3:f32)->Self{
        Self { lado1, lado2, lado3 }
    }
    pub fn calcular_perimetro(&self)->Option<f32>{
        if self.es_triangulo_valido(){
            Some(self.lado1+self.lado2+self.lado3)
        }else{
            None
        }
    }
    pub fn calcular_area(&self)->Option<f32>{
        if self.es_triangulo_valido(){
            let s=self.calcular_perimetro().unwrap()/2.0;
            let area=s*(s-self.lado1)*(s-self.lado2)*(s-self.lado3);
            return Some(area.sqrt())
        }
        None
    }
    pub fn determinar_tipo(&self)->Option<String>{
        if self.es_triangulo_valido(){
            if self.lado1==self.lado2 && self.lado1==self.lado3{
                return Some("equilatero".to_string())
            }
            if self.lado1==self.lado2 || self.lado1==self.lado3 || self.lado2==self.lado3{
                return Some("isosceles".to_string())
            }
            return Some("escaleno".to_string())
        }
        None
    }
    fn es_triangulo_valido(&self)->bool{
        self.lado1>=0.0 && self.lado2>=0.0 && self.lado3>=0.0 && self.lado1+self.lado2>self.lado3 && self.lado1+self.lado3>self.lado2 && self.lado2+self.lado3>self.lado1
    }
}

/*5- Escribir un programa que defina una estructura Producto que tenga campos para el
nombre, el precio bruto y un número identificatorio. Para dicha estructura implemente los
siguientes métodos:
➢ new: que pasando los parámetros correspondientes, crea un Producto y lo retorna.
➢ calcular_impuestos(porcentaje_de_impuestos): retorna el valor de impuestos sobre
el precio bruto
➢ aplicar_descuento(porcentaje_de_descuento): retorna el valor del porcentaje de
descuento sobre el precio bruto
➢ calcular_precio_total(porcentaje_de_impuestos, porcentaje_descuento): retorna el
precio total a pagar aplicando impuesto y descuento. Tenga en cuenta que los
parámetros son opcionales. */
pub struct Producto{
    nombre:String,
    precio:f64,
    id:u32
} 
impl Producto{
    pub fn new(nombre:String,precio:f64,id:u32)->Self{
        Self { nombre, precio, id }
    }
    pub fn calcular_impuestos(&self,porcentaje_de_impuesto:f64)->f64{
        self.precio*(porcentaje_de_impuesto/100.0)
    }
    pub fn calcular_descuento(&self,porcentaje_de_descuento:f64)->f64{
        self.precio*(porcentaje_de_descuento/100.0)
    }
    pub fn calcular_precio_total(&self,porcentaje_de_impuesto:Option<f64>,porcentaje_de_descuento:Option<f64>)->f64{
        let mut impuesto=0.0;
        let mut descuento=0.0;
        if let Some(valor_porcentaje_impuesto)=porcentaje_de_impuesto{
            if valor_porcentaje_impuesto>0.0{
                impuesto=self.calcular_impuestos(valor_porcentaje_impuesto);
            }
        }
        if let Some(valor_porcentaje_descuento)=porcentaje_de_descuento{
            if valor_porcentaje_descuento>0.0{
                descuento=self.calcular_descuento(valor_porcentaje_descuento);
            }
        }
        let precio_final=self.precio+impuesto-descuento;
        //if precio_final<0{
        //    return None       deberia chequear si el precio final es negativo?
        //}
        precio_final
    }
}
/*6- Escribir un programa que defina una estructura Estudiante que tenga campos para el
nombre, el número de identificación y las calificaciones de exámenes. De cada Examen se
conoce el nombre de la materia y la nota. Para dichas estructuras implemente los siguientes
métodos:
❖ Examen:
➢ new: que pasando los parámetros correspondientes, crea un Examen y lo
retorna.
❖ Estudiante:
➢ new: que pasando los parámetros correspondientes, crea un Estudiante y lo
retorna.
➢ obtener_promedio: retorna el promedio de las notas.
➢ obtener_calificacion_mas_alta: retorna la nota más alta.
➢ obtener_calificacion_mas_baja: retorna la nota más baja.
Nota: Tenga en cuenta que el Estudiante puede tener entre 0 y n notas de examen */
pub struct Examen{
    nombre_de_materia:String,
    nota:f64
}
impl Examen{
    pub fn new(nombre_de_materia:String,nota:f64)->Self{
        Self {nombre_de_materia, nota}
    }
}
pub struct Estudiante{
    nombre:String,
    id:u32,
    examenes:Vec<Examen>
}
impl Estudiante{
    pub fn new(nombre:String,id:u32)->Self{
        Self { nombre, id, examenes:Vec::new() }
    }
    pub fn agregar_examen(&mut self, examen:Examen){
        self.examenes.push(examen);
    }
    pub fn obtener_promedio(&self)->Option<f64>{
        if self.examenes.is_empty(){
            return None
        }
        let mut promedio=0.0;
        let mut cant_exam=0;
        for examen in &self.examenes{
            promedio+=examen.nota;
            cant_exam+=1;
        }
        promedio=promedio / cant_exam as f64;
        println!("promedio {}",promedio);
        Some(promedio)
    }
    pub fn obtener_calificacion_mas_alta(&self)->Option<f64>{
        if self.examenes.is_empty(){
            return None
        }
        let mut nota_mas_alta=0.0;
        for examen in &self.examenes{
            if examen.nota>nota_mas_alta{
                nota_mas_alta=examen.nota
            }
        }
        Some(nota_mas_alta)
    }
    pub fn obtener_calificacion_mas_baja(&self)->Option<f64>{
        if self.examenes.is_empty(){
            return None
        }
        let mut nota_mas_baja=9999.9999;
        for examen in &self.examenes{
            if examen.nota<nota_mas_baja{
                nota_mas_baja=examen.nota
            }
        }
        Some(nota_mas_baja)
    }
}
/*7- Defina una estructura llamada ConcesionarioAuto donde se conoce el nombre, la
dirección y tiene una capacidad máxima para albergar X cantidad de autos. De los autos se
conocen los campos de la marca, modelo, año, precio bruto y color que pueden ser:rojo,
verde, azul, amarillo, blanco o negro.
Para dichas estructuras implemente los siguientes métodos:
❖ ConcesionarioAuto:
➢ new: que pasando los parámetros correspondientes, crea un
ConcesionarioAuto y lo retorna.
➢ agregar_auto(auto): agrega un auto a la lista de autos que tiene sin superar
la máxima cantidad para albergarlos y retorna true, en caso de que lo supere
no lo agrega y retorna false.
➢ eliminar_auto(auto): elimina un auto de la lista de autos.
➢ buscar_auto(auto): busca un auto y si lo encuentra lo retorna.
❖ Auto:
➢ new: que pasando los parámetros correspondientes, crea un Auto y lo
retorna.
➢ calcular_precio: retorna el precio del auto aplicando los siguientes criterios:
■ si es de color primario le aplica un recargo del 25%, sino le aplica un
descuento del 10%.
■ si la marca es BMW le aplica un recargo del 15%-
■ si el año es menor a 2000 le aplica un descuento del 5%. */
pub enum Color{
    Rojo,
    Verde,
    Azul,
    Amarillo,
    Blanco,
    Negro
}
pub struct Auto{
    marca:String,
    modelo:String,
    año:u128,
    precio:f64,
    color:Color
}
pub struct ConcesionarioAuto{
    nombre:String,
    direccion:String,
    capacidad_de_autos:u128,
    autos:Vec<Auto>
}
impl ConcesionarioAuto{
    pub fn new(nombre:String,direccion:String,capacidad_de_autos:u128)->Self{
        Self { nombre, direccion, capacidad_de_autos, autos:Vec::new() }
    }
    pub fn agregar_auto(&mut self,auto:Auto){
        if self.autos.len()<=self.capacidad_de_autos as usize{
            self.autos.push(auto);
        }
    }
    pub fn eliminar_auto(&mut self,auto_a_eliminar:&Auto){
        for i in 0..self.autos.len(){
            let auto =self.autos.get(i).unwrap();
            if auto.igual_auto(auto_a_eliminar){
                self.autos.remove(i);
                break;
            }
        }
    }
    pub fn buscar_auto(&self,auto_buscado:&Auto)->Option<&Auto>{
        let mut auto: Option<&Auto>=None;
        for un_auto in &self.autos{
            if un_auto.igual_auto(auto_buscado){
                auto=Some(un_auto);
            }
            break;
        }
        auto
    }
}
impl Color{
    pub fn es_rojo(&self)->bool{
        match self{
            Color::Rojo=>true,
            _=>false
        }
    }
    pub fn es_verde(&self)->bool{
        match self{
            Color::Verde=>true,
            _=>false
        }
    }
    pub fn es_azul(&self)->bool{
        match self{
            Color::Azul=>true,
            _=>false
        }
    }
    pub fn es_amarillo(&self)->bool{
        match self{
            Color::Amarillo=>true,
            _=>false
        }
    }
    pub fn es_blanco(&self)->bool{
        match self{
            Color::Blanco=>true,
            _=>false
        }
    }
    pub fn es_negro(&self)->bool{
        match self{
            Color::Negro=>true,
            _=>false
        }
    }
    pub fn igual_color(&self,otro:&Self)->bool{
        match self{
            Color::Rojo=>otro.es_rojo(),
            Color::Verde=>otro.es_verde(),
            Color::Azul=>otro.es_azul(),
            Color::Amarillo=>otro.es_amarillo(),
            Color::Blanco=>otro.es_blanco(),
            Color::Negro=>otro.es_negro()
        }
    }
}
impl Auto {
    pub fn new(marca:String,modelo:String,año:u128,precio:f64,color:Color)->Self{
        Self { marca, modelo, año, precio, color }
    }
    pub fn calcular_precio(&self)->f64{
        let mut precio_final=self.precio.clone();
        if self.color.es_amarillo() || self.color.es_azul() || self.color.es_rojo(){
            precio_final+=self.precio*0.25;
        }else{
            precio_final-=self.precio*0.10;
        }
        if self.marca=="BMW".to_string(){
            precio_final+=self.precio*0.15;
        }
        if self.año<2000{
            precio_final-=self.precio*0.05
        }
        precio_final
    }
    pub fn igual_auto(&self,otro:&Self)->bool{
        self.año==otro.año && self.marca==otro.marca && self.modelo==otro.modelo && self.precio==otro.precio && self.color.igual_color(&otro.color)
    }
    //pub fn es_auto_valido{} no tendria que existir un auto con precio negativo o que sea del año anterior al siglo XVII
}
/*8- Defina la estructura Cancion con campos para el título, el artista y el género. El género
puede ser rock, pop, rap, jazz, otros. Luego modele una playlist. La playlist está compuesta
por una lista de canciones y un nombre, y se permiten hacer las siguientes acciones sobre
ella:
➔ agregar canción.
➔ eliminar canción.
➔ mover canción // mueve la canción a una determinada posición de la playlist.
➔ buscar canción por nombre.
➔ obtener las canciones de un determinado género.
➔ obtener las canciones de un determinado artista.
➔ modificar título de la playlist.
➔ eliminar todas las canciones. */
#[derive(Clone,Debug)]
pub enum GeneroM{
    Rock,
    Pop,
    Rap,
    Jazz,
    Otros
}
#[derive(Clone,Debug)]
pub struct Cancion{
    titulo:String,
    artista:String,
    genero:GeneroM
}
pub struct Playlist{
    nombre:String,
    lista_de_canciones:Vec<Cancion> 
}
impl GeneroM{
    pub fn es_rock(&self)->bool{
        match self{
            GeneroM::Rock=>true,
            _=>false
        }
    }
    pub fn es_pop(&self)->bool{
        match self{
            GeneroM::Pop=>true,
            _=>false
        }
    }
    pub fn es_rap(&self)->bool{
        match self{
            GeneroM::Rap=>true,
            _=>false
        }
    }
    pub fn es_jazz(&self)->bool{
        match self{
            GeneroM::Jazz=>true,
            _=>false
        }
    }
    pub fn es_otro(&self)->bool{
        match self{
            GeneroM::Otros=>true,
            _=>false
        }
    }
    pub fn igual_genero(&self,otro:&Self)->bool{
        match self{
            GeneroM::Rock=>otro.es_rock(),
            GeneroM::Pop=>otro.es_pop(),
            GeneroM::Rap=>otro.es_rap(),
            GeneroM::Jazz=>otro.es_jazz(),
            GeneroM::Otros=>otro.es_otro()
        }
    }
}
impl Cancion{
    pub fn new(titulo:String,artista:String,genero:GeneroM)->Self{
        Self {titulo, artista, genero}
    }
    pub fn igual_cancion(&self,otro:&Self)->bool{
        self.artista==otro.artista && self.titulo==otro.titulo && self.genero.igual_genero(&otro.genero)
    }
}
impl Playlist{
    pub fn new(nombre:String)->Self{
        Self { nombre, lista_de_canciones:Vec::new() }
    }
    pub fn agregar_cancion(&mut self,cancion:Cancion){
        self.lista_de_canciones.push(cancion);
    }
    pub fn cambiar_nombre_playlist(&mut self,un_nombre:String){
        self.nombre=un_nombre;
    }
    pub fn eliminar_todas_las_canciones(&mut self){
        self.lista_de_canciones.clear()
    }
    pub fn eliminar_cancion_en_especifico(&mut self,cancion_a_borrar:&Cancion){
        if !self.lista_de_canciones.is_empty(){
            for i in 0..self.lista_de_canciones.len(){
                let cancion=self.lista_de_canciones.get(i).unwrap();
                if cancion.igual_cancion(cancion_a_borrar){
                    self.lista_de_canciones.remove(i);
                    break;
                }
            }
        }
    }

    pub fn mover_cancion_pasando_una_cancion(&mut self,cancion:Cancion,posicion:usize){
        if posicion<=self.lista_de_canciones.len(){
            self.eliminar_cancion_en_especifico(&cancion);
            self.lista_de_canciones.insert(posicion, cancion)
        }
    }
    pub fn mover_cancion_pasando_referencia_de_cancion(&mut self,cancion_a_mover:&Cancion,posicion_final:usize){
        if posicion_final<=self.lista_de_canciones.len(){
            let mut posicion_base: Option<usize>=None;
            let mut i=0;
            for cancion in &self.lista_de_canciones{
                if cancion.igual_cancion(cancion_a_mover){
                    posicion_base=Some(i);
                    break;
                }
                i+=1;
            }
            if posicion_base.is_some(){
                let cancion_movida=self.lista_de_canciones.get(posicion_base.unwrap()).unwrap().clone();
                self.lista_de_canciones.insert(posicion_final, cancion_movida);
                self.lista_de_canciones.remove(posicion_base.unwrap());
            }
        }
    }
    pub fn mover_cancion_con_posiciones(&mut self,posicion_base:usize,posicion_final:usize){
        if posicion_base<=self.lista_de_canciones.len()&& posicion_final<=self.lista_de_canciones.len(){
            let cancion_movida=self.lista_de_canciones.remove(posicion_base);
            self.lista_de_canciones.insert(posicion_final, cancion_movida);
        }
    }
    pub fn mover_cancion_manteniendo_orden(&mut self,posicion_base:usize,posicion_final:usize){
        if posicion_base<=self.lista_de_canciones.len()&& posicion_final<=self.lista_de_canciones.len(){
            let cancion_movida=self.lista_de_canciones.get(posicion_base).unwrap().clone();
            self.lista_de_canciones.insert(posicion_final, cancion_movida);
            self.lista_de_canciones.remove(posicion_base);
        }
    }
    pub fn lista_de_canciones_de_determinado_genero(&self,un_genero:GeneroM)->Option<Vec<&Cancion>>{
        if self.lista_de_canciones.is_empty(){
            return None
        }
        let mut lista_de_canciones_de_un_genero=Vec::new();
        for cancion in &self.lista_de_canciones{
            if cancion.genero.igual_genero(&un_genero){
                lista_de_canciones_de_un_genero.push(cancion);
            }
        }
        Some(lista_de_canciones_de_un_genero)
    }
    pub fn lista_de_canciones_de_determinado_artista(&self,un_artista:String)->Option<Vec<&Cancion>>{
        if self.lista_de_canciones.is_empty(){
            return None;
        }
        let mut lista_de_canciones_de_un_artista=Vec::new();
        for cancion in &self.lista_de_canciones{
            if cancion.artista==un_artista{
                lista_de_canciones_de_un_artista.push(cancion);
            }
        }
        Some(lista_de_canciones_de_un_artista)
    }
}

/* 9.-Dada una cadena de veterinarias se desea implementar un sistema de atención de
pacientes para cada veterinaria, de la veterinaria se conoce el nombre, la dirección y un id.
Para la atención de mascotas se requiere administrar una cola de atención. De la mascota
se conoce el nombre, la edad, el tipo de animal(perro, gato, caballo, otros) y su dueño. Del
dueño se conoce el nombre, la dirección y un teléfono de contacto. Luego de la atención se
desea tener un registro de las atenciones realizadas guardando los datos de la mascota, el
diagnóstico final, tratamiento y fecha de la próxima visita si es que se requiere.
Dado todo lo mencionado anteriormente implemente los métodos para realizar las
siguientes acciones:
➔ crear una veterinaria.
➔ agregar una nueva mascota a la cola de atención de la veterinaria.
➔ agregar una nueva mascota a la cola de atención pero que sea la siguiente
en atender porque tiene la máxima prioridad.
➔ atender la próxima mascota de la cola.
➔ eliminar una mascota específica de la cola de atención dado que se retira.
➔ registrar una atención.
➔ buscar una atención dado el nombre de la mascota, el nombre del dueño y el
teléfono.
➔ modificar el diagnóstico de una determinada atención.
➔ modificar la fecha de la próxima visita de una determinada atención.
➔ eliminar una determinada atención.
Nota: para la fecha utilice lo implementado en el punto 3.*/
#[derive(Clone,Debug,PartialEq)]
pub enum TipoAnimal{
    Perro,
    Gato,
    Caballo,
    Otro
}
#[derive(Clone,Debug,PartialEq)]
pub struct Dueño{
    nombre:String,
    direccion:String,
    telefono:u128
}
#[derive(Clone,Debug,PartialEq)]
pub struct Mascota{
    nombre:String,
    edad:u32,
    tipo_de_animal:TipoAnimal,
    dueño:Dueño
}
#[derive(Clone,Debug,PartialEq)]
pub struct RegistroAtencion{
    mascota:Mascota,
    diagnostico:String,
    tratamiento:String,
    fecha_proxima_visita:Option<Fecha>
}
pub struct Veterinaria{
    nombre:String,
    direccion:String,
    id:u32,
    cola_de_atencion:VecDeque<Mascota>,
    registro:Vec<RegistroAtencion>
}
impl TipoAnimal {
    pub fn es_perro(&self)->bool{
        match self{
            TipoAnimal::Perro=>true,
            _=>false
        }
    }
    pub fn es_gato(&self)->bool{
        match self{
            TipoAnimal::Gato=>true,
            _=>false
        }
    }
    pub fn es_caballo(&self)->bool{
        match self{
            TipoAnimal::Caballo=>true,
            _=>false
        }
    }
    pub fn es_otro(&self)->bool{
        match self{
            TipoAnimal::Otro=>true,
            _=>false
        }
    }
    pub fn igual_tipo(&self,otro:&Self)->bool{
        match self{
            TipoAnimal::Perro=>otro.es_perro(),
            TipoAnimal::Gato=>otro.es_gato(),
            TipoAnimal::Caballo=>otro.es_caballo(),
            TipoAnimal::Otro=>otro.es_otro()
        }
    }
}
impl Dueño{
    pub fn new(nombre:String,direccion:String,telefono:u128)->Self{
        Self { nombre, direccion, telefono}
    }
    pub fn igual_dueño(&self,otro:&Self)->bool{
        self.nombre==otro.nombre && self.direccion==otro.direccion && self.telefono==otro.telefono
    }
}
impl Mascota{
    pub fn new(nombre:String,edad:u32,tipo_de_animal:TipoAnimal,dueño:Dueño)->Self{
        Self { nombre, edad, tipo_de_animal, dueño}
    }
    pub fn igual_mascota(&self,otro:&Self)->bool{
        self.nombre==otro.nombre && self.edad==otro.edad && self.tipo_de_animal.igual_tipo(&otro.tipo_de_animal)&& self.dueño.igual_dueño(&otro.dueño)
    }
}
impl RegistroAtencion{
    pub fn new(mascota:Mascota,diagnostico:String,tratamiento:String,fecha_proxima_visita:Option<Fecha>)->Self{
        Self { mascota, diagnostico, tratamiento, fecha_proxima_visita}
    }
    pub fn igual_atencion(&self,otro:&Self)->bool{
        if self.diagnostico==otro.diagnostico && self.mascota.igual_mascota(&otro.mascota) && self.tratamiento==otro.tratamiento{
            if self.fecha_proxima_visita.is_some() && otro.fecha_proxima_visita.is_some(){
                if self.fecha_proxima_visita.clone().unwrap().igual_fecha(&otro.fecha_proxima_visita.clone().unwrap()){
                    return true;       
                }
            }
            if self.fecha_proxima_visita.is_none()&&otro.fecha_proxima_visita.is_none(){
                return true;
            }
        }
    false
    }
}
impl Veterinaria{
    pub fn new(nombre:String,id:u32,direccion:String)->Self{
        Self { nombre, direccion, id, cola_de_atencion:VecDeque::new(), registro:Vec::new()}
    }
    pub fn encolar_mascota(&mut self,una_mascota:Mascota){
        self.cola_de_atencion.push_back(una_mascota);
    }
    pub fn encolar_mascota_maxima_prioridad(&mut self,una_mascota:Mascota){
        self.cola_de_atencion.push_front(una_mascota);
    }
    pub fn atender_proxima_mascota(&mut self)->Option<Mascota>{
        self.cola_de_atencion.pop_front()
    }
    pub fn retirar_una_mascota_de_la_cola(&mut self,mascota_a_borrar:&Mascota){
        if !self.cola_de_atencion.is_empty(){
            let mut i=0;
            let mut posicion_encontrada=self.cola_de_atencion.len()+1;
            for mascota in &self.cola_de_atencion{
                if mascota.igual_mascota(mascota_a_borrar){
                    posicion_encontrada=i;
                    break;
                }
                i+=1;
            }
            if posicion_encontrada!=self.cola_de_atencion.len()+1{
                self.cola_de_atencion.remove(posicion_encontrada);
            }
        }
    }
    pub fn registrar_atencion(&mut self,una_mascota:Mascota,un_diagnostico:String,un_tratamiento:String,fecha_de_proxima_visita:Option<Fecha>){
        let registro=RegistroAtencion::new(una_mascota, un_diagnostico, un_tratamiento, fecha_de_proxima_visita);
        self.registro.push(registro);
    }
    pub fn buscar_atencion_por_distintos_parametro(&mut self,nombre_mascota:String,nombre_dueño:String,telefono:u128)->Option<RegistroAtencion>{
        for atencion in &self.registro{
            if atencion.mascota.nombre==nombre_mascota  && atencion.mascota.dueño.nombre==nombre_dueño && atencion.mascota.dueño.telefono==telefono{
                return Some(atencion.clone())
            }
        }
        None
    }
    pub fn cambiar_diagnostico(&mut self,una_atencion:&RegistroAtencion,nuevo_diagnostico:String){
        let mut posicion: usize=0;
        let mut existe=false;
        for i in 0..self.registro.len(){
            let atencion=self.registro.get(i).unwrap();
            if una_atencion.igual_atencion(atencion){
                posicion=i;
                existe=true;
                break;
            }
        }
        if existe{
            let mut registro_atencion= self.registro.get_mut(posicion).unwrap();
            registro_atencion.diagnostico=nuevo_diagnostico;
        }
    }
    pub fn cambiar_fecha(&mut self,una_atencion:&RegistroAtencion,nueva_fecha:Fecha){
        let mut posicion=self.registro.len()+1;
        for i in 0..self.registro.len(){
            let atencion=self.registro.get(i).unwrap();
            if una_atencion.igual_atencion(atencion){
                posicion=i;
                break;
            }
        }
        if posicion!=self.registro.len()+1{
            self.registro.get_mut(posicion).unwrap().fecha_proxima_visita=Some(nueva_fecha);
        }
    }
    pub fn eliminar_atencion(&mut self,una_atencion:&RegistroAtencion){
        let mut posicion=self.registro.len()+1;
        for i in 0..self.registro.len(){
            let atencion=self.registro.get(i).unwrap();
            if una_atencion.igual_atencion(atencion){
                posicion=i;
                break;
            }
        }
        if posicion!=self.registro.len()+1{
            self.registro.remove(posicion);
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_veterinaria() {
        // Crear una veterinaria
        let mut veterinaria = Veterinaria::new("Veterinaria XYZ".to_string(), 1, "Dirección de la veterinaria".to_string());

        // Crear mascotas y dueños
        let dueno1 = Dueño::new("Juan".to_string(), "Dirección 1".to_string(), 123456789);
        let mascota1 = Mascota::new("Mascota 1".to_string(), 3, TipoAnimal::Perro, dueno1.clone());

        let dueno2 = Dueño::new("Pedro".to_string(), "Dirección 2".to_string(), 987654321);
        let mascota2 = Mascota::new("Mascota 2".to_string(), 5, TipoAnimal::Gato, dueno2.clone());

        // Agregar mascotas a la cola de atención
        veterinaria.encolar_mascota(mascota1.clone());
        veterinaria.encolar_mascota(mascota2.clone());

        // Atender la próxima mascota de la cola
        let mascota_atendida = veterinaria.atender_proxima_mascota().unwrap();
        assert_eq!(mascota_atendida.nombre, "Mascota 1");

        // Eliminar una mascota específica de la cola de atención
        veterinaria.retirar_una_mascota_de_la_cola(&mascota2);

        // Registrar una atención
        let atencion = RegistroAtencion::new(mascota1.clone(), "Diagnóstico 1".to_string(), "Tratamiento 1".to_string(), None);
        veterinaria.registrar_atencion(mascota1.clone(), "Diagnóstico 1".to_string(), "Tratamiento 1".to_string(), None);

        // Buscar una atención por distintos parámetros
        let atencion_buscada = veterinaria.buscar_atencion_por_distintos_parametro("Mascota 1".to_string(), "Juan".to_string(), 123456789).unwrap();
        assert_eq!(atencion_buscada, atencion);

        // Modificar la fecha de la próxima visita de una determinada atención
        let nueva_fecha = Fecha::new(10, 8, 2023);
        veterinaria.cambiar_fecha(&atencion, nueva_fecha.clone());
        assert_eq!(veterinaria.registro[0].fecha_proxima_visita.clone().unwrap(), nueva_fecha);

        // Eliminar una determinada atención
        veterinaria.eliminar_atencion(&atencion);
        assert_eq!(veterinaria.registro.len(), 1);
    }
}

/*10-Para una biblioteca se desea implementar un sistema de préstamos de libros. De la
biblioteca se conoce el nombre y la dirección, las copias de los libros a disposición para
prestar y los préstamos efectuados. Los libros a disposición es un registro donde se indica
la cantidad de ejemplares que tiene a disposición para prestar de determinado libro. De
cada libro se conoce el título, autor, número de páginas, género(novela, infantil, técnico,
otros). Para registrar un préstamo se requiere el libro, el cliente, la fecha de vencimiento del
préstamo, la fecha de devolución y el estado que puede ser devuelto o en préstamo. Del
cliente se conoce el nombre, teléfono y dirección de correo electrónico.
Implemente los métodos necesarios para realizar las siguientes acciones:
➔ obtener cantidad de copias: dado un determinado libro retorna el retorna la
cantidad de copias a disposición que hay para prestar de dicho libro.
➔ decrementar cantidad de copias a disposición; dado un libro decrementa en 1
la cantidad de copias de libros a disposición para prestar.
➔ incrementar cantidad de copias a disposición: dado un libro incremente en 1
la cantidad de copias del libro a disposición para ser prestado.
➔ contar préstamos de un cliente: devuelve la cantidad de préstamos en estado
“en préstamo” de un determinado cliente.
➔ ver la cantidad disponible de un determinado libro: retorna la cantidad de
libros disponibles del registro de “copias a disposición” de un determinado
libro.
➔ realizar un préstamo de un libro para un cliente: crea un préstamo de un libro
para un determinado cliente cumpliendo con lo siguiente
◆ el cliente no tenga más de 5 préstamos en el estado “en préstamo”
◆ haya al menos una copia disponible en el registro de copias a
disposición.
De ser así descuenta 1 en el registro de “copias a disposición” y
retorna true, si no cumple con alguna de las condiciones retorna false.
➔ ver préstamos a vencer el los próximos días: retorna una lista de préstamos a
vencer el los próximos días, el valor de días es pasado por parámetro.
➔ ver los préstamos vencidos: retorna una lista de préstamos en el estado “en
préstamos” donde la fecha de vencimiento es menor a la fecha actual.
➔ buscar préstamo: dado un libro y un cliente busca un préstamo y lo retorna si
existe.
➔ devolver libro: dado un libro y un cliente se busca el préstamo y se cambia al
estado “devuelto”, se registra la fecha de devolución y se incrementa la
cantidad de libros en 1 del libro devuelto en el registro de copias a
disposición.
Nota: para la fecha utilice lo implementado en el punto 3 */
enum Genero{
    Novela,
    Infantil,
    Tecnico,
    Otro
}
struct Libro{
    titulo:String,
    autor:String,
    numero_de_paginas:u64,
    id:u8
}
struct InfoLibro{
    libro:Libro,
    cantcopias:u64,
}
#[derive(Clone,Debug,PartialEq)]
struct Cliente{
    nombre:String,
    direccion:String,
    correo:String,
    dni:u8
}
#[derive(Clone,Debug,PartialEq)]
pub struct Prestamo{
    id_libro:u8,
    cliente:Cliente,
    fecha_de_vencimiento:Fecha,
    fecha_de_devolucion:Option<Fecha>,
    devuelto:bool
}
pub struct Biblioteca{
    nombre:String,
    direccion:String,
    informacion_de_todos_los_libros:HashMap<u8,InfoLibro>,
    prestamos:Vec<Prestamo>
}
impl Genero{
    fn es_novela(&self)->bool{
        match self{
        Genero::Novela=>true,
        _=>false
        }
    }
    fn es_infantil(&self)->bool{
        match self{
        Genero::Infantil=>true,
        _=>false
        }
    }
    fn es_tecnico(&self)->bool{
        match self{
        Genero::Tecnico=>true,
        _=>false
        }
    }
    fn es_otro(&self)->bool{
        match self{
            Genero::Otro=>true,
            _=>false
        }
    }
    fn tienen_igual_genero(&self,otro:&Genero)->bool{
        match self{
            Genero::Novela=>otro.es_novela(),
            Genero::Infantil=>otro.es_infantil(),
            Genero::Tecnico=>otro.es_tecnico(),
            _=>otro.es_otro()
        }
    }
}
impl Libro{
    fn new(titulo:String,autor:String,numero_de_paginas:u64,id:u8)->Libro{
        Libro { titulo, autor, numero_de_paginas,id}
    }
}
impl InfoLibro{
    fn new(titulo:String,autor:String,numero_de_paginas:u64,id:u8)->Self{

        let libro=Libro::new(titulo, autor, numero_de_paginas, id);
        InfoLibro { libro, cantcopias:1 }
    }
}
impl Cliente{
    fn new(nombre:String,direccion:String,correo:String,dni:u8)->Cliente{
        Cliente { nombre, direccion, correo,dni }
    }
}
impl Prestamo{
    fn new(id_libro:u8,cliente:Cliente,fecha_de_vencimiento:Fecha)->Prestamo{
        Prestamo {id_libro, cliente, fecha_de_vencimiento, fecha_de_devolucion:None, devuelto:false}
    }
}
impl Biblioteca{
    pub fn new (nombre:String,direccion:String)->Self{
        Self {nombre, direccion, informacion_de_todos_los_libros:HashMap::new(), prestamos:Vec::new()}
    }
    fn obtener_id_libro(&self,titulo:String,autor:String,numero_de_paginas:u64)->Option<u8>{
        for info_libro in &self.informacion_de_todos_los_libros{
            if info_libro.1.libro.titulo==titulo && info_libro.1.libro.autor==autor&&info_libro.1.libro.numero_de_paginas==numero_de_paginas{
                return Some(info_libro.0.clone())
            }
        }
        None
    }
    pub fn ver_la_cantidad_disponible_de_un_determinado_libro(&self,titulo:String,autor:String,numero_de_paginas:u64)->Option<u64>{
        let id_buscado=self.obtener_id_libro(titulo, autor, numero_de_paginas);
        if let Some(id_buscado)=id_buscado{
            return Some(self.informacion_de_todos_los_libros.get(&id_buscado).unwrap().cantcopias)
        }else{
            None
        }
    }
    pub fn decrementar_cantidad_de_copias_a_disposición(&mut self,titulo:String,autor:String,numero_de_paginas:u64){
        let id_buscado=self.obtener_id_libro(titulo, autor, numero_de_paginas);
        if let Some(id_buscado)=id_buscado{
            let info_libro=self.informacion_de_todos_los_libros.get_mut(&id_buscado).unwrap();
            if info_libro.cantcopias>0{
                info_libro.cantcopias-=1;
            }
        }
    }
    pub fn incrementar_cantidad_de_copias_a_disposición(&mut self,titulo:String,autor:String,numero_de_paginas:u64){
        let id_buscado=self.obtener_id_libro(titulo, autor, numero_de_paginas);
        if let Some(id_buscado)=id_buscado{
            self.informacion_de_todos_los_libros.get_mut(&id_buscado).unwrap().cantcopias+=1;
        }
    }
    fn existe_registro_de_un_cliente(&self,dni:u8)->bool{
        for prestamo in &self.prestamos{
            if prestamo.cliente.dni==dni{
                return true
            }
        }
        false
    }
    pub fn contar_prestamos_de_un_cliente(&self,dni:u8)->Option<u8>{
        if self.existe_registro_de_un_cliente(dni){
            let mut cant_prestados=0;
            let mut existe=false;
            for prestamo in &self.prestamos{
                if prestamo.cliente.dni==dni && !prestamo.devuelto{
                    existe=true;
                    cant_prestados+=1;
                }
            }
            if existe{
                return Some(cant_prestados)
            }
        }
        None
    }
    pub fn realizar_un_préstamo_de_un_libro_para_un_cliente(&mut self,nombre:String,direccion:String,correo:String,dni:u8,titulo:String,autor:String,numero_de_paginas:u64,fecha_de_vencimiento:Fecha)->bool{
        let id_buscado=self.obtener_id_libro(titulo.clone(), autor.clone(), numero_de_paginas.clone());
        if let Some(id_buscado) =id_buscado {
            if self.existe_registro_de_un_cliente(dni){
                if self.contar_prestamos_de_un_cliente(dni).unwrap()<=5{
                    if self.ver_la_cantidad_disponible_de_un_determinado_libro(titulo.clone(), autor.clone(), numero_de_paginas.clone()).unwrap()>0{
                        let cliente=Cliente::new(nombre, direccion, correo, dni);
                        let prestamo=Prestamo::new(id_buscado,cliente,fecha_de_vencimiento);
                        self.prestamos.push(prestamo);
                        self.decrementar_cantidad_de_copias_a_disposición(titulo, autor, numero_de_paginas);
                        return true;
                    }
                }
            }
            /*else{
                if self.ver_la_cantidad_disponible_de_un_determinado_libro(titulo.clone(), autor.clone(), numero_de_paginas.clone()).unwrap()>0{     esto no lo pide la consigna pero entonces no se puede registrar un nuevo cliente
                    let cliente=Cliente::new(nombre, direccion, correo, dni);
                    let prestamo=Prestamo::new(id_buscado,cliente,fecha_de_vencimiento);
                    self.prestamos.push(prestamo);
                    self.decrementar_cantidad_de_copias_a_disposición(titulo, autor, numero_de_paginas);
                    return true;
                }
            }*/
        }
    false
    }
    pub fn ver_prestamos_a_vencer_el_los_próximos_días(&self,anio:u128,mes:u8,dia:u8,cantidad_dias:u32)->Option<u8>{
        if !self.informacion_de_todos_los_libros.is_empty(){
            let fecha_de_hoy=Fecha::new(dia, mes, anio);
            if fecha_de_hoy.es_fecha_valida(){
                let mut cantidad_de_prestamos_a_vencer=0;
                let mut fecha_limite=fecha_de_hoy.clone();
                fecha_limite.sumar_dias(cantidad_dias);
                for prestamo in &self.prestamos{
                    if !prestamo.devuelto && fecha_limite.es_mayor(&prestamo.fecha_de_vencimiento)&& prestamo.fecha_de_vencimiento.es_mayor(&fecha_de_hoy){
                        cantidad_de_prestamos_a_vencer+=1;
                    }
                }
                return Some(cantidad_de_prestamos_a_vencer);
            }
        }
        None
    }
    pub fn ver_los_prestamos_vencidos(&self,anio:u128,mes:u8,dia:u8)->Option<Vec<Prestamo>>{
        if !self.prestamos.is_empty(){
            let fecha_actual=Fecha::new(dia, mes, anio);
            if fecha_actual.es_fecha_valida(){
                let mut vec_de_prestamos=Vec::new();
                for prestamo in &self.prestamos{
                    if !prestamo.devuelto && prestamo.fecha_de_vencimiento.es_mayor(&fecha_actual){
                        vec_de_prestamos.push(prestamo.clone());
                    }
                }
                return Some(vec_de_prestamos);
            }
        }
        None
    }
    pub fn buscar_prestamo(&self,dni:u8,titulo:String,autor:String,numero_de_paginas:u64)->bool{
        let id_buscado=self.obtener_id_libro(titulo, autor, numero_de_paginas);
        if let Some(id_buscado)=id_buscado{
            for prestamo in &self.prestamos{
                if prestamo.cliente.dni==dni && prestamo.id_libro==id_buscado{ // se deberia agregar  && !prestamo.devuelto si se debe buscar un prestamo no pagado
                    return true;
                }
            }
        }
        false
    }
    pub fn devolver_libro(&mut self,dni:u8,titulo:String,autor:String,numero_de_paginas:u64){
        let id_buscado=self.obtener_id_libro(titulo.clone(), autor.clone(), numero_de_paginas.clone());
        if let Some(id_buscado)=id_buscado{
            let mut existe=false;
            let mut prestamo: &mut Prestamo;
            let mut posicion=0;
            for i in 0..self.prestamos.len(){
                prestamo=self.prestamos.get_mut(i).unwrap();
                if prestamo.cliente.dni==dni && prestamo.id_libro==id_buscado && !prestamo.devuelto{
                    existe=true;
                    posicion=i;
                    break;
                }
            }
            if existe{
                prestamo=self.prestamos.get_mut(posicion).unwrap();
                prestamo.devuelto=true;
                self.incrementar_cantidad_de_copias_a_disposición(titulo, autor, numero_de_paginas);
            }
        }
    }
}
#[cfg(test)]
mod tests10 {
    use super::*;

    #[test]
    fn test_biblioteca() {
        // Crear una biblioteca
        let mut biblioteca = Biblioteca::new("Biblioteca ABC".to_string(), "Dirección de la biblioteca".to_string());

        // Crear libros
        let libro1 = Libro::new("Libro 1".to_string(), "Autor 1".to_string(), 200, 1);
        let libro2 = Libro::new("Libro 2".to_string(), "Autor 2".to_string(), 150, 2);

        // Agregar información de los libros a la biblioteca
        let info_libro1 = InfoLibro::new(libro1.titulo.clone(), libro1.autor.clone(), libro1.numero_de_paginas, libro1.id);
        let info_libro2 = InfoLibro::new(libro2.titulo.clone(), libro2.autor.clone(), libro2.numero_de_paginas, libro2.id);
        biblioteca.informacion_de_todos_los_libros.insert(libro1.id, info_libro1);
        biblioteca.informacion_de_todos_los_libros.insert(libro2.id, info_libro2);

        // Verificar la cantidad disponible de un determinado libro
        assert_eq!(biblioteca.ver_la_cantidad_disponible_de_un_determinado_libro(libro1.titulo.clone(), libro1.autor.clone(), libro1.numero_de_paginas), Some(1));

        // Decrementar la cantidad de copias a disposición de un libro
        biblioteca.decrementar_cantidad_de_copias_a_disposición(libro1.titulo.clone(), libro1.autor.clone(), libro1.numero_de_paginas);
        assert_eq!(biblioteca.ver_la_cantidad_disponible_de_un_determinado_libro(libro1.titulo.clone(), libro1.autor.clone(), libro1.numero_de_paginas), Some(0));

        // Incrementar la cantidad de copias a disposición de un libro
        biblioteca.incrementar_cantidad_de_copias_a_disposición(libro1.titulo.clone(), libro1.autor.clone(), libro1.numero_de_paginas);
        assert_eq!(biblioteca.ver_la_cantidad_disponible_de_un_determinado_libro(libro1.titulo.clone(), libro1.autor.clone(), libro1.numero_de_paginas), Some(1));
        // Crear un cliente
        let cliente = Cliente::new("Juan".to_string(), "Dirección del cliente".to_string(), "correo@cliente.com".to_string(), 1);
        biblioteca.realizar_un_préstamo_de_un_libro_para_un_cliente(cliente.nombre.clone(), cliente.direccion.clone(), cliente.correo.clone(), cliente.dni, libro1.titulo.clone(), libro1.autor.clone(), libro1.numero_de_paginas, Fecha{dia:1,mes:1,anio:2023});
        biblioteca.incrementar_cantidad_de_copias_a_disposición(libro1.titulo.clone(), libro1.autor.clone(), libro1.numero_de_paginas);
        biblioteca.incrementar_cantidad_de_copias_a_disposición(libro1.titulo.clone(), libro1.autor.clone(), libro1.numero_de_paginas);
        biblioteca.incrementar_cantidad_de_copias_a_disposición(libro1.titulo.clone(), libro1.autor.clone(), libro1.numero_de_paginas);
        // Verificar si un libro está prestado a un cliente
        assert_eq!(
            biblioteca.buscar_prestamo(cliente.dni, libro1.titulo.clone(), libro1.autor.clone(), libro1.numero_de_paginas),
            true
        );

        // Contar préstamos de un cliente
        assert_eq!(biblioteca.contar_prestamos_de_un_cliente(cliente.dni), Some(1));

        // Devolver un libro
        biblioteca.devolver_libro(cliente.dni, libro1.titulo.clone(), libro1.autor.clone(), libro1.numero_de_paginas);

        // Verificar la cantidad de préstamos a vencer en los próximos días
        assert_eq!(biblioteca.ver_prestamos_a_vencer_el_los_próximos_días(2023, 7, 6, 30), Some(0));

        // Verificar los préstamos vencidos
        assert_eq!(biblioteca.ver_los_prestamos_vencidos(2023, 7, 6), Some(vec![]));
    }
}
pub fn ejecutable(){
    print!("a");
}

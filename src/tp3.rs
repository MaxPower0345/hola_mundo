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
pub enum Genero{
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
    genero:Genero
}
pub struct Playlist{
    nombre:String,
    lista_de_canciones:Vec<Cancion> 
}
impl Genero{
    pub fn es_rock(&self)->bool{
        match self{
            Genero::Rock=>true,
            _=>false
        }
    }
    pub fn es_pop(&self)->bool{
        match self{
            Genero::Pop=>true,
            _=>false
        }
    }
    pub fn es_rap(&self)->bool{
        match self{
            Genero::Rap=>true,
            _=>false
        }
    }
    pub fn es_jazz(&self)->bool{
        match self{
            Genero::Jazz=>true,
            _=>false
        }
    }
    pub fn es_otro(&self)->bool{
        match self{
            Genero::Otros=>true,
            _=>false
        }
    }
    pub fn igual_genero(&self,otro:&Self)->bool{
        match self{
            Genero::Rock=>otro.es_rock(),
            Genero::Pop=>otro.es_pop(),
            Genero::Rap=>otro.es_rap(),
            Genero::Jazz=>otro.es_jazz(),
            Genero::Otros=>otro.es_otro()
        }
    }
}
impl Cancion{
    pub fn new(titulo:String,artista:String,genero:Genero)->Self{
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
    pub fn eliminar_ultima_cancion(&mut self){
        self.lista_de_canciones.pop();
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
    pub fn lista_de_canciones_de_determinado_genero(&self,un_genero:Genero)->Option<Vec<&Cancion>>{
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
pub fn ejecutable(){
    let mut p=Playlist::new("chilling".to_string());
    println!("{}",p.nombre);
    p.cambiar_nombre_playlist("chilleando".to_string());
    println!("{}",p.nombre);
    println!("{:#?}",p.lista_de_canciones);
    let cancion1=Cancion::new("Made in heaven".to_string(),"Led Zeppelin".to_string(),Genero::Rock);
    p.agregar_cancion(cancion1);
    let cancion2=Cancion::new("Night Dancer".to_string(),"VEGETA777".to_string(),Genero::Pop);
    p.agregar_cancion(cancion2);
    let cancion3=Cancion::new("Suavamente".to_string(),"VEGETA777".to_string(),Genero::Rock);
    p.agregar_cancion(cancion3);
    println!("{:#?}",p.lista_de_canciones);
    let cancion_aux=Cancion::new("Made in heaven".to_string(),"Led Zeppelin".to_string(),Genero::Rock);
    p.mover_cancion_pasando_referencia_de_cancion(&cancion_aux, 3);
    println!("{:#?}",p.lista_de_canciones);
}
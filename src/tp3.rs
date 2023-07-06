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
    año:u8
}

pub fn ejecutable(){
    let r=Rectángulo::new(3.0, 3.0);
    println!("{}",r.calcular_area());
    println!("{}",r.calcular_perimetro());
    println!("{}",r.es_cuadrado());
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
    pub fn calcular_perimetro(&self)->f32{
        self.lado1+self.lado2+self.lado3
    }
    pub fn calcular_area(&self)->f32{
        let s=self.calcular_perimetro()/2.0;
        let area=s*(s-self.lado1)*(s-self.lado2)*(s-self.lado3);
        area.sqrt()
    }
}
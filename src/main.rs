use std::io;
use std::str::FromStr;

fn main() {
    println!("Método Simplex General");
    
    let _cant_productos=ingreso_por_teclado("¿Cuantos productos entrarán en el cálculo del Método Simplex?: ");
    println!("El total de los productos ingresados son: {} ", _cant_productos);

    let _cant_beneficios=_cant_productos;
    println!("El total de beneficios para completar la función objetivo es: {} ", _cant_beneficios);

    let _cant_var_holgura=_cant_productos;
    println!("El total de las variables de holgura son: {}", _cant_var_holgura);

    let _cant_restricciones=ingreso_por_teclado("¿Cuantos procesos tendran los productos a fabricar?");
    println!("El total de restricciones es: {} ", _cant_restricciones);

    //productos+z+r+holgura El 2 es z+r
    let mut _total_columnas =_cant_productos+_cant_var_holgura+2;

    println!("Total de columnas es {}", _total_columnas);

    //creo un vector vacio
    let mut _vector_columnas = Vec::new();

    //asigno los espacios inicializadon en 0 (0.0 porque es f64) ojo acepta solo usize
    // vectores anidados
    _vector_columnas=vec![vec![0.0;_total_columnas as usize];_cant_restricciones as usize];

    //muestro cada linea con un salto de linea.
    for _c_r in 0 .. _cant_restricciones { //filas
        for _t_c in 0 .. _total_columnas { //columnas
            print!("Restriccion {:?}, {:?}; ", _c_r, _t_c); //muestro datos
        }
        println!(""); //salto de linea

       //print!("Restriccion {:?}; ", _c_r);
   }
        




    // const TOTAL_COLUMNAS=_total_columnas as const;

    //let _vector_datos = [0.0; ];



    //println!("{:?}", _vector_columnas);





    

}

fn ingreso_por_teclado(_mensaje: &str) -> isize {
    println!("{}",_mensaje);
    let mut _cantidad = String::new();
    io::stdin().read_line(&mut _cantidad).ok().expect("Error al ingresar los datos por el teclado!");
    let _cantidad=isize::from_str(&_cantidad.trim()).unwrap();
    _cantidad
}

// use std::arch::x86_64::_MM_GET_EXCEPTION_MASK;
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

    let _cant_restricciones=1+_cant_var_holgura;
    //let _cant_restricciones=ingreso_por_teclado("¿Cuantas restricciones tendran los productos a fabricar?");
    println!("El total de restricciones es: {} (1 es la funcion objetivo) ", _cant_restricciones);

    //productos+z+r+holgura El 2 es z+r
    let mut _total_columnas =_cant_productos+_cant_var_holgura+2;

    println!("Total de columnas es {}", _total_columnas);

    println!("");
    
    // calculo cuantos elementos tendrá la tabla
    let _elementos_tabla=(_cant_restricciones)*_total_columnas;
    let _elementos_tabla=_elementos_tabla as usize;

    //creo un vector vacio
    let mut _tabla_origen:Vec<f64> = Vec::new();

    // asigno cada valor a cada elemento
    println!("---Ingrese el valor del elemento: ");
    let mut _indice=0;
    let mut _valor_elemento_tabla=0.0;
    while _indice < (_elementos_tabla) {
        _valor_elemento_tabla=ingreso_por_teclado_para_la_tabla();
        _tabla_origen.push(_valor_elemento_tabla);
        println!("Indice: {}, Valor: {}", _indice, _valor_elemento_tabla);
        _indice = _indice + 1;
        println!("Ingrese el elemento de la tabla: ");
    }


    // mostrar la tabla 
    linea_de_la_tabla(_total_columnas);
    dibujar_encabezado_tabla(_cant_productos, _cant_var_holgura, _total_columnas);
    let mut _fila=0;
    println!("el total de elementos de la tabla es: {}", _tabla_origen.len());
    for indice in 0.._tabla_origen.len() {
        
        if indice==0 {
            print!("* {0: <7} * {1: <7}  *","Z",_tabla_origen[indice]);
        }
        if indice>0 {
            if indice % _total_columnas as usize == 0 {
                println!("");
                _fila += 1;
                concatenar_titulo_de_la_tabla(_fila, &'s');
                //let _titulo_fila = "{}{}";
                //print!("* {0: <7}  * {1: <7}  *","",_tabla_origen[indice]);
            }
            print!(" {0: <7}  *", _tabla_origen[indice]);
        }
        
    }
    println!("");
    linea_de_la_tabla(_total_columnas);

    // cargo toda la matriz de una!
    // _tabla_origen=vec![ingreso_por_teclado_para_la_tabla(); _elementos_tabla];
    //_tabla_origen=vec![0.0;_elementos_tabla as usize];

    // // for indice in 0.._elementos_tabla {
    //     let elemento=ingreso_por_teclado_para_la_tabla();
        
    //     _tabla_origen.push(elemento);
    // // }


    // ingreso_por_teclado_para_la_tabla();


}

fn ingreso_por_teclado_para_la_tabla() -> f64 {
    let mut _dato=String::new();
    io::stdin().read_line(&mut _dato).ok().expect("Error al ingresar los datos por el teclado!");
    let _dato=f64::from_str(&_dato.trim()).unwrap();
    _dato
}


fn ingreso_por_teclado(_mensaje: &str) -> isize {
    println!("{}",_mensaje);
    let mut _cantidad = String::new();
    io::stdin().read_line(&mut _cantidad).ok().expect("Error al ingresar los datos por el teclado!");
    let _cantidad=isize::from_str(&_cantidad.trim()).unwrap();
    _cantidad
}

fn variables_en_el_titulo(_cant_variables:isize, _x_o_s: &char) {
    
    for a in 1..=_cant_variables {
        // concatenar
        let _cardinal=a;
        let _letra=_x_o_s;
        // let _concatenar=format!("{}{}", _letra, _cardinal);
        // print!(" {0: <7} *", _concatenar);
        concatenar_titulo_de_la_tabla(_cardinal, _letra);
    }

}

fn concatenar_titulo_de_la_tabla(_nro_variable:isize, _letra:&char) {
    // concatenar
    let _concatenar=format!("{}{}", _letra, _nro_variable);
    print!("* {0: <7} *", _concatenar);
}

fn linea_de_la_tabla(_total_columnas:isize) {
    for i in 1..=((_total_columnas+1)*10)+3 {
        print!("*");
    }
    println!("");
}

fn dibujar_encabezado_tabla(_cant_productos:isize, _cant_var_holgura:isize, _total_columnas:isize) {
    // columna titulos encolumnados (eje y)
    print!("* {0: <7} *", "");
    print!(" {0: <7}  *", "Z");

    let _letra ='x';
    variables_en_el_titulo(_cant_productos, &_letra);

    let _letra='s';
    variables_en_el_titulo(_cant_var_holgura, &_letra);

    println!(" {0: <7}  *", "R");

    linea_de_la_tabla(_total_columnas);
}


    //asigno los espacios inicializadon en 0 (0.0 porque es f64) ojo acepta solo usize
    // vectores anidados



    //_vector_columnas=vec![vec![0.0;_total_columnas as usize];_cant_restricciones as usize];

    //cargo cada linea con un salto de linea.
    // for (i, fila) in _vector_columnas.iter().enumerate() {
    //     for(j, columna) in fila.iter().enumerate() {
            

    //         let mut _cantidad = String::new();
    //         io::stdin().read_line(&mut _cantidad).ok().expect("Error al ingresar los datos por el teclado!");
    //         let _cantidad = f64::from_str(&_cantidad.trim()).unwrap();
    //         // NO FUNCA ASI _vector_columnas=vec![fila][columna]=_cantidad;

    //         _vector_columnas.push(_cantidad as vec![fila][columna]);
    //         //print!("[f: {}][c: {}]= {}", i, j, columna);
    //         //print!("{columna: <10} ");
    //     }
    //     println!("");
    // }



    //muestro cada linea con un salto de linea.
    // for (i, fila) in _vector_columnas.iter().enumerate() {
    //     for(j, columna) in fila.iter().enumerate() {
    //         //print!("[f: {}][c: {}]= {}", i, j, columna);
    //         print!("{columna: <10} ");
    //     }
    //     println!("");
    // }
    
        
    //muestro cada linea con un salto de linea.
    // for _c_r in 0 .. _cant_restricciones { //filas
    //     for _t_c in 0 .. _total_columnas { //columnas
    //         print!("Restriccion {:?}, {:?}; ", _c_r, _t_c); //muestro datos
    //     }
    //     println!(""); //salto de linea

    // //print!("Restriccion {:?}; ", _c_r);
    // }



    // const TOTAL_COLUMNAS=_total_columnas as const;

    //let _vector_datos = [0.0; ];



    //println!("{:?}", _vector_columnas);





    // prueba con vectores (almacenar el mismo tipo de datos)

    // los vectores que se inicializan en cero (Vec::new) 
    // deben ser muteables para agregar datos posteriormente
    // let mut v: Vec<f32> = Vec::new();

    // agregar elementos a la ultima posicion del vector
    // v.push(1.0);
    // v.push(2.0);
    // v.push(3.0);
    // println!("La longitud del vector es {}", v.len());
    // println!("Eliminar el ultimo elemento ingresado al vector con pop");
    // v.pop();
    // println!("La longitud del vector es {}", v.len());

    //                      0   1   2   3   4 
    // let ve = vec![10, 12, 23, 34, 45];
    // let _cero:&i32=&ve[0];
    // let _uno:&i32=&ve[1];
    // let _dos:&i32=&ve[2];
    // let _tres:&i32=&ve[3];
    // let _cuatro:&i32=&ve[4];
    // println!("El tercer elemento de ve es: {}", _tres);

    // println!("Muestra la posición de ve.get(1) es {:?}", ve.get(1));
// Muestra la posición de ve.get(1) es Some(12)



    // match ve.get(0) {  // busca por el indice
    //     Some(0) => println!("El valor 10 es el elemento 0. Some(10)"),
        // Some(12) => println!("El valor 12 es el elemento 1. Some(12)"),
        // Some(23) => println!("El valor 23 es el elemento 2. Some(23)"),
        // Some(34) => println!("El valor 23 es el elemento 3. Some(34)"),
        // Some(45) => println!("El valor 23 es el elemento 4. Some(45)"),
        // None => println!("None"),
    // }

    // if ve.get(4) {
    //     println!("El cuarto elemento de ve");
    // }



    


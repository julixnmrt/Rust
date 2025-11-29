mod calcul;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let rayon_str = &args[1];
    let rayon = rayon_str.parse::<f64>().unwrap();

    let p = calcul::perimetre_cercle(rayon);
    println!("Le périmetre du cercle est égal a : {}",p);

    let s = calcul::surface_cercle(rayon);
    println!("La surface du cercle est égal a : {}",s);

    let s_sph = calcul::surface_sphere(rayon);
    println!("Lla surface de la sphere est égal a : {}",s_sph);

    let v_sph = calcul::volume_sphere(rayon);
    println!("Le volume de la spehere est égal a : {}",v_sph);
}
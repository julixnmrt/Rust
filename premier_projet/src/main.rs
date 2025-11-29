fn perimetre_cercle(rayon: f64)->f64{
    assert!(rayon >= 0.0);

    let perimetre = 2.0* std::f64::consts::PI * rayon;
    perimetre
}

fn surface_cercle(rayon: f64)->f64{
    assert!(rayon >= 0.0);

    let surface =  std::f64::consts::PI * rayon * rayon;
    surface
}

fn surface_sphere(rayon: f64)->f64{
    assert!(rayon >= 0.0);

    let surface_sph = 4.0* std::f64::consts::PI * rayon* rayon;
    surface_sph
}

fn volume_sphere(rayon: f64)->f64{
    assert!(rayon >= 0.0);

    let volume_sph = 4.0 / 3.0 * std::f64::consts::PI* f64::powf(rayon, 3.0);
    volume_sph
}
fn main() {
    let rayon = 5.0;

    let p = perimetre_cercle(rayon);
    println!("Le périmetre du cercle est égal a : {}",p);

    let s = surface_cercle(rayon);
    println!("La surface du cercle est égal a : {}",s);

    let s_sph = surface_sphere(rayon);
    println!("Lla surface de la sphere est égal a : {}",s_sph);

    let v_sph = volume_sphere(rayon);
    println!("Le volume de la spehere est égal a : {}",v_sph);
}

#[test]
fn test_volume_spehere(){
    let resultat = volume_sphere(5.0);
    let mut test = false;
    if resultat > 523.0 && resultat < 524.0{
        test = true
    }
    assert_eq!(test, true)
}
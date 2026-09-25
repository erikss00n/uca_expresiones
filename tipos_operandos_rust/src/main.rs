fn calcular_oleadas(derrotados: i32, por_oleada: i32) -> i32 {
    derrotados / por_oleada
}
fn calcular_dano_critico(dano_base: i32, multiplicador: f64) -> f64 {
    dano_base as f64 * multiplicador
    
}
fn calcular_dano_promedio(dano_base: i32, por_oleada: i32) -> f64 {
    dano_base as f64 / por_oleada as f64
}
fn main() {
    let dano_critico = calcular_dano_critico(50, 1.5);
    println!("Daño crítico: {}", dano_critico);
    let oleadas = calcular_oleadas(7, 2);
    println!("Oleadas: {}", oleadas);
    let dano_promedio = calcular_dano_promedio(50, 2);
    println!("Daño promedio: {}", dano_promedio);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_calcular_dano_critico() {
        assert_eq!(calcular_dano_critico(50, 1.5), 75.0);
    }
    #[test]
    fn test_calcular_oleadas() {
        assert_eq!(calcular_oleadas(7, 2), 3);
    }
        #[test]
    fn test_calcular_dano_promedio() {
        assert_eq!(calcular_dano_promedio(50, 2), 25.0);
    }
}
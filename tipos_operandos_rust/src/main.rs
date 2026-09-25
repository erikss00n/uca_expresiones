fn calcular_oleadas(derrotados: i32, por_oleada: i32) -> i32 {
    derrotados / por_oleada
}

fn main() {
    let oleadas = calcular_oleadas(7, 2);
    println!("Oleadas: {}", oleadas);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calcular_oleadas() {
        assert_eq!(calcular_oleadas(7, 2), 3);
    }
}
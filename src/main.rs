fn main() {
    let disciplina = "Logica de Programação";
    let sala = "Laboratorio 02";
    let aulas_lancadas = 10;
    let faltas = 2;

    println!("Bem Vindo ao Bússola IF");
    println!("Disciplina: {}", disciplina);
    println!("Sala: {}", sala);

    println!("Aulas Lançadas: {}", aulas_lancadas);
    println!("Faltas: {}", faltas);

    if faltas > aulas_lancadas {
        println!("Erro: Faltas maiores que aulas lançadas");
    } else if aulas_lancadas == 0 {
        println!("Sem aulas lançadas");
    } else {
        let presencas = aulas_lancadas - faltas;
        println!("Presença: {}", presencas);
        let frequencias = calcular_frequencias(presencas as f64, aulas_lancadas as f64);
        println!("Frequencia: {:.1}%", frequencias);
    }
}

fn calcular_frequencias(presencas: f64, aulas: f64) -> f64 {
    (presencas / aulas) * 100.0
}

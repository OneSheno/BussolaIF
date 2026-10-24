struct Disciplina {
    nome: String,
    sala: String,
    aulas_lancadas: u32,
    faltas: u32,
}

fn main() {
    println!("Bem Vindo ao Bússola IF");

    let disciplina = Disciplina {
        nome: String::from("Logica de Programação"),
        sala: String::from("sala 2"),
        aulas_lancadas: 8,
        faltas: 2,
    };

    let disciplina2 = Disciplina {
        nome: String::from("Matematica"),
        sala: String::from("sala 5"),
        aulas_lancadas: 12,
        faltas: 3,
    };

    mostrar_disciplina(&disciplina);
    mostrar_disciplina(&disciplina2);
}
fn mostrar_disciplina(disciplina: &Disciplina) {
    println!();

    println!("Disciplina: {}", disciplina.nome);
    println!("Sala: {}", disciplina.sala);
    println!("Aulas Lançadas: {}", disciplina.aulas_lancadas);
    println!("Faltas: {}", disciplina.faltas);
    if disciplina.faltas > disciplina.aulas_lancadas {
        println!("Erro: Faltas maiores que aulas lançadas");
    } else if disciplina.aulas_lancadas == 0 {
        println!("Sem aulas lançadas");
    } else {
        let presencas = disciplina.aulas_lancadas - disciplina.faltas;
        println!("Presença: {}", presencas);
        let frequencias = calcular_frequencias(presencas as f64, disciplina.aulas_lancadas as f64);
        println!("Frequencia: {:.1}%", frequencias);
    }
}

fn calcular_frequencias(presencas: f64, aulas: f64) -> f64 {
    (presencas / aulas) * 100.0
}

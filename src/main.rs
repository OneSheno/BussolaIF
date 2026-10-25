struct Disciplina {
    nome: String,
    sala: String,
    aulas_lancadas: u32,
    faltas: u32,
}

fn main() {
    println!("Bem Vindo ao Bússola IF");

    let logica = Disciplina {
        nome: String::from("Logica de Programação"),
        sala: String::from("sala 2"),
        aulas_lancadas: 0,
        faltas: 0,
    };

    let matematica = Disciplina {
        nome: String::from("Matematica"),
        sala: String::from("sala 5"),
        aulas_lancadas: 12,
        faltas: 3,
    };

    let ingles = Disciplina {
        nome: String::from("Ingles"),
        sala: String::from("sala 2"),
        aulas_lancadas: 4,
        faltas: 3,
    };

    let disciplinas = vec![logica, matematica, ingles];
    for disciplina in &disciplinas {
        mostrar_disciplina(disciplina);
    }
    println!("Disciplinas cadastradas: {}", disciplinas.len());
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
        match calcular_frequencias(disciplina) {
            Some(frequencia) => {
                if frequencia < 75.0 {
                    println!("Frequencia abaixo do limite")
                } else {
                    println!("Frequencia dentro do limite")
                }
                println!("Frequencia: {:.1}%", frequencia);
            }
            None => {
                println!("Não foi possível calcular a frequência.");
            }
        }
    }
}

fn calcular_frequencias(disciplina: &Disciplina) -> Option<f64> {
    if disciplina.aulas_lancadas == 0 || disciplina.faltas > disciplina.aulas_lancadas {
        return None;
    }

    let presencas = disciplina.aulas_lancadas - disciplina.faltas;
    let frequencia = (presencas as f64 / disciplina.aulas_lancadas as f64) * 100.0;

    Some(frequencia)
}

//use std::ops::{Div};

#[derive(Debug)]
pub struct Prestacao {
	pub parcela: f64,
	pub juros: f64,
	pub amortizacao: f64,
	pub saldo_devedor: f64
}

impl Prestacao {
	pub fn new(saldo_devedor: f64, qtd_meses: u32, juros: f64) -> Prestacao {
		let juros_mensais = juros / 12.0;
		let amortiz = amortizacao(saldo_devedor, qtd_meses);
		let total = saldo_devedor * juros_mensais + amortiz;

		Prestacao {
			parcela: total,
			juros: juros_mensais,
			amortizacao: amortiz,
			saldo_devedor: saldo_devedor
		}
	}
}

/*
pub fn porcento<T: Div>(x: T) -> T {
	x / 100.0
}
*/
pub fn porcento(x: f64) -> f64 {
	x / 100.0
}

pub fn amortizacao(saldo_devedor: f64, qtd_meses: u32) -> f64 {
	saldo_devedor / (qtd_meses as f64)
}

pub fn prestacoes(saldo_devedor: f64, qtd_meses: u32, juros: f64) -> Vec<Prestacao> {
	let mut saldo = saldo_devedor;
	let mut lista_prestacoes = Vec::new();
	for i in 0..qtd_meses {
		lista_prestacoes.push(Prestacao::new(saldo, qtd_meses - i, juros));
		saldo = saldo - amortizacao(saldo, qtd_meses - i);
	}
	lista_prestacoes
}

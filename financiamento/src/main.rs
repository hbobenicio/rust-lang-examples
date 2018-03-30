extern crate financiamento;

fn main() {
    let saldo_devedor = 220_000.0;
    let qtd_meses = 12;
    let juros_bb = financiamento::core::porcento(9.0);
    let juros_caixa = financiamento::core::porcento(7.85);

    let prestacoes_bb = financiamento::core::prestacoes(saldo_devedor, qtd_meses, juros_bb);
    let prestacoes_caixa = financiamento::core::prestacoes(saldo_devedor, qtd_meses, juros_caixa);

    for p in &prestacoes_bb {
        println!("{:?}", p);
    }
    for p in &prestacoes_caixa {
        println!("{:?}", p);
    }
}

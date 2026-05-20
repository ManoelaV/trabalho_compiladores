fn main ()
{
let total : f32;
let ativo : bool;
total = read_float();
ativo = true;
if total > 0.0 {
println! total;
};
while total < 10.0 {
total = total + 1.0;
println! total;
};
imprime_valor(total);
return total;
}

fn imprime_valor(valor : f32) -> void {
println! valor;
return valor;
}

fn main ()
{
let a : f32;
let b : f32;
let cond : bool;
a = 2.0;
b = 3.0;
cond = (a < b) && (a == 2.0 || false);
soma(a, b);
if cond {
println! a + b;
};
return a;
}

fn soma(x : f32, y : f32) -> f32 {
let r : f32;
r = x + y * 2.0;
println! r;
return r;
}

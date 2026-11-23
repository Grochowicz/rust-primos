use bitvec::prelude::*;




pub fn contar_primos(n: usize) -> usize {
    if n < 2 {
        return 0;
    }    
    let mut p = bitvec![1; n + 1];
    p.set(0, false);
    p.set(1, false);
    for mut i in 2..n {
        if p[i]{
            let mut j = i*i;
            while j <= n{
                p.set(j,false);
                j += i;
            }
        }
    }
    return p.iter().filter(|bit| **bit).count()
}


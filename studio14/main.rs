fn main() {
    let mut v: Vec<bool> = (0..=99).map(|x| x==2 || (x%2==1 && x>2)).collect();

    
    for i in 0..=99{
        if v[i]{
            let iter = i*i..100;
            iter.clone().step_by(2*i).for_each(|item| {
                if item/i > 1 && item%i==0{
                    v[item]=false;
                }
            });
        }
    }
    for i in 0..=99{
        if v[i]{
            print!("{:2} ", i);
        }else{
            print!("   ");
        }

        if i%10 == 9{
            println!();
        }
        
    }
}

use std::collections::{BTreeMap, BTreeSet, HashSet};

type Group = BTreeMap<u32, BTreeSet<u16>>;

fn merge_group(sub_group: Group) -> Group{
    let mut result_group:Group = BTreeMap::new(); 
    for (k1, val1) in &sub_group{
        for (k2, val2) in &sub_group{
            let front_bits1:u16 = (k1 & ((1 << 16) - 1)) as u16;
            let back_bits1:u16 = (k1 >> 16) as u16;
            let front_bits2:u16 = (k2 & ((1 << 16) - 1)) as u16;
            let back_bits2:u16 = (k2 >> 16) as u16;
            if back_bits1 == back_bits2{
                let diff_front_bits = front_bits1 ^ front_bits2;
                let already_diff_ignored_bits = diff_front_bits & (!back_bits1);
                let mut diff_count = 0;
                let mut bit_iterator = already_diff_ignored_bits;
                for _ in 0..16{
                    diff_count += bit_iterator & 1;
                    bit_iterator = bit_iterator >> 1; 
                }
                if diff_count == 1{
                    let back_bits_res: u32 = ((already_diff_ignored_bits | back_bits1) as u32) << 16; 
                    let front_bits_res: u32 = (front_bits1 | front_bits2) as u32;
                    let key = back_bits_res | front_bits_res;
                    let merged = val1 | val2;
                    result_group.insert(key, merged);       
                }
            }
        }
    }
    result_group
}

fn main() {
    let mut groups:Group =  BTreeMap::new();
    let min_terms = BTreeSet::from([2, 3, 7, 9, 11, 13]);
    let dont_care_terms = BTreeSet::from([1, 10, 15]);
    let var_count = 4;
    let max_term_symbol = ('A' as u8 + (var_count - 1)) as char;
    let mut variables = Vec::new(); 
    let mut symbol_iter = max_term_symbol;
    while symbol_iter != ('A' as u8 - 1) as char{
        variables.push(symbol_iter);
        symbol_iter = (symbol_iter as u8 - 1) as char;
    }

    for i in min_terms{
        groups.insert(i as u32,BTreeSet::from([i]));
    }
    for i in &dont_care_terms{
        groups.insert(*i as u32,BTreeSet::from([*i]));
    }
    let mut prev_groups = groups.clone();
    while groups.len() != 0{
        prev_groups = groups;
        groups = merge_group(prev_groups.clone());
    }
    let mut duplicated_term_counts:BTreeMap<u16, usize> = BTreeMap::new();
    for (_, sub_set) in &prev_groups{
        for i in sub_set{
            match duplicated_term_counts.get_mut(&i){
                Some(count) =>{*count+=1;},
                None=>{duplicated_term_counts.insert(*i, 1);}
            }
        }
    }

    let mut implicants = Vec::new();
    for (i,cnt) in duplicated_term_counts{
        if cnt == 1{
            if !dont_care_terms.contains(&i){
                implicants.push(i);
            }
        }
    }    
    let mut terms_for_sum_of_product = HashSet::new();
    for (key, subset) in &prev_groups{
        for i in &implicants{
            if subset.contains(&i){
                terms_for_sum_of_product.insert(*key);
            }
        }
    }
    let mut expr = String::new();
    for encoded_term in terms_for_sum_of_product{
        let mut front_bit_iter:u16 = (encoded_term & ((1 << 16) - 1)) as u16;
        let mut back_bit_iter:u16 = (encoded_term >> 16) as u16;
        if expr.is_empty(){
            expr.push_str("F = ");
        }else{
            expr.push_str("+ ");
        }
        
        let mut term_tokens:Vec<String> = Vec::new();

        for i in 0..var_count{
            if back_bit_iter & 1 == 0{
                let mut sub_token = String::new();
                sub_token.push(variables[i as usize]);
                if front_bit_iter &1 == 0{
                    sub_token.push('\'');
                }
                term_tokens.push(sub_token);
            }
            front_bit_iter = front_bit_iter >> 1;
            back_bit_iter = back_bit_iter >> 1;
        }


        for i in 0..term_tokens.len()-1{
            if term_tokens[i].chars().nth(0).unwrap() as u8 > term_tokens[i + 1].chars().nth(0).unwrap() as u8{
                let swp = term_tokens[i].clone();
                term_tokens[i] = term_tokens[i+1].clone();
                term_tokens[i+1] = swp.clone();
            }
        } 
        expr = term_tokens.iter().fold(expr, |acc,x|acc + x);
    }
    println!("{expr}");

}

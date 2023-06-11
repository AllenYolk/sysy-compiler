use super::exp_solve::ExpSolve;
use super::koopa_generate::KoopaTextGenerate;
use super::named_symbol::NamedSymbolCounter;
use super::scopes::*;
use super::temp_symbol::TempSymbolManager;
use crate::ast_generate::ast::*;
use crate::tools::*;

/// Generate the type of an array used in `alloc` instructions.
pub fn generate_allocate_dims(dims: &[i32], i: usize) -> String {
    let l = dims.len();
    if i == l - 1 {
        format!("[i32, {}]", dims[i])
    } else {
        format!("[{}, {}]", generate_allocate_dims(dims, i + 1), dims[i])
    }
}

/// Return a pointer to an array element, and generate the corresponding code.
///
/// Use the style of `KoopaTextGenerate`.
/// The indices are expressions, and they are solved before getting the pointer.
pub fn get_pointer_to_element_exp_idx(
    lines: &mut String,
    array: &str,
    idx: &Vec<Exp>,
    scopes: &mut Scopes,
    tsm: &mut TempSymbolManager,
    nsc: &mut NamedSymbolCounter,
) -> Result<String, ()> {
    let mut old_handler = String::from(array);
    let mut new_handler: String = String::from(array);
    for (j, exp) in idx.iter().enumerate() {
        if (j == 0) && scopes.has_cur_func_param(array) {
            let mut pre = String::new();
            let i = exp.generate(&mut pre, scopes, tsm, nsc)?;
            append_line(lines, &pre);
            let intermediate_handler = tsm.new_temp_symbol();
            new_handler = nsc.inc_and_get_named_symbol("%array_ptr")?;
            append_line(lines, &format!("  {} = load {}", intermediate_handler, old_handler));
            append_line(
                lines,
                &format!("  {} = getptr {}, {}", new_handler, intermediate_handler, i),
            );
            old_handler = new_handler.clone();
        } else {
            let mut pre = String::new();
            let i = exp.generate(&mut pre, scopes, tsm, nsc)?;
            append_line(lines, &pre);

            new_handler = nsc.inc_and_get_named_symbol("%array_ptr")?;
            append_line(
                lines,
                &format!("  {} = getelemptr {}, {}", new_handler, old_handler, i),
            );
            old_handler = new_handler.clone();
        }
    }

    Ok(new_handler)
}

/// Return a pointer to an array element, and generate the corresponding code.
///
/// Use the style of `KoopaTextGenerate`.
/// The indices are integers, and they are directly used to get the pointer.
pub fn get_pointer_to_element_int_idx(
    lines: &mut String,
    array: &str,
    idx: Vec<usize>,
    scopes: &mut Scopes,
    nsc: &mut NamedSymbolCounter,
) -> Result<String, ()> {
    let mut old_handler = String::from(array);
    let mut new_handler: String = String::from(array);
    for (j, i) in idx.iter().enumerate() {
        if (j == 0) && scopes.has_cur_func_param(array) {
            let intermediate_handler = nsc.inc_and_get_named_symbol("%array_ptr")?;
            new_handler = nsc.inc_and_get_named_symbol("%array_ptr")?;
            append_line(lines, &format!("  {} = load {}", intermediate_handler, old_handler));
            append_line(
                lines,
                &format!("  {} = getptr {}, {}", new_handler, intermediate_handler, i),
            );
            old_handler = new_handler.clone();
        } else {
            new_handler = nsc.inc_and_get_named_symbol("%array_ptr")?;
            append_line(
                lines,
                &format!("  {} = getelemptr {}, {}", new_handler, old_handler, i),
            );
            old_handler = new_handler.clone();
        }
    }

    Ok(new_handler)
}

fn walk_const_init_val(
    // information
    init: &ConstInitVal,
    full_initializer: &mut Vec<String>,
    backward_prod: &[usize],
    // states
    level: usize,
    idx: usize,
    scopes: &mut Scopes,
) -> Result<(), ()> {
    match init {
        ConstInitVal::Exp(exp) => {
            let j = exp.solve(scopes)?;
            full_initializer[idx] = j.to_string();
        }
        ConstInitVal::Array(arr) => {
            let mut current_idx = idx;
            for sub_init in arr.iter() {
                walk_const_init_val(
                    sub_init,
                    full_initializer,
                    backward_prod,
                    level + 1,
                    current_idx,
                    scopes,
                )?;
                match sub_init {
                    ConstInitVal::Exp(_) => {
                        current_idx += 1;
                    }
                    ConstInitVal::Array(_) => {
                        let l = backward_prod.len();
                        for i in level..l {
                            if current_idx % backward_prod[i] == 0 {
                                current_idx += backward_prod[i];
                                break;
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(())
}

pub fn parse_const_array_initializer(
    init: &ConstInitVal,
    dims: &[i32],
    scopes: &mut Scopes,
) -> Result<Vec<String>, ()> {
    let n_total = dims.iter().product::<i32>();
    let mut full_initializer = vec![String::from("0"); n_total as usize];
    let mut backward_prod = vec![1usize; dims.len()];
    for i in 2..=dims.len() {
        let j = dims.len() - i;
        backward_prod[j] = backward_prod[j + 1] * (dims[j + 1] as usize);
    }

    walk_const_init_val(init, &mut full_initializer, &backward_prod, 0, 0, scopes)?;

    Ok(full_initializer)
}

fn walk_var_init_val(
    pre_lines: &mut String,
    init: &InitVal,
    full_initializer: &mut Vec<String>,
    backward_prod: &[usize],
    level: usize,
    idx: usize,
    scopes: &mut Scopes,
    tsm: &mut TempSymbolManager,
    nsc: &mut NamedSymbolCounter,
) -> Result<(), ()> {
    match init {
        InitVal::Exp(exp) => {
            let handle = exp.solve(scopes);
            match handle {
                Ok(h) => {
                    full_initializer[idx] = h.to_string();
                }
                Err(_) => {
                    let handle = exp.generate(pre_lines, scopes, tsm, nsc)?;
                    full_initializer[idx] = handle;
                }
            }
        }
        InitVal::Array(arr) => {
            let mut current_idx = idx;
            for sub_init in arr.iter() {
                walk_var_init_val(
                    pre_lines,
                    sub_init,
                    full_initializer,
                    backward_prod,
                    level + 1,
                    current_idx,
                    scopes,
                    tsm,
                    nsc,
                )?;
                match sub_init {
                    InitVal::Exp(_) => {
                        current_idx += 1;
                    }
                    InitVal::Array(_) => {
                        let l = backward_prod.len();
                        for i in level..l {
                            if current_idx % backward_prod[i] == 0 {
                                current_idx += backward_prod[i];
                                break;
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(())
}

pub fn parse_var_array_initializer(
    pre_lines: &mut String,
    init: &InitVal,
    dims: &[i32],
    scopes: &mut Scopes,
    tsm: &mut TempSymbolManager,
    nsc: &mut NamedSymbolCounter,
) -> Result<Vec<String>, ()> {
    let n_total = dims.iter().product::<i32>();
    let mut full_initializer = vec![String::from("0"); n_total as usize];
    let mut backward_prod = vec![1usize; dims.len()];
    for i in 2..=dims.len() {
        let j = dims.len() - i;
        backward_prod[j] = backward_prod[j + 1] * (dims[j + 1] as usize);
    }

    walk_var_init_val(
        pre_lines,
        init,
        &mut full_initializer,
        &backward_prod,
        0,
        0,
        scopes,
        tsm,
        nsc,
    )?;

    Ok(full_initializer)
}

fn walk_full_initializer(
    full_init: &Vec<String>,
    dims: &[i32],
    backward_prod: &[usize],
    now_level: usize,
    now_idx: &mut Vec<usize>,
) -> String {
    if now_level == dims.len() {
        return full_init[now_idx
            .iter()
            .zip(backward_prod.iter())
            .map(|(a, b)| (*a) * (*b as usize))
            .sum::<usize>()]
        .clone();
    }

    let n = dims[now_level] as usize;
    let mut ans = String::from("{");
    for i in 0..n {
        now_idx[now_level] = i;
        ans.push_str(&walk_full_initializer(
            full_init,
            dims,
            backward_prod,
            now_level + 1,
            now_idx,
        ));
        if i != n - 1 {
            ans.push_str(", ");
        }
    }
    ans.push_str("}");

    ans
}

pub fn full_initializer_to_global_aggregate(full_init: &Vec<String>, dims: &[i32]) -> String {
    let mut now_idx = vec![0usize; dims.len()];
    let mut backward_prod = vec![1usize; dims.len()];
    for i in 2..=dims.len() {
        let j = dims.len() - i;
        backward_prod[j] = backward_prod[j + 1] * (dims[j + 1] as usize);
    }
    walk_full_initializer(full_init, dims, &backward_prod, 0, &mut now_idx)
}

pub fn full_initializer_to_local_lines(
    array: &str,
    full_init: &Vec<String>,
    dims: &[i32],
    scopes: &mut Scopes,
    nsc: &mut NamedSymbolCounter,
) -> Result<String, ()> {
    let l = full_init.len();
    let mut backward_prod = vec![1usize; dims.len()];
    for i in 2..=dims.len() {
        let j = dims.len() - i;
        backward_prod[j] = backward_prod[j + 1] * (dims[j + 1] as usize);
    }

    let mut lines = String::new();
    for i in 0..l {
        let from_handle = &full_init[i];

        let ll = dims.len();
        let mut idx = vec![0; ll];
        let mut ii = i;
        for j in 0..ll {
            idx[j] = ii / backward_prod[j];
            ii = ii % backward_prod[j];
        }

        let to_handle = get_pointer_to_element_int_idx(&mut lines, array, idx, scopes, nsc)?;
        append_line(
            &mut lines,
            &format!("  store {}, {}", from_handle, to_handle),
        );
    }

    Ok(lines)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_allocate_dims_test() {
        assert_eq!(generate_allocate_dims(&[1, 2, 3], 0), "[[[i32, 3], 2], 1]");
    }
}

use super::*;
use anyhow::Result;
use std::collections::HashMap;
use zerocopy::IntoBytes;

pub fn dump_tools(p: &Pdb) -> Result<()> {
    let modules: Vec<_> = p.modules()?.iter().collect();

    let mut num_unknown_tool: u32 = 0;
    let mut tools_map: HashMap<String, u32> = HashMap::new();

    for (module_index, module) in modules.iter().enumerate() {
        let Some(_module_stream) = module.stream() else {
            continue;
        };

        let module_syms = match p.read_module_symbols(&module) {
            Ok(m) => m,
            Err(e) => {
                error!("failed to read module symbols: {e:?}");
                continue;
            }
        };

        if module_syms.is_empty() {
            continue;
        }

        let mut tool_name: Option<String> = None;

        for sym in SymIter::for_module_syms(module_syms.as_bytes()) {
            match sym.kind {
                SymKind::S_COMPILE | SymKind::S_COMPILE2 | SymKind::S_COMPILE3 => {
                    match sym.parse() {
                        Ok(SymData::Compile3(compile)) => {
                            tool_name = Some(compile.name.to_string());
                        }

                        Ok(_) => {
                            error!("wrong kind of symbol data");
                        }

                        Err(e) => {
                            error!("failed to decode symbol record: {e:?}");
                            continue;
                        }
                    }

                    break;
                }

                _ => {}
            }
        }

        if let Some(tool_name) = tool_name {
            if let Some(existing_tool) = tools_map.get_mut(tool_name.as_str()) {
                *existing_tool += 1;
            } else {
                tools_map.insert(tool_name.to_string(), 1);
            }

            println!(
                "{module_index:6} | {tool_name} | {module_name:80} | {obj_file}",
                module_name = module.module_name().to_string(),
                obj_file = module.obj_file().to_string(),
            );
        } else {
            println!("{}: no tool specified", module.obj_file());
            num_unknown_tool += 1;
        }
    }

    println!();
    println!("Summary");
    println!("-------");
    println!();

    if num_unknown_tool != 0 {
        println!("Number of modules with no tool info: {num_unknown_tool:10}");
        println!();
    }

    let mut flat_tools_map: Vec<_> = tools_map.iter().collect();
    flat_tools_map.sort_unstable_by_key(|&(name, _count)| name);

    if modules.is_empty() {
        println!("Did not find any modules. PDB may be stripped.");
    } else {
        for &(ref name, &count) in flat_tools_map.iter() {
            let pct_total = count as f32 * 100.0 / modules.len() as f32;
            println!("{count:10}  {pct_total:5.1} % : {name}");
        }
    }

    Ok(())
}

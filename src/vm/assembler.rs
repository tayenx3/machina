use std::collections::HashMap;
use std::path::Path;
use super::registers::*;
use super::isa::*;

pub fn assemble(source: &str) -> Result<Vec<u8>, String> {
    let mut result = Vec::new();
    let lines = source.lines()
        .enumerate()
        .collect::<Vec<_>>();

    let mut labels = HashMap::new();
    let mut current_addr = 0;
    for (_, line) in &lines {
        let line = line.split_terminator(";").collect::<Vec<_>>();
        if line.is_empty() { continue }
        let parts = line[0].split_whitespace()
            .collect::<Vec<_>>();
        if parts.is_empty() { continue }

        match parts[0] {
            other if other.ends_with(":") => {
                let label_name = other.strip_suffix(":").unwrap();

                labels.insert(label_name, current_addr);

                if parts.len() != 1 {
                    current_addr += 6;
                }
            },
            _ => current_addr += 6,
        }
    }

    for (idx, line) in lines {
        let line = line.split_terminator(";").collect::<Vec<_>>();
        if line.is_empty() { continue }
        let parts = line[0].split_whitespace()
            .collect::<Vec<_>>();
        if parts.is_empty() { continue }

        match parts[0] {
            "nop" | "NOP" => {
                if parts.len() != 1 {
                    return Err(format!(
                        "error on line {}: invalid argument count for NOP instruction",
                        idx + 1
                    ));
                }
                result.extend([NOP, 0, 0, 0, 0, 0])
            },
            "hlt" | "HLT" => {
                if parts.len() != 1 {
                    return Err(format!(
                        "error on line {}: invalid argument count for HLT instruction",
                        idx + 1
                    ));
                }
                result.extend([HLT, 0, 0, 0, 0, 0])
            },
            "ldi" | "LDI" => {
                if parts.len() != 3 {
                    return Err(format!(
                        "error on line {}: invalid argument count for LDI instruction",
                        idx + 1
                    ));
                }
                let dest = parse_register(parts[1])
                    .map_err(|err| format!("error on line {}: {err}", idx+1))?;

                let imm = if parts[2].starts_with("+") {
                    parse_immediate(
                        parts[2].strip_prefix("+")
                            .unwrap()
                    ).map_err(|err| format!("error on line {}: {err}", idx+1))?
                } else if parts[2].starts_with("-") {
                    -(parse_immediate(
                        parts[2].strip_prefix("-")
                            .unwrap()
                    ).map_err(
                        |err|
                        format!("error on line {}: {err}", idx+1)
                    )? as i32) as u32
                } else {
                    parse_immediate(parts[2])
                        .map_err(|err| format!("error on line {}: {err}", idx+1))?
                };
                result.extend([
                    LDI,
                    dest | (((imm & 0xF) as u8) << 4),
                    ((imm >> 4) & 0xFF) as u8,
                    ((imm >> 12) & 0xFF) as u8,
                    ((imm >> 20) & 0xFF) as u8,
                    ((imm >> 28) & 0xF) as u8
                ]);
            },
            "add" | "ADD" => {
                if parts.len() != 4 {
                    return Err(format!(
                        "error on line {}: invalid argument count for ADD instruction",
                        idx + 1
                    ));
                }
                let dest = parse_register(parts[1])
                    .map_err(|err| format!("error on line {}: {err}", idx+1))?;
                let src1 = parse_register(parts[2])
                    .map_err(|err| format!("error on line {}: {err}", idx+1))?;
                let src2 = parse_register(parts[3])
                    .map_err(|err| format!("error on line {}: {err}", idx+1))?;
                
                result.extend([
                    ADD,
                    dest | (src1 << 4),
                    src2,
                    0, 0, 0
                ]);
            },
            "sub" | "SUB" => {
                if parts.len() != 4 {
                    return Err(format!(
                        "error on line {}: invalid argument count for SUB instruction",
                        idx + 1
                    ));
                }
                let dest = parse_register(parts[1])
                    .map_err(|err| format!("error on line {}: {err}", idx+1))?;
                let src1 = parse_register(parts[2])
                    .map_err(|err| format!("error on line {}: {err}", idx+1))?;
                let src2 = parse_register(parts[3])
                    .map_err(|err| format!("error on line {}: {err}", idx+1))?;
                
                result.extend([
                    SUB,
                    dest | (src1 << 4),
                    src2,
                    0, 0, 0
                ]);
            },
            "bor" | "BOR" => {
                if parts.len() != 4 {
                    return Err(format!(
                        "error on line {}: invalid argument count for BOR instruction",
                        idx + 1
                    ));
                }
                let dest = parse_register(parts[1])
                    .map_err(|err| format!("error on line {}: {err}", idx+1))?;
                let src1 = parse_register(parts[2])
                    .map_err(|err| format!("error on line {}: {err}", idx+1))?;
                let src2 = parse_register(parts[3])
                    .map_err(|err| format!("error on line {}: {err}", idx+1))?;
                
                result.extend([
                    BOR,
                    dest | (src1 << 4),
                    src2,
                    0, 0, 0
                ]);
            },
            "band" | "BAND" => {
                if parts.len() != 4 {
                    return Err(format!(
                        "error on line {}: invalid argument count for BAND instruction",
                        idx + 1
                    ));
                }
                let dest = parse_register(parts[1])
                    .map_err(|err| format!("error on line {}: {err}", idx+1))?;
                let src1 = parse_register(parts[2])
                    .map_err(|err| format!("error on line {}: {err}", idx+1))?;
                let src2 = parse_register(parts[3])
                    .map_err(|err| format!("error on line {}: {err}", idx+1))?;
                
                result.extend([
                    BAND,
                    dest | (src1 << 4),
                    src2,
                    0, 0, 0
                ]);
            },
            "bxor" | "BXOR" => {
                if parts.len() != 4 {
                    return Err(format!(
                        "error on line {}: invalid argument count for BXOR instruction",
                        idx + 1
                    ));
                }
                let dest = parse_register(parts[1])
                    .map_err(|err| format!("error on line {}: {err}", idx+1))?;
                let src1 = parse_register(parts[2])
                    .map_err(|err| format!("error on line {}: {err}", idx+1))?;
                let src2 = parse_register(parts[3])
                    .map_err(|err| format!("error on line {}: {err}", idx+1))?;
                
                result.extend([
                    BXOR,
                    dest | (src1 << 4),
                    src2,
                    0, 0, 0
                ]);
            },
            "bnot" | "BNOT" => {
                if parts.len() != 3 {
                    return Err(format!(
                        "error on line {}: invalid argument count for BNOY instruction",
                        idx + 1
                    ));
                }
                let dest = parse_register(parts[1])
                    .map_err(|err| format!("error on line {}: {err}", idx+1))?;
                let src1 = parse_register(parts[2])
                    .map_err(|err| format!("error on line {}: {err}", idx+1))?;
                
                result.extend([
                    BNOT,
                    dest | (src1 << 4),
                    0, 0, 0, 0
                ]);
            },
            "lor" | "LOR" => {
                if parts.len() != 4 {
                    return Err(format!(
                        "error on line {}: invalid argument count for LOR instruction",
                        idx + 1
                    ));
                }
                let dest = parse_register(parts[1])
                    .map_err(|err| format!("error on line {}: {err}", idx+1))?;
                let src1 = parse_register(parts[2])
                    .map_err(|err| format!("error on line {}: {err}", idx+1))?;
                let src2 = parse_register(parts[3])
                    .map_err(|err| format!("error on line {}: {err}", idx+1))?;
                
                result.extend([
                    LOR,
                    dest | (src1 << 4),
                    src2,
                    0, 0, 0
                ]);
            },
            "land" | "LAND" => {
                if parts.len() != 4 {
                    return Err(format!(
                        "error on line {}: invalid argument count for LAND instruction",
                        idx + 1
                    ));
                }
                let dest = parse_register(parts[1])
                    .map_err(|err| format!("error on line {}: {err}", idx+1))?;
                let src1 = parse_register(parts[2])
                    .map_err(|err| format!("error on line {}: {err}", idx+1))?;
                let src2 = parse_register(parts[3])
                    .map_err(|err| format!("error on line {}: {err}", idx+1))?;
                
                result.extend([
                    LAND,
                    dest | (src1 << 4),
                    src2,
                    0, 0, 0
                ]);
            },
            "lxor" | "LXOR" => {
                if parts.len() != 4 {
                    return Err(format!(
                        "error on line {}: invalid argument count for LXOR instruction",
                        idx + 1
                    ));
                }
                let dest = parse_register(parts[1])
                    .map_err(|err| format!("error on line {}: {err}", idx+1))?;
                let src1 = parse_register(parts[2])
                    .map_err(|err| format!("error on line {}: {err}", idx+1))?;
                let src2 = parse_register(parts[3])
                    .map_err(|err| format!("error on line {}: {err}", idx+1))?;
                
                result.extend([
                    LXOR,
                    dest | (src1 << 4),
                    src2,
                    0, 0, 0
                ]);
            },
            "lnot" | "LNOT" => {
                if parts.len() != 3 {
                    return Err(format!(
                        "error on line {}: invalid argument count for LNOT instruction",
                        idx + 1
                    ));
                }
                let dest = parse_register(parts[1])
                    .map_err(|err| format!("error on line {}: {err}", idx+1))?;
                let src1 = parse_register(parts[2])
                    .map_err(|err| format!("error on line {}: {err}", idx+1))?;
                
                result.extend([
                    LNOT,
                    dest | (src1 << 4),
                    0, 0, 0, 0
                ]);
            },
            "sb" | "SB" => {
                if parts.len() != 3 {
                    return Err(format!(
                        "error on line {}: invalid argument count for SB instruction",
                        idx + 1
                    ));
                }
                let dest = parse_register(parts[1])
                    .map_err(|err| format!("error on line {}: {err}", idx+1))?;
                let src1 = parse_register(parts[2])
                    .map_err(|err| format!("error on line {}: {err}", idx+1))?;
                
                result.extend([
                    SB,
                    dest | (src1 << 4),
                    0, 0, 0, 0
                ]);
            },
            "sw" | "SW" => {
                if parts.len() != 3 {
                    return Err(format!(
                        "error on line {}: invalid argument count for SW instruction",
                        idx + 1
                    ));
                }
                let dest = parse_register(parts[1])
                    .map_err(|err| format!("error on line {}: {err}", idx+1))?;
                let src1 = parse_register(parts[2])
                    .map_err(|err| format!("error on line {}: {err}", idx+1))?;
                
                result.extend([
                    SW,
                    dest | (src1 << 4),
                    0, 0, 0, 0
                ]);
            },
            "lbs" | "LBS" => {
                if parts.len() != 3 {
                    return Err(format!(
                        "error on line {}: invalid argument count for LBS instruction",
                        idx + 1
                    ));
                }
                let dest = parse_register(parts[1])
                    .map_err(|err| format!("error on line {}: {err}", idx+1))?;
                let src1 = parse_register(parts[2])
                    .map_err(|err| format!("error on line {}: {err}", idx+1))?;
                
                result.extend([
                    LBS,
                    dest | (src1 << 4),
                    0, 0, 0, 0
                ]);
            },
            "lbu" | "LBU" => {
                if parts.len() != 3 {
                    return Err(format!(
                        "error on line {}: invalid argument count for LBU instruction",
                        idx + 1
                    ));
                }
                let dest = parse_register(parts[1])
                    .map_err(|err| format!("error on line {}: {err}", idx+1))?;
                let src1 = parse_register(parts[2])
                    .map_err(|err| format!("error on line {}: {err}", idx+1))?;
                
                result.extend([
                    LBU,
                    dest | (src1 << 4),
                    0, 0, 0, 0
                ]);
            },
            "lw" | "LW" => {
                if parts.len() != 3 {
                    return Err(format!(
                        "error on line {}: invalid argument count for LW instruction",
                        idx + 1
                    ));
                }
                let dest = parse_register(parts[1])
                    .map_err(|err| format!("error on line {}: {err}", idx+1))?;
                let src1 = parse_register(parts[2])
                    .map_err(|err| format!("error on line {}: {err}", idx+1))?;
                
                result.extend([
                    LW,
                    dest | (src1 << 4),
                    0, 0, 0, 0
                ]);
            },
            "jmp" | "JMP" => {
                if parts.len() == 2 {
                    let dest = parse_register(parts[1])
                        .map_err(|err| format!("error on line {}: {err}", idx+1))?;
                    result.extend([
                        JMP,
                        dest,
                        0, 0, 0, 0
                    ]);
                } else if parts.len() == 3 {
                    if !["rel", "relative", "REL", "RELATIVE"].contains(&parts[1]) {
                        return Err(format!(
                            "error on line {}: expected register or relative flag",
                            idx + 1
                        ));
                    }
                    let dest = parse_register(parts[2])
                        .map_err(|err| format!("error on line {}: {err}", idx+1))?;
                    result.extend([
                        JMP,
                        dest | 0x10,
                        0, 0, 0, 0,
                    ]);
                } else {
                    return Err(format!(
                        "error on line {}: invalid argument count for JMP instruction",
                        idx + 1
                    ));
                }
            },
            "brif" | "BRIF" => {
                if parts.len() == 3 {
                    let dest = parse_register(parts[1])
                        .map_err(|err| format!("error on line {}: {err}", idx+1))?;
                    let src = parse_register(parts[2])
                        .map_err(|err| format!("error on line {}: {err}", idx+1))?;
                    result.extend([
                        BRIF,
                        dest | (src << 4),
                        0, 0, 0, 0
                    ]);
                } else if parts.len() == 4 {
                    if !["rel", "relative", "REL", "RELATIVE"].contains(&parts[1]) {
                        return Err(format!(
                            "error on line {}: expected register or relative flag",
                            idx + 1
                        ));
                    }
                    let dest = parse_register(parts[2])
                        .map_err(|err| format!("error on line {}: {err}", idx+1))?;
                    let src = parse_register(parts[3])
                        .map_err(|err| format!("error on line {}: {err}", idx+1))?;
                    result.extend([
                        BRIF,
                        dest | (src << 4),
                        1, 0, 0, 0,
                    ]);
                } else {
                    return Err(format!(
                        "error on line {}: invalid argument count for BRIF instruction",
                        idx + 1
                    ));
                }
            },
            "cal" | "CAL" => {
                if parts.len() == 2 {
                    let dest = parse_register(parts[1])
                        .map_err(|err| format!("error on line {}: {err}", idx+1))?;
                    result.extend([
                        CAL,
                        dest,
                        0, 0, 0, 0
                    ]);
                } else if parts.len() == 3 {
                    if !["rel", "relative", "REL", "RELATIVE"].contains(&parts[1]) {
                        return Err(format!(
                            "error on line {}: expected register or relative flag",
                            idx + 1
                        ));
                    }
                    let dest = parse_register(parts[2])
                        .map_err(|err| format!("error on line {}: {err}", idx+1))?;
                    result.extend([
                        CAL,
                        dest | 0x10,
                        0, 0, 0, 0,
                    ]);
                } else {
                    return Err(format!(
                        "error on line {}: invalid argument count for CAL instruction",
                        idx + 1
                    ));
                }
            },
            "caif" | "CAIF" => {
                if parts.len() == 3 {
                    let dest = parse_register(parts[1])
                        .map_err(|err| format!("error on line {}: {err}", idx+1))?;
                    let src = parse_register(parts[2])
                        .map_err(|err| format!("error on line {}: {err}", idx+1))?;
                    result.extend([
                        CAIF,
                        dest | (src << 4),
                        0, 0, 0, 0
                    ]);
                } else if parts.len() == 4 {
                    if !["rel", "relative", "REL", "RELATIVE"].contains(&parts[1]) {
                        return Err(format!(
                            "error on line {}: expected register or relative flag",
                            idx + 1
                        ));
                    }
                    let dest = parse_register(parts[2])
                        .map_err(|err| format!("error on line {}: {err}", idx+1))?;
                    let src = parse_register(parts[3])
                        .map_err(|err| format!("error on line {}: {err}", idx+1))?;
                    result.extend([
                        CAIF,
                        dest | (src << 4),
                        1, 0, 0, 0,
                    ]);
                } else {
                    return Err(format!(
                        "error on line {}: invalid argument count for CAIF instruction",
                        idx + 1
                    ));
                }
            },
            "ret" | "RET" => {
                if parts.len() != 1 {
                    return Err(format!(
                        "error on line {}: invalid argument count for RET instruction",
                        idx + 1
                    ));
                }
                result.extend([RET, 0, 0, 0, 0, 0])
            },
            "eq" | "EQ" => {
                if parts.len() != 4 {
                    return Err(format!(
                        "error on line {}: invalid argument count for EQ instruction",
                        idx + 1
                    ));
                }
                let dest = parse_register(parts[1])
                    .map_err(|err| format!("error on line {}: {err}", idx+1))?;
                let src1 = parse_register(parts[2])
                    .map_err(|err| format!("error on line {}: {err}", idx+1))?;
                let src2 = parse_register(parts[3])
                    .map_err(|err| format!("error on line {}: {err}", idx+1))?;
                
                result.extend([
                    EQ,
                    dest | (src1 << 4),
                    src2,
                    0, 0, 0
                ]);
            },
            "ne" | "NE" => {
                if parts.len() != 4 {
                    return Err(format!(
                        "error on line {}: invalid argument count for NE instruction",
                        idx + 1
                    ));
                }
                let dest = parse_register(parts[1])
                    .map_err(|err| format!("error on line {}: {err}", idx+1))?;
                let src1 = parse_register(parts[2])
                    .map_err(|err| format!("error on line {}: {err}", idx+1))?;
                let src2 = parse_register(parts[3])
                    .map_err(|err| format!("error on line {}: {err}", idx+1))?;
                
                result.extend([
                    NE,
                    dest | (src1 << 4),
                    src2,
                    0, 0, 0
                ]);
            },
            "gt" | "GT" => {
                if parts.len() != 4 {
                    return Err(format!(
                        "error on line {}: invalid argument count for GT instruction",
                        idx + 1
                    ));
                }
                let dest = parse_register(parts[1])
                    .map_err(|err| format!("error on line {}: {err}", idx+1))?;
                let src1 = parse_register(parts[2])
                    .map_err(|err| format!("error on line {}: {err}", idx+1))?;
                let src2 = parse_register(parts[3])
                    .map_err(|err| format!("error on line {}: {err}", idx+1))?;
                
                result.extend([
                    GT,
                    dest | (src1 << 4),
                    src2,
                    0, 0, 0
                ]);
            },
            "lt" | "LT" => {
                if parts.len() != 4 {
                    return Err(format!(
                        "error on line {}: invalid argument count for LT instruction",
                        idx + 1
                    ));
                }
                let dest = parse_register(parts[1])
                    .map_err(|err| format!("error on line {}: {err}", idx+1))?;
                let src1 = parse_register(parts[2])
                    .map_err(|err| format!("error on line {}: {err}", idx+1))?;
                let src2 = parse_register(parts[3])
                    .map_err(|err| format!("error on line {}: {err}", idx+1))?;
                
                result.extend([
                    LT,
                    dest | (src1 << 4),
                    src2,
                    0, 0, 0
                ]);
            },
            "ge" | "GE" => {
                if parts.len() != 4 {
                    return Err(format!(
                        "error on line {}: invalid argument count for GE instruction",
                        idx + 1
                    ));
                }
                let dest = parse_register(parts[1])
                    .map_err(|err| format!("error on line {}: {err}", idx+1))?;
                let src1 = parse_register(parts[2])
                    .map_err(|err| format!("error on line {}: {err}", idx+1))?;
                let src2 = parse_register(parts[3])
                    .map_err(|err| format!("error on line {}: {err}", idx+1))?;
                
                result.extend([
                    GE,
                    dest | (src1 << 4),
                    src2,
                    0, 0, 0
                ]);
            },
            "le" | "LE" => {
                if parts.len() != 4 {
                    return Err(format!(
                        "error on line {}: invalid argument count for LE instruction",
                        idx + 1
                    ));
                }
                let dest = parse_register(parts[1])
                    .map_err(|err| format!("error on line {}: {err}", idx+1))?;
                let src1 = parse_register(parts[2])
                    .map_err(|err| format!("error on line {}: {err}", idx+1))?;
                let src2 = parse_register(parts[3])
                    .map_err(|err| format!("error on line {}: {err}", idx+1))?;
                
                result.extend([
                    LE,
                    dest | (src1 << 4),
                    src2,
                    0, 0, 0
                ]);
            },
            other if !other.ends_with(":") => return Err(format!(
                "error on line {}: unrecognized instruction: `{other}`",
                idx + 1
            )),
            _ => continue,
        }
    }

    Ok(result)
}

pub fn assemble_from_path<P: AsRef<Path>>(path: P) -> Result<String, String> {
    use std::fs;

    let source = fs::read_to_string(&path)
        .map_err(|err| err.to_string())?;
    let bytes = assemble(&source)?;
    let output = format!(
        "{}.m0bin",
        path.as_ref().file_stem().unwrap().display()
    );
    fs::write(&output, bytes)
        .map_err(|err| err.to_string())?;
    Ok(output)
}

pub fn parse_register(s: &str) -> Result<u8, String> {
    match &*s.to_lowercase() {
        "r0" => Ok(R0),
        "r1" => Ok(R1),
        "r2" => Ok(R2),
        "r3" => Ok(R3),
        "r4" => Ok(R4),
        "r5" => Ok(R5),
        "r6" => Ok(R6),
        "r7" => Ok(R7),
        "r8" => Ok(R8),
        "r9" => Ok(R9),
        "r10" | "ra" => Ok(RA),
        "r11" | "rb" => Ok(RB),
        "r12" | "rc" => Ok(RC),
        "r13" | "rd" => Ok(RD),
        "sp" => Ok(SP),
        "pc" => Ok(PC),
        _ => Err(format!("invalid register: `{s}`"))
    }
}

pub fn parse_immediate(s: &str) -> Result<u32, String> {
    let lowercase = s.to_lowercase();
    if s.starts_with("0x") {
        u32::from_str_radix(&lowercase, 16)
            .map_err(|_| format!("invalid immediate: `{s}`"))
    } else if s.starts_with("0b") {
        u32::from_str_radix(&lowercase, 2)
            .map_err(|_| format!("invalid immediate: `{s}`"))
    } else if s.starts_with("0o") {
        u32::from_str_radix(&lowercase, 8)
            .map_err(|_| format!("invalid immediate: `{s}`"))
    } else {
        u32::from_str_radix(&lowercase, 10)
            .map_err(|_| format!("invalid immediate: `{s}`"))
    }
}
use std::collections::HashMap;
use std::path::Path;
use super::registers::*;
use super::isa::*;

const REL_FLAGS: &[&str] = &["rel", "relative", "REL", "RELATIVE", "r", "R"];

pub fn assemble(source: &str) -> Result<Vec<u8>, String> {
    let mut result = Vec::new();
    let lines = source.lines()
        .enumerate()
        .collect::<Vec<_>>();

    let mut labels = HashMap::new();
    let mut current_addr: u32 = 0;
    for (_, line) in &lines {
        if line.is_empty() { continue }
        let line = line.split_terminator(";").collect::<Vec<_>>();
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

    let mut current_addr: u32 = 0;
    for (idx, line) in lines {
        if line.is_empty() { continue }
        let line = line.split_terminator(";").collect::<Vec<_>>();
        let parts = line[0].split_whitespace()
            .collect::<Vec<_>>();
        if parts.is_empty() { continue }

        assemble_parts(idx, &parts, &mut result, &labels, &mut current_addr)?;
    }

    Ok(result)
}

fn assemble_parts(idx: usize, parts: &[&str], result: &mut Vec<u8>, labels: &HashMap<&str, u32>, current_addr: &mut u32) -> Result<(), String> {
    match parts[0] {
        "nop" | "NOP" => {
            if parts.len() != 1 {
                return Err(format!(
                    "error on line {}: invalid operand count for NOP instruction",
                    idx + 1
                ));
            }
            result.extend([NOP, 0, 0, 0, 0, 0])
        },
        "hlt" | "HLT" => {
            if parts.len() != 1 {
                return Err(format!(
                    "error on line {}: invalid operand count for HLT instruction",
                    idx + 1
                ));
            }
            result.extend([HLT, 0, 0, 0, 0, 0])
        },
        "ldi" | "LDI" => {
            if parts.len() != 3 {
                return Err(format!(
                    "error on line {}: invalid operand count for LDI instruction",
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
                    "error on line {}: invalid operand count for ADD instruction",
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
                    "error on line {}: invalid operand count for SUB instruction",
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
                    "error on line {}: invalid operand count for BOR instruction",
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
                    "error on line {}: invalid operand count for BAND instruction",
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
                    "error on line {}: invalid operand count for BXOR instruction",
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
                    "error on line {}: invalid operand count for BNOY instruction",
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
                    "error on line {}: invalid operand count for LOR instruction",
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
                    "error on line {}: invalid operand count for LAND instruction",
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
                    "error on line {}: invalid operand count for LXOR instruction",
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
                    "error on line {}: invalid operand count for LNOT instruction",
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
                    "error on line {}: invalid operand count for SB instruction",
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
                    "error on line {}: invalid operand count for SW instruction",
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
                    "error on line {}: invalid operand count for LBS instruction",
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
                    "error on line {}: invalid operand count for LBU instruction",
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
                    "error on line {}: invalid operand count for LW instruction",
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
        "jmr" | "JMR" => {
            if parts.len() == 2 {
                let dest = parse_register(parts[1])
                    .map_err(|err| format!("error on line {}: {err}", idx+1))?;
                result.extend([
                    JMR,
                    dest,
                    0, 0, 0, 0
                ]);
            } else if parts.len() == 3 {
                if !REL_FLAGS.contains(&parts[1]) {
                    return Err(format!(
                        "error on line {}: expected register or relative flag",
                        idx + 1
                    ));
                }
                let dest = parse_register(parts[2])
                    .map_err(|err| format!("error on line {}: {err}", idx+1))?;
                result.extend([
                    JMR,
                    dest | 0x10,
                    0, 0, 0, 0,
                ]);
            } else {
                return Err(format!(
                    "error on line {}: invalid operand count for JMR instruction",
                    idx + 1
                ));
            }
        },
        "jri" | "JRI" => {
            if parts.len() == 3 {
                let dest = parse_register(parts[1])
                    .map_err(|err| format!("error on line {}: {err}", idx+1))?;
                let src = parse_register(parts[2])
                    .map_err(|err| format!("error on line {}: {err}", idx+1))?;
                result.extend([
                    JRI,
                    dest | (src << 4),
                    0, 0, 0, 0
                ]);
            } else if parts.len() == 4 {
                if !REL_FLAGS.contains(&parts[1]) {
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
                    JRI,
                    dest | (src << 4),
                    1, 0, 0, 0,
                ]);
            } else {
                return Err(format!(
                    "error on line {}: invalid operand count for JRI instruction",
                    idx + 1
                ));
            }
        },
        "car" | "CAR" => {
            if parts.len() == 2 {
                let dest = parse_register(parts[1])
                    .map_err(|err| format!("error on line {}: {err}", idx+1))?;
                result.extend([
                    CAR,
                    dest,
                    0, 0, 0, 0
                ]);
            } else if parts.len() == 3 {
                if !REL_FLAGS.contains(&parts[1]) {
                    return Err(format!(
                        "error on line {}: expected register or relative flag",
                        idx + 1
                    ));
                }
                let dest = parse_register(parts[2])
                    .map_err(|err| format!("error on line {}: {err}", idx+1))?;
                result.extend([
                    CAR,
                    dest | 0x10,
                    0, 0, 0, 0,
                ]);
            } else {
                return Err(format!(
                    "error on line {}: invalid operand count for CAR instruction",
                    idx + 1
                ));
            }
        },
        "cri" | "CRI" => {
            if parts.len() == 3 {
                let dest = parse_register(parts[1])
                    .map_err(|err| format!("error on line {}: {err}", idx+1))?;
                let src = parse_register(parts[2])
                    .map_err(|err| format!("error on line {}: {err}", idx+1))?;
                result.extend([
                    CRI,
                    dest | (src << 4),
                    0, 0, 0, 0
                ]);
            } else if parts.len() == 4 {
                if !REL_FLAGS.contains(&parts[1]) {
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
                    CRI,
                    dest | (src << 4),
                    1, 0, 0, 0,
                ]);
            } else {
                return Err(format!(
                    "error on line {}: invalid operand count for CRI instruction",
                    idx + 1
                ));
            }
        },
        "jmi" | "JMI" => {
            if parts.len() == 2 {
                let dest = match (|| Ok::<u32, ()>(if parts[1].starts_with("+") {
                    parse_immediate(
                        parts[1].strip_prefix("+")
                            .unwrap()
                    ).map_err(|_| ())?
                } else if parts[1].starts_with("-") {
                    -(parse_immediate(
                        parts[1].strip_prefix("-")
                            .unwrap()
                    ).map_err(|_| ())? as i32) as u32
                } else {
                    parse_immediate(parts[1])
                        .map_err(|_| ())?
                }))() {
                    Ok(d) => d,
                    Err(_) => {
                        if let Some(addr) = labels.get(parts[1]) {
                            (*addr as i32 - *current_addr as i32) as u32
                        } else {
                            return Err(format!(
                                "error on line {}: invalid jump target `{}`",
                                idx+1,
                                parts[1]
                            ));
                        }
                    }
                };
                result.extend([
                    JMI,
                    (dest & 0xFF) as u8,
                    ((dest >> 8) & 0xFF) as u8,
                    ((dest >> 16) & 0xFF) as u8,
                    ((dest >> 24) & 0xFF) as u8,
                    0
                ]);
            } else if parts.len() == 3 {
                if !REL_FLAGS.contains(&parts[1]) {
                    return Err(format!(
                        "error on line {}: expected register or relative flag",
                        idx + 1
                    ));
                }
                let dest = match (|| Ok::<u32, ()>(if parts[2].starts_with("+") {
                    parse_immediate(
                        parts[2].strip_prefix("+")
                            .unwrap()
                    ).map_err(|_| ())?
                } else if parts[2].starts_with("-") {
                    -(parse_immediate(
                        parts[2].strip_prefix("-")
                            .unwrap()
                    ).map_err(|_| ())? as i32) as u32
                } else {
                    parse_immediate(parts[2])
                        .map_err(|_| ())?
                }))() {
                    Ok(d) => d,
                    Err(_) => {
                        if let Some(addr) = labels.get(parts[2]) {
                            (*addr as i32 - *current_addr as i32) as u32
                        } else {
                            return Err(format!(
                                "error on line {}: invalid jump target `{}`",
                                idx+1,
                                parts[1]
                            ));
                        }
                    }
                };
                result.extend([
                    JMI,
                    (dest & 0xFF) as u8,
                    ((dest >> 8) & 0xFF) as u8,
                    ((dest >> 16) & 0xFF) as u8,
                    ((dest >> 24) & 0xFF) as u8,
                    1
                ]);
            } else {
                return Err(format!(
                    "error on line {}: invalid operand count for JMR instruction",
                    idx + 1
                ));
            }
        },
        "jii" | "JII" => {
            if parts.len() == 3 {
                let dest = match (|| Ok::<u32, ()>(if parts[1].starts_with("+") {
                    parse_immediate(
                        parts[1].strip_prefix("+")
                            .unwrap()
                    ).map_err(|_| ())?
                } else if parts[1].starts_with("-") {
                    -(parse_immediate(
                        parts[1].strip_prefix("-")
                            .unwrap()
                    ).map_err(|_| ())? as i32) as u32
                } else {
                    parse_immediate(parts[1])
                        .map_err(|_| ())?
                }))() {
                    Ok(d) => d,
                    Err(_) => {
                        if let Some(addr) = labels.get(parts[1]) {
                            (*addr as i32 - *current_addr as i32) as u32
                        } else {
                            return Err(format!(
                                "error on line {}: invalid jump target `{}`",
                                idx+1,
                                parts[1]
                            ));
                        }
                    }
                };
                let src = parse_register(parts[2])
                    .map_err(|err| format!("error on line {}: {err}", idx+1))?;
                result.extend([
                    JII,
                    (dest & 0xFF) as u8,
                    ((dest >> 8) & 0xFF) as u8,
                    ((dest >> 16) & 0xFF) as u8,
                    ((dest >> 24) & 0xFF) as u8,
                    src
                ]);
            } else if parts.len() == 4 {
                if !REL_FLAGS.contains(&parts[1]) {
                    return Err(format!(
                        "error on line {}: expected register or relative flag",
                        idx + 1
                    ));
                }
                let dest = match (|| Ok::<u32, ()>(if parts[2].starts_with("+") {
                    parse_immediate(
                        parts[2].strip_prefix("+")
                            .unwrap()
                    ).map_err(|_| ())?
                } else if parts[2].starts_with("-") {
                    -(parse_immediate(
                        parts[2].strip_prefix("-")
                            .unwrap()
                    ).map_err(|_| ())? as i32) as u32
                } else {
                    parse_immediate(parts[2])
                        .map_err(|_| ())?
                }))() {
                    Ok(d) => d,
                    Err(_) => {
                        if let Some(addr) = labels.get(parts[2]) {
                            (*addr as i32 - *current_addr as i32) as u32
                        } else {
                            return Err(format!(
                                "error on line {}: invalid jump target `{}`",
                                idx+1,
                                parts[1]
                            ));
                        }
                    }
                };
                let src = parse_register(parts[3])
                    .map_err(|err| format!("error on line {}: {err}", idx+1))?;
                result.extend([
                    JII,
                    (dest & 0xFF) as u8,
                    ((dest >> 8) & 0xFF) as u8,
                    ((dest >> 16) & 0xFF) as u8,
                    ((dest >> 24) & 0xFF) as u8,
                    src | 0x10
                ]);
            } else {
                return Err(format!(
                    "error on line {}: invalid operand count for JII instruction",
                    idx + 1
                ));
            }
        },
        "cai" | "CAI" => {
            if parts.len() == 2 {
                let dest = match (|| Ok::<u32, ()>(if parts[1].starts_with("+") {
                    parse_immediate(
                        parts[1].strip_prefix("+")
                            .unwrap()
                    ).map_err(|_| ())?
                } else if parts[1].starts_with("-") {
                    -(parse_immediate(
                        parts[1].strip_prefix("-")
                            .unwrap()
                    ).map_err(|_| ())? as i32) as u32
                } else {
                    parse_immediate(parts[1])
                        .map_err(|_| ())?
                }))() {
                    Ok(d) => d,
                    Err(_) => {
                        if let Some(addr) = labels.get(parts[1]) {
                            (*addr as i32 - *current_addr as i32) as u32
                        } else {
                            return Err(format!(
                                "error on line {}: invalid jump target `{}`",
                                idx+1,
                                parts[1]
                            ));
                        }
                    }
                };
                result.extend([
                    CAI,
                    (dest & 0xFF) as u8,
                    ((dest >> 8) & 0xFF) as u8,
                    ((dest >> 16) & 0xFF) as u8,
                    ((dest >> 24) & 0xFF) as u8,
                    0
                ]);
            } else if parts.len() == 3 {
                if !REL_FLAGS.contains(&parts[1]) {
                    return Err(format!(
                        "error on line {}: expected register or relative flag",
                        idx + 1
                    ));
                }
                let dest = match (|| Ok::<u32, ()>(if parts[2].starts_with("+") {
                    parse_immediate(
                        parts[2].strip_prefix("+")
                            .unwrap()
                    ).map_err(|_| ())?
                } else if parts[2].starts_with("-") {
                    -(parse_immediate(
                        parts[2].strip_prefix("-")
                            .unwrap()
                    ).map_err(|_| ())? as i32) as u32
                } else {
                    parse_immediate(parts[2])
                        .map_err(|_| ())?
                }))() {
                    Ok(d) => d,
                    Err(_) => {
                        if let Some(addr) = labels.get(parts[2]) {
                            (*addr as i32 - *current_addr as i32) as u32
                        } else {
                            return Err(format!(
                                "error on line {}: invalid jump target `{}`",
                                idx+1,
                                parts[1]
                            ));
                        }
                    }
                };
                result.extend([
                    CAI,
                    (dest & 0xFF) as u8,
                    ((dest >> 8) & 0xFF) as u8,
                    ((dest >> 16) & 0xFF) as u8,
                    ((dest >> 24) & 0xFF) as u8,
                    1
                ]);
            } else {
                return Err(format!(
                    "error on line {}: invalid operand count for CAi instruction",
                    idx + 1
                ));
            }
        },
        "cii" | "CII" => {
            if parts.len() == 3 {
                let dest = match (|| Ok::<u32, ()>(if parts[1].starts_with("+") {
                    parse_immediate(
                        parts[1].strip_prefix("+")
                            .unwrap()
                    ).map_err(|_| ())?
                } else if parts[1].starts_with("-") {
                    -(parse_immediate(
                        parts[1].strip_prefix("-")
                            .unwrap()
                    ).map_err(|_| ())? as i32) as u32
                } else {
                    parse_immediate(parts[1])
                        .map_err(|_| ())?
                }))() {
                    Ok(d) => d,
                    Err(_) => {
                        if let Some(addr) = labels.get(parts[1]) {
                            (*addr as i32 - *current_addr as i32) as u32
                        } else {
                            return Err(format!(
                                "error on line {}: invalid jump target `{}`",
                                idx+1,
                                parts[1]
                            ));
                        }
                    }
                };
                let src = parse_register(parts[2])
                    .map_err(|err| format!("error on line {}: {err}", idx+1))?;
                result.extend([
                    CII,
                    (dest & 0xFF) as u8,
                    ((dest >> 8) & 0xFF) as u8,
                    ((dest >> 16) & 0xFF) as u8,
                    ((dest >> 24) & 0xFF) as u8,
                    src
                ]);
            } else if parts.len() == 4 {
                if !REL_FLAGS.contains(&parts[1]) {
                    return Err(format!(
                        "error on line {}: expected register or relative flag",
                        idx + 1
                    ));
                }
                let dest = match (|| Ok::<u32, ()>(if parts[2].starts_with("+") {
                    parse_immediate(
                        parts[2].strip_prefix("+")
                            .unwrap()
                    ).map_err(|_| ())?
                } else if parts[2].starts_with("-") {
                    -(parse_immediate(
                        parts[2].strip_prefix("-")
                            .unwrap()
                    ).map_err(|_| ())? as i32) as u32
                } else {
                    parse_immediate(parts[2])
                        .map_err(|_| ())?
                }))() {
                    Ok(d) => d,
                    Err(_) => {
                        if let Some(addr) = labels.get(parts[2]) {
                            (*addr as i32 - *current_addr as i32) as u32
                        } else {
                            return Err(format!(
                                "error on line {}: invalid jump target `{}`",
                                idx+1,
                                parts[1]
                            ));
                        }
                    }
                };
                let src = parse_register(parts[3])
                    .map_err(|err| format!("error on line {}: {err}", idx+1))?;
                result.extend([
                    CII,
                    (dest & 0xFF) as u8,
                    ((dest >> 8) & 0xFF) as u8,
                    ((dest >> 16) & 0xFF) as u8,
                    ((dest >> 24) & 0xFF) as u8,
                    src | 0x10
                ]);
            } else {
                return Err(format!(
                    "error on line {}: invalid operand count for CII instruction",
                    idx + 1
                ));
            }
        },
        "ret" | "RET" => {
            if parts.len() != 1 {
                return Err(format!(
                    "error on line {}: invalid operand count for RET instruction",
                    idx + 1
                ));
            }
            result.extend([RET, 0, 0, 0, 0, 0])
        },
        "eq" | "EQ" => {
            if parts.len() != 4 {
                return Err(format!(
                    "error on line {}: invalid operand count for EQ instruction",
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
                    "error on line {}: invalid operand count for NE instruction",
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
                    "error on line {}: invalid operand count for GT instruction",
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
                    "error on line {}: invalid operand count for LT instruction",
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
                    "error on line {}: invalid operand count for GE instruction",
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
                    "error on line {}: invalid operand count for LE instruction",
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
        "inc" | "INC" => {
            if parts.len() != 2 {
                return Err(format!(
                    "error on line {}: invalid operand count for INC instruction",
                    idx + 1
                ));
            }
            let dest = parse_register(parts[1])
                .map_err(|err| format!("error on line {}: {err}", idx+1))?;
            
            result.extend([
                INC,
                dest,
                0, 0, 0, 0
            ]);
        },
        "dec" | "DEC" => {
            if parts.len() != 2 {
                return Err(format!(
                    "error on line {}: invalid operand count for DEC instruction",
                    idx + 1
                ));
            }
            let dest = parse_register(parts[1])
                .map_err(|err| format!("error on line {}: {err}", idx+1))?;
            
            result.extend([
                DEC,
                dest,
                0, 0, 0, 0
            ]);
        },
        "addi" | "ADDI" => {
            if parts.len() != 4 {
                return Err(format!(
                    "error on line {}: invalid operand count for ADDI instruction",
                    idx + 1
                ));
            }
            let dest = parse_register(parts[1])
                .map_err(|err| format!("error on line {}: {err}", idx+1))?;
            let src1 = parse_register(parts[2])
                .map_err(|err| format!("error on line {}: {err}", idx+1))?;
            let imm = if parts[3].starts_with("+") {
                parse_immediate(
                    parts[3].strip_prefix("+")
                        .unwrap()
                ).map_err(|err| format!("error on line {}: {err}", idx+1))?
            } else if parts[3].starts_with("-") {
                -(parse_immediate(
                    parts[3].strip_prefix("-")
                        .unwrap()
                ).map_err(|err| format!("error on line {}: {err}", idx+1))? as i32) as u32
            } else {
                parse_immediate(parts[3])
                    .map_err(|err| format!("error on line {}: {err}", idx+1))?
            }.to_le_bytes();
            
            result.extend([
                ADDI,
                dest | (src1 << 4),
                imm[0], imm[1], imm[2], imm[3]
            ]);
        },
        "subi" | "SUBI" => {
            if parts.len() != 4 {
                return Err(format!(
                    "error on line {}: invalid operand count for SUBI instruction",
                    idx + 1
                ));
            }
            let dest = parse_register(parts[1])
                .map_err(|err| format!("error on line {}: {err}", idx+1))?;
            let src1 = parse_register(parts[2])
                .map_err(|err| format!("error on line {}: {err}", idx+1))?;
            let imm = if parts[3].starts_with("+") {
                parse_immediate(
                    parts[3].strip_prefix("+")
                        .unwrap()
                ).map_err(|err| format!("error on line {}: {err}", idx+1))?
            } else if parts[3].starts_with("-") {
                -(parse_immediate(
                    parts[3].strip_prefix("-")
                        .unwrap()
                ).map_err(|err| format!("error on line {}: {err}", idx+1))? as i32) as u32
            } else {
                parse_immediate(parts[3])
                    .map_err(|err| format!("error on line {}: {err}", idx+1))?
            }.to_le_bytes();
            
            result.extend([
                SUBI,
                dest | (src1 << 4),
                imm[0], imm[1], imm[2], imm[3]
            ]);
        },
        "shl" | "SHL" => {
            if parts.len() != 4 {
                return Err(format!(
                    "error on line {}: invalid operand count for SHL instruction",
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
                SHL,
                dest | (src1 << 4),
                src2,
                0, 0, 0
            ]);
        },
        "lshr" | "LSHR" => {
            if parts.len() != 4 {
                return Err(format!(
                    "error on line {}: invalid operand count for LSHR instruction",
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
                LSHR,
                dest | (src1 << 4),
                src2,
                0, 0, 0
            ]);
        },
        "ashr" | "ASHR" => {
            if parts.len() != 4 {
                return Err(format!(
                    "error on line {}: invalid operand count for ASHR instruction",
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
                ASHR,
                dest | (src1 << 4),
                src2,
                0, 0, 0
            ]);
        },
        "rotl" | "ROTL" => {
            if parts.len() != 4 {
                return Err(format!(
                    "error on line {}: invalid operand count for ROTL instruction",
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
                ROTL,
                dest | (src1 << 4),
                src2,
                0, 0, 0
            ]);
        },
        "rotr" | "ROTR" => {
            if parts.len() != 4 {
                return Err(format!(
                    "error on line {}: invalid operand count for ROTR instruction",
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
                ROTR,
                dest | (src1 << 4),
                src2,
                0, 0, 0
            ]);
        },
        "pb" | "PB" => {
            if parts.len() != 2 {
                return Err(format!(
                    "error on line {}: invalid operand count for PB instruction",
                    idx + 1
                ));
            }
            let src = parse_register(parts[1])
                .map_err(|err| format!("error on line {}: {err}", idx+1))?;

            result.extend([
                PB,
                src,
                0, 0, 0, 0
            ]);
        },
        "pw" | "PW" => {
            if parts.len() != 2 {
                return Err(format!(
                    "error on line {}: invalid operand count for PW instruction",
                    idx + 1
                ));
            }
            let src = parse_register(parts[1])
                .map_err(|err| format!("error on line {}: {err}", idx+1))?;

            result.extend([
                PW,
                src,
                0, 0, 0, 0
            ]);
        },
        "pobs" | "POBS" => {
            if parts.len() != 2 {
                return Err(format!(
                    "error on line {}: invalid operand count for POBS instruction",
                    idx + 1
                ));
            }
            let dest = parse_register(parts[1])
                .map_err(|err| format!("error on line {}: {err}", idx+1))?;

            result.extend([
                POBS,
                dest,
                0, 0, 0, 0
            ]);
        },
        "pobu" | "POBU" => {
            if parts.len() != 2 {
                return Err(format!(
                    "error on line {}: invalid operand count for POBU instruction",
                    idx + 1
                ));
            }
            let dest = parse_register(parts[1])
                .map_err(|err| format!("error on line {}: {err}", idx+1))?;

            result.extend([
                POBU,
                dest,
                0, 0, 0, 0
            ]);
        },
        "pow" | "POW" => {
            if parts.len() != 2 {
                return Err(format!(
                    "error on line {}: invalid operand count for POW instruction",
                    idx + 1
                ));
            }
            let dest = parse_register(parts[1])
                .map_err(|err| format!("error on line {}: {err}", idx+1))?;

            result.extend([
                POW,
                dest,
                0, 0, 0, 0
            ]);
        },
        "mul" | "MUL" => {
            if parts.len() != 4 {
                return Err(format!(
                    "error on line {}: invalid operand count for MUL instruction",
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
                MUL,
                dest | (src1 << 4),
                src2,
                0, 0, 0
            ]);
        },
        "div" | "DIV" => {
            if parts.len() != 4 {
                return Err(format!(
                    "error on line {}: invalid operand count for DIV instruction",
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
                DIV,
                dest | (src1 << 4),
                src2,
                0, 0, 0
            ]);
        },
        "rem" | "REM" => {
            if parts.len() != 4 {
                return Err(format!(
                    "error on line {}: invalid operand count for REM instruction",
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
                REM,
                dest | (src1 << 4),
                src2,
                0, 0, 0
            ]);
        },
        "fadd" | "FADD" => {
            if parts.len() != 4 {
                return Err(format!(
                    "error on line {}: invalid operand count for FADD instruction",
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
                FADD,
                dest | (src1 << 4),
                src2,
                0, 0, 0
            ]);
        },
        "fsub" | "FSUB" => {
            if parts.len() != 4 {
                return Err(format!(
                    "error on line {}: invalid operand count for FSUB instruction",
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
                FSUB,
                dest | (src1 << 4),
                src2,
                0, 0, 0
            ]);
        },
        "fmul" | "FMUL" => {
            if parts.len() != 4 {
                return Err(format!(
                    "error on line {}: invalid operand count for FMUL instruction",
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
                FMUL,
                dest | (src1 << 4),
                src2,
                0, 0, 0
            ]);
        },
        "fdiv" | "FDIV" => {
            if parts.len() != 4 {
                return Err(format!(
                    "error on line {}: invalid operand count for FDIV instruction",
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
                FDIV,
                dest | (src1 << 4),
                src2,
                0, 0, 0
            ]);
        },
        "frem" | "FREM" => {
            if parts.len() != 4 {
                return Err(format!(
                    "error on line {}: invalid operand count for FREM instruction",
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
                FREM,
                dest | (src1 << 4),
                src2,
                0, 0, 0
            ]);
        },
        "muhs" | "MUHS" => {
            if parts.len() != 4 {
                return Err(format!(
                    "error on line {}: invalid operand count for MUHS instruction",
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
                MUHS,
                dest | (src1 << 4),
                src2,
                0, 0, 0
            ]);
        },
        "muhu" | "MUHU" => {
            if parts.len() != 4 {
                return Err(format!(
                    "error on line {}: invalid operand count for MUHU instruction",
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
                MUHU,
                dest | (src1 << 4),
                src2,
                0, 0, 0
            ]);
        },
        other if other.ends_with(":") => if parts.len() != 1 {
            assemble_parts(idx, &parts[1..], result, labels, current_addr)?;
            return Ok(());
        } else {
            return Ok(());
        },
        other => return Err(format!(
            "error on line {}: unrecognized instruction: `{other}`",
            idx + 1
        )),
    }

    *current_addr += 6;

    Ok(())
}

pub fn assemble_from_path<P: AsRef<Path>>(path: P) -> Result<String, String> {
    use std::fs;

    let source = fs::read_to_string(&path)
        .map_err(|err| err.to_string())?;
    let bytes = assemble(&source)?;
    let output = format!(
        "{}.bin",
        path.as_ref().file_stem().unwrap().display()
    );
    fs::write(&output, bytes)
        .map_err(|err| err.to_string())?;
    Ok(output)
}

pub fn parse_register(s: &str) -> Result<u8, String> {
    match &*s.to_lowercase() {
        "rds" => Ok(RDS),
        "gr0" => Ok(GR0),
        "gr1" => Ok(GR1),
        "gr2" => Ok(GR2),
        "gr3" => Ok(GR3),
        "gr4" => Ok(GR4),
        "gr5" => Ok(GR5),
        "gr6" => Ok(GR6),
        "gr7" => Ok(GR7),
        "gr8" => Ok(GR8),
        "gr9" => Ok(GR9),
        "gr10" | "gra" => Ok(GRA),
        "gr11" | "grb" => Ok(GRB),
        "rsp" => Ok(RSP),
        "csp" => Ok(CSP),
        "rpc" => Ok(RPC),
        _ => Err(format!("invalid register: `{s}`"))
    }
}

pub fn parse_immediate(s: &str) -> Result<u32, String> {
    let lowercase = s.to_lowercase();
    if let Ok(f) = s.parse::<f32>() {
        Ok(f.to_bits())
    } else if s.starts_with("0x") {
        u32::from_str_radix(&lowercase[2..], 16)
            .map_err(|_| format!("invalid immediate: `{s}`"))
    } else if s.starts_with("0b") {
        u32::from_str_radix(&lowercase[2..], 2)
            .map_err(|_| format!("invalid immediate: `{s}`"))
    } else if s.starts_with("0o") {
        u32::from_str_radix(&lowercase[2..], 8)
            .map_err(|_| format!("invalid immediate: `{s}`"))
    } else if s.starts_with("'") && s.starts_with("'") {
        let c = s[1..s.len()-1].chars().collect::<Vec<_>>();
        if c.is_empty() {
            return Err(format!("invalid immediate: `{s}`"));
        }
        match c[0] {
            '\\' if c.len() == 2 => {
                Ok(match c[1] {
                    'n' => '\n',
                    '\\' => '\\',
                    't' => '\t',
                    'r' => '\r',
                    '\'' => '\'',
                    _ => return Err(format!("invalid immediate: `{s}`"))
                } as u32)
            },
            ch if c.len() == 1 => Ok(ch as u32),
            _ => Err(format!("invalid immediate: `{s}`"))
        }
    } else {
        u32::from_str_radix(&lowercase, 10)
            .map_err(|_| format!("invalid immediate: `{s}`"))
    }
}
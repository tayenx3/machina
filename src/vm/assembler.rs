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
                    "error on line {}: invalid argument count for JMR instruction",
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
                    "error on line {}: invalid argument count for JRI instruction",
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
                    "error on line {}: invalid argument count for CAR instruction",
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
                    "error on line {}: invalid argument count for CRI instruction",
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
                            (*addr as i32 - *current_addr as i32 + 6) as u32
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
                            (*addr as i32 - *current_addr as i32 + 6) as u32
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
                    "error on line {}: invalid argument count for JMR instruction",
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
                            (*addr as i32 - *current_addr as i32 + 6) as u32
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
                            (*addr as i32 - *current_addr as i32 + 6) as u32
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
                    "error on line {}: invalid argument count for JII instruction",
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
                            (*addr as i32 - *current_addr as i32 + 6) as u32
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
                            (*addr as i32 - *current_addr as i32 + 6) as u32
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
                    "error on line {}: invalid argument count for CAi instruction",
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
                            (*addr as i32 - *current_addr as i32 + 6) as u32
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
                            (*addr as i32 - *current_addr as i32 + 6) as u32
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
                    "error on line {}: invalid argument count for CII instruction",
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
        other if other.ends_with(":") => if parts.len() != 1 {
            assemble_parts(idx, &parts[1..], result, labels, current_addr)?;
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
        "{}.m0bin",
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
#![feature(iter_intersperse)]

use std::path::PathBuf;
use tools::{dsp::InputState, dspemit::Emitter};

fn main() -> anyhow::Result<()> {
    let output = PathBuf::from("../tests/dsp/shared.h");

    let mut e = Emitter::default();
    tools::dsp::test_prologue(&mut e, InputState::random());
    let prologue_len = e.len();
    e.clear();

    let mut prologues: Vec<Vec<u16>> = vec![];
    let mut inputs: Vec<[u16; 32]> = vec![];
    for _ in 0..tools::dsp::GEN_DSP_INPUTS {
        let input_state = InputState::random();
        tools::dsp::test_prologue(&mut e, input_state.clone());
        prologues.push(e.drain(e.len()));
        inputs.push(input_state.regs);
    }

    tools::dsp::test_epilogue(&mut e);
    let epilogue = e.drain(e.len());

    tools::dsp::block_start(&mut e);
    let block_start = e.drain(e.len());

    tools::dsp::block_end(&mut e);
    let block_end = e.drain(e.len());

    std::fs::write(
        output,
        format!(
            "\
                #include <stdint.h>\n\
                \
                uint16_t test_input[][32] = {{\n\
                \t{inputs}\n\
                }};\n\
                \
                uint16_t test_prologue[][{prologue_len}] = {{\n\
                \t{prologue}\n\
                }};\n\
                \
                uint16_t test_epilogue[] = {{\n\
                \t{epilogue}\n\
                }};\n\
                \
                uint16_t block_start[] = {{\n\
                \t{block_start}\n\
                }};\n\
                \
                uint16_t block_end[] = {{\n\
                \t{block_end}\n\
                }};\n\
            ",
            prologue = {
                prologues
                    .iter()
                    .map(|v| {
                        format!(
                            "{{ {} }}",
                            v.iter()
                                .map(|&v| format!("{v:#x}"))
                                .intersperse(",".to_string())
                                .collect::<String>()
                        )
                    })
                    .intersperse(",".to_string())
                    .collect::<String>()
            },
            inputs = {
                inputs
                    .iter()
                    .cloned()
                    .map(|i| {
                        format!(
                            "{{{}}}",
                            i.iter()
                                .cloned()
                                .map(|v| format!("{v:#x}"))
                                .intersperse(",".to_string())
                                .collect::<String>()
                        )
                    })
                    .intersperse(",".to_string())
                    .collect::<String>()
            },
            epilogue = {
                epilogue
                    .iter()
                    .map(|v| format!("{v:#x}"))
                    .intersperse(",".to_string())
                    .collect::<String>()
            },
            block_start = {
                block_start
                    .iter()
                    .map(|v| format!("{v:#x}"))
                    .intersperse(",".to_string())
                    .collect::<String>()
            },
            block_end = {
                block_end
                    .iter()
                    .map(|v| format!("{v:#x}"))
                    .intersperse(",".to_string())
                    .collect::<String>()
            },
        ),
    )?;
    Ok(())
}

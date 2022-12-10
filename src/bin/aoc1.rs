fn main() {
    compute_max_elf("input/01/01.test").unwrap_or(());
    compute_max_elf("input/01/01").unwrap_or(());
}

fn compute_max_elf(input_file: &str) -> Result<(), Box<dyn std::error::Error>> {
    let text = std::fs::read_to_string(input_file)?;
    let rations: Vec<u32> = text
        .split("\n\n")
        .map(|a_elf_ration| {
            a_elf_ration
                .split("\n")
                .map(|i| i.parse::<u32>())
                .flatten()
                .sum::<u32>()
        })
        .collect();
    let max = rations.iter().max().copied().unwrap_or_default();
    let top_three = rations.iter().fold((0, 0, 0), |(top1, top2, top3), next| {
        if top1 < *next {
            (*next, top1, top2)
        } else if top2 < *next {
            (top1, *next, top2)
        } else if top3 < *next {
            (top1, top2, *next)
        } else {
            (top1, top2, top3)
        }
    });
    println!("for {} max output = {:?}", input_file, max);
    println!(
        "top_three {:?} top-three sum = {:?}",
        top_three,
        top_three.0 + top_three.1 + top_three.2
    );
    Ok(())
}

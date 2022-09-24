use win_partitions::windows_partitions::get_partitions;

fn main() {
    let list = get_partitions();
    for i in list.unwrap() {
        println!("Drive {} ({})", i.letter, i.name);
        println!("Ready: {}", i.ready);
        println!("File System: {}", i.file_system_name);
        println!("Free Space: {} / {} Bytes", i.free_space, i.size);

        println!();
    }
}
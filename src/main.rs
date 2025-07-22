use user_data_structure_collection::*;

fn main() {
    loop {
        print_menu();
        let selected_value = get_user_option();
        let option = menu_option_from_i8(selected_value);

        match option {
            MenuOption::HashMap => build_hashmap(),
            MenuOption::BTreeMap => build_btreemap(),
            MenuOption::BTreeSet => build_btreeset(),
            MenuOption::BinaryHeap => build_binaryheap(),
            MenuOption::Exit => {
                println!("👋 Exiting... Goodbye!");
                break;
            }
            MenuOption::Invalid => {
                println!("⚠️ Invalid option. Please enter a number between 1 and 5.");
            }
        }

        println!("\n------------------------------\n");
    }
}

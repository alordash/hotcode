use hotcode::*;
use lib_bin_together_release::*;
use std::sync::Arc;

fn main() -> Result<(), std::io::Error> {
    let old_values = get_values();
    let expected_old_values = [
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
        26, 27, 28, 29, 30, 31,
    ];
    dbg!(old_values);
    assert_eq!(old_values, expected_old_values);

    let old_library = Arc::downgrade(&provide_library_wrapper(&get_platform_library_file_name(
        "lib_bin_together_release",
    )));

    println!();
    let _sync = std::io::stdin().read_line(&mut String::new())?;

    // Wait for old library to be updated and unloaded
    while old_library.strong_count() > 0 {
        std::hint::spin_loop();
    }

    let new_values = get_values();
    dbg!(new_values);
    assert_eq!(new_values, expected_old_values);

    Ok(())
}

fn get_values() -> [i32; 31] {
    [
        standalone_get(),
        Struct.get(),
        Struct.mut_get(),
        Struct.consume_get(),
        Box::new(Struct).box_get(),
        Struct::static_get(),
        Struct.trait_get(),
        Struct.trait_mut_get(),
        Struct.trait_consume_get(),
        Box::new(Struct).trait_box_get(),
        Struct::trait_static_get(),
        Enum::Variant.get(),
        Enum::Variant.mut_get(),
        Enum::Variant.consume_get(),
        Box::new(Enum::Variant).box_get(),
        Enum::static_get(),
        Enum::Variant.trait_get(),
        Enum::Variant.trait_mut_get(),
        Enum::Variant.trait_consume_get(),
        Box::new(Enum::Variant).trait_box_get(),
        Enum::trait_static_get(),
        Union::new().get(),
        Union::new().mut_get(),
        Union::new().consume_get(),
        Box::new(Union::new()).box_get(),
        Union::static_get(),
        Union::new().trait_get(),
        Union::new().trait_mut_get(),
        Union::new().trait_consume_get(),
        Box::new(Union::new()).trait_box_get(),
        Union::trait_static_get(),
    ]
}

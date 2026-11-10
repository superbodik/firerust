use fr_layout::table::distribute_columns::distribute_columns;

fn close(left: &[f32], right: &[f32]) -> bool {
    left.len() == right.len() && left.iter().zip(right).all(|(a, b)| (a - b).abs() < 0.001)
}

#[test]
fn a_roomy_table_keeps_the_widest_widths_when_it_does_not_grow() {
    let widths = distribute_columns(&[10.0, 20.0], &[50.0, 100.0], 500.0, false);

    assert!(close(&widths, &[50.0, 100.0]));
}

#[test]
fn a_roomy_table_scales_up_when_it_grows_to_its_target() {
    let widths = distribute_columns(&[10.0, 20.0], &[50.0, 100.0], 300.0, true);

    assert!(close(&widths, &[100.0, 200.0]));
}

#[test]
fn a_tight_table_gives_each_column_a_share_of_the_flexible_range() {
    let widths = distribute_columns(&[10.0, 20.0], &[50.0, 100.0], 105.0, false);

    assert!(close(&widths, &[35.0, 70.0]));
    assert!((widths.iter().sum::<f32>() - 105.0).abs() < 0.001);
}

#[test]
fn a_very_narrow_table_shrinks_below_the_minimum_widths_proportionally() {
    let widths = distribute_columns(&[40.0, 60.0], &[80.0, 120.0], 50.0, false);

    assert!(close(&widths, &[20.0, 30.0]));
}

#[test]
fn equal_minimum_and_maximum_widths_stay_fixed() {
    let widths = distribute_columns(&[30.0, 30.0], &[30.0, 30.0], 60.0, false);

    assert!(close(&widths, &[30.0, 30.0]));
}

#[test]
fn no_columns_give_no_widths() {
    assert!(distribute_columns(&[], &[], 100.0, true).is_empty());
}

#[test]
fn all_zero_columns_stay_zero() {
    let widths = distribute_columns(&[0.0, 0.0], &[0.0, 0.0], 100.0, true);

    assert!(close(&widths, &[0.0, 0.0]));
}

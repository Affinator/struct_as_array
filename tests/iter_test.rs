use struct_as_array::*;

#[derive(AsArray)]
struct TestStruct {
    t1: i32,
    t2: i32,
    t3: i32,
}

#[test]
fn iter_test() {
    let t = TestStruct {
        t1: 0,
        t2: 1,
        t3: 2,
    };

    for i in t.as_array() {
        println!("{}", i);
    }

    assert_eq!(t.as_array(), [&0, &1, &2]);
}
